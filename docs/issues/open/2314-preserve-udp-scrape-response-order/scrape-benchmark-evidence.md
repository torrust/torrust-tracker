---
doc-type: benchmark-report
parent-issue: 2314
status: open
last-updated-utc: "2026-09-23 10:55"
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
| Date (UTC) | TODO |
| CPU model and core count (`lscpu`) | TODO |
| RAM (`free -g`) | TODO |
| Kernel (`uname -srm`) | TODO |
| Rust toolchain (`rustc --version`) | TODO |
| CPU frequency governor during runs | TODO (`performance` preferred; record actual value and the `lscpu` scaling percentage) |
| Machine otherwise idle | TODO (yes/no; list notable background load if no) |
| Tracker build profile | `release` (`cargo build --release`) |
| Tracker config | `share/default/config/tracker.udp.benchmarking.toml` (logging `error`, UDP `0.0.0.0:3000`) |
| `aquatic` commit | TODO (`git rev-parse HEAD` in the aquatic checkout) |

## Code Under Test

| Phase | Branch / commit | Notes |
| ----- | --------------- | ----- |
| P1 (pre-fix) | TODO | Contains the P0 microbenchmark; `build_response` still iterates `scrape_data.files`. |
| P2 (post-fix) | TODO | Same P0 microbenchmark; `build_response` iterates `request.info_hashes`. |

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
| 1 | TODO | TODO | TODO | TODO |
| 2 | TODO | TODO | TODO | TODO |
| 3 | TODO | TODO | TODO | TODO |
| **Median** | TODO | - | TODO | - |

Delta (P2 median - P1 median): TODO (absolute and percentage)

Expected from analysis: an increase of at most a few microseconds per request (74 SipHash lookups
of 20-byte keys); anything materially larger needs explanation before merge.

### Raw output

P1:

```text
TODO
```

P2:

```text
TODO
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
TODO
```

Planned starting values (replace with the recorded config above once run):

```toml
duration = 30
summarize_last = 20

[requests]
scrape_max_torrents = 74
weight_connect = 10
weight_announce = 10
weight_scrape = 80
```

Five iterations per phase; the median is the comparison value.

### Results

| Run | P1 scrape responses/s | P1 total responses/s | P2 scrape responses/s | P2 total responses/s |
| --- | --------------------- | -------------------- | --------------------- | -------------------- |
| 1 | TODO | TODO | TODO | TODO |
| 2 | TODO | TODO | TODO | TODO |
| 3 | TODO | TODO | TODO | TODO |
| 4 | TODO | TODO | TODO | TODO |
| 5 | TODO | TODO | TODO | TODO |
| **Median** | TODO | TODO | TODO | TODO |

Scrape median delta (P2 - P1): TODO (absolute and percentage)

Error responses observed: TODO (cookie errors at start-up burst are expected; see
`docs/benchmarking.md`, Troubleshooting)

### Raw output

P1 (summary block of each run):

```text
TODO
```

P2 (summary block of each run):

```text
TODO
```

## Comparison and Conclusion

Rule from `ISSUE.md`, Performance Verification: P2 end-to-end scrape median within 5% of P1 passes
AC7. A drop between 5% and 10% is within documented run-to-run variance but must be investigated
(rerun, check governor and background load, consult the microbenchmark delta) and the conclusion
recorded here before merge.

- End-to-end scrape median delta: TODO
- Microbenchmark median delta: TODO
- Consistent with the analysis in `ISSUE.md`, Performance Considerations: TODO (yes/no, why)
- AC7 outcome: TODO (`PASS` / `INVESTIGATE` / `FAIL`)

## Anomalies and Follow-up

Record reruns, environment differences between phases, outliers excluded (with reason), and any
follow-up issue opened (for example, on the pre-existing cost of copying scrape data across layers,
using the P1 baseline as its reference number).

- TODO
