---
doc-type: benchmark-report
parent-issue: 1505
status: completed
last-updated-utc: 2026-06-26 16:30
semantic-links:
  related-artifacts:
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/ISSUE.md
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/baseline-performance.md
    - docs/issues/open/1505-optimize-peer-ip-list-from-swarm/aquatic-benchmarking-guide.md
---

# Post-Implementation Performance Report for Issue #1505

> **Status**: `COMPLETED` — implementation rejected due to performance regression.

This report captures the announce throughput and latency after the compact peer optimization
was implemented. Compare with the [baseline report](baseline-performance.md).

## Methodology

Same methodology as the [baseline](baseline-performance.md#methodology) — identical tools,
environment, config, and scenarios. The comparison focuses on the microbenchmark (B4) since
the E2E UDP load test results are bottlenecked at the connection/socket layer and were
unaffected by the optimization at the swarm level.

## Results

### B4 — Coordinator::peers_excluding vs peers_excluding_compact

Run with `cargo run --package torrust-tracker-swarm-coordination-registry --example bench_peers --release`.

| Peers | Old (ns) | Compact (ns) | Delta (ns) | Speedup |
| ----: | -------: | -----------: | ---------: | ------: |
|    10 |    93.17 |       179.53 |     −86.37 |  0.52×  |
|    74 |   407.23 |       823.54 |    −416.32 |  0.49×  |
|   100 |   406.67 |       839.87 |    −433.20 |  0.48×  |
|   500 |   423.87 |       864.57 |    −440.69 |  0.49×  |
|  1000 |   424.05 |       869.43 |    −445.38 |  0.49×  |

### Analysis

The compact path is **~2× slower** than the old `Arc<Peer>` path. The root cause:

- **Old path**: `peers_excluding` calls `.cloned()` on each `Arc<peer::Peer>` in the `BTreeMap`.
  `Arc::clone` is an atomic refcount increment + 8-byte pointer copy — very cheap.
- **Compact path**: `peers_excluding_compact` calls `.map(|peer| CompactPeer::from(peer.as_ref()))`.
  `CompactPeer::from` copies the full 52 bytes (20 PeerId + 32 SocketAddr) for each peer.
  The iteration still dereferences the `Arc` to access the underlying `Peer`.

**Why the expected benefit didn't materialize**: The pre-implementation analysis (R2) correctly
identified that no `Peer` cloning occurs in the old path — only `Arc` clones. The optimization
adds a conversion cost (52-byte copy per peer) at the swarm layer without the compensating
benefit (simpler response builder), because the benefit would only appear downstream if the
swarm stored `CompactPeer` directly. The parallel path adds overhead but not enough
downstream savings to offset it.

### B1–B3 — E2E benchmarks

No meaningful delta expected for B1–B3. The E2E UDP throughput is bottlenecked at the
connection/socket layer (as established in the baseline report). The HTTP announce
microbenchmark is broken (see ISSUE.md follow-up). Skipped.

## Summary

| ID  | Metric                      | Baseline | After | Delta  |
| --- | --------------------------- | -------- | ----- | ------ |
| B4  | `peers_excluding` (74 peers) | 407 ns  | 824 ns | **−49%** |

## Verdict

- [ ] Performance improved significantly (merge implementation)
- [ ] Performance unchanged within noise (merge for code clarity improvements)
- [x] Performance regressed (do not merge; document why)

**Decision**: The implementation is **rejected**. The compact path adds conversion overhead
at the swarm layer without sufficient downstream savings to compensate. The 2× slowdown is
not acceptable. The spec documents, baseline measurements, and this report serve as a
permanent record to prevent future re-litigation of this approach.
