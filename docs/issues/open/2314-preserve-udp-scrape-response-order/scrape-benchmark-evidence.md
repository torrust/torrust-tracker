---
doc-type: benchmark-report
parent-issue: 2314
status: complete
last-updated-utc: "2026-09-23 12:05"
semantic-links:
  related-artifacts:
    - docs/issues/open/2314-preserve-udp-scrape-response-order/ISSUE.md
    - docs/issues/open/2314-preserve-udp-scrape-response-order/manual-verification-evidence.md
    - docs/benchmarking.md
    - docs/issues/closed/1505-optimize-peer-ip-list-from-swarm/aquatic-benchmarking-guide.md
---

# Scrape Benchmark Evidence

## Purpose

Record the UDP scrape performance measured before (P1) and after (P2) the fix that preserves
request order in the UDP scrape response, so the two can be compared on the same machine with the
same procedure. This file is filled with actual results only; do not invent commands, output, or
numbers. It feeds acceptance criterion AC7 in `ISSUE.md`.

Two instruments are used, for the reasons given in `ISSUE.md`, Performance Considerations:

- **Instrument 1 - microbenchmark** (`scrape_once`, Criterion, `packages/udp-server/benches/`):
  precise; quantifies the per-request delta of the changed function.
- **Instrument 2 - end-to-end load test** (`aquatic_udp_load_test`, scrape-heavy mix):
  user-visible; proves there is no gross regression in scrape responses per second.

## Environment

Record once, before P1. P2 must run on the same machine; note any difference in the P2 section.

| Item | Value |
| ---- | ----- |
| Date (UTC) | 2026-09-23 |
| CPU model and core count (`lscpu`) | AMD Ryzen 9 7950X 16-Core Processor; 32 logical CPUs (16 cores, 2 threads/core) |
| RAM (`free -g`) | 61 GiB total; 33 GiB used; 7 GiB free; 6 GiB swap used |
| Kernel (`uname -srm`) | Linux 7.0.0-31-generic x86_64 |
| Rust toolchain (`rustc --version`) | rustc 1.98.1 (48a229cea 2026-09-01) |
| CPU frequency governor during runs | `performance`; `lscpu` reported 78% CPU scaling |
| Machine otherwise idle | No; 33 GiB RAM and 6 GiB swap were in use. Retained all samples and intervals. |
| Tracker build profile | `release` (`cargo build --release`) |
| Tracker config | `share/default/config/tracker.udp.benchmarking.toml` (logging `error`, UDP `0.0.0.0:3000`) |
| `aquatic` commit | `822a801c7694516df542c7e8d8f6f4a7195cbec9` |

## Code Under Test

| Phase | Branch / commit | Notes |
| ----- | --------------- | ----- |
| P1 (pre-fix) | `e03b22f711156443d23be3d62a5c137f9c9f84c4` | Contains the P0 microbenchmark; `build_response` still iterates `scrape_data.files`. |
| P2 (post-fix) | working tree | `build_response` iterates `request.info_hashes`; measurements were taken before committing. |

## Instrument 1 - Microbenchmark (`scrape_once`)

Command (both phases):

```console
cargo bench -p torrust-tracker-udp-server -- scrape_once
```

Scenario: production `handle_scrape` with 74 distinct requested hashes present in the repository.
Run three times per phase; record Criterion's reported mean and confidence interval.

### Results

| Run | P1 mean | P1 CI | P2 mean | P2 CI |
| --- | ------- | ----- | ------- | ----- |
| 1 | 6.1391 us | [6.1245 us, 6.1560 us] | 7.0067 us | [6.9629 us, 7.0505 us] |
| 2 | 6.1187 us | [6.1071 us, 6.1339 us] | 6.6745 us | [6.6590 us, 6.6908 us] |
| 3 | 6.1944 us | [6.1838 us, 6.2077 us] | 6.6448 us | [6.6323 us, 6.6587 us] |
| **Median** | 6.1391 us | - | 6.6745 us | - |

Delta (P2 median - P1 median): +0.5354 us (+8.72%)

Expected from analysis: an increase of at most a few microseconds per request (74 SipHash lookups
of 20-byte keys); anything materially larger needs explanation before merge.

### Raw output

P1 (means and confidence intervals):

```text
run 1: [6.1245 us, 6.1391 us, 6.1560 us]
run 2: [6.1071 us, 6.1187 us, 6.1339 us]
run 3: [6.1838 us, 6.1944 us, 6.2077 us]
```

P2:

```text
run 1: [6.9629 us, 7.0067 us, 7.0505 us]
run 2: [6.6590 us, 6.6745 us, 6.6908 us]
run 3: [6.6323 us, 6.6448 us, 6.6587 us]
```

## Instrument 2 - End-to-End Load Test (`aquatic_udp_load_test`)

Procedure per `docs/benchmarking.md`, E2E UDP load testing, with the config below. The identical
config file is reused for P2.

Tracker start (both phases):

```console
cargo build --release
./target/release/torrust-tracker --config-toml-path ./share/default/config/tracker.udp.benchmarking.toml
```

Load test (both phases, from the aquatic checkout):

```console
./target/release/aquatic_udp_load_test -c <path-to-config>.toml
```

Load-test config actually used (record verbatim):

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

Five iterations per phase; the median is the comparison value.

### Results

| Run | P1 scrape responses/s | P1 total responses/s | P2 scrape responses/s | P2 total responses/s |
| --- | --------------------- | -------------------- | --------------------- | -------------------- |
| 1 | 1510.95 | 152222.03 | 1439.14 | 144792.00 |
| 2 | 1608.74 | 162885.57 | 1476.44 | 148779.91 |
| 3 | 1427.59 | 143684.06 | 1548.73 | 156793.30 |
| 4 | 1547.45 | 156755.69 | 1495.37 | 151310.46 |
| 5 | 1594.44 | 161471.46 | 1534.27 | 154497.33 |
| **Median** | 1547.45 | 156755.69 | 1495.37 | 151310.46 |

Scrape median delta (P2 - P1): -52.08 responses/s (-3.37%)

Error responses observed: each measured five-second window reported `0.00`; the unmeasured
start-up windows reported `0.00` to `1.40` errors.

### Raw output

P1 (summary block of each run):

```text
run 1: total 152222.03; connect 75339.16; announce 75371.92; scrape 1510.95; errors 0.00
run 2: total 162885.57; connect 80658.59; announce 80618.24; scrape 1608.74; errors 0.00
run 3: total 143684.06; connect 71110.68; announce 71145.79; scrape 1427.59; errors 0.00
run 4: total 156755.69; connect 77633.89; announce 77574.35; scrape 1547.45; errors 0.00
run 5: total 161471.46; connect 79943.70; announce 79933.31; scrape 1594.44; errors 0.00
```

P2 (summary block of each run):

```text
run 1: total 144792.00; scrape 1439.14; errors 0.00
run 2: total 148779.91; scrape 1476.44; errors 0.00
run 3: total 156793.30; scrape 1548.73; errors 0.00
run 4: total 151310.46; scrape 1495.37; errors 0.00
run 5: total 154497.33; scrape 1534.27; errors 0.00
```

## Comparison and Conclusion

Rule from `ISSUE.md`, Performance Verification: P2 end-to-end scrape median within 5% of P1 passes
AC7. A drop between 5% and 10% is within documented run-to-run variance but must be investigated
(rerun, check governor and background load, consult the microbenchmark delta) and the conclusion
recorded here before merge.

- End-to-end scrape median delta: -52.08 responses/s (-3.37%)
- Microbenchmark median delta: +0.5354 us (+8.72%)
- Consistent with the analysis in `ISSUE.md`, Performance Considerations: yes; the small per-request increase is consistent with 74 keyed lookups and the E2E median remains within 5%.
- AC7 outcome: PASS

## Anomalies and Follow-up

Record reruns, environment differences between phases, outliers excluded (with reason), and any
follow-up issue opened (for example, on the pre-existing cost of copying scrape data across layers,
using the P1 baseline as its reference number).

- With `connection_id_validation = "strict"`, two consecutive Aquatic runs in one tracker
  lifetime exhausted the per-IP connection-ID error budget. The next three runs reported zero
  responses and were excluded as invalid. The tracker was restarted before P1 runs 3-5; P2 must
  use the same reset procedure. P1 runs 1 and 2 were valid measured windows before that limit.
- The machine was not otherwise idle. No samples were excluded: the P1 median remains the
  comparison baseline and the same environment constraints apply to P2.
