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

try:
    import feedparser
except ImportError:
    print("feedparser is required: pip install feedparser", file=sys.stderr)
    raise

REPO_ROOT = Path(__file__).resolve().parent.parent
OUTPUT = REPO_ROOT / "static" / "situation-monitor" / "feed.json"
USER_AGENT = (
    "Mozilla/5.0 (compatible; SituationMonitor/1.0; "
    "+https://github.com/saptreekly/Research-Publications)"
)
MAX_ITEMS_PER_SOURCE = 12
MAX_TOTAL_ITEMS = 120

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
]

CATEGORY_LABELS = {
    "nz-pacific": "NZ & Pacific",
    "apac-security": "APAC Security",
    "cyber": "Cyber",
    "global": "Global",
}


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

        items.append(
            {
                "id": item_id(feed["id"], link, title),
                "title": title,
                "summary": summary,
                "url": link,
                "published_at": published_at,
                "published_label": format_label(published) if published else "Unknown",
                "source_id": feed["id"],
                "source_name": feed["name"],
                "category": feed["category"],
            }
        )

    return items, None


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

    all_items.sort(
        key=lambda item: item.get("published_at") or "",
        reverse=True,
    )
    all_items = all_items[:MAX_TOTAL_ITEMS]

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
