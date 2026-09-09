---
doc-type: performance-evidence
issue: 2149
package: torrust-tracker-udp-server
status: planned
---

# UDP Server Performance Evidence

This document defines the reproducible performance baseline required before changing a UDP server
hot-path production file for Issue #2149. It contains no benchmark result yet: no production code
has changed. The completed request-buffer plan added tests only, so its performance baseline remains
deferred until an approved non-test change affects the hot path.

## Policy

`server/request_buffer.rs` is invoked by `Launcher::run_udp_server_main` for every accepted UDP
request. Any change to its non-test production code requires a baseline before implementation and
an equivalent after measurement before the related commit or pull request.

Test-only changes do not alter the release artifact. They still require focused tests and normal
quality checks, but do not require a throughput measurement unless they change production code,
benchmark configuration, release dependencies, or the runtime workload.

If testing requires a production refactor, stop the current test increment. Record the proposed
production change, obtain maintainer approval, establish the baseline described below, and only
then resume the increment. A main-loop, task-spawning, event-publication, or shutdown-policy change
is outside Issue #2149 and must be coordinated with the relevant UDP lifecycle/main-loop work.

## Measurement Levels

| Level                         | When required                                                                                     | Tool and result                                                                                                                                                                                                                     |
| ----------------------------- | ------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Whole tracker UDP throughput  | Every approved hot-path production change                                                         | Run Aquatic's `aquatic_udp_load_test` against the release-built tracker. Record response rate, response classes, errors, peers per announce, workload config, tracker config, host details, and median of repeated equivalent runs. |
| Request-buffer microbenchmark | An approved change affects `ActiveRequests` algorithm, allocation, capacity, or eviction behavior | Add or use a focused release microbenchmark for capacity available, completed-handle reclamation, and active-handle eviction. Do not infer whole-tracker throughput from it.                                                        |
| Comparative tracker benchmark | Only when assessing tracker competitiveness or a material performance regression                  | Optionally run Aquatic's `aquatic_bencher`; it is not a gate for focused tests because it is expensive and depends on external tracker setup.                                                                                       |

The current repository has no `udp-server`/`ActiveRequests` microbenchmark. Do not add one merely
because a test changes. Add one only when an approved production algorithm change requires a direct
measurement.

## Reproducible Whole-Tracker Baseline

Use the current repository guidance as the source of truth:

1. Build the tracker with `cargo build --release`.
2. Start that artifact with `share/default/config/tracker.udp.benchmarking.toml` through
   `TORRUST_TRACKER_CONFIG_TOML_PATH`.
3. Build the current Aquatic source's `aquatic_udp_load_test` release binary.
4. Generate its configuration with `aquatic_udp_load_test -p`; record the complete workload file
   with the evidence.
5. Run at least three equivalent, fixed-duration iterations after confirming no unrelated local
   workload dominates the host. Record every run and compare medians, not a single observation.
6. Use the same tracker commit/worktree state, Aquatic revision, release profile, host/kernel,
   tracker config, load-test config, CPU-affinity policy, and measurement window for before/after.

The benchmark configuration disables verbose logging and binds UDP to port 3000. Do not compare a
run using a different configuration, logging level, client workload, or host condition as if it
were an A/B result.

## Interpretation Rules

- Treat the expected non-dedicated-host variance of approximately 5–10% as measurement noise until
  repeated median results show otherwise.
- Report before/after response-rate differences as observations, including response and error mix;
  do not declare causation from throughput alone.
- A functional test or coverage increase is not evidence of unchanged performance.
- Historical website articles are background only. Their 2024 commands, environment, tool versions,
  configuration-variable names, and results may be outdated; verify every command against the
  current repository guide and the checked-out Aquatic revision.

## Planned Evidence Table

| Measurement                             | Baseline                                                  | Latest       | Status   | Evidence                                                                      |
| --------------------------------------- | --------------------------------------------------------- | ------------ | -------- | ----------------------------------------------------------------------------- |
| Aquatic UDP load test, release tracker  | Not required until an approved hot-path production change | Not measured | DEFERRED | Completed request-buffer work is test-only; no hot-path production change was approved or implemented. |
| `ActiveRequests` focused microbenchmark | Not applicable; no algorithm change proposed              | Not measured | DEFERRED | Add only after approval of a production algorithm/allocation/capacity change. |

## References

- Canonical guide: `docs/benchmarking.md`
- Current benchmark tracker configuration: `share/default/config/tracker.udp.benchmarking.toml`
- Detailed historical repository guide: `docs/issues/closed/1505-optimize-peer-ip-list-from-swarm/aquatic-benchmarking-guide.md`
- Historical baseline format: `docs/issues/closed/1505-optimize-peer-ip-list-from-swarm/baseline-performance.md`
- Historical website background (verify before use):
  <https://torrust.com/blog/benchmarking-the-torrust-bittorrent-tracker>
- Historical website background (operational packet-path context, not a code benchmark):
  <https://torrust.com/blog/how-we-fixed-a-one-core-packet-processing-bottleneck-in-torrust-tracker>
