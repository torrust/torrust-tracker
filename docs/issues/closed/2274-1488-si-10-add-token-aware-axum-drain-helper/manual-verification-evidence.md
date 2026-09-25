<!-- markdownlint-disable MD003 -->
issue-spec: docs/issues/closed/2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
last-updated-utc: 2026-09-25
last-updated-utc: 2026-09-22 06:23
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-09-21 18:12
- Artifact: `packages/axum-server/examples/token_aware_drain.rs` and the
  token-aware helper in `packages/axum-server/src/signals.rs`
- Environment: Linux, local Cargo workspace, `rustc 1.100.0-nightly
  (330d31712 2026-09-17)`
- Branch / starting commit: `2274-1488-si-10-add-token-aware-axum-drain-helper`
  at `e9cf4ae4`
- Setup: Built the local `axum-server` example. It uses ephemeral loopback
  listeners and does not subscribe to OS signals.

## Verification Processes

### V1 - Inspect Helper Ownership

- Goal: Confirm injected-token cancellation and owned task lifecycle.
- Status: `DONE`

#### Steps Performed

1. Reviewed `graceful_shutdown_on_cancellation` in `signals.rs`.
2. Confirmed it accepts `CancellationToken` and `drain_timeout`, calls
   `Handle::graceful_shutdown(None)` only after cancellation, returns
   `GracefulShutdownOutcome`, has no `Halted` parameter or OS-signal call, and
   contains no `tokio::spawn`.
3. Reviewed `token_aware_drain.rs`; each scenario awaits its helper task and
   then its server task.

#### Observed Result

The helper is an awaitable future with an injected cancellation token and a
post-cancellation timeout. The caller retains all spawned `JoinHandle` values.

#### Conclusion

The ownership and cancellation contract matches M1. The helper neither owns
OS-signal policy nor leaks a detached drain task.

### V2 - Exercise Graceful Drain

- Goal: Verify an in-flight loopback request can drain after token
  cancellation.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo run -p torrust-tracker-axum-server --example token_aware_drain`.
2. The first scenario cancelled its injected token and waited until a new real
  loopback connection was refused.
3. It released the blocked handler, read the real `Connection: close` response,
  and asserted `GracefulShutdownOutcome::Drained`.

#### Observed Result

```text
Finished `dev` profile [optimized + debuginfo] target(s) in 0.85s
Running `target/debug/examples/token_aware_drain`
```

#### Conclusion

The command exited with status 0. Its first scenario observed that new
connections were refused, then asserted `Drained` after the in-flight request
completed before its two-second drain budget.

### V3 - Exercise Timeout Outcome

- Goal: Verify a held loopback request reports timeout without an OS signal.
- Status: `DONE`

#### Steps Performed

1. Used the same successful command from V2.
2. The second scenario cancelled its injected token, retained the in-flight
   request past a 100 ms budget, asserted `GracefulShutdownOutcome::TimedOut`,
   then dropped the client connection and awaited the server task.

#### Observed Result

The example exited with status 0 only after the held-request scenario asserted
`GracefulShutdownOutcome::TimedOut` and cleaned up its owned server task.

#### Conclusion

The helper reported a timeout without an OS signal. A component caller can
observe that typed outcome and owns escalation policy.

### V4 - Direct Tracker Process Regression

- Goal: Verify that the supported configured tracker process remains healthy,
  handles `SIGTERM` at its executable boundary, exits cleanly, and releases its
  HTTP listener after this additive helper change.
- Scope: This does not execute `graceful_shutdown_on_cancellation` in
  production because no tracker consumer uses it yet. It is compatibility
  regression evidence for the existing legacy HTTP path; SI-11 owns the first
  migrated-path proof.
- Status: `DONE`

#### Tested Configuration

The ignored local configuration `.tmp/2274-tracker-shutdown.toml` used SQLite
storage under `.tmp/` and configured:

```toml
[[http_trackers]]
bind_address = "127.0.0.1:17070"
tracker_usage_statistics = false

[health_check_api]
bind_address = "127.0.0.1:11313"
```

#### Steps Performed

1. Built or confirmed `./target/debug/torrust-tracker`.
2. Started that direct binary, not `cargo run`, with the isolated
   configuration and captured each run's output in `.tmp/2274-<run>.log`.
3. Waited for `GET http://127.0.0.1:11313/health_check` to return success.
4. Sent `kill -TERM <exact-direct-binary-pid>` and waited for the process.
5. Immediately repeated the same start and stop using the identical bindings.

#### Terminal Output

```text
first_PID=520315
first_HEALTH_CHECK=OK
first_SIGTERM_EXIT=0
2026-09-22T06:23:28.817373Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-09-22T06:23:28.826997Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-09-22T06:23:28.827336Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
restart_PID=520361
restart_HEALTH_CHECK=OK
restart_SIGTERM_EXIT=0
2026-09-22T06:23:28.864796Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-09-22T06:23:28.875266Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-09-22T06:23:28.875598Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
HTTP_LISTENER_REBIND=OK
CONFIGURED_HTTP_TRACKER=127.0.0.1:17070
CONFIGURED_HEALTH_CHECK=127.0.0.1:11313
```

The first and restart logs both record `HTTP TRACKER Started on:
http://127.0.0.1:17070`, a successful health-check response, the legacy Axum
drain completion, and `Torrust tracker successfully shutdown.`

#### Conclusion

Both direct processes became healthy, shut down with `SIGTERM` and exit status
0, and the immediate restart rebound the configured HTTP listener. The current
tracker's legacy shutdown behavior remains operational.

## Failures and Follow-up

The first V2 run released the handler but did not read the client response.
Axum therefore kept the connection active and the helper correctly returned
`TimedOut`. The retained example now reads the `Connection: close` response
before awaiting the drained outcome; the rerun passed both scenarios.
