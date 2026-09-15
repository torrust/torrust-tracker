---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2221-1488-si-5-migrate-activity-metrics-updater/ISSUE.md
last-updated-utc: 2026-09-15 11:50
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-15 09:46–09:49
- Artifact under test: `./target/debug/torrust-tracker` built from branch `2221-migrate-activity-metrics-updater`
- Operating system / environment: Linux
- Prerequisites and setup performed: Created a git-ignored isolated tracker configuration at `.tmp/2221-activity-metrics.toml`, with `tracker_usage_statistics = true`, torrent cleanup disabled, and listeners bound to UDP `127.0.0.1:16969`, HTTP `127.0.0.1:17070`, and health API `127.0.0.1:11313`.

## Verification Processes

### V1 - No direct OS-signal dependency

- Goal: Verify that no code in the package listens for Ctrl+C.
- Initial state: The token-aware runner migration was built locally.
- Status: `DONE`

#### Steps Performed

1. Ran `rg 'ctrl_c' packages/swarm-coordination-registry/`.

#### Observed Result

```text
(no matches)
```

#### Conclusion

The `swarm-coordination-registry` package has no remaining direct `ctrl_c()` dependency.

### V2 - Direct-binary SIGTERM cancellation

- Goal: Verify that the direct tracker binary cancels and cooperatively joins the activity metrics updater.
- Initial state: The isolated configuration enabled activity metrics. The direct binary process was started with `RUST_LOG=debug` and `TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/2221-activity-metrics.toml`; its log confirmed `Tracker shutdown signal handlers installed.`.
- Status: `DONE`

#### Steps Performed

1. Built the binary with `cargo build --bin torrust-tracker`.
2. Started `./target/debug/torrust-tracker` directly and waited for its signal-handler log entry.
3. Sent `SIGTERM` to its direct process ID with `kill -TERM <pid>`.
4. Waited for process exit and inspected `.tmp/2221-runtime.log`.

#### Observed Result

```text
INFO  ... activity_metrics_updater: Stopping peers activity metrics update job ...
INFO  ... bootstrap::jobs::manager: Job completed after cooperative cancellation job=peers_inactivity_update
INFO  torrust_tracker: Torrust tracker successfully shutdown.
```

The process exited with status `0`. No grace-deadline abort outcome for `peers_inactivity_update` appeared in the log.

#### Conclusion

SIGTERM reached the application signal boundary, `JobManager` cancellation reached the updater, and the updater completed cooperatively.

### V3 - Metrics still update

- Goal: Verify normal operation still executes activity metrics at the existing 15-second interval.
- Initial state: The same direct binary and isolated configuration were run with `RUST_LOG=debug`.
- Status: `DONE`

#### Steps Performed

1. Started the tracker and waited for the signal-handler log entry.
2. Observed two activity update intervals in `.tmp/2221-metrics-runtime.log`.
3. Sent `SIGTERM` and waited for clean exit.

#### Observed Result

```text
2026-09-15T09:48:24.114318Z DEBUG ... Updating peers and torrents activity metrics (executed every 15 secs) ...
2026-09-15T09:48:24.114389Z DEBUG ... Peers and torrents activity metrics updated in 0 ms
2026-09-15T09:48:39.113404Z DEBUG ... Updating peers and torrents activity metrics (executed every 15 secs) ...
2026-09-15T09:48:39.113467Z DEBUG ... Peers and torrents activity metrics updated in 0 ms
2026-09-15T09:48:40.212427Z INFO  ... Stopping peers activity metrics update job ...
2026-09-15T09:48:40.212716Z INFO  torrust_tracker: Torrust tracker successfully shutdown.
```

The process exited with status `0`.

#### Conclusion

The migration preserves the normal 15-second metrics update loop and clean cancellation.

### V4 - Remaining migration boundary

- Goal: Verify this task removed only the activity updater from legacy registration.
- Initial state: The migrated application wiring was present.
- Status: `DONE`

#### Steps Performed

1. Ran `rg -n -A3 'register_legacy\(' src/app.rs`.

#### Observed Result

```text
375:    job_manager.register_legacy(
376-        "udp_ban_cleanup",
377-        jobs::udp_tracker_server::start_ban_cleanup_job(
```

#### Conclusion

`udp_ban_cleanup` is the sole remaining compatibility registration; this task does not change its lifecycle.

## Failures and Follow-up

The first paused-time test compile exposed that `tokio::time::advance` requires Tokio's `test-util` feature. The feature is now declared only under this package's `[dev-dependencies]`; after that adjustment, focused lifecycle tests passed. No runtime verification scenario failed.
