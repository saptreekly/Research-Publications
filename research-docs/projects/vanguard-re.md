High-speed, memory-safe static malware triage as an interactive TUI. Samples are never executed: the tool maps binaries in memory, parses PE / ELF / Mach-O formally, ranks candidates, runs signature scans, and supports deep-dive disassembly exploration.

## Problem

Static triage often means juggling hex viewers, disassemblers, and ad-hoc scripts while passworded malware packs spill runnable files onto disk. Vanguard-RE keeps the workflow in one Rust TUI with zero-copy I/O, in-memory ZIP decryption, and containment-first defaults.

## Three pillars

| Pillar | How |
| --- | --- |
| **Speed** | `memmap2` zero-copy I/O + rayon data-parallel bulk scanning |
| **Accuracy** | Formal PE / ELF / Mach-O parsing (`goblin`), ImpHash, Shannon entropy maps, IAT heuristics, iced-x86 disassembly |
| **Safety** | Rust memory safety + in-memory quarantine — samples are never executed |

## Architecture

```plaintext
┌────────────────────────────────────────┐
│         Vanguard-RE TUI (ratatui)      │
└───────────────────┬────────────────────┘
                    │
     ┌──────────────┼──────────────┐
     ▼              ▼              ▼
 Static Triage   Disassembly    Signature Engine
 Header Parser   Call Profiler  (hashes / YARA-X)
```

## Workflow

1. Launch `vanguard`.
2. Menu → **Investigate sample / ZIP** → paste path, set password if needed → Run.
3. Passworded packs (e.g. `infected`) decrypt into RAM only, then rank, YARA-scan, and deep-dive in the UI.

### Keys (selection)

| Key | Action |
| --- | --- |
| ↑↓ / j k | Move / step instructions |
| Enter | Select / run / deep-dive / follow call |
| d | Open function-map disasm explorer |
| [ ] | Previous / next recovered function |
| c | Cycle k-means function cluster filter |
| b | Back to ranking (from deep-dive) |
| Esc / q | Back / quit |

## Containment

- **Static-only** — nothing is executed on the host.
- ZIP members stay in process memory; never written as runnable files.
- Dynamic analysis (if added later) would use a real microVM, not host exec.

## Status

Public Rust TUI. Build with `cargo build --release` and install the `vanguard` binary. MIT licensed.
