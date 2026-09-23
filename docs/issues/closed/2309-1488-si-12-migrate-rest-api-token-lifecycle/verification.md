# Verification Evidence — REST API Token Lifecycle Migration

> **Status**: Complete — deterministic and direct-process evidence is recorded
> for this REST API-only vertical slice.

## Environment

- Date: 2026-09-23
- OS: Linux
- Rust version (`rustc --version`): `rustc 1.100.0-nightly (1303417c4 2026-09-21)`
- Tracker commit/branch: `78d7e0fe` / `2309-1488-si-12-migrate-rest-api-token-lifecycle`

## Deterministic Tests

### Test 1: Injected cancellation drains REST API

- [x] Start the REST API component with an injected `CancellationToken`.
- [x] Cancel the token without delivering `SIGINT` or `SIGTERM`.
- [x] Verify the component awaits its server and drain-controller children.
- [x] Verify the component reports the named `http_api` outcome.

**Evidence:**

Rust nightly `1.100.0-nightly`:

```text
cargo test -p torrust-tracker-axum-rest-api-server server::tests
running 3 tests
test server::tests::it_should_drain_the_token_aware_rest_api_when_its_cancellation_token_is_cancelled ... ok
test server::tests::it_should_release_the_listener_when_token_aware_startup_registration_fails ... ok
test server::tests::it_should_be_able_to_start_and_stop ... ok
test result: ok. 3 passed; 0 failed
```

### Test 2: Unexpected server completion is reported

- [x] Cause or simulate REST API server-task completion/failure without
      cancellation.
- [x] Verify an explicit outcome is reported without detaching the drain
      controller.

**Evidence:**

Rust nightly `1.100.0-nightly`:

```text
cargo test -p torrust-tracker bootstrap::jobs::tracker_apis
running 3 tests
test bootstrap::jobs::tracker_apis::tests::it_should_complete_after_draining_when_the_rest_api_server_stops_independently ... ok
test bootstrap::jobs::tracker_apis::tests::it_should_fail_after_draining_when_the_rest_api_server_task_panics ... ok
test bootstrap::jobs::tracker_apis::tests::it_should_start_http_tracker ... ok
test result: ok. 3 passed; 0 failed
```

The completion and failure tests use a drain controller that remains blocked
after observing cancellation. Each test asserts the supervisor is unfinished
until the test explicitly releases that controller.

### Test 3: Bootstrap wiring

- [x] Start bootstrap with the REST API enabled.
- [x] Request root-token cancellation without an OS signal.
- [x] Verify the `http_api` managed component completes.

**Evidence:**

Rust nightly `1.100.0-nightly`:

```text
cargo test -p torrust-tracker it_should_cancel_the_rest_api_component_through_the_job_manager
running 1 test
test app::tests::it_should_cancel_the_rest_api_component_through_the_job_manager ... ok
test result: ok. 1 passed; 0 failed
```

## Compatibility and Manual Evidence

- [x] Existing legacy REST API start/stop tests pass without call-site changes.
- [x] After SI-1, SIGTERM sent to the tracker binary reaches `main()` and the
      migrated REST API records token-driven drain completion.
- [ ] The token-aware REST API path has no OS-signal subscription.
- [ ] Every drain-controller handle created by the new path is retained and
      awaited by its REST API component owner.

**Evidence:**

The unchanged `ApiServer::start` / `ApiServer::stop` test passes in Test 1.
Source review confirms `start_with_cancellation` uses the injected token only;
the new path does not import or call a signal subscription API. Runtime and
drain-controller handles are returned in `CancellationRunning` and consumed by
`tracker_apis::supervise_token_aware_server`.

Direct binary PID SIGTERM and listener rebind evidence is recorded in
`manual-verification-evidence.md`.

## Summary

| Check                       | Result  | Evidence link or note |
| --------------------------- | ------- | --------------------- |
| Token-driven REST API drain | Passed  | Test 1               |
| Joined child tasks          | Passed  | Test 2               |
| Unexpected server outcome   | Passed  | Test 2               |
| Bootstrap propagation       | Passed  | Test 3               |
| Legacy API compatibility    | Passed  | Test 1               |
| Manual SIGTERM path         | Passed  | `manual-verification-evidence.md` |
