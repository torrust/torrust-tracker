---
doc-type: research-report
parent-issue: 1505
status: completed
last-updated-utc: 2026-07-15
semantic-links:
  related-artifacts:
    - docs/issues/closed/1505-optimize-peer-ip-list-from-swarm/ISSUE.md
    - packages/primitives/src/announce.rs
    - packages/primitives/src/peer.rs
    - packages/swarm-coordination-registry/src/swarm/coordinator.rs
    - packages/swarm-coordination-registry/src/swarm/registry.rs
    - packages/tracker-core/src/announce_handler.rs
    - packages/http-protocol/src/v1/responses/announce.rs
    - packages/udp-server/src/handlers/announce.rs
    - packages/tracker-client/src/http/client/responses/announce.rs
    - packages/axum-http-server/src/v1/handlers/announce.rs
---

# Pre-Implementation Analysis for Issue #1505

This document records the research findings that informed the design decisions in the [main issue spec](ISSUE.md). It answers the "why" behind the implementation strategy.

> **Status**: All research topics (R1–R4) are complete. See the decision log at the bottom of this document for a summary.

---

## R1: CompactPeer IPv4/IPv6 support

**Question**: Should `CompactPeer` support both IPv4 and IPv6, or only IPv4?

The existing `CompactPeer` in `packages/tracker-client/src/http/client/responses/announce.rs` (line 79) uses `Ipv4Addr` and panics if given an IPv6 address:

```rust
pub struct CompactPeer {
    ip: Ipv4Addr,
    port: u16,
}

// ...
IpAddr::V6(_ip) => panic!("IPV6 is not supported for compact peer"),
```

### BEP findings

**BEP 23 (Tracker Returns Compact Peer Lists)**: Defines compact format as 6 bytes per peer (4 bytes IPv4 + 2 bytes port). Only IPv4. No IPv6.

**BEP 7 (IPv6 Tracker Extension)**: Adds a `peers6` key to HTTP tracker responses. Compact format uses 18 bytes per peer (16 bytes IPv6 + 2 bytes port). The original `peers` key remains IPv4-only (6 bytes per peer).

**BEP 15 (UDP Tracker Protocol)**: IPv4 announces use 6-byte stride per peer. IPv6 announces use 18-byte stride per peer. The format is determined by the address family of the underlying UDP packet. Both IPv4 and IPv6 are supported in the protocol, layered by the transport.

### Current Torrust tracker implementation

- `packages/http-protocol/src/v1/responses/announce.rs`: The `CompactPeer` is an `enum` with `V4(CompactPeerData<Ipv4Addr>)` and `V6(CompactPeerData<Ipv6Addr>)` variants — it handles **both** IPv4 and IPv6 correctly for the HTTP protocol layer.
- `packages/udp-server/src/handlers/announce.rs`: The `build_response` function checks `remote_addr.is_ipv4()` and creates different `ResponsePeer` types for IPv4 and IPv6 — both are supported.
- `packages/tracker-client/src/http/client/responses/announce.rs`: The `CompactPeer` uses `Ipv4Addr` and panics on IPv6. This is a **client-side** deserialization struct that only handles the `peers` (IPv4 compact) key from BEP 23, not the `peers6` key from BEP 7. This is a separate concern from the domain-level `CompactPeer`.
- `packages/axum-http-server/tests/server/responses/announce.rs`: Same pattern — test `CompactPeer` uses `Ipv4Addr` and panics on IPv6. Tests exist for IPv6 in dictionary (normal) format but not in compact format for the test client struct.

### Decision

The new domain-level `CompactPeer` will use `peer_addr: SocketAddr`, which is IP-version-agnostic. It will not split into IPv4/IPv6 at the domain level — that partitioning is a protocol-layer concern (BEP 7 `peers` vs `peers6`, UDP v4 vs v6 format).

---

## R2: Arc usage and data copying analysis

**Question**: How is `peer::Peer` data currently passed between layers? Is it via `Arc` (shared, no copy) or cloned?

### How data flows from swarm to response builder

1. **Coordinator internal storage**: `BTreeMap<SocketAddr, Arc<PeerAnnouncement>>`. Peers are stored as `Arc`-wrapped full `Peer` structs.
2. **`Coordinator::peers_excluding`** (coordinator.rs:68): Calls `.cloned()` on each `Arc<peer::Peer>` value — this **clones the `Arc`** (increments the reference count), **not the `Peer` data itself**. The `Peer` stays in its heap allocation.
3. **`Registry::get_peers_peers_excluding`** (registry.rs:211): Acquires the swarm lock (`swarm_handle.lock().await`), calls `swarm.peers_excluding(...)`, then the lock guard `swarm` is dropped when the function returns. **The lock is released before the peer vector is passed up the call chain.** This is critical — it means the lock is NOT held during response building.
4. **`InMemoryTorrentRepository::get_peers_for`** (in_memory.rs): Passes through the result unchanged (no clones).
5. **`AnnounceHandler::build_announce_data`** (announce_handler.rs:220): Constructs `AnnounceData { peers, stats, policy }`. The peers vector is **moved**, not cloned.
6. **HTTP path**: `to_protocol_announce_data` (axum-http-server/src/v1/handlers/announce.rs:104) iterates the `Vec<Arc<peer::Peer>>`, dereferences each `Arc` to access `peer.peer_id` and `peer.peer_addr`, and creates new `responses::announce::Peer` values. The `Arc` is consumed/moved, and the underlying `Peer` allocation is dropped when the `Arc` is dropped.
7. **UDP path**: `build_response` (udp-server/src/handlers/announce.rs) iterates `announce_data.peers`, dereferences each `Arc` for `peer.peer_addr.ip()` and `peer.peer_addr.port()`.

### Key insight — no `Peer` cloning occurs

The full `Peer` struct (80+ bytes) is **never copied** during announce processing. The `Arc` clone is cheap (just a refcount increment + pointer copy). The `Peer` data lives on the heap and is shared across all concurrent requests for the same peer — it's read-only at that point.

### What the optimization actually buys us

| Aspect                               | Current (`Vec<Arc<Peer>>`)                       | Proposed (`Vec<CompactPeer>`)                 | Benefit                    |
| ------------------------------------ | ------------------------------------------------ | --------------------------------------------- | -------------------------- |
| Heap allocation                      | `Peer` on heap (96 bytes) + `Arc` control block  | No heap — `CompactPeer` is `Copy`             | Reduced allocator pressure |
| Per-peer data carried through layers | Pointer to full `Peer` (96 bytes reachable)      | `CompactPeer` (52 bytes, no indirection)      | Smaller working set        |
| Cache locality                       | `Vec<Arc>` → dereference → heap → `Peer` data    | `Vec<CompactPeer>` — contiguous in memory     | Better cache behavior      |
| Lock timing                          | Lock released before response building (same)    | Lock released before response building (same) | No change                  |
| Arc refcount contention              | Multiple `Arc` clones across concurrent requests | No refcount operations after conversion       | Less atomic traffic        |
| Memory fragmentation                 | `Peer` allocations scattered across heap         | `CompactPeer` is contiguous in `Vec`          | Better allocator behavior  |

### Conclusion

The performance gain is not from avoiding `Peer` copies (there are none), but from:

- Removing the heap indirection per peer (one less pointer chase)
- Better cache locality from a contiguous `Vec<CompactPeer>` vs following pointers from `Vec<Arc<Peer>>`
- More compact working set (26 bytes/peer vs pointer + 80+ bytes reachable)
- The conversion itself adds work (mapping each `Arc<Peer>` to `CompactPeer`) but this is offset by simpler iteration in the response builder

The parallel compact path strategy (new methods alongside old) is confirmed as the right approach — it lets us benchmark before committing to the change.

---

## R3: AnnounceData.peers usage sites

**Question**: Where is `AnnounceData.peers` used across the entire codebase? Are there consumers that use the extra metadata (`updated`, `uploaded`, `downloaded`, `left`, `event`)?

### Domain `AnnounceData` (from `packages/primitives/src/announce.rs`)

| Location                             | File                                              | How `.peers` is used                                                                                                                        |
| ------------------------------------ | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| AnnounceHandler::build_announce_data | `tracker-core/src/announce_handler.rs:220`        | Returns `AnnounceData` by moving the peer vector in                                                                                         |
| HTTP service                         | `http-core/src/services/announce.rs:81`           | Passes `AnnounceData` through unchanged                                                                                                     |
| UDP service                          | `udp-core/src/services/announce.rs`               | Passes `AnnounceData` through unchanged                                                                                                     |
| HTTP handler                         | `axum-http-server/src/v1/handlers/announce.rs:90` | Calls `to_protocol_announce_data` which maps each `Arc<Peer>` → `Peer { peer_id, peer_addr }` — **only `peer_id` and `peer_addr` are used** |
| UDP handler                          | `udp-server/src/handlers/announce.rs`             | Iterates peers for `peer_addr.ip()` and `peer_addr.port()` — **only `peer_addr` is used**                                                   |
| Tracker-core tests                   | `tracker-core/tests/integration.rs:42`            | Checks `announce_data.peers.len()`                                                                                                          |
| Tracker-core test env                | `tracker-core/tests/common/test_env.rs:99`        | Creates `AnnounceData` for tests                                                                                                            |
| HTTP-core tests                      | `http-core/src/services/announce.rs:432`          | Asserts `AnnounceData` values in tests                                                                                                      |

### Protocol `AnnounceData` (from `packages/http-protocol/src/v1/responses/announce.rs`)

| Location         | File                                             | How `.peers` is used                                  |
| ---------------- | ------------------------------------------------ | ----------------------------------------------------- |
| Normal response  | `http-protocol/src/v1/responses/announce.rs:108` | Maps each `Peer` → `NormalPeer { peer_id, ip, port }` |
| Compact response | `http-protocol/src/v1/responses/announce.rs:145` | Maps each `Peer` → `CompactPeer::V4/V6(ip, port)`     |
| Protocol tests   | `http-protocol/src/v1/responses/announce.rs:340` | Sets up test data                                     |

### Key findings

- **No consumer** uses `updated`, `uploaded`, `downloaded`, `left`, or `event` from `AnnounceData.peers` in the announce response path
- The extra metadata fields are only used within the **swarm management** layer (Coordinator, Registry) and in the **event system** (for statistics/telemetry, sent as separate event messages, not via AnnounceData)
- The `peer::Peer` struct itself is only _constructed_ in the HTTP/UDP service layers (from request parameters), then passed into `AnnounceHandler`, which returns it in `AnnounceData.peers`
- All test code that compares `AnnounceData` values uses `AnnounceData { peers: vec![Arc::new(peer::Peer { ... })] }` — these would need updating to use `CompactPeer`
- The HTTP protocol `AnnounceData` is a **separate** type from the domain one — it's a protocol-level DTO that already only carries `Peer { peer_id, peer_addr }`. The optimization does not affect this type directly.

### Conclusion

The `CompactPeer` type is safe to introduce — it covers every field that any consumer of `AnnounceData.peers` actually needs.

---

## R4: Aquatic bencher and benchmarking setup

**Question**: How to set up and run the aquatic bencher for before/after comparison?

### Aquatic bencher

The aquatic repository can be cloned from `https://github.com/greatest-ape/aquatic`.

**Current state**: The bencher binary has not been built yet (`target/release-debug/` does not exist).

**Requirements from README:**

- Linux 6.0+
- Dependencies: `cmake`, `build-essential`, `pkg-config`, `git`, `screen`, `cvs`, `zlib1g-dev`, `golang`
- Build the bencher:

  ```text
  cd aquatic
  . ./scripts/env-native-cpu-without-avx-512
  cargo build --profile "release-debug" -p aquatic_bencher --features udp
  ```

**Capabilities:**

- Currently **UDP only** (no HTTP tracker benchmarking)
- Benchmarks multiple trackers: aquatic_udp, opentracker, chihaya, torrust-tracker
- Known working commit for torrust-tracker: `eaa86a7` (likely outdated)
- Metrics collected: throughput and latency under load
- Supports `--min-priority medium --cpu-mode subsequent-one-per-pair` for VMs

### Torrust-specific benchmarking assets

- **Config**: `share/default/config/tracker.udp.benchmarking.toml` — disables logging, tracking usage stats, persistent metrics, and peerless torrent removal. Binds UDP tracker to `0.0.0.0:3000`. This is the recommended config for running aquatic bencher against the torrust tracker.
- **Microbenchmarks script**: `contrib/dev-tools/benches/run-benches.sh` — runs `cargo bench` on three packages: `torrust-tracker-torrent-repository`, `torrust-tracker-http-core`, and `torrust-tracker-udp-core`. These are Rust benchmark harnesses (not aquatic), useful for targeted microbenchmarks of specific layers.

### Decision

The bencher setup is deferred to T13 (benchmark comparison). For a quick sanity check, run `cargo bench -p torrent-repository-benchmarking` which tests the coordinator/swarm layer directly.

---

## Decision Log

| ID  | Status | Findings     | Decision                                                                                                                                                                                                                                                                                             |
| --- | ------ | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1  | DONE   | See R1 above | `CompactPeer` will use `peer_addr: SocketAddr` (IP-agnostic). The IPv4-only `CompactPeer` in `tracker-client` is a separate client-side concern.                                                                                                                                                     |
| R2  | DONE   | See R2 above | The optimization gain comes from bypassing `Arc` heap indirection and better cache locality, not from avoiding `Peer` copies (which don't happen). The lock is already released before response building in the current code. The parallel compact path strategy is confirmed as the right approach. |
| R3  | DONE   | See R3 above | No consumer uses the extra `peer::Peer` metadata from `AnnounceData.peers`. A `CompactPeer` is safe to introduce — it provides everything the response builders need.                                                                                                                                |
| R4  | DONE   | See R4 above | The bencher needs to be built first. It currently only supports UDP. A before/after benchmark run can be done once the compact path is complete.                                                                                                                                                     |
