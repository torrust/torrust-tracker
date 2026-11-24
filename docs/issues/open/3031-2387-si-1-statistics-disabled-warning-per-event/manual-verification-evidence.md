---
doc-type: manual-verification-evidence
issue: 3031
last-updated-utc: "2026-11-24 18:24"
---

# Manual Verification Evidence

## Purpose

Real runs of the release `torrust-tracker` binary before and after the fix of issue #3031: trackers with `tracker_usage_statistics = false` wrote a `WARN` for every event they did not count.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-24, runs between 18:06 and 18:11 (the timestamps of the log lines).
- Artifact under test: release build (`cargo build --release -p torrust-tracker --bin torrust-tracker`, `rustc 1.100.0 (0ef1bc386 2026-11-09)`, Linux x86_64).
  - Before: `develop` at `392d97775` (a commit on `develop`), no change.
  - After: the implementation branch with the commit `fix(statistics): [#3031] stop warning for each event of a statistics-disabled instance` and its tests.
- Configurations, stored verbatim beside this file: [evidence/enabled.toml](evidence/enabled.toml) is `share/default/config/tracker.development.sqlite3.toml` with only the SQLite path changed to a scratch directory. [evidence/disabled.toml](evidence/disabled.toml) is the same with `tracker_usage_statistics = false` in the two `[[udp_trackers]]` and the two `[[http_trackers]]` tables (4 lines changed).
- Scenario script (kept outside the repository, run from the repository root, one step after the other): start the binary with `--config-toml-path <config>`; poll `curl -fsS http://127.0.0.1:1313/health_check` until it succeeds; wait 1 s; `tracker_client udp announce 127.0.0.1:6868 443c7602b4fde83d1154d6d9da48808418b181b6 --event started`; `tracker_client http announce http://127.0.0.1:7070 443c7602b4fde83d1154d6d9da48808418b181b6 --event started`; read `/api/v1/metrics?format=prometheus` with the admin token; wait 1 s; `SIGTERM`; wait for exit; strip ANSI codes. The health check itself sends one UDP check to each UDP tracker, so it also produces events.

## Verification Processes

### V1 - Reproduce the defect (scenario M1)

- Goal: observe the per-event `WARN` with statistics disabled.
- Initial state: release binary of `develop` `392d97775`, [evidence/disabled.toml](evidence/disabled.toml).
- Status: `DONE`. Outcome: **Reproduced**.

#### Observed Result

Exit status `0`, 275 log lines, **17 `WARN`** and 0 `ERROR` ([full log](evidence/v1-before-statistics-disabled.log)). Examples:

```text
WARN UDP TRACKER: Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }
WARN UDP TRACKER: Ignoring UDP tracker event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }
```

The UDP announce and the HTTP announce both answered normally (the announce responses show the peer registered), so the warnings are noise, not a failure.

#### Conclusion

The defect is reproduced: a supported configuration writes a `WARN` per event.

### T2 - Regression tests are red

The new tests failed on the unfixed production code.

- Unit tests, three crates: [evidence/t2-red-unit-tests.txt](evidence/t2-red-unit-tests.txt). In each of `torrust-tracker-udp-server`, `torrust-tracker-udp-core` and `torrust-tracker-http-core`, 3 of the new tests fail (`..._metrics_disabled_instance`, `..._unknown_instance`, `..._when_starting`); the metrics-enabled test passes, as it should.
- Tracker level, `cargo test --test metrics-statistics-disabled-logs`: [evidence/t2-red-tracker-test.txt](evidence/t2-red-tracker-test.txt), 13 `WARN` lines listed in the failure.

### V2 - Recheck after the fix (scenario M2)

- Goal: the same scenario on the fixed binary.
- Initial state: release binary rebuilt from the implementation branch after the last production change, [evidence/disabled.toml](evidence/disabled.toml).
- Status: `DONE`.

#### Observed Result

Exit status `0`, 258 log lines, **0 `WARN` and 0 `ERROR`** ([full log](evidence/v2-after-statistics-disabled.log)). The start lines show the state once:

```text
INFO HTTP TRACKER: Starting HTTP tracker core event listener enabled_instances=0 disabled_instances=2
INFO UDP TRACKER: Starting UDP tracker core event listener enabled_instances=0 disabled_instances=2
INFO UDP TRACKER: Starting UDP tracker server event listener enabled_instances=0 disabled_instances=2
```

#### Conclusion

Met: no per-event line, and the operator sees at startup that statistics are disabled (the `Starting ... instance` lines keep `tracker_usage_statistics=false`).

### V3 - Statistics enabled (scenario M3)

- Goal: the counted metrics are unchanged.
- Initial state: [evidence/enabled.toml](evidence/enabled.toml), same scenario, before and after the fix.
- Status: `DONE`.

#### Observed Result

Both runs: 0 `WARN`, 0 `ERROR`, 258 lines. The sums of the Prometheus counters read from the REST API after the requests are identical:

| Counter | Before | After |
| --- | --- | --- |
| `udp_tracker_core_requests_received_total` (connect 6868: 2, announce 6868: 1, connect 6969: 1) | 4 | 4 |
| `udp_tracker_server_requests_received_total` | 4 | 4 |
| `http_tracker_core_requests_received_total` | 1 | 1 |

After the fix the start lines read `enabled_instances=2 disabled_instances=0`.

#### Conclusion

Met: counting is unchanged for enabled instances.

## Failures and Follow-up

None. The unknown-listener `ERROR` cannot be produced with the real binary, because its wiring gives every configured instance a policy entry; the unit tests of the three listeners cover it.
