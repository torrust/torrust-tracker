<!-- markdownlint-disable MD003 -->
doc-type: manual-verification-evidence
spec-path: docs/issues/closed/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
last-updated-utc: 2026-09-25
last-updated-utc: 2026-09-16 10:50
semantic-links:
      related-artifacts:
            - docs/issues/closed/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
---

# Manual Verification Evidence - Additive Server Lifecycle API

> **Status**: Complete. This evidence applies only to the additive API
> introduction and does not claim removal of legacy shutdown behavior.

## Environment

- Date and time (UTC): 2026-09-16 10:22
- OS: Linux
- Tracker commit/branch: `2234-1488-si-2-token-server-lifecycle` (tracker adoption worktree)
- Tracker source: the verification worktree content is recorded in signed commit `25f0ab34`; the tests ran before that commit was created from the identical pending adoption diff on base revision `222cf975f1977117fa653b7f3b9e4481aba4f569`.
- Rust toolchain: `rustc 1.98.1 (48a229cea 2026-09-01)`
- Artifact under test: `./target/debug/torrust-tracker`, built from the tracker adoption worktree
- Configuration provenance: `.tmp/2221-activity-metrics.toml`. The path is gitignored and was used only as a local temporary file; the complete tested configuration is recorded below so this evidence remains reproducible.

### Tested Configuration

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "debug"
trace_style = "full"

[core]
listed = false
private = false
tracker_usage_statistics = true
inactive_peer_cleanup_interval = 0

[core.tracker_policy]
persistent_torrent_completed_stat = false

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:16969"

[[http_trackers]]
bind_address = "127.0.0.1:17070"

[health_check_api]
bind_address = "127.0.0.1:11313"
```

## Release Evidence

- Server-lib pull request: [#1](https://github.com/torrust/torrust-server-lib/pull/1), merged 2026-09-16 at `2b3f8ec6e20a81fcd0c25dcc1c4e3a29fe4fea17`.
- Additive API commit: `ea333ec4898c56d028e6d100af34ffd454de95c4`.
- Published crate: [`torrust-server-lib` `0.3.0`](https://crates.io/crates/torrust-server-lib/0.3.0).
- Publication output: `Published torrust-server-lib v0.3.0 at registry crates-io`.
- Release validation: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo publish --dry-run` passed before merge; required upstream CI then passed.
- Tracker final validation: the mandatory pre-commit gate passed before signed commit `25f0ab34`; the required pre-push suite passed nightly formatting, nightly workspace checks, documentation build, and the full test suite after that commit was pushed.

## Test Results

### Additive Compatibility

- [x] Existing `Halted` channel consumers compile and preserve their current
      stop behavior.
- [x] The new lifecycle API accepts injected cancellation without requiring an
      OS-signal subscription.

### Deterministic Lifecycle Test

- [x] `cargo test -p torrust-tracker-axum-server it_should_compile_and_resolve_the_server_lib_cancellation_signal_when_token_is_cancelled` requests cancellation without delivering an OS signal.
- [x] The test awaits the token-wait task's completion outcome from the published crate.

### Release Evidence

- [x] Recorded the published `torrust-server-lib` `0.3.0` release and the tracker dependency adoption above.

## Verification Processes

### V1 - Legacy consumer remains usable

- Goal: Verify that the released crate retains the tracker’s legacy `Halted`-based shutdown path.
- Initial state: The direct tracker binary used the isolated configuration above. Existing server consumers were not migrated.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo build --bin torrust-tracker`.
2. Started `RUST_LOG=debug ./target/debug/torrust-tracker --config-toml-path .tmp/2221-activity-metrics.toml >.tmp/2234-m1-sigint.log 2>&1 &` directly and recorded its PID.
3. Waited for `Tracker shutdown signal handlers installed.` in `.tmp/2234-m1-sigint.log`.
4. Ran `kill -INT <direct-binary-pid>` and `wait <direct-binary-pid>`.

#### Observed Result

```text
M1_SIGINT_EXIT=0
INFO torrust_tracker: Torrust tracker shutting down (SIGINT) ...
INFO torrust_tracker: Torrust tracker successfully shutdown.
```

#### Conclusion

The tracker compiled and ran against `torrust-server-lib 0.3.0`; its unchanged legacy consumer path completed a clean `SIGINT` shutdown.

### V2 - Token-aware wait has no signal dependency

- Goal: Verify that the new public wait primitive resolves from injected cancellation alone.
- Initial state: A fresh `CancellationToken` and a task awaiting `torrust_server_lib::signals::cancellation_signal` are constructed by the tracker contract test. The test does not deliver `SIGINT` or `SIGTERM`.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo test -p torrust-tracker-axum-server it_should_compile_and_resolve_the_server_lib_cancellation_signal_when_token_is_cancelled`.
2. The test cancelled its injected token and awaited the wait task’s completion.

#### Observed Result

```text
running 1 test
test signals::tests::it_should_compile_and_resolve_the_server_lib_cancellation_signal_when_token_is_cancelled ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

#### Conclusion

The tracker resolves the released API through token cancellation with no operating-system signal delivery.

### V3 - Tracker process cleanly restarts

- Goal: Verify that direct `SIGTERM` shutdown releases the configured listener bindings before an immediate restart.
- Initial state: The direct tracker binary used the same isolated loopback configuration for both runs.
- Status: `DONE`

#### Steps Performed

1. Started `RUST_LOG=debug ./target/debug/torrust-tracker --config-toml-path .tmp/2221-activity-metrics.toml >.tmp/2234-m3-first-sigterm.log 2>&1 &` and waited for its shutdown-signal readiness marker.
2. Ran `kill -TERM <first-direct-binary-pid>` and `wait <first-direct-binary-pid>`.
3. Started the same command immediately with `.tmp/2234-m3-restart-sigterm.log`, waited for readiness, then ran `kill -TERM <restart-direct-binary-pid>` and `wait <restart-direct-binary-pid>`.
4. Inspected `.tmp/2234-m3-first-sigterm.log` and `.tmp/2234-m3-restart-sigterm.log` for clean shutdown and address-binding failures.

#### Observed Result

```text
M3_FIRST_SIGTERM_EXIT=0
M3_RESTART_SIGTERM_EXIT=0

INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
INFO torrust_tracker: Torrust tracker successfully shutdown.
```

Neither log contained `address already in use`.

#### Conclusion

The tracker completed direct `SIGTERM` shutdown and immediately rebound all configured services on restart.
