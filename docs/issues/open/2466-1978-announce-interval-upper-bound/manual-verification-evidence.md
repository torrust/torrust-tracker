---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2466-1978-announce-interval-upper-bound/ISSUE.md
last-updated-utc: "2026-10-28 07:35"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-07 10:18 (pre-review defect reproduction).
- Artifact under test: `target/debug/torrust-tracker` and `target/debug/tracker_client`, built
  with the stable Rust toolchain `rustc 1.99.0 (b940084d7 2026-09-28)` from `develop` at
  `7836471b3` plus this specification PR's documentation-only commits (no code change).
- Operating system / environment: Linux; repository root as current working directory.
- Setup: the configuration below, saved as `.tmp/repro-2466/tracker-over.toml`.

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
listed = false
private = false
tracker_usage_statistics = false

[core.announce_policy]
interval = 2147483648
interval_min = 120
max_peers_per_announce = 2

[core.database]
driver = "sqlite3"
path = "./.tmp/repro-2466/sqlite3.db"

[core.tracker_policy]
persistent_torrent_completed_stat = false
remove_peerless_torrents = false

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:16971"

[[http_trackers]]
bind_address = "127.0.0.1:17071"
```

## Verification Processes

### V0 - Pre-Review Reproduction (M1 "Before")

- Goal: Observe that the tracker accepts `interval = 2147483648` and that UDP and HTTP announces
  report different intervals.
- Initial state: a fresh tracker with an empty SQLite database; one announce per protocol on
  info-hash `9c38422213e30bff212b30c360d26f9a02136422`.
- Status: `DONE`
- Outcome: **Reproduced**.

#### Steps Performed

1. Started the tracker:

   ```sh
   TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/repro-2466/tracker-over.toml ./target/debug/torrust-tracker \
     > .tmp/repro-2466/tracker-over.log 2>&1 &
   ```

2. Announced over UDP:

   ```sh
   ./target/debug/tracker_client udp announce 127.0.0.1:16971 9c38422213e30bff212b30c360d26f9a02136422 \
     --peer-id=-qB00000000000000001 --port 7001 --event started --left 100
   ```

3. Announced over HTTP:

   ```sh
   ./target/debug/tracker_client http announce http://127.0.0.1:17071 9c38422213e30bff212b30c360d26f9a02136422
   ```

4. Stopped the tracker.

#### Observed Result

The tracker started and logged the configured value:

```text
"interval": 2147483648
```

UDP announce (clamped by #2245):

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":2147483647,"leechers":1,"seeders":0,"peers":[]}}
```

HTTP announce (configured value):

```text
{"complete":1,"incomplete":1,"interval":2147483648,"min interval":120,"peers":[{"ip":"127.0.0.1","peer id":[45,113,66,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,49],"port":7001}]}
```

#### Conclusion

Reproduced: the configuration is accepted, UDP clients receive `2147483647`, and HTTP clients
receive `2147483648` for the same tracker. M1's "after" half and M2-M3 run after the fix.

### T2 - Regression Tests, Red Then Green

- Goal: show that the configuration-load regression tests fail against the `u32` fields and pass
  after the fix.
- Toolchain: stable `rustc 1.99.1 (58cf8f9c3 2026-10-13)`.
- Status: `DONE`
- Outcome: **Red before the fix, green after it.**

Red (T2 tests only, before the bounded type was adopted by `AnnouncePolicy`):

```sh
cargo test -p torrust-tracker-configuration announce_interval
```

```text
running 2 tests
test v3_0_0::tests::smoke::it_should_reject_a_minimum_announce_interval_above_the_i32_maximum_naming_the_field_value_and_limit ... FAILED
test v3_0_0::tests::smoke::it_should_reject_an_announce_interval_above_the_i32_maximum_naming_the_field_value_and_limit ... FAILED
...
an announce interval above i32::MAX should be rejected: Configuration { ... announce_policy: AnnouncePolicy { interval: 2147483648, interval_min: 120, ... } ... }
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 129 filtered out
```

Both tests failed because `Configuration::load` returned `Ok` with the out-of-range value.

Green (after `AnnouncePolicy` uses `AnnounceIntervalInSecs`; the key path assertion and the
boundary test were added in the same commit):

```text
test v3_0_0::tests::smoke::it_should_accept_the_i32_maximum_for_both_announce_intervals ... ok
test v3_0_0::tests::smoke::it_should_reject_an_announce_interval_above_the_i32_maximum_naming_the_field_value_and_limit ... ok
test v3_0_0::tests::smoke::it_should_reject_a_minimum_announce_interval_above_the_i32_maximum_naming_the_field_value_and_limit ... ok
```

### Environment for V1-V3 (after the fix)

- Date and time (UTC): 2026-10-28 07:25-07:30.
- Artifact under test: `target/debug/torrust-tracker` and `target/debug/tracker_client`, built
  with stable `rustc 1.99.1 (58cf8f9c3 2026-10-13)` from branch
  `2466-1978-announce-interval-upper-bound` at `d5c8fcddc`.
- Setup: the V0 configuration with only the announce policy changed, saved as
  `.tmp/repro-2466/tracker-over.toml` (`interval = 2147483648`),
  `.tmp/repro-2466/tracker-min-over.toml` (`interval_min = 2147483648`, `interval = 120`), and
  `.tmp/repro-2466/tracker-boundary.toml` (`interval = 2147483647`).

### V1 - Reject an Out-of-Range Interval (M1 "After")

- Goal: the tracker refuses to start when `interval = 2147483648`.
- Status: `DONE`
- Outcome: **Rejected as expected.**

```sh
TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/repro-2466/tracker-over.toml ./target/debug/torrust-tracker
```

```text
Tracker startup failed: ... Failed processing the configuration: the interval 2147483648 is above the maximum of 2147483647 seconds that every delivery protocol can encode for key "default.core.announce_policy.interval" in .tmp/repro-2466/tracker-over.toml TOML file, packages/configuration/src/v3_0_0/mod.rs:392:28
```

Exit code `1`. The error names the field (`core.announce_policy.interval`), the value
(`2147483648`), and the limit (`2147483647`). No listener was opened.

### V2 - Reject an Out-of-Range Minimum Interval (M2)

- Goal: the tracker refuses to start when `interval_min = 2147483648`.
- Status: `DONE`
- Outcome: **Rejected as expected.**

```sh
TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/repro-2466/tracker-min-over.toml ./target/debug/torrust-tracker
```

```text
Tracker startup failed: ... Failed processing the configuration: the interval 2147483648 is above the maximum of 2147483647 seconds that every delivery protocol can encode for key "default.core.announce_policy.interval_min" in .tmp/repro-2466/tracker-min-over.toml TOML file, packages/configuration/src/v3_0_0/mod.rs:392:28
```

Exit code `1`. The error names the field (`core.announce_policy.interval_min`), the value, and the
limit.

### V3 - Accept the Boundary (M3)

- Goal: `interval = 2147483647` is accepted, and UDP and HTTP announces report the same interval.
- Status: `DONE`
- Outcome: **Accepted; both protocols report `2147483647`.**

```sh
TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/repro-2466/tracker-boundary.toml ./target/debug/torrust-tracker \
  > .tmp/repro-2466/tracker-boundary.log 2>&1 &
./target/debug/tracker_client udp announce 127.0.0.1:16971 9c38422213e30bff212b30c360d26f9a02136422 \
  --peer-id=-qB00000000000000001 --port 7001 --event started --left 100
./target/debug/tracker_client http announce http://127.0.0.1:17071 9c38422213e30bff212b30c360d26f9a02136422
```

The tracker logged the configured value:

```text
"interval": 2147483647,
```

UDP announce:

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":2147483647,"leechers":1,"seeders":0,"peers":[]}}
```

HTTP announce:

```text
{"complete":1,"incomplete":1,"interval":2147483647,"min interval":120,"peers":[{"ip":"127.0.0.1","peer id":[45,113,66,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,49],"port":7001}]}
```

The tracker was stopped afterwards and no listener remained on ports `16971` and `17071`.

#### Conclusion

Before the fix the same tracker reported `2147483647` over UDP and `2147483648` over HTTP. After
the fix, an out-of-range `interval` or `interval_min` is rejected at startup with the key, the
value, and the limit, and the boundary value is accepted and reported identically by both
protocols.

## Failures and Follow-up

None.
