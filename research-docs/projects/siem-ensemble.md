High-performance log ingestion and real-time analytics built as a polyglot ensemble. Rust handles ingestion throughput, Zig moves frames into shared memory with minimal copying, Odin consumes the ring buffer for low-latency analysis, and Elixir supervises process lifecycle and recovery.

## Problem

Single-language SIEM stacks often trade ingestion speed against analytics flexibility. This project splits the pipeline by responsibility and connects components through a strict binary wire format over shared memory so each stage can be optimized independently.

## Architecture

```plaintext
TCP ingest (Rust) → dedupe/cache → Zig forwarder → shared memory ring
                                                      ↓
                                            Odin analytics engine
                                                      ↓
                                            Elixir control plane (supervision, API)
```

| Component | Language | Role |
| --- | --- | --- |
| Core performer | Rust | TCP log ingestion, FNV-1a deduplication, dispatch |
| Forwarder | Zig | Zero-copy routing into `/tmp/siem_shm.bin` circular buffer |
| Analytics | Odin | Structured log consumption and real-time event processing |
| Control plane | Elixir | Orchestration, crash recovery, health and threshold API |
| VM-Exit / low-level paths | Assembly | Platform-specific hot paths where required |

## Wire protocol

Rust and Odin share a fixed-layout `ShmFrame` struct for byte-level compatibility across the shared-memory boundary:

```rust
#[repr(C)]
pub struct ShmFrame {
    pub timestamp: i64,
    pub severity: [u8; 24],
    pub source_ip: [u8; 24],
    pub facility: [u8; 24],
    pub message: [u8; 24],
}
```

Ingestion uses a richer `LogEvent` layout internally. Frames are normalized before crossing into shared memory to keep the binary contract stable.

## Resilience

The Elixir supervisor traps exits from the Rust performer. On crash it cleans stale Unix domain socket artifacts, restarts the core process, and logs the failure for diagnosis. The Makefile exposes `build`, `run`, `stop`, and `stress-test` targets for local orchestration and load testing.

## Requirements

- Rust (Cargo)
- Zig 0.16+
- Odin (2026-05+)
- Elixir/Erlang OTP 27+

## Status

Active development on macOS. See the repository for Makefile workflows and extension notes when modifying the wire protocol.
