High-speed Go CLI for external infrastructure recon and phishing URL investigation. Maps DNS, TLS, banners, HTTP, and CDN/ASN enrichment; for full URLs it builds campaign hop graphs with page signals, MITRE ATT&CK tags, a verdict score/narrative, and IOC indicators.

## Problem

Phishing delivery chains and external attack surface are often investigated with ad-hoc curl scripts, browser hops, and disconnected scanners. CASRE collapses host recon and URL campaign crawling into one concurrent CLI so analysts can follow ESP → cloaker → deepview → lander paths and leave with a structured verdict plus IOCs.

## Architecture

| Module | What it does |
| --- | --- |
| **dns** | A, AAAA, CNAME, MX, NS, TXT (+ SPF/DMARC signals) |
| **tls** | Handshake version, cipher, cert chain, SANs, expiry |
| **banner** | TCP connect + banner grab on selected ports |
| **http** | Redirects, headers, security-header gaps, tech fingerprints, page analysis |
| **enrich** | CDN detection, Team Cymru ASN, mail/hosting hints |

### URL campaign mode

1. Follows delivery hops in parallel (configurable hop workers and wall-clock budget).
2. Classifies nodes (`tracker` / `cloaker` / `deepview` / `lander` / `decoy`). Landers require strong signals (cleartext from cloaker, credentials, suspicious TLD); brand deepview destinations are not auto-landers.
3. Stops expanding brand / CDN / social decoys unless `-full-crawl` is set.
4. Emits a **VERDICT** (score + short narrative) and an **IOC** section in the report tree.

## Usage sketch

```bash
go install github.com/saptreekly/casre/cmd/casre@latest

casre example.com
casre -modules dns,tls,enrich -ports 22,80,443 scanme.nmap.org
casre -v 'https://bit.ly/suspicious'
casre -evidence ./evidence 'https://lure.example/path'
```

Batch, JSON, and baseline-diff modes support scoped recon workflows (`-f`, `-json`, `-o`, `-diff`).

## Output contract

- **Verdict:** `score` (0–100), `story` narrative, and top contributing `signals`.
- **IOC tree:** deduped domain / IP / URL / ASN indicators (truncated unless `-v`).
- **MITRE ATT&CK:** findings tagged with confidence (`high` / `medium` / `low`); low-confidence rollups hidden unless verbose.
- **Evidence:** optional HTML snapshots of cloaker/lander pages under `-evidence`.

## Status

Public Go 1.22+ CLI. Authorization required: only scan hosts and URLs you own or have explicit permission to test.
