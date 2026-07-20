import os
import json
import requests
from datetime import datetime, timezone

GITHUB_USER = "saptreekly"
REPOS_URL = f"https://api.github.com/users/{GITHUB_USER}/repos"


def _github_get(url, headers, params=None):
    response = requests.get(url, headers=headers, params=params, timeout=30)
    try:
        payload = response.json()
    except ValueError:
        payload = None

    if not response.ok:
        message = payload.get("message") if isinstance(payload, dict) else response.text
        raise RuntimeError(
            f"GitHub API request failed ({response.status_code}) for {url}: {message}"
        )

    return payload


def _fetch_repos(headers):
    repos = []
    page = 1
    while True:
        batch = _github_get(
            REPOS_URL,
            headers,
            params={"per_page": 100, "page": page, "type": "owner"},
        )
        if not isinstance(batch, list):
            raise RuntimeError(
                f"Expected a list of repos from GitHub API, got {type(batch).__name__}: {batch!r}"
            )
        if not batch:
            break
        repos.extend(batch)
        if len(batch) < 100:
            break
        page += 1
    return repos


def run_audit():
    token = os.environ.get("GH_PAT")
    if not token:
        raise ValueError("GH_PAT environment variable not set")

    headers = {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }

    stats = {}
    for repo in _fetch_repos(headers):
        if not isinstance(repo, dict):
            raise RuntimeError(f"Unexpected repo entry type {type(repo).__name__}: {repo!r}")
        if repo.get("fork", False):
            continue

        languages_url = repo.get("languages_url")
        if not languages_url:
            continue

        langs = _github_get(languages_url, headers)
        if not isinstance(langs, dict):
            raise RuntimeError(
                f"Expected language map for {repo.get('full_name')}, got {type(langs).__name__}: {langs!r}"
            )

        for lang, byte_count in langs.items():
            if isinstance(byte_count, int):
                stats[lang] = stats.get(lang, 0) + byte_count

    sorted_stats = sorted(
        [{"language": k.upper(), "bytes": v} for k, v in stats.items()],
        key=lambda x: x["bytes"],
        reverse=True,
    )

    # Increase limit to 12 to include less frequent languages
    top_languages = sorted_stats[:12]

    data = {
        "updated_at": datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC"),
        "languages": top_languages,
    }

    with open("static/stack.json", "w") as f:
        json.dump(data, f)


if __name__ == "__main__":
    run_audit()
