#!/usr/bin/env python3
"""Generate sitemap.xml and route-specific HTML shells for crawler-friendly indexing."""

from __future__ import annotations

import html
import re
import sys
from dataclasses import dataclass
from datetime import date
from pathlib import Path

SITE_URL = "https://saptreekly.github.io/Research-Publications"
OG_IMAGE_URL = f"{SITE_URL}/static/og-card.png"
REPO_ROOT = Path(__file__).resolve().parent.parent
DIST = REPO_ROOT / "dist"
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
    {
        "path": "/tidy-tuesday",
        "title": "Tidy Tuesday | Jack Weekly",
        "description": "Weekly Julia data explorations using TidyTuesday community datasets.",
    },
    {
        "path": "/tidy-tuesday/se4all-2026-05-26",
        "title": "Sustainable Energy for All | Jack Weekly",
        "description": "Tidy Tuesday · SE4ALL country-level energy metrics · Julia analysis · Data analysis",
    },
]

for slug in MODULE_SLUGS:
    file_prefix = slug.replace("-", "_")
    ROUTES.append(
        {
            "path": f"/curriculum/{slug}",
            "title": f"Julia Crypto Module {slug} | Jack Weekly",
            "description": f"Theory module from the Julia cryptography curriculum ({slug}).",
            "content_src": f"research-docs/julia-crypto/{file_prefix}.md",
        }
    )
    ROUTES.append(
        {
            "path": f"/curriculum/lab/{slug}",
            "title": f"Julia Crypto Lab {slug} | Jack Weekly",
            "description": f"Interactive browser lab from the Julia cryptography curriculum ({slug}).",
            "content_src": f"research-docs/julia-crypto/{file_prefix}_lab.md",
        }
    )

ROUTE_CONTENT: dict[str, list[str]] = {
    "/research/lovely-malware": ["research-docs/reports/lovely-malware.md"],
    "/research/thesis-summary": ["research-docs/thesis-summary.md"],
    "/projects/hlidskjalf": ["research-docs/projects/hlidskjalf.md"],
    "/projects/siem-ensemble": ["research-docs/projects/siem-ensemble.md"],
    "/projects/geospatial-intel": ["research-docs/projects/geospatial-intel.md"],
    "/tidy-tuesday/se4all-2026-05-26": ["research-docs/tidy-tuesday/se4all-2026-05-26.md"],
}

for route in ROUTES:
    src = route.get("content_src")
    if src:
        ROUTE_CONTENT.setdefault(route["path"], [src])

CURRICULUM_MODULES = [
    ("mod-01", "01.1 / Modular Inverses", "Number Theory"),
    ("mod-02", "01.2 / Modular Exponentiation", "Number Theory"),
    ("mod-03", "01.3 / Chinese Remainder Theorem", "Number Theory"),
    ("mod-04", "01.4 / Fermat and Euler Theorems", "Number Theory"),
    ("mod-05", "02.1 / Sieve of Eratosthenes", "Primes"),
    ("mod-06", "02.2 / Primes.jl", "Primes"),
    ("mod-07", "03.1 / RSA Key Generation", "RSA"),
    ("mod-08", "03.2 / RSA Encryption and Decryption", "RSA"),
]

SEO_STATIC_PATTERN = re.compile(
    r'(<div id="seo-static" class="seo-static"(?: aria-hidden="true")?>\s*)(.*?)(\s*</div>\s*<noscript>)',
    re.S,
)


@dataclass(frozen=True)
class ValidationIssue:
    file: str
    line: int | None
    message: str


class MarkdownStructureError(Exception):
    def __init__(self, file: str, line: int | None, message: str) -> None:
        self.file = file
        self.line = line
        self.message = message
        super().__init__(self.format_message())

    def format_message(self) -> str:
        if self.line is not None:
            return f"{self.file}:{self.line}: {self.message}"
        return f"{self.file}: {self.message}"


def collect_markdown_sources() -> list[str]:
    sources: set[str] = set()
    for route in ROUTES:
        content_src = route.get("content_src")
        if content_src:
            sources.add(content_src)
    for content_paths in ROUTE_CONTENT.values():
        sources.update(content_paths)
    return sorted(sources)


def log_validation_issues(issues: list[ValidationIssue]) -> None:
    for issue in issues:
        location = f"{issue.file}:{issue.line}" if issue.line is not None else issue.file
        print(f"ERROR: {location}: {issue.message}", file=sys.stderr)
    print(
        f"\nPreflight validation failed ({len(issues)} issue(s)). "
        "Compilation aborted to prevent corrupt SEO HTML.",
        file=sys.stderr,
    )


def validate_route_source_files() -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []

    for route in ROUTES:
        content_src = route.get("content_src")
        if not content_src:
            continue
        source_path = REPO_ROOT / content_src
        if not source_path.is_file():
            issues.append(
                ValidationIssue(
                    content_src,
                    None,
                    f"Route {route['path']!r} references missing content_src.",
                )
            )

    for route_path, content_paths in ROUTE_CONTENT.items():
        for content_src in content_paths:
            source_path = REPO_ROOT / content_src
            if not source_path.is_file():
                issues.append(
                    ValidationIssue(
                        content_src,
                        None,
                        f"ROUTE_CONTENT[{route_path!r}] references missing markdown source.",
                    )
                )

    return issues


def table_column_count(line: str) -> int:
    stripped = line.strip()
    if not stripped.startswith("|"):
        return 0
    return len([cell for cell in stripped.strip("|").split("|")])


def validate_code_fences(relative_path: str, lines: list[str]) -> list[ValidationIssue]:
    fence_lines = [
        line_number
        for line_number, line in enumerate(lines, start=1)
        if line.strip().startswith("```")
    ]
    if len(fence_lines) % 2 == 0:
        return []

    return [
        ValidationIssue(
            relative_path,
            fence_lines[-1],
            (
                f"Unclosed code fence: found {len(fence_lines)} delimiter line(s) "
                "starting with ```; expected an even number."
            ),
        )
    ]


def validate_tables(relative_path: str, lines: list[str]) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    index = 0

    while index < len(lines):
        stripped = lines[index].strip()
        if stripped.startswith("|") and index + 1 < len(lines) and is_table_divider(lines[index + 1]):
            header_line = index + 1
            header_count = table_column_count(lines[index])
            if header_count == 0:
                issues.append(
                    ValidationIssue(
                        relative_path,
                        header_line,
                        "Markdown table header has no columns.",
                    )
                )

            divider_count = table_column_count(lines[index + 1])
            if header_count and divider_count != header_count:
                issues.append(
                    ValidationIssue(
                        relative_path,
                        index + 2,
                        (
                            f"Table divider column count ({divider_count}) does not match "
                            f"header column count ({header_count})."
                        ),
                    )
                )

            index += 2
            while index < len(lines) and lines[index].strip().startswith("|"):
                row_count = table_column_count(lines[index])
                if header_count and row_count != header_count:
                    issues.append(
                        ValidationIssue(
                            relative_path,
                            index + 1,
                            (
                                f"Table row column count ({row_count}) does not match "
                                f"header column count ({header_count})."
                            ),
                        )
                    )
                index += 1
            continue

        index += 1

    return issues


def validate_markdown_structure(relative_path: str, markdown_text: str) -> list[ValidationIssue]:
    lines = markdown_text.replace("\r\n", "\n").split("\n")
    issues: list[ValidationIssue] = []
    issues.extend(validate_code_fences(relative_path, lines))
    issues.extend(validate_tables(relative_path, lines))
    return issues


def run_preflight_validation() -> None:
    issues: list[ValidationIssue] = []
    issues.extend(validate_route_source_files())

    for relative_path in collect_markdown_sources():
        source_path = REPO_ROOT / relative_path
        if not source_path.is_file():
            continue
        try:
            markdown_text = source_path.read_text(encoding="utf-8")
        except OSError as exc:
            issues.append(
                ValidationIssue(
                    relative_path,
                    None,
                    f"Could not read markdown source: {exc}",
                )
            )
            continue
        issues.extend(validate_markdown_structure(relative_path, markdown_text))

    if issues:
        log_validation_issues(issues)
        sys.exit(1)


def canonical_url(path: str) -> str:
    if path == "/":
        return f"{SITE_URL}/"
    return f"{SITE_URL}{path}"


def page_heading(title: str) -> str:
    return title.split(" | ", 1)[0]


def site_nav_html() -> str:
    return """
        <nav aria-label="Site index">
            <h2>Pages</h2>
            <ul>
                <li><a href="/Research-Publications/">Home</a></li>
                <li><a href="/Research-Publications/contact">Contact</a></li>
                <li><a href="/Research-Publications/curriculum">Julia Cryptography Curriculum</a></li>
            </ul>
            <h2>Research &amp; Writing</h2>
            <ul>
                <li><a href="/Research-Publications/research/lovely-malware">LovelyMalware Analysis Report</a></li>
                <li><a href="/Research-Publications/research/thesis-summary">5th-Gen Fighter Exports as Strategic Competition</a></li>
            </ul>
            <h2>Engineering Projects</h2>
            <ul>
                <li><a href="/Research-Publications/projects/hlidskjalf">Project Hliðskjálf</a></li>
                <li><a href="/Research-Publications/projects/siem-ensemble">SIEM Ensemble</a></li>
                <li><a href="/Research-Publications/projects/geospatial-intel">Geospatial Intel Server</a></li>
            </ul>
        </nav>""".strip()


def curriculum_index_html() -> str:
    sections: dict[str, list[tuple[str, str]]] = {}
    for slug, title, section in CURRICULUM_MODULES:
        sections.setdefault(section, []).append((slug, title))

    parts = ['<section class="seo-curriculum-index">', "<h2>Curriculum modules</h2>"]
    for section, modules in sections.items():
        parts.append(f"<h3>{html.escape(section)}</h3>")
        parts.append("<ul>")
        for slug, title in modules:
            parts.append(
                "<li>"
                f'<a href="/Research-Publications/curriculum/{slug}">{html.escape(title)}</a>'
                f' · <a href="/Research-Publications/curriculum/lab/{slug}">Lab</a>'
                "</li>"
            )
        parts.append("</ul>")
    parts.append("</section>")
    return "\n            ".join(parts)


def load_markdown(relative_path: str) -> str:
    path = REPO_ROOT / relative_path
    if not path.exists():
        raise FileNotFoundError(f"Missing markdown source for prerender: {path}")
    return path.read_text(encoding="utf-8")


def render_inline(text: str) -> str:
    placeholders: list[str] = []

    def stash(value: str) -> str:
        token = f"@@SEO{len(placeholders)}@@"
        placeholders.append(value)
        return token

    def code_repl(match: re.Match[str]) -> str:
        return stash(f"<code>{html.escape(match.group(1))}</code>")

    def link_repl(match: re.Match[str]) -> str:
        label = html.escape(match.group(1))
        url = html.escape(match.group(2), quote=True)
        return stash(f'<a href="{url}">{label}</a>')

    def bold_repl(match: re.Match[str]) -> str:
        return stash(f"<strong>{html.escape(match.group(1))}</strong>")

    def italic_repl(match: re.Match[str]) -> str:
        return stash(f"<em>{html.escape(match.group(1))}</em>")

    # Process inline syntax from least to most ambiguous:
    # code and links before emphasis so inner delimiters stay literal.
    text = re.sub(r"`(.*?)`", code_repl, text)
    text = re.sub(r"\[(.*?)\]\((.*?)\)", link_repl, text)
    text = re.sub(r"\*\*(.*?)\*\*", bold_repl, text)
    text = re.sub(r"(?<!\*)\*(.*?)\*(?!\*)", italic_repl, text)
    text = html.escape(text, quote=False)

    # Restore placeholders highest-index first so token ids never overlap.
    for index in range(len(placeholders) - 1, -1, -1):
        text = text.replace(f"@@SEO{index}@@", placeholders[index])

    return text


def is_table_divider(line: str) -> bool:
    stripped = line.strip()
    if not stripped.startswith("|"):
        return False
    cells = [cell.strip() for cell in stripped.strip("|").split("|")]
    return bool(cells) and all(re.fullmatch(r":?-{3,}:?", cell) for cell in cells)


def render_table(lines: list[str]) -> str:
    header_cells = [cell.strip() for cell in lines[0].strip().strip("|").split("|")]
    body_rows = lines[2:] if len(lines) > 2 else []
    parts = ["<table>", "<thead><tr>"]
    for cell in header_cells:
        parts.append(f"<th>{render_inline(cell)}</th>")
    parts.append("</tr></thead><tbody>")
    for row in body_rows:
        if not row.strip():
            continue
        cells = [cell.strip() for cell in row.strip().strip("|").split("|")]
        parts.append("<tr>")
        for cell in cells:
            parts.append(f"<td>{render_inline(cell)}</td>")
        parts.append("</tr>")
    parts.append("</tbody></table>")
    return "".join(parts)


def render_markdown(markdown_text: str, *, source_path: str = "<markdown>") -> str:
    lines = markdown_text.replace("\r\n", "\n").split("\n")
    output: list[str] = []
    index = 0

    while index < len(lines):
        line = lines[index]
        stripped = line.strip()

        if not stripped:
            index += 1
            continue

        if stripped.startswith("```"):
            fence_line = index + 1
            index += 1
            code_lines: list[str] = []
            while index < len(lines) and not lines[index].strip().startswith("```"):
                code_lines.append(lines[index])
                index += 1
            if index >= len(lines):
                raise MarkdownStructureError(
                    source_path,
                    fence_line,
                    "Unclosed code fence reached end of file during compilation.",
                )
            index += 1
            code = html.escape("\n".join(code_lines))
            output.append(f"<pre><code>{code}</code></pre>")
            continue

        if stripped.startswith("#"):
            level = len(stripped) - len(stripped.lstrip("#"))
            level = min(max(level, 1), 4)
            text = stripped[level:].strip()
            output.append(f"<h{level}>{render_inline(text)}</h{level}>")
            index += 1
            continue

        if stripped.startswith(">"):
            quote_lines: list[str] = []
            while index < len(lines) and lines[index].strip().startswith(">"):
                quote_lines.append(lines[index].strip().lstrip(">").strip())
                index += 1
            quote_html = " ".join(render_inline(part) for part in quote_lines if part)
            output.append(f"<blockquote><p>{quote_html}</p></blockquote>")
            continue

        if stripped.startswith("|") and index + 1 < len(lines) and is_table_divider(lines[index + 1]):
            table_lines = [lines[index], lines[index + 1]]
            header_count = table_column_count(lines[index])
            divider_count = table_column_count(lines[index + 1])
            if header_count == 0:
                raise MarkdownStructureError(
                    source_path,
                    index + 1,
                    "Markdown table header has no columns.",
                )
            if divider_count != header_count:
                raise MarkdownStructureError(
                    source_path,
                    index + 2,
                    (
                        f"Table divider column count ({divider_count}) does not match "
                        f"header column count ({header_count})."
                    ),
                )
            index += 2
            while index < len(lines) and lines[index].strip().startswith("|"):
                row_count = table_column_count(lines[index])
                if row_count != header_count:
                    raise MarkdownStructureError(
                        source_path,
                        index + 1,
                        (
                            f"Table row column count ({row_count}) does not match "
                            f"header column count ({header_count})."
                        ),
                    )
                table_lines.append(lines[index])
                index += 1
            output.append(render_table(table_lines))
            continue

        if re.match(r"[-*]\s+", stripped):
            items: list[str] = []
            while index < len(lines) and re.match(r"[-*]\s+", lines[index].strip()):
                item = re.sub(r"^[-*]\s+", "", lines[index].strip())
                items.append(f"<li>{render_inline(item)}</li>")
                index += 1
            output.append("<ul>" + "".join(items) + "</ul>")
            continue

        if re.match(r"\d+\.\s+", stripped):
            items = []
            while index < len(lines) and re.match(r"\d+\.\s+", lines[index].strip()):
                item = re.sub(r"^\d+\.\s+", "", lines[index].strip())
                items.append(f"<li>{render_inline(item)}</li>")
                index += 1
            output.append("<ol>" + "".join(items) + "</ol>")
            continue

        paragraph_lines = [stripped]
        index += 1
        while index < len(lines):
            next_line = lines[index].strip()
            if (
                not next_line
                or next_line.startswith("#")
                or next_line.startswith("```")
                or next_line.startswith(">")
                or next_line.startswith("|")
                or re.match(r"[-*]\s+", next_line)
                or re.match(r"\d+\.\s+", next_line)
            ):
                break
            paragraph_lines.append(next_line)
            index += 1
        paragraph = " ".join(paragraph_lines)
        output.append(f"<p>{render_inline(paragraph)}</p>")

    return "\n            ".join(output)


def build_seo_static(route: dict[str, str]) -> str:
    path = route["path"]
    heading = page_heading(route["title"])
    description = route["description"]

    parts = [
        "<header>",
        f"            <h1>{html.escape(heading)}</h1>",
        f"            <p>{html.escape(description)}</p>",
        "        </header>",
    ]

    if path == "/curriculum":
        parts.append(curriculum_index_html())
    elif path == "/contact":
        parts.append(
            """
        <section class="seo-contact">
            <p>Contact form and interactive navigation require JavaScript. Email inquiries are handled through the contact page in the live portfolio.</p>
        </section>""".strip()
        )
    elif path == "/":
        parts.append(
            """
        <section class="seo-home">
            <p>Strategic analysis and technical security portfolio. Malware forensics, CTI-style reporting, engineering projects, and graduate strategic studies writing.</p>
        </section>""".strip()
        )

    content_sources = ROUTE_CONTENT.get(path, [])
    if content_sources:
        parts.append('        <article class="seo-article">')
        for source in content_sources:
            parts.append(render_markdown(load_markdown(source), source_path=source))
        parts.append("        </article>")

    parts.append(f"        {site_nav_html()}")
    return "\n        ".join(parts)


def inject_meta(html_doc: str, title: str, description: str, path: str) -> str:
    canonical = canonical_url(path)
    html_doc = re.sub(r"<title>.*?</title>", f"<title>{title}</title>", html_doc, count=1, flags=re.S)

    def upsert_meta(name: str, content: str, *, prop: bool = False) -> None:
        nonlocal html_doc
        attr = "property" if prop else "name"
        pattern = rf'<meta {attr}="{re.escape(name)}" content="[^"]*">'
        tag = f'<meta {attr}="{name}" content="{content}">'
        if re.search(pattern, html_doc):
            html_doc = re.sub(pattern, tag, html_doc, count=1)
        else:
            html_doc = html_doc.replace("</head>", f"    {tag}\n</head>", 1)

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
    if re.search(canonical_pattern, html_doc):
        html_doc = re.sub(canonical_pattern, canonical_tag, html_doc, count=1)
    else:
        html_doc = html_doc.replace("</head>", f"    {canonical_tag}\n</head>", 1)

    return html_doc


def inject_seo_static(html_doc: str, route: dict[str, str]) -> str:
    body = build_seo_static(route)
    has_article = route["path"] in ROUTE_CONTENT or route["path"] == "/curriculum"
    opening = (
        '<div id="seo-static" class="seo-static">'
        if has_article
        else '<div id="seo-static" class="seo-static" aria-hidden="true">'
    )

    def replacer(match: re.Match[str]) -> str:
        return f"{opening}\n        {body}\n    {match.group(3)}"

    updated, count = SEO_STATIC_PATTERN.subn(replacer, html_doc, count=1)
    if count != 1:
        raise RuntimeError("Could not locate #seo-static block in dist/index.html")
    return updated


def write_route_html(template: str, route: dict[str, str]) -> None:
    html_doc = inject_meta(template, route["title"], route["description"], route["path"])
    html_doc = inject_seo_static(html_doc, route)
    if route["path"] == "/":
        (DIST / "index.html").write_text(html_doc, encoding="utf-8")
        return

    target_dir = DIST / route["path"].strip("/")
    target_dir.mkdir(parents=True, exist_ok=True)
    (target_dir / "index.html").write_text(html_doc, encoding="utf-8")


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
    run_preflight_validation()

    if not INDEX.exists():
        print("dist/index.html not found. Run trunk build first.", file=sys.stderr)
        return 1

    template = INDEX.read_text(encoding="utf-8")
    content_routes = sum(1 for route in ROUTES if route["path"] in ROUTE_CONTENT or route["path"] == "/curriculum")

    try:
        for route in ROUTES:
            write_route_html(template, route)
    except MarkdownStructureError as exc:
        print(f"ERROR: {exc.format_message()}", file=sys.stderr)
        print("Compilation aborted to prevent corrupt SEO HTML.", file=sys.stderr)
        return 1
    except FileNotFoundError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        print("Compilation aborted to prevent corrupt SEO HTML.", file=sys.stderr)
        return 1
    except RuntimeError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        print("Compilation aborted to prevent corrupt SEO HTML.", file=sys.stderr)
        return 1

    write_sitemap()
    print(
        f"Generated sitemap.xml and {len(ROUTES)} prerendered route shells "
        f"({content_routes} with markdown body content) in {DIST}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
