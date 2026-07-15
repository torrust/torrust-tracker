---
doc-type: how-to-guide
parent-issue: 1505
status: completed
last-updated-utc: 2026-07-15
semantic-links:
  related-artifacts:
    - docs/issues/closed/1505-optimize-peer-ip-list-from-swarm/ISSUE.md
    - docs/issues/closed/1505-optimize-peer-ip-list-from-swarm/baseline-performance.md
    - docs/issues/closed/1505-optimize-peer-ip-list-from-swarm/pre-implementation-analysis.md
    - docs/benchmarking.md
    - share/default/config/tracker.udp.benchmarking.toml
---

# Aquatic Benchmarking Guide for Torrust Tracker

> This document records all commands, outputs, troubleshooting, and setup steps for using
> the [Aquatic](https://github.com/greatest-ape/aquatic) benchmarking tools against the
> Torrust Tracker. Created during issue #1505 baseline performance analysis.
>
> For the canonical project-wide benchmarking docs, see [docs/benchmarking.md](../../../benchmarking.md).
> This guide is an issue-specific supplement with full output and troubleshooting detail.

## Overview

The Aquatic repository provides two benchmarking tools:

| Tool                    | Purpose                                               | Build profile             |
| ----------------------- | ----------------------------------------------------- | ------------------------- |
| `aquatic_udp_load_test` | Single-tracker UDP load test (request/response rates) | `--release`               |
| `aquatic_bencher`       | Comparative UDP benchmarking across multiple trackers | `--profile release-debug` |

### Prerequisites

- Linux 6.0+ (for `io_uring` support)
- Rust toolchain (same as Torrust Tracker)
- System packages: `cmake`, `build-essential`, `pkg-config`, `git`, `screen`, `cvs`, `zlib1g-dev`, `golang` (for comparative bencher with other trackers)
- For `io_uring` feature: `libhwloc-dev`

### Repository location

```text
/path/to/aquatic/
```

## 1. Installation

### 1.1 Clone the repository

```bash
cd /tmp
git clone git@github.com:greatest-ape/aquatic.git
cd aquatic
```

### 1.2 Build the UDP load test tool

```bash
cargo build --release -p aquatic_udp_load_test
```

Build output (successful):

```text
   Compiling rand v0.8.5
   Compiling rand_distr v0.4.3
   Compiling aquatic_common v0.9.0
   Compiling aquatic_udp_load_test v0.9.0
    Finished `release` profile [optimized] target(s) in 7.36s
```

### 1.3 Build the comparative bencher (optional)

```bash
cargo build --profile release-debug -p aquatic_bencher
```

Build output (successful):

```text
warning: `aquatic_bencher` (bin "aquatic_bencher") generated 1 warning
    Finished `release-debug` profile [optimized + debuginfo] target(s) in 12.76s
```

> **Warning**: The single warning is an unused import — not a blocker.

### 1.4 Torrust support

The aquatic bencher already supports `torrust-tracker` as a benchmark target:

```text
crates/bencher/src/main.rs:44:    /// Benchmark UDP BitTorrent trackers aquatic_udp, opentracker, chihaya and torrust-tracker
crates/bencher/src/protocols/udp.rs:36:            Self::TorrustTracker => "torrust-tracker".into(),
crates/bencher/src/protocols/udp.rs:55:    /// Path to torrust-tracker binary
crates/bencher/src/protocols/udp.rs:56:    #[arg(long, default_value = "torrust-tracker")]
```

## 2. Running the UDP Load Test

### 2.1 Build the Torrust Tracker release binary

```bash
cd /path/to/torrust-tracker
cargo build --release
```

### 2.2 Generate default load test config

```bash
cd /path/to/aquatic
./target/release/aquatic_udp_load_test -p
```

This prints the default config to stdout. Redirect to a file:

```bash
./target/release/aquatic_udp_load_test -p > load-test-config.toml
```

Default config generated:

```toml
# aquatic_udp_load_test configuration

# Server address
#
# If you want to send IPv4 requests to a IPv4+IPv6 tracker, put an IPv4
# address here.
server_address = "127.0.0.1:3000"
# Log level. Available values are off, error, warn, info, debug and trace.
log_level = "error"
# Number of workers sending requests
workers = 1
# Run duration (quit and generate report after this many seconds)
duration = 0
# Only report summary for the last N seconds of run
#
# 0 = include whole run
summarize_last = 0
# Display extra statistics
extra_statistics = true

[network]
# True means bind to one localhost IP per socket.
#
# The point of multiple IPs is to cause a better distribution
# of requests to servers with SO_REUSEPORT option.
#
# Setting this to true can cause issues on macOS.
multiple_client_ipv4s = true
# Number of sockets to open per worker
sockets_per_worker = 4
# Size of socket recv buffer. Use 0 for OS default.
#
# This setting can have a big impact on dropped packets. It might
# require changing system defaults. Some examples of commands to set
# values for different operating systems:
#
# macOS:
# $ sudo sysctl net.inet.udp.recvspace=8000000
#
# Linux:
# $ sudo sysctl -w net.core.rmem_max=8000000
# $ sudo sysctl -w net.core.rmem_default=8000000
recv_buffer = 8000000

[requests]
# Number of torrents to simulate
number_of_torrents = 1000000
# Number of peers to simulate
number_of_peers = 2000000
# Maximum number of torrents to ask about in scrape requests
scrape_max_torrents = 10
# Ask for this number of peers in announce requests
announce_peers_wanted = 30
# Probability that a generated request is a connect request as part
# of sum of the various weight arguments.
weight_connect = 50
# Probability that a generated request is a announce request, as part
# of sum of the various weight arguments.
weight_announce = 50
# Probability that a generated request is a scrape request, as part
# of sum of the various weight arguments.
weight_scrape = 1
# Probability that a generated peer is a seeder
peer_seeder_probability = 0.75
```

> **Important**: The default config binds to port **3000**, but the Torrust benchmarking config
> `share/default/config/tracker.udp.benchmarking.toml` also uses port **3000**. If you want
> to use a different port, change it in both places.

### 2.3 Start the Torrust Tracker with benchmarking config

```bash
cd /path/to/torrust-tracker
TORRUST_TRACKER_CONFIG_TOML_PATH="./share/default/config/tracker.udp.benchmarking.toml" \
  ./target/release/torrust-tracker
```

The benchmarking config disables logging, tracking usage stats, persistent metrics,
and peerless torrent removal. It binds the UDP tracker to `0.0.0.0:3000`.

### 2.4 Run the UDP load test

```bash
cd /path/to/aquatic
./target/release/aquatic_udp_load_test -c load-test-config.toml
```

### 2.5 Example output

#### Scenario: `announce_peers_wanted = 10` (B1 — low load)

```text
Requests out: 169283.04/second
Responses in: 168973.37/second
  - Connect responses:  83688.94
  - Announce responses: 83607.42
  - Scrape responses:   1676.21
  - Error responses:    0.80
Peers per announce response: 7.24

# aquatic load test report
Test ran for 10 seconds (only last 5 included in summary)
Average responses per second: 171579.90
  - Connect responses:  85019.83
  - Announce responses: 84873.04
  - Scrape responses:   1687.02
  - Error responses:    0.00
```

#### Scenario: `announce_peers_wanted = 74` (B2 — high load)

```text
Requests out: 172510.83/second
Responses in: 172383.48/second
  - Connect responses:  85442.62
  - Announce responses: 85242.81
  - Scrape responses:   1698.05
  - Error responses:    0.00
Peers per announce response: 20.40

Test ran for 10 seconds (only last 5 included in summary)
Average responses per second: 171718.89
  - Connect responses:  85084.98
  - Announce responses: 84945.36
  - Scrape responses:   1688.55
  - Error responses:    0.00
```

> **Note**: The `announce_peers_wanted = 74` scenario yields `Peers per announce response: 20.40`
> because the load test only populates a subset of torrents with 74+ peers during the 10-second
> run. The `announce_peers_wanted` is the **maximum** the client requests, not a guarantee of
> how many peers the tracker has for each torrent.

## 3. Configurations for issue #1505 Scenarios

### B1 — Low load (`announce_peers_wanted = 10`)

```toml
server_address = "127.0.0.1:3000"
log_level = "error"
workers = 1
duration = 10
summarize_last = 5
extra_statistics = true

[network]
multiple_client_ipv4s = true
sockets_per_worker = 4
recv_buffer = 8000000

[requests]
number_of_torrents = 1000000
number_of_peers = 2000000
scrape_max_torrents = 10
announce_peers_wanted = 10
weight_connect = 50
weight_announce = 50
weight_scrape = 1
peer_seeder_probability = 0.75
```

### B2 — High load (`announce_peers_wanted = 74`)

```toml
server_address = "127.0.0.1:3000"
log_level = "error"
workers = 1
duration = 10
summarize_last = 5
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

## 4. Running the Comparative Bencher

The bencher requires all trackers to be built before running:

1. Build `aquatic_udp` (with optional `io_uring`)
2. Install `opentracker`
3. Install `chihaya`
4. Build `torrust-tracker`

Then run:

```bash
cd /path/to/aquatic
./target/release-debug/aquatic_bencher \
  --min-priority medium --cpu-mode subsequent-one-per-pair
```

See the [Aquatic documentation](https://github.com/greatest-ape/aquatic/tree/master/crates/bencher)
for full details.

## 5. Troubleshooting

### 5.1 Cookie errors during load test

```text
ERROR UDP TRACKER: response error error=tracker announce error:
  Connection cookie error: cookie value is expired: ...
```

This is **normal**. The load test sends a burst of requests at the start, and some
arrive before the tracker's cookie system expects them. These errors account for
a tiny fraction of requests (typically `< 0.001%` of error responses) and do not
affect the overall throughput measurement.

### 5.2 Result variance between runs

The benchmark results vary between runs due to system load, CPU frequency scaling,
and background processes. Typical variance for the UDP load test is **±5–10%**
on a non-dedicated machine. For example, the B1 scenario ranged from ~157k to
~172k responses/second across independent runs. For comparison purposes (before/after),
run multiple iterations and use the median.

Similarly, the microbenchmark (`bench_peers.rs`) shows ±3–5% variance across runs.
The 74-peer scenario ranged from ~400 ns to ~421 ns across runs. Again, median
over several runs is more reliable than any single measurement.

### 5.2 "Peers per announce response: 0.00" on initial runs

If the load test just started, the tracker may not have enough peers stored yet.
The load test includes a warm-up phase; the 5-second window at the end should
show non-zero values. Increase `duration` if needed.

### 5.3 `io_uring` not available

If the system doesn't support `io_uring` (kernels < 6.0), the bencher will fall
back to epoll-based networking. This is fine — the relative comparison is still
valid.

### 5.4 Multiple tracker processes left running

After aborting a bencher run, check for leftover tracker processes:

```bash
pkill -f torrust-tracker
pkill -f chihaya
pkill -f opentracker
pkill -f aquatic  # careful: also kills the load test/bencher
```

## 6. Key Observations

### Performance characteristics

- The UDP load test achieves **~172k responses/second** with a single worker.
- The majority (~85k) are connect responses, ~85k are announce responses, ~1.7k are scrape.
- **Error rate is negligible** (~0.00 errors/second in steady state).
- Increasing `announce_peers_wanted` from 10 to 74 **does not significantly affect throughput**
  (~172k vs ~172k responses/second). This suggests the bottleneck is elsewhere
  (cookie handling, socket I/O, or the worker thread) rather than peer-list serialization.

### Comparison with previous results (2024)

The old blog post (2024) reported **222,330 responses/second** for torrust-tracker with
8 load test workers. Our single-worker result of 172k is lower, but that is expected
with fewer workers. The machine and tracker code have also changed since then.

### Benchmark port convention

| Context                                                       | Port   |
| ------------------------------------------------------------- | ------ |
| Torrust benchmarking config (`tracker.udp.benchmarking.toml`) | `3000` |
| Torrust default tracker config                                | `6969` |
| Load test default config                                      | `3000` |
| Blog post example (port change needed)                        | `6969` |

For convenience, the Torrust benchmarking config binds to port **3000**, which matches
the aquatic load test default — no config change needed.
