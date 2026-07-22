High-speed, memory-safe static malware triage from the command line. Samples are never executed: the tool maps binaries in memory, parses PE / ELF / Mach-O formally, ranks candidates, and deep-dives with iced-x86 disassembly, crypto / XOR recovery, and network IOC extraction.

Repo: [github.com/saptreekly/Vanguard-RE](https://github.com/saptreekly/Vanguard-RE)

## Problem

Static triage often means juggling hex viewers, disassemblers, and ad-hoc scripts while passworded malware packs spill runnable files onto disk. Vanguard-RE keeps the workflow in one Rust CLI with zero-copy I/O, in-memory ZIP decryption, bomb-bounded ingest, and containment-first defaults.

## Three pillars

| Pillar | How |
| --- | --- |
| **Speed** | `memmap2` zero-copy I/O + focused static pipelines |
| **Accuracy** | Formal PE / ELF / Mach-O parsing (`goblin`), ImpHash, Shannon entropy, IAT + delay-load string capabilities, iced-x86 disassembly, crypto fingerprints, weak XOR recovery, network IOCs, toolchain fingerprinting |
| **Safety** | Rust memory safety + in-memory quarantine. Samples are never executed. ZIP decompression and corpus walks are bounded against zip bombs and symlink poison |

## Architecture

```plaintext
┌──────────────────────────────────────────────────┐
│              Vanguard-RE CLI (vanguard)          │
└────────────────────────┬─────────────────────────┘
                         │
    ┌──────────┬─────────┼─────────┬──────────┐
    ▼          ▼         ▼         ▼          ▼
 Static    Disasm +   Signatures  Network   Crypto
 Triage    Code       (hashes /   IOC       + XOR
           Analysis   builtins)   Extractor recovery
```

## What it surfaces

| Layer | Signals |
| --- | --- |
| **Triage** | PE/ELF/Mach-O headers, ImpHash clusters, entropy / packer hints, capability tags, content-format mix (PE, ZIP, RTF, images, text, known encrypted headers) |
| **Capabilities** | Evidence-backed labels only (`http_client`, `c2_suspect`, `persistence`, `injection`, …). Thin-IAT / delay-load samples can earn `http_client` from exact WinINet name strings when `LoadLibrary`+`GetProcAddress` are present |
| **Toolchain** | Go, Rust, .NET, MSVC Rich header, GCC/MinGW, Delphi, VB6, Nim, AutoIt, PyInstaller |
| **Embedded archives** | Carves ZIPs from resources, recovers passwords from the sample’s own strings (e.g. WannaCry `WNcry@2ol7`), unlocks members in RAM, recurses with bomb limits |
| **Network IOCs** | IPv4 / `ip:port`, URLs, domains, `.onion`, emails, checksum-validated BTC — ranked and noise-filtered |
| **Crypto / XOR** | AES / ChaCha20 / SHA / MD5 / Blowfish / PEM / CryptoAPI constants; deep-dive weak repeating-XOR and reused-keystream recovery (not real ransomware crypto) |
| **Disassembly** | iced-x86 function recovery, interest ranking, technique insights (XOR loops, PEB access, API hashing, …) |

Stdout is analyst-first: banner summary, ranking table, ImpHash clusters, then one merged block per interesting sample. Defaults hide score-0 noise; `--full` dumps everything.

## Workflow

```bash
vanguard /path/to/sample.zip -p infected
vanguard /path/to/malware.exe --password ""
vanguard /path/to/sample.zip --deep 3 --max-deep 8 --disasm-count 8000
vanguard /path/to/sample.zip -p infected --full
```

| Flag | Default | Meaning |
| --- | --- | --- |
| `--password` / `-p` | `infected` | Password for encrypted ZIP archives |
| `--deep` | `3` | Number of top-scoring samples to deep-dive |
| `--max-deep` | `8` | Absolute ceiling on deep-dives (stops tied-score pack explosions) |
| `--disasm-count` | `4000` | Max instructions to decode per deep-dive |
| `--min-deep-score` | `70` | Also deep-dive lower ranks at/above this score, up to `--max-deep` |
| `--full` | off | Keep demoted noise in ranking and print every member / triage block |

## Stress-tested on this site

I drive Vanguard against real packs and publish honest scorecards, including where it under-ranked or mislabeled:

| Sample | What it exercised | Report |
| --- | --- | --- |
| Conti Locker v2 | Native MSVC crypto + ImpHash ranking against leaked source | [Conti analysis](research/conti-locker) |
| WannaCry | Embedded encrypted ZIP carve, password recovery, onion/BTC IOCs, format mix | [WannaCry stress test](research/wannacry) |
| Raccoon Stealer v2 | Thin-IAT / delay-load WinINet stealer pack, ImpHash cluster, deep-dive budget | [Raccoon WIP](research/raccoon-stealer) |

Recent scoring work came directly from those runs: PE-child floors for WannaCry helpers, string-resolved `http_client` for Raccoon-class stealers, and `--max-deep` so a low `--min-deep-score` cannot expand to the whole archive.

## Containment

- **Static-only.** Nothing is executed on the host.
- ZIP members stay in process memory. Never written as runnable files.
- Decompression size, central-directory scans, and host sample size are capped; corpus walks skip symlinks.
- Dynamic analysis (if added later) would use a real microVM, not host exec.

## Status

Public Rust CLI. Build with `cargo build --release` and install the `vanguard` binary. MIT licensed.
