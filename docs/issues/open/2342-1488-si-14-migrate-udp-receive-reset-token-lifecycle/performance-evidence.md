---
doc-type: performance-evidence
issue-spec: docs/issues/open/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
last-updated-utc: 2026-09-26
---

# Performance Evidence - UDP Throughput Before and After SI-14

Scenario M4 and acceptance criterion AC12 of issue #2342 (decision D6). The
baseline is measured on the `develop` commit the implementation branch starts
from, before any code change. The second measurement uses the implementation
branch on the same machine with the same build profile, tracker configuration,
and load-test configuration.

## Method

Follows the E2E UDP load test in [`docs/benchmarking.md`](../../../benchmarking.md).

- Tracker: `cargo build --release --bin torrust-tracker`, started with the
  unmodified `share/default/config/tracker.udp.benchmarking.toml` (UDP on
  `0.0.0.0:3000`, `error` logging, usage statistics off). The working directory
  is a scratch folder so the relative SQLite path stays out of the repository.
- A fresh tracker process per run, so in-memory swarm state does not carry over;
  each run ends with `SIGTERM` to the tracker PID.
- Load generator: `aquatic_udp_load_test` built from source (`cargo build
  --release -p aquatic_udp_load_test`) at aquatic commit `a2ddc4b3`.
- Load-test configuration (the documented example with a longer window to
  reduce noise):

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

- Metric: `Average responses per second` over the last 20 seconds of each run.
- Pass rule (AC12): the implementation mean is within the baseline min-max
  spread; a larger drop is investigated before closing.

## Machine

- CPU: AMD Ryzen 9 7950X 16-Core Processor (32 logical CPUs)
- Memory: 61 GiB
- OS: Linux, kernel `7.0.0-34-generic`
- Socket buffers: `net.core.rmem_max = 4194304`, `net.core.rmem_default = 212992`
  (the load test's 8 MB `recv_buffer` request is capped by `rmem_max`)
- Toolchain: `rustc 1.100.0-nightly (6eeff9a52 2026-09-23)` (repository
  directory override)
- Background load: desktop session running; load average 5.80 / 5.11 / 3.25
  right after the baseline runs

## Baseline - `develop` at `0f1dcd28`

Date: 2026-09-26, 13:10-13:24 local time.

| Run | Responses/s | Connect/s | Announce/s | Scrape/s | Errors/s |
| --- | ----------- | --------- | ---------- | -------- | -------- |
| 1 | 155327.06 | 76886.59 | 76888.09 | 1552.38 | 0.00 |
| 2 | 142150.63 | 70368.57 | 70357.38 | 1424.67 | 0.00 |
| 3 | 149945.64 | 74224.57 | 74222.82 | 1498.26 | 0.00 |
| 4 | 149591.28 | 74051.00 | 74044.20 | 1496.09 | 0.00 |
| 5 | 145016.70 | 71783.13 | 71783.03 | 1450.54 | 0.00 |

- Mean: 148406.26 responses/s
- Min-max spread: 142150.63 - 155327.06 (about 8.9% of the mean)
- Tracker logs contained no `error` or `panic` lines.

## After Implementation

Not measured yet. Run T7 with the same method, machine, and configuration, and
under a similar background load.

| Run | Responses/s | Connect/s | Announce/s | Scrape/s | Errors/s |
| --- | ----------- | --------- | ---------- | -------- | -------- |

## Comparison

Pending the after-implementation measurement.
