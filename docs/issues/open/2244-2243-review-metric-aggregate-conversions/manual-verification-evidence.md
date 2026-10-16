---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2244-2243-review-metric-aggregate-conversions/ISSUE.md
last-updated-utc: 2026-10-15 20:55
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

The runs wrote their files under the git-ignored `.tmp/2244/` directory. Everything the evidence
relies on is preserved in this issue folder:

- The tracker configuration is embedded below.
- [`evidence/reproduce_avg_processing_time.py`](evidence/reproduce_avg_processing_time.py) is the
  load and comparison script.
- [`evidence/before-fix-log-excerpt.txt`](evidence/before-fix-log-excerpt.txt) and
  [`evidence/after-fix-log-excerpt.txt`](evidence/after-fix-log-excerpt.txt) hold the first and last
  moving-average debug lines of each run, plus sample statistics-receiver lag warnings. The full
  debug logs (about 1 GB each) were not kept.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-15 20:36-20:47
- Artifact under test:
  - Before: `target/release/torrust-tracker` built from `develop` plus
    `docs(issues): [#2244] record metric conversion review and maintainer decisions` (no code
    change from `develop`).
  - After: `target/release/torrust-tracker` built from the working tree whose production code is
    identical to `refactor(swarm-coordination-registry): [#2244] document inactive-count gauge
    conversions`. Only a test helper loop changed after the build.
- Operating system / environment: Linux 7.0.0-38-generic, rustc 1.99.0 (b940084d7 2026-09-28),
  Python 3.14.4, release profile.
- Prerequisites and setup performed: `cargo build --release --bin torrust-tracker`. The tracker
  used non-default loopback ports because the machine is shared. The database file was deleted
  before each run.

Tracker configuration (`.tmp/2244/tracker.toml`). M1 used the same file with
`trace_filter = "info"`:

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "debug"
trace_style = "compact"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "./.tmp/2244/sqlite3.db"

[[udp_trackers]]
bind_address = "127.0.0.1:26969"
tracker_usage_statistics = true

[http_api]
bind_address = "127.0.0.1:21212"

[http_api.access_tokens]
admin = "MyAccessToken"
```

## Bug Analysis

`Metrics::recalculate_udp_avg_processing_time_ns` in `packages/udp-server/src/statistics/metrics.rs`
updates a per-label-set moving average:

```text
new_avg = previous_avg + (req_processing_time - previous_avg) / processed_requests_total
```

Before the fix, `previous_avg` came from `udp_avg_processing_time_ns`, which cast the `f64` gauge to
`u64`. Every update therefore started from the truncated previous average. The running sum
`n * avg` lost `(n - 1) * fraction(previous_avg)` per request. Once `n` exceeded the spread of the
processing times, a faster-than-average request lowered the average by about 1 ns, and a slower one
could not raise it unless it was more than `n` ns slower. The average ratcheted toward the fastest
requests and stuck there. The REST API reports this gauge through
`udp_avg_{connect,announce,scrape}_processing_time_ns`.

Reproduction outcome: **Reproduced**: the REST API value itself was observed to be wrong.

## Verification Processes

### V1 - Reproduce the Biased UDP Average Processing Time (Before Fix)

- Goal: observe the REST API average diverge from the true mean of the processing times the tracker
  averaged.
- Initial state: fresh tracker on the before build, empty database.
- Status: `DONE`

#### Steps Performed

1. `./target/release/torrust-tracker --config-toml-path .tmp/2244/tracker.toml > .tmp/2244/tracker-before.log 2>&1`
2. `python3 .tmp/2244/repro.py 50000 .tmp/2244/tracker-before.log`. The script sends 50,000
   sequential BEP 15 connect requests, waits until the log stops growing, reads
   `GET /api/v1/stats`, and averages every `req_processing_time` from the moving-average debug
   lines.

#### Observed Result

```text
requests sent:                       50000
processing times logged (averaged):  39113
min / max logged processing time:    8516 / 408876 ns
true mean of logged times:           17812.4 ns
API udp_avg_connect_processing_time_ns: 10227 ns
```

Debug logging slows the statistics handler, so its broadcast receiver lagged and skipped events
(`Udp tracker server statistics receiver lagged by N events`). Only the 39,113 processing times
that were averaged are logged, and the comparison uses exactly those.

#### Conclusion

Reproduced. The API reported 10,227 ns against a true mean of 17,812.4 ns, 43% low and near the
8,516 ns minimum.

A first attempt read the API before the lagging handler finished and was discarded. The script now
waits for the log to settle.

### V2 - Regression Tests Red Before the Fix

- Goal: prove the maintained regression tests fail against the defect.
- Initial state: tests added to `packages/udp-server/src/statistics/metrics.rs`
  (`processing_time_moving_average` module) with the production code unchanged.
- Status: `DONE`

#### Steps Performed

1. `cargo test -p torrust-tracker-udp-server --lib processing_time_moving_average`

#### Observed Result

```text
test statistics::metrics::tests::processing_time_moving_average::it_should_keep_the_fractional_part_of_the_previous_average ... FAILED
test statistics::metrics::tests::processing_time_moving_average::it_should_not_drift_toward_the_fastest_requests_over_many_requests ... FAILED

---- ...::it_should_keep_the_fractional_part_of_the_previous_average stdout ----
panicked at packages/udp-server/src/statistics/metrics.rs:1486:13:
average was 100.25

---- ...::it_should_not_drift_toward_the_fastest_requests_over_many_requests stdout ----
panicked at packages/udp-server/src/statistics/metrics.rs:1496:13:
average was 100.0001

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 222 filtered out; finished in 0.01s
```

#### Conclusion

Red as expected: the averages should be 100.75 and 100.5.

### V3 - Regression Tests Green After the Fix

- Goal: confirm the fix makes the regression tests pass.
- Initial state: `fix(udp-server): [#2244] keep the fractional UDP average processing time`
  applied.
- Status: `DONE`

#### Steps Performed

1. `cargo test -p torrust-tracker-udp-server --lib processing_time_moving_average`

#### Observed Result

```text
test statistics::metrics::tests::processing_time_moving_average::it_should_keep_the_fractional_part_of_the_previous_average ... ok
test statistics::metrics::tests::processing_time_moving_average::it_should_not_drift_toward_the_fastest_requests_over_many_requests ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 222 filtered out; finished in 0.01s
```

#### Conclusion

Green.

### V4 - Like-for-Like Recheck (After Fix)

- Goal: repeat V1 against the fixed build.
- Initial state: fresh tracker on the after build, empty database.
- Status: `DONE`

#### Steps Performed

1. `./target/release/torrust-tracker --config-toml-path .tmp/2244/tracker.toml > .tmp/2244/tracker-after.log 2>&1`
2. `python3 .tmp/2244/repro.py 50000 .tmp/2244/tracker-after.log`

#### Observed Result

```text
requests sent:                       50000
processing times logged (averaged):  38341
min / max logged processing time:    7414 / 2444971 ns
true mean of logged times:           13851.4 ns
API udp_avg_connect_processing_time_ns: 13851 ns
```

#### Conclusion

Fixed. The API value equals the true mean rounded toward zero to whole nanoseconds. That is what the
averaged accessors report: `as u64` rounds toward zero, and the REST API documents the field as an
average rounded time.

### V5 - M1: Integer and Average Metrics Match Their Semantics

- Goal: issue scenario M1. Inspect the statistics endpoint and confirm integer counters are exact
  and averages are whole nanoseconds.
- Initial state: fresh tracker on the after build at `info` log level, so no statistics events are
  dropped to lag. Empty database.
- Status: `DONE`

#### Steps Performed

1. `./target/release/torrust-tracker --config-toml-path .tmp/2244/tracker-info.toml > .tmp/2244/tracker-m1.log 2>&1`
2. Sent 1,000 sequential BEP 15 connect requests to `127.0.0.1:26969` and waited for each response.
   Waited 2 s, then read `GET http://127.0.0.1:21212/api/v1/stats?token=MyAccessToken`.

#### Observed Result

```text
sent 1000 UDP connect requests over IPv4
udp4_requests: 1000
udp4_connections_handled: 1000
udp_requests_aborted: 0
udp_requests_banned: 0
udp_banned_ips_total: 0
udp_avg_connect_processing_time_ns: 6231
udp_avg_announce_processing_time_ns: 0
udp_avg_scrape_processing_time_ns: 0
udp4_responses: 1000
udp4_errors_handled: 0
tcp4_announces_handled: 0
```

#### Conclusion

Met. The UDP server and UDP core counters (`udp4_requests`, `udp4_responses`,
`udp4_connections_handled`) equal the 1,000 requests sent. The average is a whole number of
nanoseconds, and the kinds with no samples report `0`.

## Failures and Follow-up

- The first V1 attempt read the API and log while the statistics handler was still behind. It was
  discarded and rerun with the settle wait.
- One stats capture raced with stopping the V4 tracker and returned no body. M1 was therefore run
  as the separate V5 process.
- The statistics receiver lag under debug logging is existing broadcast-channel behavior and out of
  scope here.
