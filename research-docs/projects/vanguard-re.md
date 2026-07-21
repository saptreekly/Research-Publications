High-speed, memory-safe static malware triage from the command line. Samples are never executed: the tool maps binaries in memory, parses PE / ELF / Mach-O formally, ranks candidates, runs signature scans, and deep-dives with iced-x86 disassembly.

## Problem

Static triage often means juggling hex viewers, disassemblers, and ad-hoc scripts while passworded malware packs spill runnable files onto disk. Vanguard-RE keeps the workflow in one Rust CLI with zero-copy I/O, in-memory ZIP decryption, and containment-first defaults.

## Three pillars

| Pillar | How |
| --- | --- |
| **Speed** | `memmap2` zero-copy I/O + focused static pipelines |
| **Accuracy** | Formal PE / ELF / Mach-O parsing (`goblin`), ImpHash, Shannon entropy, IAT heuristics, iced-x86 disassembly, crypto fingerprints, network IOCs, toolchain fingerprinting |
| **Safety** | Rust memory safety + in-memory quarantine. Samples are never executed |

## Architecture

```plaintext
┌──────────────────────────────────────────────────┐
│              Vanguard-RE CLI (vanguard)          │
└────────────────────────┬─────────────────────────┘
                         │
    ┌──────────┬─────────┼─────────┬──────────┐
    ▼          ▼         ▼         ▼          ▼
 Static    Disasm +   Signatures  Network   Crypto
 Triage    Code       (hashes /   IOC       Constants
           Analysis   builtins)   Extractor Fingerprints
```

## Workflow

```bash
vanguard /path/to/sample.zip -p infected
vanguard /path/to/malware.exe --password ""
vanguard /path/to/sample.zip --deep 1 --disasm-count 8000
```

Passworded packs (for example `infected`) decrypt into RAM only, then Vanguard ranks members, scans signatures, and prints triage plus deep-dives to stdout.

| Flag | Default | Meaning |
| --- | --- | --- |
| `--password` / `-p` | `infected` | Password for encrypted ZIP archives |
| `--deep` | `3` | Number of top-scoring samples to deep-dive |
| `--disasm-count` | `4000` | Max instructions to decode per deep-dive |
| `--min-deep-score` | `70` | Minimum triage score required for a deep-dive |

## Containment

- **Static-only.** Nothing is executed on the host.
- ZIP members stay in process memory. Never written as runnable files.
- Dynamic analysis (if added later) would use a real microVM, not host exec.

## Status

Public Rust CLI. Build with `cargo build --release` and install the `vanguard` binary. MIT licensed.
