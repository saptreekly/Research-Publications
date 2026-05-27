#!/usr/bin/env python3
"""Generate sitemap.xml and route-specific HTML shells for crawler-friendly indexing."""

from __future__ import annotations

import re
import sys
from datetime import date
from pathlib import Path

SITE_URL = "https://saptreekly.github.io/Research-Publications"
OG_IMAGE_URL = f"{SITE_URL}/static/og-card.png"
DIST = Path("dist")
INDEX = DIST / "index.html"

MODULE_SLUGS = [f"mod-{i:02d}" for i in range(1, 9)]

ROUTES: list[dict[str, str]] = [
    {
        "path": "/",
        "title": "Jack Weekly | Analysis & Security",
        "description": "Portfolio of Jack Weekly: malware forensics, strategic studies, security engineering, and applied cryptography research. Dual US & NZ citizen.",
    },
    {
        "path": "/contact",
        "title": "Contact | Jack Weekly",
        "description": "Contact Jack Weekly for national security, cyber, intelligence-adjacent, and technical analysis roles.",
    },
    {
        "path": "/curriculum",
        "title": "Julia Cryptography Curriculum | Jack Weekly",
        "description": "Eight-module Julia cryptography curriculum with theory notes and interactive browser labs covering number theory, primes, and RSA.",
    },
    {
        "path": "/research/lovely-malware",
        "title": "LovelyMalware | Jack Weekly",
        "description": "Malware analysis report · HackTheBox Insane · Malware analysis",
    },
    {
        "path": "/research/thesis-summary",
        "title": "5th-Gen Fighter Exports as Strategic Competition | Jack Weekly",
        "description": "Strategic studies writing sample · Victoria University of Wellington · MStrat with Merit · Strategic analysis",
    },
    {
        "path": "/projects/hlidskjalf",
        "title": "Project Hliðskjálf | Jack Weekly",
        "description": "Type-1.5 thin hypervisor for legacy x86_64 host hardening · Rust · Assembly",
    },
    {
        "path": "/projects/siem-ensemble",
        "title": "SIEM Ensemble | Jack Weekly",
        "description": "Polyglot log ingestion and real-time analytics pipeline · Rust · Zig · Odin · Elixir · Assembly",
    },
    {
        "path": "/projects/geospatial-intel",
        "title": "Geospatial Intel Server | Jack Weekly",
        "description": "Viewport-filtered aircraft tracking over WebSocket with H3 indexing · Go · Rust · Wasm",
    },
]

for slug in MODULE_SLUGS:
    ROUTES.append(
        {
            "path": f"/curriculum/{slug}",
            "title": f"Julia Crypto Module {slug} | Jack Weekly",
            "description": f"Theory module from the Julia cryptography curriculum ({slug}).",
        }
    )
    ROUTES.append(
        {
            "path": f"/curriculum/lab/{slug}",
            "title": f"Julia Crypto Lab {slug} | Jack Weekly",
            "description": f"Interactive browser lab from the Julia cryptography curriculum ({slug}).",
        }
    )


def canonical_url(path: str) -> str:
    if path == "/":
        return f"{SITE_URL}/"
    return f"{SITE_URL}{path}"


def inject_meta(html: str, title: str, description: str, path: str) -> str:
    canonical = canonical_url(path)
    html = re.sub(r"<title>.*?</title>", f"<title>{title}</title>", html, count=1, flags=re.S)

    def upsert_meta(name: str, content: str, *, prop: bool = False) -> None:
        nonlocal html
        attr = "property" if prop else "name"
        pattern = rf'<meta {attr}="{re.escape(name)}" content="[^"]*">'
        tag = f'<meta {attr}="{name}" content="{content}">'
        if re.search(pattern, html):
            html = re.sub(pattern, tag, html, count=1)
        else:
            html = html.replace("</head>", f"    {tag}\n</head>", 1)

    upsert_meta("description", description)
    upsert_meta("og:title", title, prop=True)
    upsert_meta("og:description", description, prop=True)
    upsert_meta("og:url", canonical, prop=True)
    upsert_meta("og:image", OG_IMAGE_URL, prop=True)
    upsert_meta("og:image:width", "1200", prop=True)
    upsert_meta("og:image:height", "630", prop=True)
    upsert_meta(
        "og:image:alt",
        "Jack Weekly portfolio: strategic and technical analysis.",
        prop=True,
    )
    upsert_meta("twitter:card", "summary_large_image")
    upsert_meta("twitter:title", title)
    upsert_meta("twitter:description", description)
    upsert_meta("twitter:image", OG_IMAGE_URL)

    canonical_pattern = r'<link rel="canonical" href="[^"]*">'
    canonical_tag = f'<link rel="canonical" href="{canonical}">'
    if re.search(canonical_pattern, html):
        html = re.sub(canonical_pattern, canonical_tag, html, count=1)
    else:
        html = html.replace("</head>", f"    {canonical_tag}\n</head>", 1)

    return html


def write_route_html(template: str, route: dict[str, str]) -> None:
    html = inject_meta(template, route["title"], route["description"], route["path"])
    if route["path"] == "/":
        (DIST / "index.html").write_text(html, encoding="utf-8")
        return

    target_dir = DIST / route["path"].strip("/")
    target_dir.mkdir(parents=True, exist_ok=True)
    (target_dir / "index.html").write_text(html, encoding="utf-8")


def write_sitemap() -> None:
    today = date.today().isoformat()
    lines = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">',
    ]
    for route in ROUTES:
        loc = canonical_url(route["path"])
        lines.extend(
            [
                "  <url>",
                f"    <loc>{loc}</loc>",
                f"    <lastmod>{today}</lastmod>",
                "  </url>",
            ]
        )
    lines.append("</urlset>")
    (DIST / "sitemap.xml").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    if not INDEX.exists():
        print("dist/index.html not found. Run trunk build first.", file=sys.stderr)
        return 1

    template = INDEX.read_text(encoding="utf-8")
    for route in ROUTES:
        write_route_html(template, route)

    write_sitemap()
    print(f"Generated sitemap.xml and {len(ROUTES)} prerendered route shells in {DIST}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
