---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2560-adopt-torrust-metrics-sum-counter/ISSUE.md
last-updated-utc: 2026-10-19 11:40
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands or interactions actually performed against the artifact; no command, output, log, or result here is invented.

## Preserving Verification Artifacts

The runs wrote their files under the git-ignored `.tmp/2560/` directory. Everything the evidence relies on is preserved here:

- The tracker configuration is embedded below.
- [`evidence/red-http-core.txt`](evidence/red-http-core.txt), [`evidence/red-udp-server.txt`](evidence/red-udp-server.txt), and [`evidence/green-udp-server.txt`](evidence/green-udp-server.txt) hold the red and green test runs. UDP core's red run is embedded in V1.
- [`evidence/m1-stats.txt`](evidence/m1-stats.txt) and [`evidence/m2-stats.txt`](evidence/m2-stats.txt) hold the statistics read in M1 and M2.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-19 11:20-11:40
- Artifact under test: `target/release/torrust-tracker` and `target/release/tracker_client`, built from the tree of `refactor(udp-server): [#2560] read counter totals through sum_counter`. The later commits only change the issue folder.
- Operating system / environment: Linux 7.0.0-38-generic, rustc 1.99.0 (b940084d7 2026-09-28), release profile, `torrust-metrics` 0.2.0 from crates.io.
- Prerequisites and setup performed: `cargo build --release --bin torrust-tracker` and `cargo build --release -p torrust-tracker-client --bin tracker_client`. The tracker used loopback ports because the machine is shared. The database file was deleted before the run, and no tracker was running on those ports.

Tracker configuration (`.tmp/2560/tracker.toml`):

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "compact"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "./.tmp/2560/sqlite3.db"

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 3
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:26970"
tracker_usage_statistics = true

[[http_trackers]]
bind_address = "127.0.0.1:27071"
tracker_usage_statistics = true

[http_api]
bind_address = "127.0.0.1:21213"

[http_api.access_tokens]
admin = "MyAccessToken"
```

## Verification Processes

### V1 - Regression Tests Red Against the Old Path, Green After (AC4)

- Goal: prove each package's test fails when the counter is read through `Sum::sum` and `as u64`, and passes through `sum_counter`.
- Initial state: `torrust-metrics` 0.2.0 already bumped, so both APIs exist.
- Status: `DONE`

#### Steps Performed

1. UDP core: the test was added with the accessors still on the old path.
   `cargo test -p torrust-tracker-udp-core --lib exactly_a_total`
2. HTTP core: the test was added after the accessors moved. The mutate-then-restore method followed: `tcp4_announces_handled` was temporarily rewritten to the old `Sum::sum(...) as u64`, the test run, and the file restored from a copy.
   `cargo test -p torrust-tracker-http-core --lib exactly_a_total`
3. UDP server: the test was added with the accessors still on the old path.
   `cargo test -p torrust-tracker-udp-server --lib f64_cannot_represent`
4. The same commands were rerun after the production change.

#### Observed Result

UDP core, red (run before the production change):

```text
test statistics::metrics::tests::it_should_project_exactly_a_total_that_an_f64_cannot_represent ... FAILED
thread '...' panicked at packages/udp-core/src/statistics/metrics.rs:228:9:
assertion `left == right` failed
  left: 9007199254740992
 right: 9007199254740993
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 96 filtered out
```

HTTP core, red (mutated tree): see [`evidence/red-http-core.txt`](evidence/red-http-core.txt), `left: 9007199254740992`, `right: 9007199254740993`. UDP server, red: see [`evidence/red-udp-server.txt`](evidence/red-udp-server.txt), the same values. Green after the change, for example UDP server ([`evidence/green-udp-server.txt`](evidence/green-udp-server.txt)):

```text
test statistics::metrics::tests::udp_general_metrics::it_should_return_exactly_a_udp_requests_aborted_total_that_an_f64_cannot_represent ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 224 filtered out
```

#### Conclusion

Each test fails on the old path with the value an `f64` rounds to, 2^53 = 9007199254740992, instead of 2^53 + 1 = 9007199254740993, and passes after the change.

### V2 - M1: UDP Counters Over the REST API

- Goal: confirm the UDP counters read through `sum_counter` match the number of requests sent.
- Initial state: fresh tracker, all counters 0.
- Status: `DONE`

#### Steps Performed

1. `./target/release/torrust-tracker --config-toml-path .tmp/2560/tracker.toml > .tmp/2560/tracker.log 2>&1`
2. 1,000 sequential invocations of `./target/release/tracker_client udp announce 127.0.0.1:26970 9c38422213e30bff212b30c360d26f9a02136422`. Each invocation sends one connect request and one announce request. All 1,000 succeeded.
3. After 2 seconds, `GET http://127.0.0.1:21213/api/v1/stats?token=MyAccessToken`.

#### Observed Result

```text
udp4_requests: 2000
udp4_connections_handled: 1000
udp4_announces_handled: 1000
udp4_responses: 2000
udp4_errors_handled: 0
udp_requests_aborted: 0
udp_banned_ips_total: 0
udp_avg_connect_processing_time_ns: 15599
udp_avg_announce_processing_time_ns: 22608
```

#### Conclusion

Met. Every counter equals the number of requests sent, and the averages are whole numbers of nanoseconds. The spec scenario said 1,000 connect requests. This run sent 1,000 connect and 1,000 announce requests, because that is what `tracker_client udp announce` does.

### V3 - M2: HTTP Counters and the Banned-IP Gauge

- Goal: confirm the HTTP counter and the one retained gauge cast on a non-zero value.
- Initial state: the tracker of V2 still running, with its counters as left by V2.
- Status: `DONE`

#### Steps Performed

1. 50 invocations of `./target/release/tracker_client http announce http://127.0.0.1:27071 <info-hash>`, each with a different 40-hex-digit info hash. All succeeded. Then `GET /api/v1/stats`.
2. Six UDP announce packets of 98 bytes with a made-up 8-byte connection ID (the first 16 hex digits of the packet below), to trigger a ban with `max_connection_id_errors_per_ip = 3`. `tracker_client` cannot send an invalid connection ID, so each packet was written with the shell (a disposable one-liner, not kept as a script):

   ```sh
   H="deadbeefdeadbeef0000000100000007$(printf '%040d' 0 | sed 's/0/a/g')$(printf '%040d' 0 | sed 's/0/b/g')$(printf '%048d' 0)$(printf '%024d' 0)ffffffff1ae1"
   for i in 1 2 3 4 5 6; do echo -n "$H" | xxd -r -p > /dev/udp/127.0.0.1/26970; sleep 0.3; done
   ```

3. After 2 seconds, `GET /api/v1/stats` and the Prometheus metrics.

#### Observed Result

```text
tcp4_announces_handled: 50  udp_banned_ips_total: 0

udp_banned_ips_total: 1
udp_requests_banned: 2
udp4_errors_handled: 4
udp_requests_aborted: 0
udp4_requests: 2006
udp4_connections_handled: 1000

# TYPE udp_tracker_server_ips_banned_total gauge
udp_tracker_server_ips_banned_total 1
```

#### Conclusion

Met. `tcp4_announces_handled` equals the 50 announces sent. Of the 6 invalid packets, 4 produced a connection-ID error, which banned the IP, and the other 2 were rejected as banned (4 + 2 = 6). The banned-IP gauge reads 1 through the retained cast in `udp_banned_ips_total`, and Prometheus reports the same value as a gauge.

## Failures and Follow-up

None. One planning note: the spec said M1 would send 1,000 connect requests. The maintained client sends a connect and an announce per invocation, so the evidence records both.
