# Verification Evidence - Token-Aware, Joinable Axum Drain Helper

> **Status**: Complete. Focused helper and compatibility verification, manual
> scenarios, direct tracker regression verification, completion review, and the
> full pre-push quality gate passed.

## Environment

- Date: 2026-09-22 06:23 UTC
- OS: Linux
- Rust version (`rustc --version`): `rustc 1.100.0-nightly (330d31712 2026-09-17)`
- Tracker commit/branch: `822120f9` on
      `2274-1488-si-10-add-token-aware-axum-drain-helper`

## Deterministic Helper Tests

### Test 1: Cancellation starts graceful drain

- [x] Inject a `CancellationToken` into the new helper.
- [x] Cancel the token without sending `SIGINT` or `SIGTERM`.
- [x] Verify the helper requests graceful shutdown and returns a drained outcome.

**Evidence:**

```text
cargo test -p torrust-tracker-axum-server

running 5 tests
test signals::tests::it_should_return_drained_when_the_token_is_cancelled_with_no_active_connections ... ok
test signals::tests::it_should_return_timed_out_when_a_connection_remains_active_after_cancellation ... ok
test result: ok. 5 passed; 0 failed
```

### Test 2: Drain deadline returns a timeout outcome

- [x] Hold the helper's connection-count condition above zero using a controlled
      test double or fixture.
- [x] Verify the helper returns its deadline-reached outcome.
- [x] Verify the test does not deliver an OS signal.

**Evidence:**

```text
The paused-time collaboration test uses a real local Axum server and a pending
request. It advances the visible 500 ms timeout and asserts the helper finishes
before its one-second status tick with GracefulShutdownOutcome::TimedOut.
```

## Compatibility

- [x] Existing `graceful_shutdown` call sites remain unchanged.
- [x] Existing HTTP tracker, REST API, and health-check server tests pass.
- [x] The new helper has no OS-signal subscription or shutdown `Halted` channel
      parameter.
- [x] The new helper creates no detached task.

**Evidence:**

```text
cargo test -p torrust-tracker-axum-http-server \
      -p torrust-tracker-axum-rest-api-server \
      -p torrust-tracker-axum-health-check-api-server

All selected package suites passed. The health-check package reported 58
passing tests. Source review confirmed the legacy helper and its three caller
sites were unchanged.

cargo clippy -p torrust-tracker-axum-server --all-targets -- -D warnings
cargo fmt --all -- --check

Both passed after adding the retained manual verifier example.
```

## Direct Tracker Process Regression

The new SI-10 helper is additive and is not yet used by a production tracker
component. This process-level check therefore proves that the supported legacy
HTTP shutdown path remains intact; SI-11 owns the first production execution
of the token-aware helper.

- [x] Direct binary ran with one configured HTTP tracker and health-check API.
- [x] Each healthy process received `SIGTERM` at its exact PID and exited 0.
- [x] The process logged main-boundary shutdown and successful completion.
- [x] An immediate restart with the same bindings succeeded, proving release.

The complete command, isolated configuration, PIDs, output, and relevant logs
are recorded in `manual-verification-evidence.md` section V4.

## Summary

| Check                              | Result  | Evidence link or note |
| ---------------------------------- | ------- | --------------------- |
| Token cancellation initiates drain | Pass | Helper tests and manual V2-V3 |
| Drained outcome                    | Pass | Helper test and manual V2 |
| Deadline-reached outcome           | Pass | Helper test and manual V3 |
| Legacy helper compatibility        | Pass | Focused consumer suites |
| No OS-signal or detached task      | Pass | Source review and manual V1 |
| Direct tracker legacy shutdown     | Pass | Manual V4; additive compatibility regression only |
