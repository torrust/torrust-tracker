# Verification Evidence — UDP Receive Token Lifecycle Migration

> **Status**: Complete — deterministic, manual, and performance evidence is
> recorded for this UDP ownership slice.

## Environment

- Date: 2026-09-26
- OS: Linux
- Rust version (`rustc --version`): `rustc 1.100.0-nightly (6eeff9a52 2026-09-23)`
- Tracker commit/branch: "docs(shutdown): [#2342] update task inventory and follow-up drafts after SI-14" / `2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle`

## Deterministic Tests

### Test 1: Cancellation joins the receive task

- [x] Start one UDP component with an injected `CancellationToken`.
- [x] Cancel the token without delivering `SIGINT` or `SIGTERM`.
- [x] Verify new packet admission stops.
- [x] Verify the receive loop is awaited.
- [x] Verify the separate application-level UDP IP-ban cleanup job remains
      manager-owned and token-cancellable.
- [x] Verify the component reports its named UDP outcome.

**Evidence:**

Rust nightly `1.100.0-nightly`:

```text
cargo test -p torrust-tracker-udp-server --lib -- token_aware_start receive_loop_admission legacy
test server::tests::token_aware_start::it_should_stop_the_receive_loop_and_release_the_socket_when_its_cancellation_token_is_cancelled ... ok
test server::tests::token_aware_start::it_should_register_the_service_with_a_health_check_that_reaches_the_running_server ... ok
test server::tests::token_aware_start::it_should_release_the_socket_when_registration_fails ... ok
test result: ok. 9 passed; 0 failed
```

The loop returns `Ok(())` only from its cancellation branch, so a returned
`Ok(())` proves admission stopped. The ban-cleanup job is covered by the
unchanged `udp_tracker_server::tests::it_should_stop_the_ban_cleanup_job_when_cancelled`.
The named outcome is covered by Test 4.

### Test 2: Unexpected receive-loop completion is reported

- [x] Cause or simulate receive-loop completion/failure without cancellation.
- [x] Verify an explicit UDP component outcome is reported.

**Evidence:**

Rust nightly `1.100.0-nightly`:

```text
test server::launcher::tests::receive_loop_admission::it_should_admit_a_received_datagram ... ok
test server::launcher::tests::receive_loop_admission::it_should_stop_with_the_receive_error_when_receiving_fails ... ok
test server::launcher::tests::receive_loop_admission::it_should_stop_with_the_receive_error_when_receiving_is_interrupted ... ok
test server::launcher::tests::receive_loop_admission::it_should_stop_with_an_unexpected_end_error_when_the_receive_stream_ends ... ok
test bootstrap::jobs::udp_tracker::tests::it_should_report_cancelled_when_the_receive_loop_stops_after_cancellation ... ok
test bootstrap::jobs::udp_tracker::tests::it_should_fail_when_the_receive_loop_stops_with_an_error ... ok
test bootstrap::jobs::udp_tracker::tests::it_should_fail_when_the_receive_loop_task_panics ... ok
test bootstrap::jobs::udp_tracker::tests::it_should_release_the_socket_when_the_component_is_dropped_before_it_runs ... ok
```

The drop test fails when the `OwnedTask` owner is created inside the returned
future instead of before it.

### Test 3: Active-request compatibility

- [x] Verify existing `ActiveRequests` capacity and deliberate abort behavior
      remain unchanged.
- [x] Do not add request deadlines, drain behavior, or outcome metrics here.

**Evidence:**

`request_buffer.rs` and `processor.rs` have no diff against `develop`; the
three `request_buffer` tests pass unmodified (`test result: ok. 3 passed`).

### Test 4: Bootstrap wiring

- [x] Start bootstrap with one UDP binding.
- [x] Request root-token cancellation without an OS signal.
- [x] Verify the `udp_instance_<index>_<address>` component completes.

**Evidence:**

```text
test app::tests::it_should_cancel_the_udp_tracker_component_through_the_job_manager ... ok
```

The test asserts `JobOutcome { name: "udp_instance_0_<address>", status: Cancelled }`.

## Compatibility and Manual Evidence

- [x] Existing legacy UDP start/stop tests pass without call-site changes.
- [x] After SI-1, SIGTERM sent to the tracker binary reaches `main()` and the
      migrated UDP component completes through the token path.
- [x] The token-aware UDP path has no OS-signal subscription.
- [x] The receive-loop handle is retained and awaited by the UDP component
      owner; UDP IP-ban cleanup remains separately manager-owned.

**Evidence:**

The legacy contract tests (11) and the `Server::start` / `Server::stop` unit
tests pass unchanged, and the two legacy launcher regression tests pass after
failing against the previous launcher. In `packages/udp-server/src`, only the
legacy adapter references `global_shutdown_signal`. Direct-PID SIGTERM,
cooperative cancellation, rebind, and the legacy example run are recorded in
`manual-verification-evidence.md`; throughput before and after is in
`performance-evidence.md`.

## Summary

| Check                        | Result | Evidence link or note                         |
| ---------------------------- | ------ | --------------------------------------------- |
| Token-driven UDP stop        | Passed | Test 1                                        |
| Joined receive loop          | Passed | Tests 1-2                                     |
| Managed UDP IP-ban cleanup   | Passed | Test 1                                        |
| Unexpected receive outcome   | Passed | Test 2                                        |
| Active-request compatibility | Passed | Test 3                                        |
| Bootstrap propagation        | Passed | Test 4                                        |
| Legacy API compatibility     | Passed | Compatibility evidence; M3                    |
| Manual SIGTERM path          | Passed | `manual-verification-evidence.md` M1-M2       |
| UDP throughput               | Passed | `performance-evidence.md` (no regression)     |
