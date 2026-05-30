#!/usr/bin/env python3
"""Aggregate public RSS feeds into a static situation-monitor snapshot."""

from __future__ import annotations

import hashlib
import html
import json
import re
import sys
from datetime import datetime, timezone
from email.utils import parsedate_to_datetime
from pathlib import Path
from typing import Any
from urllib.request import Request, urlopen

sys.path.insert(0, str(Path(__file__).resolve().parent))

try:
    import feedparser
except ImportError:
    print("feedparser is required: pip install feedparser", file=sys.stderr)
    raise

from x_list_client import fetch_list_timeline, load_credentials

REPO_ROOT = Path(__file__).resolve().parent.parent
OUTPUT = REPO_ROOT / "static" / "situation-monitor" / "feed.json"
USER_AGENT = (
    "Mozilla/5.0 (compatible; SituationMonitor/1.0; "
    "+https://github.com/saptreekly/Research-Publications)"
)
MAX_ITEMS_PER_SOURCE = 15
MAX_TOTAL_ITEMS = 160
MAX_X_LIST_ITEMS = 25

FEEDS: list[dict[str, str]] = [
    {
        "id": "rnz-news",
        "name": "RNZ News",
        "category": "nz-pacific",
        "url": "https://www.rnz.co.nz/rss/news.xml",
    },
    {
        "id": "rnz-pacific",
        "name": "RNZ Pacific",
        "category": "nz-pacific",
        "url": "https://www.rnz.co.nz/rss/pacific.xml",
    },
    {
        "id": "rnz-national",
        "name": "RNZ National",
        "category": "nz-pacific",
        "url": "https://www.rnz.co.nz/rss/national.xml",
    },
    {
        "id": "bbc-asia",
        "name": "BBC Asia",
        "category": "apac-security",
        "url": "https://feeds.bbci.co.uk/news/world/asia/rss.xml",
    },
    {
        "id": "diplomat",
        "name": "The Diplomat",
        "category": "apac-security",
        "url": "https://thediplomat.com/feed/",
    },
    {
        "id": "scmp",
        "name": "South China Morning Post",
        "category": "apac-security",
        "url": "https://www.scmp.com/rss/91/feed",
    },
    {
        "id": "usni",
        "name": "USNI News",
        "category": "apac-security",
        "url": "https://news.usni.org/feed",
    },
    {
        "id": "bbc-world",
        "name": "BBC World",
        "category": "global",
        "url": "https://feeds.bbci.co.uk/news/world/rss.xml",
    },
    {
        "id": "defense-one",
        "name": "Defense One",
        "category": "global",
        "url": "https://www.defenseone.com/rss/all/",
    },
    {
        "id": "cisa",
        "name": "CISA Advisories",
        "category": "cyber",
        "url": "https://www.cisa.gov/cybersecurity-advisories/all.xml",
    },
    {
        "id": "the-record",
        "name": "The Record",
        "category": "cyber",
        "url": "https://therecord.media/feed/",
    },
    {
        "id": "krebs",
        "name": "Krebs on Security",
        "category": "cyber",
        "url": "https://krebsonsecurity.com/feed/",
    },
    {
        "id": "lowy-interpreter",
        "name": "Lowy Interpreter",
        "category": "apac-security",
        "url": "https://www.lowyinstitute.org/the-interpreter/rss.xml",
    },
    {
        "id": "nikkei-asia",
        "name": "Nikkei Asia",
        "category": "apac-security",
        "url": "https://asia.nikkei.com/rss/feed/nar",
    },
    {
        "id": "war-on-the-rocks",
        "name": "War on the Rocks",
        "category": "apac-security",
        "url": "https://warontherocks.com/feed/",
    },
]

X_LISTS: list[dict[str, str]] = [
    {
        "id": "x-osint-list",
        "name": "OSINT Watchlist",
        "category": "osint",
        "list_id": "1978231089639690329",
        "url": "https://x.com/i/lists/1978231089639690329",
    },
]

CATEGORY_LABELS = {
    "nz-pacific": "NZ & Pacific",
    "apac-security": "APAC Security",
    "cyber": "Cyber",
    "global": "Global",
    "osint": "OSINT",
}

REGION_KEYWORDS: dict[str, list[str]] = {
    "nz": ["new zealand", "wellington", "auckland", "christchurch", "nzdf", "rnz"],
    "pacific": [
        "pacific",
        "fiji",
        "samoa",
        "tonga",
        "vanuatu",
        "papua",
        "solomon",
        "micronesia",
        "polynesia",
        "guam",
    ],
    "australia": ["australia", "sydney", "canberra", "melbourne", "australian"],
    "china": ["china", "beijing", "shanghai", "chinese", "prc", "south china sea"],
    "taiwan": ["taiwan", "taipei", "strait"],
    "japan": ["japan", "tokyo", "japanese", "okinawa"],
    "korea": ["korea", "seoul", "pyongyang", "dprk"],
    "se-asia": [
        "indonesia",
        "malaysia",
        "singapore",
        "philippines",
        "vietnam",
        "thailand",
        "asean",
        "myanmar",
        "cambodia",
    ],
    "india": ["india", "delhi", "mumbai", "indian", "modi"],
    "middle-east": [
        "iran",
        "israel",
        "gaza",
        "syria",
        "yemen",
        "saudi",
        "middle east",
        "red sea",
        "houthi",
    ],
    "europe": ["europe", "ukraine", "nato", "european", "london", "berlin", "france", "eu "],
    "us": ["united states", "u.s.", "washington", "pentagon", "american", "white house"],
    "africa": ["africa", "sudan", "sahel", "nigeria", "congo"],
    "russia": ["russia", "moscow", "kremlin", "putin"],
}

PRIORITY_KEYWORDS = [
    "cve-",
    "zero-day",
    "zero day",
    "breaking",
    "missile",
    "sanction",
    "cyber attack",
    "earthquake",
    "nuclear",
    "invasion",
    "coup",
    "kinetic",
    "critical",
    "emergency",
]

TREND_STOPWORDS = {
    "about",
    "after",
    "from",
    "into",
    "more",
    "news",
    "over",
    "says",
    "that",
    "their",
    "there",
    "this",
    "under",
    "will",
    "with",
    "have",
    "been",
    "than",
    "they",
    "what",
    "when",
    "your",
}


def infer_regions(title: str, summary: str, category: str) -> list[str]:
    text = f"{title} {summary}".lower()
    regions = [
        region for region, keywords in REGION_KEYWORDS.items() if any(key in text for key in keywords)
    ]
    if category == "nz-pacific" and not any(r in regions for r in ("nz", "pacific", "australia")):
        regions.append("pacific")
    if category == "apac-security" and not regions:
        regions.append("se-asia")
    return regions or ["global"]


def priority_score(title: str, summary: str, category: str, published: datetime | None) -> int:
    text = f"{title} {summary}".lower()
    score = 0
    if category == "cyber":
        score += 12
    if category == "osint":
        score += 8
    score += sum(18 for keyword in PRIORITY_KEYWORDS if keyword in text)
    if published:
        age_hours = (datetime.now(timezone.utc) - published).total_seconds() / 3600
        if age_hours < 1:
            score += 30
        elif age_hours < 6:
            score += 20
        elif age_hours < 24:
            score += 10
    return min(score, 100)


def extract_trends(items: list[dict[str, Any]], limit: int = 10) -> list[dict[str, Any]]:
    counts: dict[str, int] = {}
    for item in items:
        words = re.findall(r"[a-z0-9][a-z0-9-]{3,}", item["title"].lower())
        for word in words:
            if word in TREND_STOPWORDS:
                continue
            counts[word] = counts.get(word, 0) + 1
    ranked = sorted(counts.items(), key=lambda pair: (-pair[1], pair[0]))
    return [{"term": term, "count": count} for term, count in ranked[:limit]]


def clean_text(value: str | None, limit: int = 280) -> str:
    if not value:
        return ""
    text = html.unescape(value)
    text = re.sub(r"<[^>]+>", " ", text)
    text = re.sub(r"\s+", " ", text).strip()
    if len(text) > limit:
        return text[: limit - 1].rstrip() + "…"
    return text


def parse_published(entry: dict[str, Any]) -> datetime | None:
    for key in ("published_parsed", "updated_parsed"):
        parsed = entry.get(key)
        if parsed:
            try:
                return datetime(*parsed[:6], tzinfo=timezone.utc)
            except (TypeError, ValueError):
                pass

    for key in ("published", "updated"):
        raw = entry.get(key)
        if not raw:
            continue
        try:
            dt = parsedate_to_datetime(raw)
            if dt.tzinfo is None:
                dt = dt.replace(tzinfo=timezone.utc)
            return dt.astimezone(timezone.utc)
        except (TypeError, ValueError):
            continue
    return None


def format_label(dt: datetime) -> str:
    return dt.strftime("%d %b %Y · %H:%MZ")


def age_label(dt: datetime) -> str:
    delta = datetime.now(timezone.utc) - dt
    seconds = max(int(delta.total_seconds()), 0)
    if seconds < 60:
        return "just now"
    if seconds < 3600:
        return f"{seconds // 60}m ago"
    if seconds < 86400:
        return f"{seconds // 3600}h ago"
    if seconds < 604_800:
        return f"{seconds // 86400}d ago"
    return format_label(dt)


def normalize_title_key(title: str) -> str:
    text = re.sub(r"[^\w\s]", "", title.lower())
    return " ".join(text.split())[:96]


def dedupe_items(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
    seen: set[str] = set()
    deduped: list[dict[str, Any]] = []
    for item in items:
        key = normalize_title_key(item["title"])
        if not key or key in seen:
            continue
        seen.add(key)
        deduped.append(item)
    return deduped


def item_id(source_id: str, link: str, title: str) -> str:
    digest = hashlib.sha1(f"{source_id}|{link}|{title}".encode("utf-8")).hexdigest()
    return digest[:16]


def fetch_feed(feed: dict[str, str]) -> tuple[list[dict[str, Any]], str | None]:
    request = Request(feed["url"], headers={"User-Agent": USER_AGENT})
    try:
        with urlopen(request, timeout=20) as response:
            body = response.read()
    except Exception as exc:  # noqa: BLE001 - report per-source failures
        return [], str(exc)

    parsed = feedparser.parse(body)
    items: list[dict[str, Any]] = []

    for entry in parsed.entries[:MAX_ITEMS_PER_SOURCE]:
        title = clean_text(entry.get("title"), limit=160)
        link = entry.get("link") or entry.get("id") or ""
        if not title or not link:
            continue

        published = parse_published(entry)
        published_at = published.isoformat().replace("+00:00", "Z") if published else None
        summary = clean_text(entry.get("summary") or entry.get("description"))
        regions = infer_regions(title, summary, feed["category"])
        cluster_key = normalize_title_key(title)

        items.append(
            {
                "id": item_id(feed["id"], link, title),
                "title": title,
                "summary": summary,
                "url": link,
                "published_at": published_at,
                "published_label": format_label(published) if published else "Unknown",
                "age_label": age_label(published) if published else "Unknown",
                "source_id": feed["id"],
                "source_name": feed["name"],
                "source_kind": "rss",
                "category": feed["category"],
                "regions": regions,
                "priority": priority_score(title, summary, feed["category"], published),
                "cluster_key": cluster_key,
            }
        )

    return items, None


def fetch_x_list(source: dict[str, str]) -> tuple[list[dict[str, Any]], str | None]:
    credentials = load_credentials()
    if credentials is None:
        return [], "Set AUTH_TOKEN and CT0 to enable X list ingestion"

    auth_token, ct0 = credentials
    try:
        tweets = fetch_list_timeline(
            source["list_id"],
            auth_token=auth_token,
            ct0=ct0,
            count=MAX_X_LIST_ITEMS,
        )
    except Exception as exc:  # noqa: BLE001 - report per-source failures
        return [], str(exc)

    items: list[dict[str, Any]] = []
    for tweet in tweets[:MAX_X_LIST_ITEMS]:
        text = clean_text(tweet.get("text"), limit=280)
        if not text:
            continue

        published = _parse_created_at(tweet.get("created_at"))
        published_at = published.isoformat().replace("+00:00", "Z") if published else None
        screen_name = tweet.get("screen_name") or "unknown"
        link = tweet.get("url") or source["url"]
        headline = clean_text(tweet.get("text"), limit=160)
        regions = infer_regions(headline, text, source["category"])
        cluster_key = normalize_title_key(headline)

        items.append(
            {
                "id": item_id(source["id"], link, headline),
                "title": headline,
                "summary": text if text != headline else f"Post from @{screen_name}",
                "url": link,
                "published_at": published_at,
                "published_label": format_label(published) if published else "Unknown",
                "age_label": age_label(published) if published else "Unknown",
                "source_id": source["id"],
                "source_name": f"@{screen_name}",
                "source_kind": "social",
                "category": source["category"],
                "regions": regions,
                "priority": priority_score(headline, text, source["category"], published),
                "cluster_key": cluster_key,
            }
        )

    return items, None


def _parse_created_at(raw: str | None) -> datetime | None:
    if not raw:
        return None
    try:
        dt = parsedate_to_datetime(raw)
        if dt.tzinfo is None:
            dt = dt.replace(tzinfo=timezone.utc)
        return dt.astimezone(timezone.utc)
    except (TypeError, ValueError):
        return None


def aggregate() -> dict[str, Any]:
    all_items: list[dict[str, Any]] = []
    source_status: list[dict[str, Any]] = []

    for feed in FEEDS:
        items, error = fetch_feed(feed)
        source_status.append(
            {
                "id": feed["id"],
                "name": feed["name"],
                "category": feed["category"],
                "url": feed["url"],
                "item_count": len(items),
                "status": "ok" if error is None else "error",
                "error": error,
            }
        )
        all_items.extend(items)

    for source in X_LISTS:
        items, error = fetch_x_list(source)
        source_status.append(
            {
                "id": source["id"],
                "name": source["name"],
                "category": source["category"],
                "url": source["url"],
                "item_count": len(items),
                "status": "ok" if error is None else "error",
                "error": error,
            }
        )
        all_items.extend(items)

    all_items.sort(
        key=lambda item: item.get("published_at") or "",
        reverse=True,
    )
    all_items = dedupe_items(all_items)[:MAX_TOTAL_ITEMS]

    category_counts = {key: 0 for key in CATEGORY_LABELS}
    for item in all_items:
        category_counts[item["category"]] = category_counts.get(item["category"], 0) + 1

    return {
        "updated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%MZ"),
        "updated_label": datetime.now(timezone.utc).strftime("%d %b %Y · %H:%MZ"),
        "categories": [
            {"id": key, "label": label, "count": category_counts.get(key, 0)}
            for key, label in CATEGORY_LABELS.items()
        ],
        "sources": source_status,
        "items": all_items,
        "trends": extract_trends(all_items),
    }


def main() -> int:
    payload = aggregate()
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Wrote {len(payload['items'])} items to {OUTPUT}")
    failed = [source for source in payload["sources"] if source["status"] != "ok"]
    if failed:
        print(f"{len(failed)} source(s) failed:", ", ".join(source["name"] for source in failed))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
