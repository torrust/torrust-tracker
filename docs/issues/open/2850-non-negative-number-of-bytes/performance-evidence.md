---
doc-type: performance-evidence
issue-spec: docs/issues/open/2850-non-negative-number-of-bytes/ISSUE.md
last-updated-utc: "2026-11-15 20:50"
---

# Performance Evidence - Non-Negative Number of Bytes

Performance Evidence plan of issue #2850 (see the specification, "Verification Plan"). The baseline
is measured on the code the branch starts from plus the benchmark-only commit of T0; the after
measurement is repeated after T3 on the same machine with the same build profile, configuration and
load-test settings. Only actual results are recorded here.

## Method

Two instruments, following [`docs/benchmarking.md`](../../../benchmarking.md) and the method of
[issue #2458](../../closed/2458-inject-udp-cookie-cipher/performance-evidence.md).

### Microbenchmarks (Criterion)

- `udp_tracker/announce_once/announce_once`, added by T0 in
  `packages/udp-core/benches/udp_tracker_core_benchmark.rs`: one awaited
  `AnnounceService::handle_announce` of a repeat announce on a public tracker with events disabled
  and cookie validation on. It includes the wire-to-domain conversion that T2 makes fallible.
- `http_tracker/announce_once/announce_once` from
  `packages/http-core/benches/http_tracker_core_benchmark.rs`, the existing HTTP announce
  benchmark.
- Commands, run three times in a row (disposable script `.tmp/perf-2850/run-criterion.sh`):

  ```sh
  cargo bench -q -p torrust-tracker-udp-core --bench udp_tracker_core_benchmark -- announce_once
  cargo bench -q -p torrust-tracker-http-core --bench http_tracker_core_benchmark -- announce_once
  ```

- Metric: the median point estimate from each benchmark's
  `target/criterion/<group>/<function>/new/estimates.json`, read after each run with `LC_ALL=C`.

### End-to-End Load Test (`aquatic_udp_load_test`)

- Tracker: `cargo build --release --bin torrust-tracker`, started with the unmodified
  `share/default/config/tracker.udp.benchmarking.toml`; a fresh tracker per run, each in its own
  scratch folder, stopped with `SIGTERM`.
- Load generator: `aquatic_udp_load_test` built from source at aquatic commit `a2ddc4b3`.
- Configuration: the one of issue #2458 (`workers = 1`, `duration = 30`, `summarize_last = 20`,
  four sockets, connect and announce weights 50/50, scrape weight 1, `announce_peers_wanted = 74`).
- Metric: `Average responses per second` from aquatic's report (the last 20 seconds of each run).

### Isolation of the Measured Code

Each measurement is built in its own detached `git worktree` of the measured commit under
`.tmp/perf-2850/`, with its own `target` directory, so edits in the working branch cannot leak into
the build.

A first baseline attempt (20:37 to 20:41 UTC) was built from the working branch while the T1 and T2
edits were already in the working tree: the release binary finished at 20:37:47 UTC and
`packages/udp-core/src/peer_builder.rs` had been edited at 20:36:59 UTC. It is discarded and not
used below.

## Machine

- CPU: AMD Ryzen 9 7950X 16-Core Processor (32 threads); RAM: 61 GiB
- OS: Linux 7.0.0-43-generic
- Toolchain: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `release` profile
- A shared desktop with other agent sessions: the load average was 4.8 to 7.1 during the
  baseline (recorded at the start of each run). No other tracker was running, and UDP `3000` and
  TCP `127.0.0.1:1313` were free.

## Baseline

Commit `7b5dd310f` ("test(udp-core): [#2850] add a UDP announce benchmark"), on `develop` at
`abd7a10ff`. Measured on 2026-11-15.

### Criterion Medians

| Run | Start (UTC) | Load average (1 min) | UDP `announce_once` | HTTP `announce_once` |
| --- | --- | --- | --- | --- |
| 1 | 20:44:11 | 4.84 | 284.90 ns | 338.57 ns |
| 2 | 20:45:56 | 5.06 | 286.77 ns | 336.82 ns |
| 3 | 20:46:07 | 5.06 | 303.52 ns | 342.05 ns |

Spread: UDP 284.90 to 303.52 ns (6.5%); HTTP 336.82 to 342.05 ns (1.6%).

### Load Test

| Run | Start (UTC) | Load average (1 min) | Average responses per second | Error responses |
| --- | --- | --- | --- | --- |
| 1 | 20:46:15 | 5.12 | 135,528.53 | 0.00 |
| 2 | 20:46:49 | 6.72 | 141,469.34 | 0.00 |
| 3 | 20:47:23 | 7.09 | 145,907.69 | 0.00 |
| 4 | 20:47:57 | 6.49 | 139,970.87 | 0.00 |
| 5 | 20:48:31 | 6.71 | 142,644.38 | 0.00 |

Mean 141,104.16; lowest run 135,528.53; spread 7.7%.

## Pass Rule

From the specification:

- the after mean of the load test is not below the lowest baseline run (135,528.53);
- the after HTTP `announce_once` median is not above the highest baseline median (342.05 ns);
- the after UDP `announce_once` median is not above the highest baseline median (303.52 ns) plus a
  tolerance for the three sign checks T2 adds. The tolerance is agreed with the maintainer before
  the after measurement and recorded here.

## After

Not measured yet.
