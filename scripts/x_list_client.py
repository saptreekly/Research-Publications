"""Fetch public X/Twitter list timelines via the internal GraphQL API."""

from __future__ import annotations

import json
import os
import re
import urllib.error
import urllib.parse
import urllib.request
from datetime import datetime, timezone
from email.utils import parsedate_to_datetime
from typing import Any

USER_AGENT = (
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"
)
BEARER_TOKEN = (
    "AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D"
    "1Zv7ttfk8LFVt1IUqPHC5FhGql3xZ0H0wP6t4cE"
)
FALLBACK_QUERY_IDS = {
    "ListLatestTweetsTimeline": "RlZzktZY_9wJynoepm8ZsA",
}
OPENAPI_PLACEHOLDER_URL = (
    "https://raw.githubusercontent.com/fa0311/twitter-openapi/"
    "refs/heads/main/src/config/placeholder.json"
)
GRAPHQL_FEATURES = {
    "responsive_web_graphql_exclude_directive_enabled": True,
    "verified_phone_label_enabled": False,
    "creator_subscriptions_tweet_preview_api_enabled": True,
    "responsive_web_graphql_timeline_navigation_enabled": True,
    "responsive_web_graphql_skip_user_profile_image_extensions_enabled": False,
    "c9s_tweet_anatomy_moderator_badge_enabled": True,
    "tweetypie_unmention_optimization_enabled": True,
    "responsive_web_edit_tweet_api_enabled": True,
    "graphql_is_translatable_rweb_tweet_is_translatable_enabled": True,
    "view_counts_everywhere_api_enabled": True,
    "longform_notetweets_consumption_enabled": True,
    "responsive_web_twitter_article_tweet_consumption_enabled": True,
    "tweet_awards_web_tipping_enabled": False,
    "longform_notetweets_rich_text_read_enabled": True,
    "longform_notetweets_inline_media_enabled": True,
    "rweb_video_timestamps_enabled": True,
    "responsive_web_media_download_video_enabled": True,
    "freedom_of_speech_not_reach_fetch_enabled": True,
    "standardized_nudges_misinfo": True,
    "responsive_web_enhance_cards_enabled": False,
}

_query_id_cache: dict[str, str] = {}


def _deep_get(data: Any, *keys: Any) -> Any:
    current = data
    for key in keys:
        if isinstance(key, int):
            if isinstance(current, list) and 0 <= key < len(current):
                current = current[key]
            else:
                return None
        elif isinstance(current, dict):
            current = current.get(key)
        else:
            return None
    return current


def _fetch_text(url: str, headers: dict[str, str] | None = None) -> str:
    request = urllib.request.Request(url, headers=headers or {"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=30) as response:
        return response.read().decode("utf-8", errors="replace")


def _resolve_query_id(operation_name: str) -> str:
    cached = _query_id_cache.get(operation_name)
    if cached:
        return cached

    fallback = FALLBACK_QUERY_IDS.get(operation_name)
    if fallback:
        _query_id_cache[operation_name] = fallback
        return fallback

    try:
        payload = json.loads(_fetch_text(OPENAPI_PLACEHOLDER_URL))
        query_id = payload.get(operation_name, {}).get("queryId")
        if isinstance(query_id, str) and query_id:
            _query_id_cache[operation_name] = query_id
            return query_id
    except (urllib.error.URLError, json.JSONDecodeError, KeyError, TypeError):
        pass

    try:
        html = _fetch_text("https://x.com")
        script_urls = re.findall(
            r'(?:src|href)=["\'](https://abs\.twimg\.com/responsive-web/client-web[^"\']+\.js)["\']',
            html,
        )
        pattern = re.compile(
            r'queryId:\s*"([A-Za-z0-9_-]+)"[^}]{0,200}operationName:\s*"([^"]+)"'
        )
        for script_url in script_urls:
            try:
                bundle = _fetch_text(script_url)
            except urllib.error.URLError:
                continue
            for match in pattern.finditer(bundle):
                query_id, name = match.group(1), match.group(2)
                _query_id_cache.setdefault(name, query_id)
        cached = _query_id_cache.get(operation_name)
        if cached:
            return cached
    except urllib.error.URLError:
        pass

    if fallback:
        return fallback
    raise RuntimeError(f"Unable to resolve GraphQL query id for {operation_name}")


def _build_graphql_url(operation_name: str, variables: dict[str, Any]) -> str:
    query_id = _resolve_query_id(operation_name)
    compact_features = {key: value for key, value in GRAPHQL_FEATURES.items() if value is not False}
    return (
        f"https://x.com/i/api/graphql/{query_id}/{operation_name}"
        f"?variables={urllib.parse.quote(json.dumps(variables, separators=(',', ':')))}"
        f"&features={urllib.parse.quote(json.dumps(compact_features, separators=(',', ':')))}"
    )


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


def _unwrap_tweet_result(result: dict[str, Any]) -> dict[str, Any] | None:
    if result.get("__typename") == "TweetTombstone":
        return None
    if result.get("__typename") == "TweetWithVisibilityResults" and result.get("tweet"):
        return result["tweet"]
    return result


def _parse_tweet(result: dict[str, Any]) -> dict[str, Any] | None:
    tweet_data = _unwrap_tweet_result(result)
    if not tweet_data:
        return None

    legacy = tweet_data.get("legacy")
    core = tweet_data.get("core")
    if not isinstance(legacy, dict) or not isinstance(core, dict):
        return None

    user = _deep_get(core, "user_results", "result") or {}
    user_legacy = user.get("legacy", {}) if isinstance(user, dict) else {}
    user_core = user.get("core", {}) if isinstance(user, dict) else {}

    actual_data = tweet_data
    actual_legacy = legacy
    actual_user = user
    actual_user_legacy = user_legacy
    actual_user_core = user_core

    if _deep_get(legacy, "retweeted_status_result", "result"):
        retweet_result = _deep_get(legacy, "retweeted_status_result", "result") or {}
        retweet_result = _unwrap_tweet_result(retweet_result) or {}
        rt_legacy = retweet_result.get("legacy")
        rt_core = retweet_result.get("core")
        if isinstance(rt_legacy, dict) and isinstance(rt_core, dict):
            actual_data = retweet_result
            actual_legacy = rt_legacy
            actual_user = _deep_get(rt_core, "user_results", "result") or {}
            actual_user_legacy = actual_user.get("legacy", {}) if isinstance(actual_user, dict) else {}
            actual_user_core = actual_user.get("core", {}) if isinstance(actual_user, dict) else {}

    note_text = _deep_get(actual_data, "note_tweet", "note_tweet_results", "result", "text")
    text = (note_text or actual_legacy.get("full_text") or "").strip()
    if not text:
        return None

    screen_name = (
        actual_user_core.get("screen_name")
        or actual_user_legacy.get("screen_name")
        or "unknown"
    )
    tweet_id = actual_data.get("rest_id") or legacy.get("id_str") or ""
    if not tweet_id:
        return None

    return {
        "id": str(tweet_id),
        "text": text,
        "screen_name": str(screen_name),
        "created_at": actual_legacy.get("created_at"),
        "url": f"https://x.com/{screen_name}/status/{tweet_id}",
    }


def _parse_timeline(data: dict[str, Any]) -> list[dict[str, Any]]:
    instructions = _deep_get(
        data,
        "data",
        "list",
        "tweets_timeline",
        "timeline",
        "instructions",
    )
    if not isinstance(instructions, list):
        return []

    tweets: list[dict[str, Any]] = []
    for instruction in instructions:
        entries = instruction.get("entries") or instruction.get("moduleItems") or []
        if not isinstance(entries, list):
            continue
        for entry in entries:
            content = entry.get("content", {})
            if not isinstance(content, dict):
                continue
            result = _deep_get(content, "itemContent", "tweet_results", "result")
            if isinstance(result, dict):
                parsed = _parse_tweet(result)
                if parsed:
                    tweets.append(parsed)
            for nested_item in content.get("items") or []:
                nested_result = _deep_get(
                    nested_item,
                    "item",
                    "itemContent",
                    "tweet_results",
                    "result",
                )
                if isinstance(nested_result, dict):
                    parsed = _parse_tweet(nested_result)
                    if parsed:
                        tweets.append(parsed)
    return tweets


def fetch_list_timeline(
    list_id: str,
    *,
    auth_token: str,
    ct0: str,
    count: int = 20,
) -> list[dict[str, Any]]:
    variables = {"listId": list_id, "count": count}
    url = _build_graphql_url("ListLatestTweetsTimeline", variables)
    headers = {
        "User-Agent": USER_AGENT,
        "Authorization": f"Bearer {BEARER_TOKEN}",
        "Cookie": f"auth_token={auth_token}; ct0={ct0}",
        "x-csrf-token": ct0,
        "x-twitter-active-user": "yes",
        "x-twitter-auth-type": "OAuth2Session",
        "x-twitter-client-language": "en",
        "Accept": "*/*",
        "Referer": f"https://x.com/i/lists/{list_id}",
    }

    request = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(request, timeout=30) as response:
        payload = json.loads(response.read().decode("utf-8"))

    if payload.get("errors"):
        messages = "; ".join(
            str(error.get("message", error)) for error in payload["errors"] if error
        )
        raise RuntimeError(messages or "X GraphQL request failed")

    return _parse_timeline(payload)


def load_credentials() -> tuple[str, str] | None:
    auth_token = os.environ.get("AUTH_TOKEN", "").strip()
    ct0 = os.environ.get("CT0", "").strip()
    if auth_token and ct0:
        return auth_token, ct0
    return None
