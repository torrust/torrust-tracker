---
doc-type: benchmark-report
parent-issue: 1505
status: completed
last-updated-utc: 2026-06-26 14:00
semantic-links:
  related-artifacts:
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/ISSUE.md
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/post-performance.md
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/aquatic-benchmarking-guide.md
---

# Baseline Performance Report for Issue #1505

> **Status**: `COMPLETED` — baseline established before implementation.

This report captures the announce throughput and latency of the **current** codebase (before the compact peer optimization). The results serve as a comparison point against the [post-implementation report](post-performance.md).

## Methodology

### Benchmark tools

- **UDP**: `aquatic_udp_load_test` (see [aquatic benchmarking guide](aquatic-benchmarking-guide.md) for full commands and setup)
- **HTTP**: TBD (aquatic tools are UDP-only; consider `wrk2`, `oha`, or a custom load test)
- **Microbenchmarks**: `cargo run --package torrust-tracker-swarm-coordination-registry --example bench_peers --release`

### Environment

| Parameter      | Value                                            |
| -------------- | ------------------------------------------------ |
| Machine        | Ubuntu 26.04 LTS                                 |
| CPU            | AMD Ryzen 9 7950X 16-Core Processor (32 threads) |
| RAM            | 61 GiB                                           |
| Kernel         | 7.0.0-22-generic                                 |
| Rust version   | rustc 1.98.0-nightly (8b6558a02 2026-06-20)      |
| Torrust commit | f940543f59fd29020ef21f07bbeb1a196802ed26         |

### Tracker config

Standard production config, or the benchmarking config at `share/default/config/tracker.udp.benchmarking.toml`.

### Scenarios

| ID  | Scenario                                      | Tool                             | Parameters                                     |
| --- | --------------------------------------------- | -------------------------------- | ---------------------------------------------- |
| B1  | UDP announce throughput (low load)            | `aquatic_udp_load_test`          | `announce_peers_wanted=10`, 10s run, 5s window |
| B2  | UDP announce throughput (high load)           | `aquatic_udp_load_test`          | `announce_peers_wanted=74`, 10s run, 5s window |
| B3  | HTTP announce throughput (normal)             | TBD                              | 74 peers/torrent, compact=1                    |
| B4  | Micro-benchmark: Coordinator::peers_excluding | `examples/bench_peers` (release) | 74 peers, limit=74, 100k iterations            |

## Results

### B4 — Coordinator::peers_excluding microbenchmark

Run with `cargo run --package torrust-tracker-swarm-coordination-registry --example bench_peers --release`.

| Peers in swarm | Time (ns/iter) | Per-peer (ns) |
| -------------: | -------------: | ------------: |
|             10 |          93.29 |          9.33 |
|             74 |         421.51 |          5.70 |
|            100 |         400.27 |          4.00 |
|            500 |         423.41 |          0.85 |
|           1000 |         420.42 |          0.42 |

The ~420 ns floor at 74+ peers is dominated by the `BTreeMap` iteration + `Arc::clone` + `Vec::collect`.

### Memory per peer

| Type                 | Size                                                |
| -------------------- | --------------------------------------------------- |
| `Peer` struct        | 96 bytes                                            |
| `Arc<Peer>`          | 8 bytes                                             |
| `Vec<Arc<Peer>>(74)` | 616 bytes stack + 74 × 96 bytes heap = ~7.1 KB heap |
| `CompactPeer` (est)  | 52 bytes (20 PeerId + 32 SocketAddr)                |

### B1/B2 — UDP announce throughput (aquatic_udp_load_test)

Run with `aquatic_udp_load_test` against the Torrust tracker using the
`tracker.udp.benchmarking.toml` config (binds to `0.0.0.0:3000`). Tracker was built
with `cargo build --release`. Load test run for 10 seconds; the 5-second window at the
end is summarized. See the [aquatic benchmarking guide](aquatic-benchmarking-guide.md) for
full setup instructions.

| ID  | `announce_peers_wanted` | Avg responses/s | Connect/s | Announce/s | Scrape/s | Errors/s | Peers/announce |
| --- | ----------------------: | --------------: | --------: | ---------: | -------: | -------: | -------------: |
| B1  |                      10 |      171,579.90 | 85,019.83 |  84,873.04 | 1,687.02 |     0.00 |           7.23 |
| B2  |                      74 |      171,718.89 | 85,084.98 |  84,945.36 | 1,688.55 |     0.00 |          47.58 |

**Key observation**: Increasing `announce_peers_wanted` from 10 to 74 has **no significant
effect** on overall throughput (~171.6k vs ~171.7k responses/second). This suggests the
bottleneck is at the connection/socket layer, not the peer-list iteration or serialization.
The optimization in this issue focuses on the latter, so its impact may not be visible in
E2E UDP benchmarks. The microbenchmark (B4) is the more relevant measurement.

### B3 — HTTP announce benchmark (`packages/http-core/benches`)

**Broken**: The HTTP announce benchmark uses a sync-adapted helper
(`helpers::sync::return_announce_data_once`) that wraps an async call in
`b.iter(|| ...)` instead of `b.to_async(..).iter(...)`. The measured value of
**260 ns/iter** is the cost of creating the future (no awaiting), not the cost
of executing the announce path. This benchmark must be rewritten to use
`b.to_async` with a proper Tokio runtime before it can produce meaningful
before/after comparisons. Tracked as a follow-up in the main issue spec.

### Summary

| ID  | Metric                      | Value      | Unit  |
| --- | --------------------------- | ---------- | ----- |
| B1  | UDP responses/sec (low)     | 171,579.90 | req/s |
| B2  | UDP responses/sec (high)    | 171,718.89 | req/s |
| B4  | `peers_excluding(74 peers)` | 421.51     | ns    |
