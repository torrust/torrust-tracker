---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2274-1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
last-updated-utc: 2026-09-21 18:12
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

## Failures and Follow-up

The first V2 run released the handler but did not read the client response.
Axum therefore kept the connection active and the helper correctly returned
`TimedOut`. The retained example now reads the `Connection: close` response
before awaiting the drained outcome; the rerun passed both scenarios.
