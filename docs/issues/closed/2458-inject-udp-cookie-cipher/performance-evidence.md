---
doc-type: performance-evidence
issue-spec: docs/issues/closed/2458-inject-udp-cookie-cipher/ISSUE.md
last-updated-utc: "2026-10-07 10:06"
---

# Performance Evidence - UDP Cookie Cipher Before and After Injection

Acceptance criterion AC7 of issue #2458 (Performance Considerations). P1 is measured on the code
before any production change; P2 is measured after the fix, on the same machine with the same
build profile, tracker configuration, and load-test settings. Only actual results are recorded
here.

## Method

Two instruments, following issues #2314 and #2342 and [`docs/benchmarking.md`](../../../benchmarking.md).

### Microbenchmarks (Criterion)

- `udp_tracker/connection_cookie/make` and `udp_tracker/connection_cookie/check` from
  `packages/udp-core/benches/connection_cookie_benchmark.rs`: one call each, with a realistic
  fingerprint and a 240-second valid range.
- `udp_tracker/connect_once/connect_once` from
  `packages/udp-core/benches/udp_tracker_core_benchmark.rs`: one awaited
  `ConnectService::handle_connect` with events disabled, which is the client fingerprint plus
  `make`.
- Command, run three times in a row:

  ```sh
  cargo bench -q -p torrust-tracker-udp-core --bench connection_cookie_benchmark --bench udp_tracker_core_benchmark
  ```

- Metric: the median point estimate from each benchmark's
  `target/criterion/<group>/<function>/new/estimates.json`, read after each run by this disposable
  script (`.tmp/perf-2458/criterion-medians.py`):

  ```python
  import json, sys
  base = 'target/criterion'
  parts = []
  for name, path in [('make', 'udp_tracker_connection_cookie/make'),
                     ('check', 'udp_tracker_connection_cookie/check'),
                     ('connect_once', 'udp_tracker_connect_once/connect_once')]:
      e = json.load(open(f'{base}/{path}/new/estimates.json'))
      parts.append(f"{name} median {e['median']['point_estimate']:.2f} ns")
  print(f"{sys.argv[1]} run {sys.argv[2]}: " + '; '.join(parts))
  ```

### End-to-End Load Test (`aquatic_udp_load_test`)

- Tracker: `cargo build --release --bin torrust-tracker`, started with the unmodified
  `share/default/config/tracker.udp.benchmarking.toml` (UDP on `0.0.0.0:3000`, `error` logging,
  usage statistics off, `connection_id_validation = "strict"`). A fresh tracker per run, in its own
  scratch folder so the relative SQLite path stays out of the repository; each run ends with
  `SIGTERM`.
- Load generator: `aquatic_udp_load_test` built from source (`cargo build --release -p
  aquatic_udp_load_test`) at aquatic commit `a2ddc4b3`, the commit issue #2342 used.
- Load-test configuration, the same as issue #2342 (connect and announce weights 50/50, so `make`
  and `check` each run for about half of the requests):

  ```toml
  server_address = "127.0.0.1:3000"
  log_level = "error"
  workers = 1
  duration = 30
  summarize_last = 20
  extra_statistics = true

  [network]
  multiple_client_ipv4s = true
  sockets_per_worker = 4
  recv_buffer = 8000000

  [requests]
  number_of_torrents = 1000000
  number_of_peers = 2000000
  scrape_max_torrents = 10
  announce_peers_wanted = 74
  weight_connect = 50
  weight_announce = 50
  weight_scrape = 1
  peer_seeder_probability = 0.75
  ```

- Disposable run loop (`.tmp/perf-2458/run-load-tests.sh`, called as `run-load-tests.sh p1 5`):

  ```bash
  #!/bin/bash
  # Usage: run-load-tests.sh <label> <runs>
  # Runs the documented E2E UDP load test <runs> times, each with a fresh tracker.
  set -euo pipefail
  label="$1"; runs="$2"
  repo="$(git rev-parse --show-toplevel)"
  here="$repo/.tmp/perf-2458"
  aquatic="$HOME/Documents/git/greatest-ape/aquatic/target/release/aquatic_udp_load_test"
  for i in $(seq 1 "$runs"); do
    scratch="$here/scratch-$label-$i"; rm -rf "$scratch"; mkdir -p "$scratch"
    (cd "$scratch" && exec "$repo/target/release/torrust-tracker" \
        --config-toml-path "$repo/share/default/config/tracker.udp.benchmarking.toml" \
        >"$scratch/tracker.log" 2>&1) &
    tracker_pid=$!
    sleep 3
    "$aquatic" -c "$here/load-test-config.toml" >"$here/$label-run-$i.txt" 2>&1
    kill -TERM "$tracker_pid"; wait "$tracker_pid" || true
    echo "$label run $i: $(grep -m1 'Responses in' "$here/$label-run-$i.txt")"
  done
  ```

- Metric: `Average responses per second` from aquatic's report, which covers the last 20 seconds
  of each run.

### Pass Rule (AC7)

One-sided, as the issue #2342 retrospective recommends:

- the P2 mean responses per second is not below the lowest P1 run; and
- each P2 Criterion median (the median of three P2 runs) is not above the highest of the three P1
  medians.

A failure is investigated before the issue closes.

## Machine

- CPU: AMD Ryzen 9 7950X 16-Core Processor (16 cores, 32 logical CPUs)
- Memory: 61 GiB
- OS: Linux, kernel `7.0.0-34-generic` x86_64
- Socket buffers: `net.core.rmem_max = 4194304`, `net.core.rmem_default = 212992` (the load
  test's 8 MB `recv_buffer` request is capped by `rmem_max`)
- Toolchain: stable `rustc 1.99.0 (b940084d7 2026-09-28)`; `bench` and `release` Cargo profiles
- Background load: a shared desktop running other development sessions. Load average 2.25 before
  the P1 load tests and 9.51 right after the P1 Criterion runs.

## B1 - `connect_once` Before and After the Repair

Before the repair, the benchmark passed the `async fn` helper to `b.iter` without awaiting it, so
Criterion timed only the creation of a future. Run on `develop` `7836471b3`, before the
`perf(udp-core): [#2458] benchmark connection cookies and repair connect_once` commit:

```text
$ cargo bench -p torrust-tracker-udp-core --bench udp_tracker_core_benchmark
Benchmarking udp_tracker/connect_once/connect_once: Collecting 100 samples in estimated 1.0000 s (287M iterations)
udp_tracker/connect_once/connect_once
                        time:   [3.4834 ns 3.4874 ns 3.4930 ns]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
```

3.49 ns was meant to be the time of 100 connects; one Blowfish block encryption alone takes longer.
After the repair, on the same machine minutes later:

```text
$ cargo bench -p torrust-tracker-udp-core --bench udp_tracker_core_benchmark --bench connection_cookie_benchmark
udp_tracker/connection_cookie/make
                        time:   [44.590 ns 44.674 ns 44.769 ns]
udp_tracker/connection_cookie/check
                        time:   [47.265 ns 47.337 ns 47.415 ns]
udp_tracker/connect_once/connect_once
                        time:   [57.466 ns 57.567 ns 57.681 ns]
                        change: [+1549.5% +1554.4% +1560.5%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

The "regression" is the benchmark starting to measure the connect: about 45 ns for `make` plus the
fingerprint hash.

## P1 - Baseline Before Any Production Change

Code: the implementation branch at the
`perf(udp-core): [#2458] benchmark connection cookies and repair connect_once` commit, which is
`develop` `7836471b3` plus documentation and benchmark-only commits. No production code differs
from `develop`.

### P1 Load Test

Date: 2026-10-07, 09:22-09:25 UTC.

| Run | Responses/s | Connect/s | Announce/s | Scrape/s | Errors/s |
| --- | ----------- | --------- | ---------- | -------- | -------- |
| 1 | 154460.07 | 76458.41 | 76457.23 | 1544.42 | 0.00 |
| 2 | 154267.97 | 76359.23 | 76365.43 | 1543.31 | 0.00 |
| 3 | 155558.76 | 76998.32 | 77004.77 | 1555.68 | 0.00 |
| 4 | 154546.83 | 76507.32 | 76493.45 | 1546.06 | 0.00 |
| 5 | 142155.29 | 70375.79 | 70354.77 | 1424.73 | 0.00 |

- Mean: 152197.78 responses/s.
- Min-max spread: 142155.29 - 155558.76, about 8.8% of the mean. Run 5 is the outlier; runs 1-4
  are within 0.9% of each other.
- Every tracker log was empty: no `error` or `panic` lines.

### P1 Criterion

Date: 2026-10-07, 09:26 UTC.

| Run | `make` median | `check` median | `connect_once` median |
| --- | ------------- | -------------- | --------------------- |
| 1 | 43.54 ns | 46.77 ns | 58.71 ns |
| 2 | 49.93 ns | 48.30 ns | 58.51 ns |
| 3 | 43.67 ns | 46.88 ns | 59.10 ns |

- Highest P1 medians, the P2 limits: `make` 49.93 ns, `check` 48.30 ns, `connect_once` 59.10 ns.
- Noise: `make` varied by about 14% across runs (run 2 is the outlier), `check` by about 3%, and
  `connect_once` by about 1%. The machine load rose during these runs (see Machine).
- An earlier set of three runs, made just before, was discarded because the median script used the
  wrong Criterion folder names (`udp_tracker/...` instead of `udp_tracker_...`), and Criterion
  keeps only the latest estimates. Their mean estimates from the `time:` lines were `make` 45.26,
  44.63, and 45.41 ns; `check` 49.09, 47.30, and 46.94 ns; and `connect_once` 58.89, 59.21, and
  60.19 ns, consistent with the recorded runs.

## P2 - After the Fix

Code: the implementation branch at
`fix(udp-core): [#2458] remove the global key statics and the startup seed check`, the last
production change. Release build made right before the runs. Same machine, toolchain, tracker
configuration, load-test configuration, and run loop (`run-load-tests.sh p2 5`).

The benchmark code differs from P1 only where the new signatures require it:
`connection_cookie_benchmark` builds one `CookieCipher::random()` outside the measured loop and
passes it to `make` and `check`, and the `connect_once` helper passes a random key to
`ConnectService::new`.

The first attempt was postponed: at 09:57 UTC other development sessions on the machine were
compiling (`rustc` processes using several cores; load average 18.37). The runs started once the
1-minute load average was back to 2.45, close to the P1 conditions.

### P2 Load Test

Date: 2026-10-07, 10:01-10:04 UTC. Load average 2.15 at the start and 5.88 at the end.

| Run | Responses/s | Connect/s | Announce/s | Scrape/s | Errors/s |
| --- | ----------- | --------- | ---------- | -------- | -------- |
| 1 | 155147.22 | 76784.86 | 76809.75 | 1552.61 | 0.00 |
| 2 | 159091.53 | 78736.25 | 78759.98 | 1595.29 | 0.00 |
| 3 | 145576.51 | 72069.46 | 72051.65 | 1455.41 | 0.00 |
| 4 | 157456.07 | 77931.80 | 77947.24 | 1577.03 | 0.00 |
| 5 | 154032.94 | 76249.84 | 76242.18 | 1540.92 | 0.00 |

- Mean: 154260.85 responses/s.
- Min-max spread: 145576.51 - 159091.53, about 8.8% of the mean.
- Every tracker log was empty: no `error` or `panic` lines.

### P2 Criterion

Date: 2026-10-07, 10:04 UTC. Load average about 5.8 during the runs.

| Run | `make` median | `check` median | `connect_once` median |
| --- | ------------- | -------------- | --------------------- |
| 1 | 43.99 ns | 46.56 ns | 60.06 ns |
| 2 | 45.55 ns | 47.57 ns | 59.05 ns |
| 3 | 44.77 ns | 46.99 ns | 58.98 ns |

## Comparison

| Measurement | P1 | P2 | Limit (pass rule) | Result |
| --- | --- | --- | --- | --- |
| Load test, mean responses/s | 152197.78 | 154260.85 | not below 142155.29 (lowest P1 run) | Pass |
| `make`, median of three runs | 43.67 ns | 44.77 ns | not above 49.93 ns (highest P1 median) | Pass |
| `check`, median of three runs | 46.88 ns | 46.99 ns | not above 48.30 ns | Pass |
| `connect_once`, median of three runs | 58.71 ns | 59.05 ns | not above 59.10 ns | Pass |

- AC7 passes: no measurement crosses its one-sided limit.
- The load-test mean is 1.4% above P1, and both spreads are about 8.8%, so the end-to-end test
  shows no regression but can only rule out large ones.
- The microbenchmarks are more precise. `check` is unchanged (+0.2%). `make` moved +2.5%, within
  its P1 noise of about 14%. `connect_once` moved +0.6%, within its P1 noise of about 1%. Both
  Criterion sessions ran on a loaded desktop (P1 ended at load 9.51, P2 ran at about 5.8). None of these differences
  is attributed to the change: replacing a `LazyLock` read with a reference adds no work, and the
  key schedule is still built once.
