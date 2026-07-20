Medium-interaction Rust honeynet for collecting threat intelligence. Emulates SSH, HTTP, SMTP, and SMB services, ships structured events to a collector, and stores them locally or in S3 for later intel export.

## Problem

Low-interaction honeypots capture little beyond connection metadata; full high-interaction systems are expensive and risky to operate. This honeynet sits in the middle: enough protocol surface to attract and profile attackers, with a clean event pipeline for scoring and export—not a laptop exposed to the open internet.

## Architecture

| Component | Role |
| --- | --- |
| Honeypots (`ssh`, `http`, `smtp`, `smb`) | Accept attacker traffic and emit JSON events |
| Collector | Ingests via `POST /v1/events`; writes partitioned JSONL to file or S3 |
| Intel export CLI | Scores source IPs and exports malicious IP lists from local files or S3 |

### Crates

| Crate | Binary | Purpose |
| --- | --- | --- |
| `net-common` | — | Shared event schema and collector client |
| `net-collector` | `net-collector` | Event ingest API |
| `net-ssh-honeypot` | `net-ssh-honeypot` | SSH fake shell |
| `net-http-honeypot` | `net-http-honeypot` | HTTP decoy routes |
| `net-smtp-honeypot` | `net-smtp-honeypot` | Inbound SMTP capture |
| `net-smb-honeypot` | `net-smb-honeypot` | SMB2 negotiate/auth logging |
| `net-intel-export` | `net-intel-export` | Malicious IP export CLI |

## Personas and deployment

Environment variables use the `NET_` prefix. Set `NET_PERSONA=wellington_water` for an NZ water-utility critical-infrastructure decoy profile across honeypots.

Deployment paths include Docker Compose for local integration testing, Terraform for AWS scaffolding, and Kubernetes overlays for persona-specific manifests. Use an isolated AWS account/VPC—do not run production honeypots on a personal laptop exposed to the internet.

## Local sketch

```bash
cargo build --release
cargo run -p net-collector &
cargo run -p net-ssh-honeypot -- --bind 0.0.0.0:2222
```

Or `docker compose up --build` under `deploy/docker` (non-privileged ports for local dev).

Intel export:

```bash
cargo run -p net-intel-export -- local --input-dir ./data/events --output ./data/malicious-ips.txt
```

## Status

Public research codebase for defensive security on infrastructure you control. Review data retention and privacy requirements before any deployment.
