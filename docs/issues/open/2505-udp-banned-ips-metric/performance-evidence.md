---
doc-type: performance-evidence
issue-spec: docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
last-updated-utc: "2026-10-13 18:20"
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

Not yet measured.

## Comparison

Not yet available.
