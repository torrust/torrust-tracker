# Verification Evidence — Health-Check API Token Lifecycle Migration

> **Status**: Complete — deterministic and direct-process evidence is recorded
> for this health-check API-only vertical slice.

## Environment

- Date: 2026-09-24
- OS: Linux
- Rust version (`rustc --version`): `rustc 1.100.0-nightly (6eeff9a52 2026-09-23)`
- Tracker commit/branch: `2c9e1978` / `2324-1488-si-13-migrate-health-check-api-token-lifecycle`

## Deterministic Tests

### Test 1: Injected cancellation drains health-check API

- [x] Start the health-check API with an injected `CancellationToken`.
- [x] Cancel the token without delivering `SIGINT` or `SIGTERM`.
- [x] Verify the component awaits its server and drain-controller children.
- [x] Verify the component reports the named `health_check_api` outcome.

**Evidence:**

Rust nightly `1.100.0-nightly`:

```text
cargo test -p torrust-tracker-axum-health-check-api-server --lib
test server::tests::it_should_release_the_listener_when_token_aware_startup_registration_fails ... ok
test server::tests::it_should_drain_the_token_aware_health_check_api_when_its_cancellation_token_is_cancelled ... ok
test server::tests::it_should_register_the_token_aware_health_check_api_service ... ok
test result: ok. 3 passed; 0 failed
```

The named outcome is covered by Test 3.

### Test 2: Unexpected server completion is reported

- [x] Cause or simulate health-check server-task completion/failure without
      cancellation.
- [x] Verify an explicit outcome is reported without detaching the drain
      controller.

**Evidence:**

Rust nightly `1.100.0-nightly`:

```text
cargo test -p torrust-tracker --lib -- health_check_api
test bootstrap::jobs::health_check_api::tests::it_should_complete_after_draining_when_the_health_check_api_server_stops_independently ... ok
test bootstrap::jobs::health_check_api::tests::it_should_fail_after_draining_when_the_health_check_api_server_returns_an_error ... ok
test bootstrap::jobs::health_check_api::tests::it_should_fail_after_draining_when_the_health_check_api_server_task_panics ... ok
test bootstrap::jobs::health_check_api::tests::it_should_release_the_listener_when_the_component_is_dropped_before_it_runs ... ok
test app::tests::it_should_cancel_the_health_check_api_component_through_the_job_manager ... ok
test result: ok. 5 passed; 0 failed
```

The completion and failure tests use a drain controller that stays blocked
after observing cancellation. Each asserts the supervisor is unfinished until
the test releases that controller. The drop test was proved by mutation:
creating the owner inside the returned future made it fail after its 5-second
bound, because the detached server kept the listener bound.

### Test 3: Bootstrap wiring

- [x] Start bootstrap with the health-check API enabled.
- [x] Request root-token cancellation without an OS signal.
- [x] Verify the `health_check_api` managed component completes.

**Evidence:**

`app::tests::it_should_cancel_the_health_check_api_component_through_the_job_manager`
in Test 2 output reports `health_check_api` as `Cancelled`.

## Compatibility and Manual Evidence

- [x] Existing legacy health-check start/stop tests pass without call-site
      changes.
- [x] Existing health-check responses and readiness semantics are unchanged.
- [x] After SI-1, SIGTERM sent to the tracker binary reaches `main()` and the
      migrated health-check API records token-driven drain completion.
- [x] The token-aware path has no OS-signal subscription.
- [x] Every new drain-controller handle is retained and awaited by its
      health-check API component owner.
- [x] Do not claim unhealthy-on-shutdown behavior in this migration; SI-21 owns
      Q6's approved readiness-before-drain behavior.

**Evidence:**

The unchanged `environment::Started` start/stop contract tests in
`packages/axum-health-check-api-server/tests/server/contract.rs` pass (8
passed). Both paths share `router()` and the unchanged handler. Source review
confirms `start_with_cancellation` uses only the injected token. Its runtime
and drain-controller handles are returned in `CancellationRunning` and wrapped
in `TokenAwareServerTask` before `start_job` returns.

Direct binary PID SIGTERM and listener rebind evidence is recorded in
`manual-verification-evidence.md`.

## Summary

| Check                           | Result | Evidence link or note             |
| ------------------------------- | ------ | --------------------------------- |
| Token-driven health-check drain | Passed | Test 1                            |
| Joined child tasks              | Passed | Test 2                            |
| Unexpected server outcome       | Passed | Test 2                            |
| Bootstrap propagation           | Passed | Test 3                            |
| Legacy API compatibility        | Passed | Contract tests                    |
| Unchanged readiness behavior    | Passed | Shared router and handler         |
| Manual SIGTERM path             | Passed | `manual-verification-evidence.md` |
