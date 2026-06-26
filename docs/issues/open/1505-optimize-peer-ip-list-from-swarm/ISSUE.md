---
doc-type: issue
issue-type: task
status: planned
priority: p3
github-issue: 1505
spec-path: docs/issues/open/1505-optimize-peer-ip-list-from-swarm/ISSUE.md
branch: "1505-optimize-peer-ip-list-from-swarm"
related-pr: null
last-updated-utc: 2026-06-26 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - issue #1366
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/pre-implementation-analysis.md
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/baseline-performance.md
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/post-performance.md
    - packages/primitives/src/announce.rs
    - packages/primitives/src/peer.rs
    - packages/primitives/src/lib.rs
    - packages/swarm-coordination-registry/src/swarm/coordinator.rs
    - packages/swarm-coordination-registry/src/swarm/registry.rs
    - packages/tracker-core/src/announce_handler.rs
    - packages/tracker-core/src/torrent/repository/in_memory.rs
    - packages/http-core/src/services/announce.rs
    - packages/udp-core/src/services/announce.rs
    - packages/http-protocol/src/v1/responses/announce.rs
    - packages/udp-server/src/handlers/announce.rs
    - packages/axum-http-server/src/v1/handlers/announce.rs
    - packages/tracker-client/src/http/client/responses/announce.rs
---

<!-- skill-link: create-issue -->

# Issue #1505 — Optimization: return peer IP list from swarm (lowest-level layer) to servers (highest-level layer)

> **Important — commit & merge policy**: This issue's artifacts are committed in a strict sequence, each as a separate commit. This ensures each artifact is independently reviewable and that the analysis is preserved regardless of whether the implementation is ultimately merged.
>
> 1. **Commit 1 — Spec documents**: `ISSUE.md`, `pre-implementation-analysis.md`, `baseline-performance.md`, `post-performance.md`. These are committed first regardless of whether the implementation proceeds. They document the analysis, design decisions, and the intended before/after measurement framework.
> 2. **Commit 2 — Baseline performance**: Run benchmarks on the current (unchanged) codebase, fill in `baseline-performance.md`, and commit it. This locks in the measurement before any code changes.
> 3. **Commit 3 — Implementation**: The compact-path code changes. Developed and iterated on the same branch.
> 4. **Commit 4 — Post-implementation performance**: Run the same benchmarks after the implementation, fill in `post-performance.md`, and commit it.
> 5. **Merge decision**: The entire branch may or may not be merged. If the implementation is **not** merged (e.g., no performance improvement or poor code clarity), commits 1–2 are still merged — they serve as a permanent record of why the optimization was considered and rejected, preventing future re-litigation. If the implementation **is** merged, the commit history makes it clear which parts were analysis and which were code.

## Goal

Reduce memory allocation and data copying overhead across the announce call chain by introducing a lightweight `CompactPeer` type at the primitive/domain level and using it from the swarm layer up through the server response builders. The full `peer::Peer` struct (which carries `updated`, `uploaded`, `downloaded`, `left`, `event` — metadata only needed for swarm management, not for announce responses) is currently passed through every layer via `Arc`, and then immediately destructured to extract only the IP address and port (and peer ID for HTTP) for response serialization.

> For the full research that informed this design, see the [Pre-Implementation Analysis](pre-implementation-analysis.md).

## Background

### Current call chain

```text
UDP/HTTP Server Handler
  ⬇️
Service Layer (udp-core / http-core)
  ⬇️
AnnounceHandler (tracker-core)
  ⬇️
InMemoryTorrentRepository
  ⬇️
Swarms (swarm-coordination-registry)
  ⬇️
Coordinator (swarm-coordination-registry)
```

### Current `AnnounceData`

```rust
pub struct AnnounceData {
    pub peers: Vec<Arc<peer::Peer>>,
    pub stats: SwarmMetadata,
    pub policy: AnnouncePolicy,
}
```

`peer::Peer` has seven fields: `peer_id`, `peer_addr`, `updated`, `uploaded`, `downloaded`, `left`, `event`. The response builders only use `peer_id` and `peer_addr` (HTTP normal) or just `peer_addr.ip()` and `peer_addr.port()` (UDP / HTTP compact). The other five fields are purely for swarm management.

## Optimization Design

### New type: `CompactPeer`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactPeer {
    pub peer_id: PeerId,
    pub peer_addr: SocketAddr,
}
```

`Copy`, no `Arc` wrapping, 52 bytes instead of 96.

### Implementation strategy: parallel compact path

Introduce new compact-returning methods alongside existing ones — never modify existing signatures in-place:

1. `Coordinator`: new methods `peers_excluding_compact()` and `peers_compact()` returning `Vec<CompactPeer>`
2. `Registry`: new method `get_peers_peers_excluding_compact()` returning `Vec<CompactPeer>`
3. `InMemoryTorrentRepository`: new method `get_peers_for_compact()` returning `Vec<CompactPeer>`
4. New type `AnnounceDataCompact` (or add `peers_compact` field to `AnnounceData`)
5. Wire compact path through UDP/HTTP service layers
6. UDP and HTTP response builders use the compact path
7. After verification: delete old path, rename compact types back to canonical names

### Design decisions

- **Keep `peer_id` in `CompactPeer`** — simplicity over splitting; only split if benchmarks show a measurable difference
- **IPv4/IPv6 split** (#1366) — out of scope for this issue
- **Parallel path** — enables incremental work, easy rollback, and clear before/after comparison

## Scope

### In Scope

- Add `CompactPeer` struct to `packages/primitives/`
- Add compact-returning methods on `Coordinator`, `Registry`, `InMemoryTorrentRepository`
- Add `AnnounceDataCompact` (or equivalent)
- Wire through UDP and HTTP service/response builder layers
- Remove old path and rename once verified
- Full test suite and benchmark comparison

### Out of Scope

- Splitting `CompactPeer` into variants with/without `peer_id` (deferred)
- IPv4/IPv6 peer list separation (#1366)
- Changing swarm internal storage or `peer::Peer` struct
- Removing `Arc<peer::Peer>` from swarm storage

## Follow-up Issues

### IPv6 support in tracker-client `CompactPeer`

The `tracker-client` crate (`packages/tracker-client/src/http/client/responses/announce.rs`) has its own `CompactPeer` struct that only supports IPv4 (it panics on IPv6). The HTTP tracker server already supports IPv6 compact peers via the `peers6` key (BEP 7), and the new domain-level `CompactPeer` (introduced in this issue) is IP-version-agnostic using `SocketAddr`.

If the `tracker-client` needs to fully deserialize HTTP tracker responses containing IPv6 compact peers, a follow-up should extend or replace the client-side `CompactPeer` to support both `peers` (IPv4) and `peers6` (IPv6) keys. This is **not** required for the server-side optimization in this issue — the server response builders already handle both IPv4 and IPv6 correctly. The follow-up is a client-side concern.

## Memory Impact

| Config   | Current                         | Proposed               |
| -------- | ------------------------------- | ---------------------- |
| Per peer | 96 bytes (stack) + Arc overhead | 52 bytes (stack, Copy) |
| 74 peers | ~7 KB heap + ~600 B stack       | ~4 KB stack contiguous |

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                               | Notes                                                            |
| --- | ------ | -------------------------------------------------- | ---------------------------------------------------------------- |
| T1  | TODO   | Add `CompactPeer` struct to `packages/primitives/` | New file; `From<&peer::Peer>` and `From<peer::Peer>` conversions |
| T2  | TODO   | Add compact methods to `Coordinator`               | `peers_excluding_compact()`, `peers_compact()`                   |
| T3  | TODO   | Add compact method to `Registry`                   | `get_peers_peers_excluding_compact()`                            |
| T4  | TODO   | Add compact method to `InMemoryTorrentRepository`  | `get_peers_for_compact()`                                        |
| T5  | TODO   | Add `AnnounceDataCompact`                          | Same as `AnnounceData` with `Vec<CompactPeer>`                   |
| T6  | TODO   | Wire UDP service + handler                         | New method on UDP `AnnounceService`                              |
| T7  | TODO   | Wire HTTP service + handler                        | New method on HTTP `AnnounceService`                             |
| T8  | TODO   | Update UDP response builder                        | Uses `AnnounceDataCompact.peers`                                 |
| T9  | TODO   | Update HTTP response builder                       | Uses `AnnounceDataCompact.peers`                                 |
| T10 | TODO   | Cleanup: remove old path, rename                   | Delete old methods; `AnnounceDataCompact` to `AnnounceData`      |
| T11 | TODO   | Run full test suite                                | All targets, all features                                        |
| T12 | TODO   | Run pre-commit checks                              | `./contrib/dev-tools/git/hooks/pre-commit.sh`                    |
| T13 | TODO   | Run benchmark comparison                           | Aquatic bencher (UDP) + microbenchmarks                          |

## Acceptance Criteria

- [ ] AC1: `CompactPeer` struct exists with `From` conversions
- [ ] AC2: Compact methods on Coordinator, Registry, InMemoryTorrentRepository
- [ ] AC3: Compact response data type exists
- [ ] AC4: UDP and HTTP response builders work correctly
- [ ] AC5: Old path removed and compact types renamed back to canonical
- [ ] AC6: Full test suite passes
- [ ] AC7: `linter all` passes
- [ ] AC8: Pre-commit checks pass
- [ ] AC9: Performance baseline and post-implementation reports completed

## Verification Plan

### Manual Verification

| ID  | Scenario               | Steps                                         |
| --- | ---------------------- | --------------------------------------------- |
| M1  | UDP announce works     | Start tracker, `tracker_client udp announce`  |
| M2  | HTTP announce works    | Start tracker, `tracker_client http announce` |
| M3  | Both HTTP formats work | Query with `compact=0` and `compact=1`        |
| M4  | Benchmark comparison   | Aquatic bencher before vs after               |

## Risks and Trade-offs

- **No measurable improvement**: The optimization reduces memory and indirection but the bottleneck may be elsewhere (mutex contention, serialization/IO). If benchmarks show no improvement, the change is still worthwhile for code clarity (interfaces no longer promise data they don't deliver).
- **Backward compatibility**: `AnnounceData.peers` type changes. Acceptable for `3.0.0-develop`.
- **Lock contention unchanged**: The coordinator lock is released before response building regardless.

## Related documents

- [Pre-Implementation Analysis](pre-implementation-analysis.md) — detailed research findings for all design decisions
- [Baseline Performance](baseline-performance.md) — benchmark results before the change (to be filled)
- [Post-Implementation Performance](post-performance.md) — benchmark results after the change (to be filled)

## References

- GitHub issue: [#1505](https://github.com/torrust/torrust-tracker/issues/1505)
- Related issue: [#1366](https://github.com/torrust/torrust-tracker/issues/1366)
- BEP 23: [Tracker Returns Compact Peer Lists](https://www.bittorrent.org/beps/bep_0023.html)
- BEP 15: [UDP Tracker Protocol](https://www.bittorrent.org/beps/bep_0015.html)
- Aquatic bench: [Benchmarking the Torrust BitTorrent Tracker](https://torrust.com/blog/benchmarking-the-torrust-bittorrent-tracker)
