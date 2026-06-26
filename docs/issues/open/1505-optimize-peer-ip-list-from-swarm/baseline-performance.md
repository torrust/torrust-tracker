---
doc-type: benchmark-report
parent-issue: 1505
status: pending
last-updated-utc: 2026-06-26 12:00
semantic-links:
  related-artifacts:
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/ISSUE.md
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/post-performance.md
---

# Baseline Performance Report for Issue #1505

> **Status**: `PENDING` — run this before starting implementation to establish a baseline.

This report captures the announce throughput and latency of the **current** codebase (before the compact peer optimization). The results serve as a comparison point against the [post-implementation report](post-performance.md).

## Methodology

### Benchmark tools

- **UDP**: aquatic bencher (see [pre-implementation analysis](pre-implementation-analysis.md#r4-aquatic-bencher-and-benchmarking-setup) for setup)
- **HTTP**: TBD (aquatic bencher is UDP-only; consider `wrk2`, `oha`, or a custom load test)
- **Microbenchmarks**: `cargo bench --package torrust-tracker-torrent-repository`

### Environment

| Parameter      | Value |
| -------------- | ----- |
| Machine        | TBD   |
| CPU            | TBD   |
| RAM            | TBD   |
| Kernel         | TBD   |
| Rust version   | TBD   |
| Torrust commit | TBD   |

### Tracker config

Standard production config, or the benchmarking config at `share/default/config/tracker.udp.benchmarking.toml`.

### Scenarios

| ID  | Scenario                            | Tool            | Parameters                      |
| --- | ----------------------------------- | --------------- | ------------------------------- |
| B1  | UDP announce throughput (low load)  | aquatic bencher | 10 peers/torrent, 100 torrents  |
| B2  | UDP announce throughput (high load) | aquatic bencher | 74 peers/torrent, 1000 torrents |
| B3  | HTTP announce throughput (normal)   | TBD             | 74 peers/torrent, compact=1     |
| B4  | Micro-benchmark: swarm get_peers    | `cargo bench`   | n/a                             |

## Results

| ID  | Metric                | Value | Unit  |
| --- | --------------------- | ----- | ----- |
| B1  | Announce requests/sec | TBD   | req/s |
| B2  | Announce requests/sec | TBD   | req/s |
| B3  | Announce requests/sec | TBD   | req/s |
| B4  | Swarm iteration time  | TBD   | ns    |

_Fill in after running benchmarks._
