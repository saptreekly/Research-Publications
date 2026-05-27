Real-time geospatial data server that streams aircraft positions from OpenSky Network to WebSocket clients. Instead of broadcasting every entity globally, the server filters by client viewport and clusters out-of-view traffic with H3 hex bins.

## Problem

Naive map backends push full global state to every browser tab. At OpenSky scale that wastes bandwidth and makes interactive clients sluggish. This server sends only what the viewport needs plus cluster counts for everything else.

## Architecture

| Module | Language | Role |
| --- | --- | --- |
| `spatial/`, `store/`, `server/`, `seeder/` | Go | H3 indexing, entity storage, WebSocket server, OpenSky ingest |
| `spatial_engine/` | Rust | Spatial query and indexing performance path |
| `frontend_wasm/` | Rust (Wasm) | Browser map client compiled to WebAssembly |
| Demo UI | HTML / Leaflet | Embedded live map for integration testing |

### Data flow

1. Seeder polls OpenSky on an interval (5s authenticated, 10s anonymous).
2. Store applies entity updates and evicts stale tracks after missed polls.
3. Each WebSocket client declares a bounding box and zoom level.
4. Server emits delta messages (`added`, `updated`, `removed`, `clusters`) rather than full snapshots.

### H3 resolution by zoom

| Zoom | H3 resolution | Approx. cell area |
| --- | --- | --- |
| 0–4 | 2 | ~86,700 km² |
| 5–7 | 4 | ~1,770 km² |
| 8–10 | 6 | ~36 km² |
| 11+ | 7 | ~5 km² |

Low zoom shows cluster counts. High zoom resolves individual tracks.

## Protocol sketch

Client viewport declaration:

```json
{
  "type": "viewport",
  "north": 40.8,
  "south": 40.0,
  "east": -73.8,
  "west": -74.2,
  "zoom": 6
}
```

Server delta response includes sequenced entity changes and H3 cluster keys with counts for cells outside the viewport.

## Operational notes

- Viewports crossing the antimeridian split into two polygon queries before union.
- Per-client push rate limiting defaults to 500ms minimum interval.
- OpenSky seeder uses exponential backoff on 429 and 5xx responses.
- Suitable for sub-100k entity workloads. Larger deployments would need quadrant partitioning or snapshot streaming.

## Status

Public repository with integration test script and embedded demo UI. Run with `go run .` and open `http://localhost:8080`.
