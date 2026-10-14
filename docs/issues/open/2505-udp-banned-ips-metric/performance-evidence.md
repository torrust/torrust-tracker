---
doc-type: performance-evidence
issue-spec: docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
last-updated-utc: "2026-10-14 06:59"
---

# Performance Evidence - UDP Banned-IPs Total Before and After the Fix

This file records the measurements planned in the specification's "Performance Evidence Plan":
the B1 instrument check, the P1 baseline before any production change (T3), and later the P2
measurements after the fix (T9) with the comparison against the one-sided pass rule (AC5, AC7).

## Method

### Microbenchmarks (Criterion)

- Benchmark: `packages/udp-core/benches/ban_service_benchmark.rs`, groups
  `udp_ban_service/increase_counter`, `udp_ban_service/is_banned`, and
  `udp_ban_service/record_error` (added by B1). `record_error` times one invalid-connection-ID
  event as the banning handler runs it (`increase_counter` followed by `get_banned_ips_total`) with
  10, 1,000, and 10,000 tracked IPs.
- Each measurement is three full runs of the benchmark binary. After each run, every benchmark's
  median (nanoseconds) is read from `target/criterion/<group>/<function>/new/estimates.json`
  (`.median.point_estimate`). Disposable script (`.tmp/perf-2505/run-criterion.sh`, called as
  `run-criterion.sh p1 3`):

  ```bash
  #!/bin/bash
  # Usage: run-criterion.sh <label> <runs>
  # Runs ban_service_benchmark <runs> times and saves every benchmark median (ns) after each run.
  set -euo pipefail
  label="$1"; runs="$2"
  repo="$(git rev-parse --show-toplevel)"
  here="$repo/.tmp/perf-2505"
  for i in $(seq 1 "$runs"); do
    cargo bench -p torrust-tracker-udp-core --bench ban_service_benchmark >"$here/$label-criterion-run-$i.txt" 2>&1
    (cd "$repo/target/criterion" && find udp_ban_service_* -path '*/new/estimates.json' | sort | while read -r f; do
       printf '%s %s\n' "${f%/new/estimates.json}" "$(jq '.median.point_estimate' "$f")"
     done) >"$here/$label-criterion-medians-$i.txt"
    echo "$label criterion run $i: $(wc -l <"$here/$label-criterion-medians-$i.txt") medians"
  done
  ```

### End-to-End Load Test (`aquatic_udp_load_test`)

- Tracker: `cargo build --release --bin torrust-tracker`, started with the unmodified
  `share/default/config/tracker.udp.benchmarking.toml` (UDP on `0.0.0.0:3000`, `error` logging,
  usage statistics off, `connection_id_validation = "strict"`, limit 10). A fresh tracker per run
  in its own scratch folder; each run ends with `SIGTERM`.
- Load generator: `aquatic_udp_load_test` built from source at aquatic commit `a2ddc4b3`, the
  commit issues #2342 and #2458 used.
- Load-test configuration (the issue #2342 settings):

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

- Disposable run loop (`.tmp/perf-2505/run-load-tests.sh`, called as `run-load-tests.sh p1 5`):

  ```bash
  #!/bin/bash
  # Usage: run-load-tests.sh <label> <runs>
  # Runs the documented E2E UDP load test <runs> times, each with a fresh tracker.
  set -euo pipefail
  label="$1"; runs="$2"
  repo="$(git rev-parse --show-toplevel)"
  here="$repo/.tmp/perf-2505"
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

- Metric: `Average responses per second` from aquatic's report (last 20 seconds of each run).
- Scope: the load test sends only valid connection IDs (every run reports `Error responses: 0.00`),
  so it measures the launcher's `is_banned` check on every request, not the banning error path.
  The error path is covered by `udp_ban_service/record_error`.

### Pass Rule (AC5, AC7)

One-sided, as in the specification:

- the P2 mean `Average responses per second` is not below the lowest P1 run; and
- each P2 Criterion median (the median of three P2 runs) is not above the highest of the three P1
  medians for the same benchmark.

AC7 additionally requires the P2 `udp_ban_service/record_error` medians not to grow with the
number of tracked IPs. A failure is investigated before the issue closes.

## Machine

- CPU: AMD Ryzen 9 7950X 16-Core Processor (16 cores, 32 logical CPUs)
- Memory: 61 GiB
- OS: Ubuntu 26.04.1 LTS, Linux kernel `7.0.0-38-generic` x86_64
- Socket buffers: `net.core.rmem_max = 4194304`, `net.core.rmem_default = 212992`
- Toolchain: stable `rustc 1.99.0 (b940084d7 2026-09-28)`; `bench` and `release` Cargo profiles
- Background load: a shared desktop running other development sessions; load averages are recorded
  with each measurement below.

## B1 - Instrument Check for `udp_ban_service/record_error`

Goal: show that the new benchmark measures the per-event step and would detect a cost that grows
with the number of tracked IPs (an AC7 violation). Both runs used Criterion's `--quick` mode, so
they check the instrument and are not part of the P1/P2 comparison.

1. On `develop` plus the B1 benchmark (before it was formatted with `cargo +nightly fmt`, which
   changed only whitespace). Command:

   ```bash
   uptime; cargo bench -p torrust-tracker-udp-core --bench ban_service_benchmark -- record_error --quick 2>&1 | grep -E "^udp_ban_service|time:|error|warning" | head -30
   ```

   Output (verbatim):

   ```text
    18:53:40 up 4 days, 12:17,  1 user,  load average: 5,15, 4,20, 3,48
      Compiling thiserror v2.0.21
      Compiling thiserror-impl v2.0.21
      Compiling torrust-located-error v3.0.0
   Benchmarking udp_ban_service/record_error/ipv4/10
   Benchmarking udp_ban_service/record_error/ipv4/10: Analyzing
   udp_ban_service/record_error/ipv4/10
                           time:   [11.001 ns 11.044 ns 11.214 ns]
   Benchmarking udp_ban_service/record_error/ipv4/1000
   Benchmarking udp_ban_service/record_error/ipv4/1000: Analyzing
   udp_ban_service/record_error/ipv4/1000
                           time:   [11.491 ns 11.570 ns 11.884 ns]
   Benchmarking udp_ban_service/record_error/ipv4/10000
   Benchmarking udp_ban_service/record_error/ipv4/10000: Analyzing
   udp_ban_service/record_error/ipv4/10000
                           time:   [11.366 ns 11.413 ns 11.602 ns]
   Benchmarking udp_ban_service/record_error/ipv6/10
   Benchmarking udp_ban_service/record_error/ipv6/10: Analyzing
   udp_ban_service/record_error/ipv6/10
                           time:   [14.153 ns 14.270 ns 14.734 ns]
   Benchmarking udp_ban_service/record_error/ipv6/1000
   Benchmarking udp_ban_service/record_error/ipv6/1000: Analyzing
   udp_ban_service/record_error/ipv6/1000
                           time:   [12.735 ns 12.753 ns 12.826 ns]
   Benchmarking udp_ban_service/record_error/ipv6/10000
   Benchmarking udp_ban_service/record_error/ipv6/10000: Analyzing
   udp_ban_service/record_error/ipv6/10000
                           time:   [12.604 ns 12.638 ns 12.771 ns]
   ```

2. With `BanService::get_banned_ips_total` temporarily replaced by the filtered scan P2
   (`self.accurate_error_counter.values().filter(|count| **count > self.max_connection_id_errors_per_ip).count()`),
   then restored with `git checkout -- packages/udp-core/src/services/banning.rs` without staging.
   Command run between the edit and the restore:

   ```bash
   cargo bench -p torrust-tracker-udp-core --bench ban_service_benchmark -- record_error --quick 2>&1 | grep -E "^udp_ban_service/record_error|time:" | paste - - | awk '{print $1, $3, $4, $5, $6, $7}'
   ```

   Output (verbatim; the `awk` field selection drops the closing bracket):

   ```text
   udp_ban_service/record_error/ipv4/10 [16.971 ns 16.973 ns 16.981
   udp_ban_service/record_error/ipv4/1000 [591.49 ns 592.20 ns 595.00
   udp_ban_service/record_error/ipv4/10000 [5.5570 µs 5.5818 µs 5.6810
   udp_ban_service/record_error/ipv6/10 [19.976 ns 19.999 ns 20.088
   udp_ban_service/record_error/ipv6/1000 [592.06 ns 593.47 ns 593.82
   udp_ban_service/record_error/ipv6/10000 [5.6423 µs 5.6873 µs 5.6986
   ```

Conclusion: on `develop` the per-event cost is flat (about 11-14 ns) from 10 to 10,000 tracked
IPs; with a scan it grows linearly (about 17 ns, 592 ns, 5.6 µs). The benchmark detects an AC7
violation.

## P1 - Baseline Before Any Production Change (T3)

- Code: `develop` at `b5f1801c6` plus `docs(issues): [#2505] start the implementation` and
  `test(udp-core): [#2505] add the per-event ban service benchmark` (no production change).
- Release build: `cargo build --release --bin torrust-tracker` (`Finished release profile
  [optimized + debuginfo] target(s) in 2m 33s`). UDP port 3000 was free
  (`ss -ulnp | grep -E ':3000\b'` printed nothing).

### P1 Load Test

- Started 2026-10-13 18:00:53 UTC (load average 7.72, 7.08, 5.18); finished 18:03:44 UTC (load
  average 8.20, 7.60, 5.71).
- Command: `./.tmp/perf-2505/run-load-tests.sh p1 5`.

| Run | `Responses in` (first match, per second) | `Average responses per second` |
| --- | --------------------------------- | ------------------------------ |
| 1 | 151402.40 | 148018.51 |
| 2 | 146285.98 | 151688.96 |
| 3 | 155604.13 | 150771.24 |
| 4 | 157187.94 | 155833.22 |
| 5 | 150907.36 | 148488.07 |

- Mean of `Average responses per second`: **150960.00**; lowest run: **148018.51**; highest:
  155833.22; spread 5.2% of the mean. Each run reported `Error responses: 0.00`.

### P1 Criterion

- Started 2026-10-13 18:03:56 UTC (load average 7.93, 7.57, 5.73); finished 18:18:24 UTC (load
  average 2.72, 4.00, 4.55).
- Command: `./.tmp/perf-2505/run-criterion.sh p1 3`. Medians in nanoseconds; the last column is
  the pass-rule limit for P2.

| Benchmark | Run 1 | Run 2 | Run 3 | Highest (limit) |
| --------- | ----- | ----- | ----- | --------------- |
| `udp_ban_service_increase_counter/distinct/ipv4` | 288005.53 | 292950.03 | 284651.92 | 292950.03 |
| `udp_ban_service_increase_counter/distinct/ipv6` | 340205.55 | 332527.15 | 333142.39 | 340205.55 |
| `udp_ban_service_increase_counter/repeated/ipv4` | 101633.99 | 102398.54 | 100495.67 | 102398.54 |
| `udp_ban_service_increase_counter/repeated/ipv6` | 122219.95 | 121537.99 | 121182.69 | 122219.95 |
| `udp_ban_service_is_banned/ipv4_above_threshold/10000` | 9.14 | 9.27 | 9.08 | 9.27 |
| `udp_ban_service_is_banned/ipv4_above_threshold/1000` | 9.13 | 9.26 | 9.13 | 9.26 |
| `udp_ban_service_is_banned/ipv4_above_threshold/10` | 9.22 | 9.22 | 9.19 | 9.22 |
| `udp_ban_service_is_banned/ipv4_at_threshold/10000` | 9.12 | 9.19 | 9.08 | 9.19 |
| `udp_ban_service_is_banned/ipv4_at_threshold/1000` | 9.20 | 9.34 | 9.10 | 9.34 |
| `udp_ban_service_is_banned/ipv4_at_threshold/10` | 9.16 | 9.22 | 9.29 | 9.29 |
| `udp_ban_service_is_banned/ipv4_below_threshold/10000` | 9.11 | 9.28 | 9.05 | 9.28 |
| `udp_ban_service_is_banned/ipv4_below_threshold/1000` | 9.27 | 9.30 | 9.15 | 9.30 |
| `udp_ban_service_is_banned/ipv4_below_threshold/10` | 9.15 | 9.19 | 9.39 | 9.39 |
| `udp_ban_service_is_banned/ipv6_above_threshold/10000` | 10.95 | 11.12 | 10.76 | 11.12 |
| `udp_ban_service_is_banned/ipv6_above_threshold/1000` | 11.24 | 11.04 | 11.10 | 11.24 |
| `udp_ban_service_is_banned/ipv6_above_threshold/10` | 11.24 | 11.01 | 10.83 | 11.24 |
| `udp_ban_service_is_banned/ipv6_at_threshold/10000` | 10.84 | 11.00 | 11.18 | 11.18 |
| `udp_ban_service_is_banned/ipv6_at_threshold/1000` | 11.02 | 11.20 | 10.80 | 11.20 |
| `udp_ban_service_is_banned/ipv6_at_threshold/10` | 10.95 | 11.18 | 10.83 | 11.18 |
| `udp_ban_service_is_banned/ipv6_below_threshold/10000` | 11.22 | 11.08 | 11.16 | 11.22 |
| `udp_ban_service_is_banned/ipv6_below_threshold/1000` | 11.27 | 11.06 | 10.80 | 11.27 |
| `udp_ban_service_is_banned/ipv6_below_threshold/10` | 10.89 | 11.04 | 10.82 | 11.04 |
| `udp_ban_service_record_error/ipv4/10000` | 10.75 | 10.87 | 10.33 | 10.87 |
| `udp_ban_service_record_error/ipv4/1000` | 10.69 | 10.60 | 10.34 | 10.69 |
| `udp_ban_service_record_error/ipv4/10` | 10.59 | 10.69 | 10.34 | 10.69 |
| `udp_ban_service_record_error/ipv6/10000` | 12.46 | 12.49 | 12.27 | 12.49 |
| `udp_ban_service_record_error/ipv6/1000` | 12.48 | 12.58 | 12.32 | 12.58 |
| `udp_ban_service_record_error/ipv6/10` | 12.42 | 12.53 | 12.42 | 12.53 |

Observed P1 noise: the three medians of each benchmark differ by up to about 3% (for example
`record_error/ipv4/10000` 10.33-10.87 ns, `increase_counter/distinct/ipv4` 284651.92-292950.03 ns).
The `record_error` medians are flat across 10, 1,000, and 10,000 tracked IPs (10.33-10.87 ns for
IPv4, 12.27-12.58 ns for IPv6), as expected while the total is the map length.

## P2 - After the Fix (T9)

### First Attempt, Aborted

On 2026-10-13 at 20:08:42 UTC the first P2 load-test run measured `Average responses per second:
0.00`: the tracker exited at startup because the T10 recheck tracker (started at 20:05:55 UTC with
the default health check API address) held `127.0.0.1:1313`
(`Could not bind the health check API listener ... Address already in use (os error 98)`). The run
loop stopped after that run. Overriding the health check port would have changed the P1
configuration, so P2 waited until the recheck tracker was stopped.

### P2a - First Form of the Fix

Measured on `docs(udp-server): [#2505] describe the banned-IPs metric as IPs over the limit`, with
the first form of the crossing check (compare before and after the increment). Release build
`Finished release profile [optimized + debuginfo] target(s) in 1m 30s`.

- Load test: 2026-10-13 21:06:32 to 21:09:21 UTC, load average 0.72 at the start and 6.03 at the
  end. Command `./.tmp/perf-2505/run-load-tests.sh p2 5` (the files were later renamed to `p2a-*`).

| Run | `Responses in` (first match, per second) | `Average responses per second` |
| --- | ---------------------------------------- | ------------------------------ |
| 1 | 160801.49 | 163244.05 |
| 2 | 160686.21 | 163060.25 |
| 3 | 157614.41 | 159607.63 |
| 4 | 163538.14 | 162312.50 |
| 5 | 167005.56 | 163963.71 |

- Mean 162437.63, lowest 159607.63, spread 2.7%: passes (P1 lowest run 148018.51). Every summary
  reported `Error responses: 0.00`.

- Criterion: 2026-10-13 21:09:45 to 21:23:58 UTC, load average 4.38 at the start and 3.83 at the
  end. Command `./.tmp/perf-2505/run-criterion.sh p2 3`.

| Benchmark | Run 1 | Run 2 | Run 3 | Median | P1 limit | Result |
| --------- | ----- | ----- | ----- | ------ | -------- | ------ |
| `udp_ban_service_increase_counter/distinct/ipv4` | 282228.76 | 279931.09 | 280806.39 | 280806.39 | 292950.03 | pass |
| `udp_ban_service_increase_counter/distinct/ipv6` | 331963.81 | 328744.67 | 330008.16 | 330008.16 | 340205.55 | pass |
| `udp_ban_service_increase_counter/repeated/ipv4` | 101704.22 | 102223.41 | 100733.75 | 101704.22 | 102398.54 | pass |
| `udp_ban_service_increase_counter/repeated/ipv6` | 124121.71 | 123745.87 | 123419.60 | 123745.87 | 122219.95 | fail |
| `udp_ban_service_is_banned/ipv4_above_threshold/10000` | 9.07 | 9.02 | 9.01 | 9.02 | 9.27 | pass |
| `udp_ban_service_is_banned/ipv4_above_threshold/1000` | 9.05 | 9.11 | 8.99 | 9.05 | 9.26 | pass |
| `udp_ban_service_is_banned/ipv4_above_threshold/10` | 9.05 | 9.06 | 9.01 | 9.05 | 9.22 | pass |
| `udp_ban_service_is_banned/ipv4_at_threshold/10000` | 9.04 | 9.05 | 9.04 | 9.04 | 9.19 | pass |
| `udp_ban_service_is_banned/ipv4_at_threshold/1000` | 9.05 | 9.29 | 9.04 | 9.05 | 9.34 | pass |
| `udp_ban_service_is_banned/ipv4_at_threshold/10` | 9.05 | 9.05 | 9.02 | 9.05 | 9.29 | pass |
| `udp_ban_service_is_banned/ipv4_below_threshold/10000` | 9.06 | 9.02 | 8.99 | 9.02 | 9.28 | pass |
| `udp_ban_service_is_banned/ipv4_below_threshold/1000` | 9.03 | 9.09 | 8.99 | 9.03 | 9.30 | pass |
| `udp_ban_service_is_banned/ipv4_below_threshold/10` | 9.05 | 9.07 | 9.00 | 9.05 | 9.39 | pass |
| `udp_ban_service_is_banned/ipv6_above_threshold/10000` | 11.02 | 11.02 | 11.29 | 11.02 | 11.12 | pass |
| `udp_ban_service_is_banned/ipv6_above_threshold/1000` | 11.22 | 11.26 | 11.08 | 11.22 | 11.24 | pass |
| `udp_ban_service_is_banned/ipv6_above_threshold/10` | 11.16 | 11.01 | 11.24 | 11.16 | 11.24 | pass |
| `udp_ban_service_is_banned/ipv6_at_threshold/10000` | 11.10 | 11.03 | 11.12 | 11.10 | 11.18 | pass |
| `udp_ban_service_is_banned/ipv6_at_threshold/1000` | 11.12 | 11.25 | 11.27 | 11.25 | 11.20 | fail |
| `udp_ban_service_is_banned/ipv6_at_threshold/10` | 11.25 | 11.05 | 11.16 | 11.16 | 11.18 | pass |
| `udp_ban_service_is_banned/ipv6_below_threshold/10000` | 11.20 | 11.24 | 11.03 | 11.20 | 11.22 | pass |
| `udp_ban_service_is_banned/ipv6_below_threshold/1000` | 11.05 | 11.23 | 11.12 | 11.12 | 11.27 | pass |
| `udp_ban_service_is_banned/ipv6_below_threshold/10` | 11.37 | 11.06 | 11.17 | 11.17 | 11.04 | fail |
| `udp_ban_service_record_error/ipv4/10000` | 11.00 | 10.83 | 10.98 | 10.98 | 10.87 | fail |
| `udp_ban_service_record_error/ipv4/1000` | 10.96 | 10.84 | 10.98 | 10.96 | 10.69 | fail |
| `udp_ban_service_record_error/ipv4/10` | 10.82 | 10.86 | 10.77 | 10.82 | 10.69 | fail |
| `udp_ban_service_record_error/ipv6/10000` | 13.39 | 12.84 | 13.28 | 13.28 | 12.49 | fail |
| `udp_ban_service_record_error/ipv6/1000` | 13.49 | 12.86 | 13.28 | 13.28 | 12.58 | fail |
| `udp_ban_service_record_error/ipv6/10` | 13.28 | 12.80 | 13.21 | 13.21 | 12.53 | fail |

Nine medians exceeded their P1 limit, all six `record_error` cases among them.

### P2 Investigation - Same-Machine Comparison

To separate the cost of the change from machine drift (P1 ran under load averages of 2.7-8.2, P2a
under 0.7-6.0), the `record_error` and `increase_counter` groups were measured twice each,
back to back, on 2026-10-13 between 21:24:33 and 21:35:33 UTC (load average 1.3-2.9), for three
versions of `packages/udp-core/src/services/banning.rs`:

- **A**: the first form, as committed in `fix(udp-core): [#2505] count only IPs over the limit as
  banned`.
- **B**: one comparison after the increment,
  `if Some(*count) == self.max_connection_id_errors_per_ip.checked_add(1)`.
- **develop**: the unfixed file, written with
  `git show b5f1801c6:packages/udp-core/src/services/banning.rs > packages/udp-core/src/services/banning.rs`
  (the benchmark file is unchanged; the old `get_banned_ips_total` returns the map length). The B
  file was restored afterwards and `cmp` confirmed it.

Each measurement ran this command, with `<label>` set to `variant-a`, `variant-b`, or
`baseline-now`:

```bash
for r in 1 2; do cargo bench -q -p torrust-tracker-udp-core --bench ban_service_benchmark -- "record_error|increase_counter" >/dev/null 2>&1; (cd target/criterion && for f in udp_ban_service_record_error/*/*/new/estimates.json udp_ban_service_increase_counter/*/*/new/estimates.json; do printf '%s %s\n' "${f%/new/estimates.json}" "$(jq '.median.point_estimate' $f)"; done) > .tmp/perf-2505/<label>-$r.txt; done
```

Medians in nanoseconds:

| Benchmark | develop 1 | develop 2 | A 1 | A 2 | B 1 | B 2 |
| --------- | --------- | --------- | --- | --- | --- | --- |
| `udp_ban_service_record_error/ipv4/10000` | 10.40 | 10.19 | 10.98 | 10.88 | 10.81 | 10.82 |
| `udp_ban_service_record_error/ipv4/1000` | 10.21 | 10.15 | 10.88 | 10.73 | 10.81 | 10.81 |
| `udp_ban_service_record_error/ipv4/10` | 10.46 | 10.40 | 10.86 | 10.92 | 10.82 | 10.92 |
| `udp_ban_service_record_error/ipv6/10000` | 12.34 | 12.44 | 13.00 | 13.26 | 12.91 | 12.91 |
| `udp_ban_service_record_error/ipv6/1000` | 12.41 | 12.42 | 12.84 | 13.17 | 12.79 | 12.82 |
| `udp_ban_service_record_error/ipv6/10` | 12.43 | 12.46 | 12.88 | 13.26 | 12.81 | 12.91 |
| `udp_ban_service_increase_counter/distinct/ipv4` | 272913.93 | 272265.46 | 279935.74 | 279514.01 | 280548.01 | 281733.23 |
| `udp_ban_service_increase_counter/distinct/ipv6` | 327460.48 | 327445.44 | 330762.21 | 329051.14 | 324592.72 | 325390.12 |
| `udp_ban_service_increase_counter/repeated/ipv4` | 97431.86 | 97322.90 | 102240.64 | 102590.02 | 98308.52 | 98018.87 |
| `udp_ban_service_increase_counter/repeated/ipv6` | 121762.63 | 121028.22 | 123450.79 | 123330.44 | 126856.78 | 121429.63 |

Findings:

- The fix adds about 0.4-0.6 ns (about 4%) to `record_error` relative to `develop` under the same
  conditions, flat across 10, 1,000, and 10,000 tracked IPs. It is the conditional update that
  maintains the counter, so it is inherent to option P1, and AC7 (no growth with the map size)
  holds.
- B is as fast as A or faster, and closer to `develop` on `increase_counter/repeated/ipv4`.
- `is_banned` did not change, so its P2a failures (0.4-1.2%) are measurement noise.

Maintainer decision (2026-10-13, shortly after the comparison above ended at 21:35:33 UTC): accept
the cost as inherent to the fix, adopt B, and rerun P2 on the final code. The specification's
progress-log entry describing it is stamped 2026-10-14 05:32 UTC because it was written when work
resumed after an overnight pause. B was committed as
`perf(udp-core): [#2505] detect the ban crossing with one comparison`.

### P2 - Final Code

Measured on `perf(udp-core): [#2505] detect the ban crossing with one comparison`. Release build
`Finished release profile [optimized + debuginfo] target(s) in 1m 27s`; UDP 3000 and TCP 1313 free;
no other tracker running.

- Load test: 2026-10-14 05:44:02 to 05:46:52 UTC, load average 2.65 at the start and 5.40 at the
  end. Command `./.tmp/perf-2505/run-load-tests.sh p2 5`.

| Run | `Responses in` (first match, per second) | `Average responses per second` | Summary `Error responses` |
| --- | ---------------------------------------- | ------------------------------ | ------------------------- |
| 1 | 166234.07 | 162617.41 | 0.00 |
| 2 | 159935.72 | 157252.11 | 0.00 |
| 3 | 161828.57 | 153821.24 | 0.00 |
| 4 | 157219.42 | 161901.26 | 0.00 |
| 5 | 163779.36 | 156691.55 | 0.00 |

- Mean of `Average responses per second`: **158456.71**; lowest run 153821.24; highest 162617.41;
  spread 5.6% of the mean.

- Criterion: 2026-10-14 05:46:52 to 06:01:08 UTC, load average 5.40 at the start and 2.56 at the
  end. Command `./.tmp/perf-2505/run-criterion.sh p2 3`. Medians in nanoseconds.

| Benchmark | Run 1 | Run 2 | Run 3 | Median | P1 limit | Result |
| --------- | ----- | ----- | ----- | ------ | -------- | ------ |
| `udp_ban_service_increase_counter/distinct/ipv4` | 283115.85 | 281713.76 | 281684.99 | 281713.76 | 292950.03 | pass |
| `udp_ban_service_increase_counter/distinct/ipv6` | 329025.07 | 326335.93 | 325698.13 | 326335.93 | 340205.55 | pass |
| `udp_ban_service_increase_counter/repeated/ipv4` | 101128.51 | 100150.87 | 101136.99 | 101128.51 | 102398.54 | pass |
| `udp_ban_service_increase_counter/repeated/ipv6` | 128052.00 | 126954.05 | 125203.55 | 126954.05 | 122219.95 | fail |
| `udp_ban_service_is_banned/ipv4_above_threshold/10000` | 9.00 | 9.07 | 8.92 | 9.00 | 9.27 | pass |
| `udp_ban_service_is_banned/ipv4_above_threshold/1000` | 8.96 | 8.93 | 9.00 | 8.96 | 9.26 | pass |
| `udp_ban_service_is_banned/ipv4_above_threshold/10` | 8.97 | 8.98 | 8.91 | 8.97 | 9.22 | pass |
| `udp_ban_service_is_banned/ipv4_at_threshold/10000` | 9.07 | 9.04 | 8.95 | 9.04 | 9.19 | pass |
| `udp_ban_service_is_banned/ipv4_at_threshold/1000` | 8.98 | 9.35 | 8.98 | 8.98 | 9.34 | pass |
| `udp_ban_service_is_banned/ipv4_at_threshold/10` | 8.96 | 8.95 | 8.98 | 8.96 | 9.29 | pass |
| `udp_ban_service_is_banned/ipv4_below_threshold/10000` | 9.01 | 8.97 | 8.99 | 8.99 | 9.28 | pass |
| `udp_ban_service_is_banned/ipv4_below_threshold/1000` | 9.02 | 8.93 | 8.97 | 8.97 | 9.30 | pass |
| `udp_ban_service_is_banned/ipv4_below_threshold/10` | 8.98 | 8.96 | 8.94 | 8.96 | 9.39 | pass |
| `udp_ban_service_is_banned/ipv6_above_threshold/10000` | 11.26 | 11.27 | 11.10 | 11.26 | 11.12 | fail |
| `udp_ban_service_is_banned/ipv6_above_threshold/1000` | 11.28 | 11.08 | 11.22 | 11.22 | 11.24 | pass |
| `udp_ban_service_is_banned/ipv6_above_threshold/10` | 11.26 | 11.07 | 11.02 | 11.07 | 11.24 | pass |
| `udp_ban_service_is_banned/ipv6_at_threshold/10000` | 11.08 | 11.08 | 11.15 | 11.08 | 11.18 | pass |
| `udp_ban_service_is_banned/ipv6_at_threshold/1000` | 11.06 | 11.21 | 11.54 | 11.21 | 11.20 | fail |
| `udp_ban_service_is_banned/ipv6_at_threshold/10` | 11.27 | 11.05 | 11.07 | 11.07 | 11.18 | pass |
| `udp_ban_service_is_banned/ipv6_below_threshold/10000` | 11.18 | 11.30 | 11.10 | 11.18 | 11.22 | pass |
| `udp_ban_service_is_banned/ipv6_below_threshold/1000` | 11.09 | 11.31 | 11.05 | 11.09 | 11.27 | pass |
| `udp_ban_service_is_banned/ipv6_below_threshold/10` | 11.27 | 11.26 | 11.06 | 11.26 | 11.04 | fail |
| `udp_ban_service_record_error/ipv4/10000` | 10.86 | 10.90 | 10.84 | 10.86 | 10.87 | pass |
| `udp_ban_service_record_error/ipv4/1000` | 10.88 | 10.88 | 10.82 | 10.88 | 10.69 | fail |
| `udp_ban_service_record_error/ipv4/10` | 11.05 | 10.89 | 10.85 | 10.89 | 10.69 | fail |
| `udp_ban_service_record_error/ipv6/10000` | 12.99 | 12.93 | 12.94 | 12.94 | 12.49 | fail |
| `udp_ban_service_record_error/ipv6/1000` | 12.77 | 12.91 | 12.75 | 12.77 | 12.58 | fail |
| `udp_ban_service_record_error/ipv6/10` | 12.90 | 12.91 | 12.85 | 12.90 | 12.53 | fail |

The `record_error` medians are flat across 10, 1,000, and 10,000 tracked IPs (10.86-10.89 ns for
IPv4, 12.77-12.94 ns for IPv6), which is the AC7 evidence.

## Comparison

| Measurement | P1 | P2 (final) | Pass rule | Result |
| ----------- | -- | ---------- | --------- | ------ |
| Load test mean `Average responses per second` | lowest run 148018.51 | mean 158456.71 | P2 mean not below the lowest P1 run | **pass** |
| Criterion medians | highest of three per benchmark | median of three per benchmark | P2 median not above the P1 limit | **19 pass, 9 fail** |

The nine Criterion failures:

| Benchmark | P2 median | P1 limit | Difference | Explanation |
| --------- | --------- | -------- | ---------- | ----------- |
| `record_error/ipv4/1000` | 10.88 | 10.69 | +1.7% | Inherent cost of maintaining the counter (investigation above) |
| `record_error/ipv4/10` | 10.89 | 10.69 | +1.9% | Same |
| `record_error/ipv6/10000` | 12.94 | 12.49 | +3.6% | Same |
| `record_error/ipv6/1000` | 12.77 | 12.58 | +1.5% | Same |
| `record_error/ipv6/10` | 12.90 | 12.53 | +2.9% | Same |
| `increase_counter/repeated/ipv6` | 126954.05 | 122219.95 | +3.9% | Same check on 10,000 repeated errors; this case varied between 121429.63 and 126856.78 in the B runs |
| `is_banned/ipv6_above_threshold/10000` | 11.26 | 11.12 | +1.3% | Unchanged code: noise |
| `is_banned/ipv6_at_threshold/1000` | 11.21 | 11.20 | +0.1% | Unchanged code: noise |
| `is_banned/ipv6_below_threshold/10` | 11.26 | 11.04 | +2.0% | Unchanged code: noise |

Maintainer confirmation (2026-10-14 06:59 UTC, after the independent pre-PR review asked for it):
the accepted-cost decision covers this final P2 table, including all nine Criterion medians above
their P1 limit.

Conclusion: the end-to-end load test shows no regression. The microbenchmarks show the expected
small cost on the invalid-connection-ID path (about 0.4-0.6 ns per event, accepted by the
maintainer), no growth with the number of tracked IPs (AC7), and noise-level differences in
unchanged code. The observed P1 noise was up to about 3% between Criterion runs and 5.2% between
load-test runs, so differences below those levels are not detectable with these measurements.
