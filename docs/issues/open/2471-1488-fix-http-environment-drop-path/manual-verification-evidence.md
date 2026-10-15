---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2471-1488-fix-http-environment-drop-path/ISSUE.md
last-updated-utc: 2026-10-15 17:20
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

`.tmp/` and other git-ignored paths are not part of the repository, so a path
there is not evidence once the run ends. Record everything a reviewer needs in
this issue folder:

- Embed small artifacts, such as a tracker configuration file, verbatim in
  this document.
- Store larger artifacts, such as full logs or captured responses, in an
  `evidence/` subfolder of the issue folder and link them from here.
- Record runtime outputs (database rows, responses, relevant log lines) inline
  in each process.

A local `.tmp/` path may still be mentioned to show where a run wrote its
files, but never as the only copy of something the evidence relies on.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-07 11:00-11:06
- Artifact under test: `develop` at `24bf4746a` (merge of #2465), the
  `torrust-tracker-axum-http-server` test environment.
- Operating system / environment: Linux; nightly Rust toolchain
  `rustc 1.101.0-nightly (ea137335b 2026-10-05)`, `cargo 1.101.0-nightly
  (f3865b2a4 2026-09-29)`.
- Prerequisites and setup performed: none. The environment binds an ephemeral
  local port.

## Verification Processes

### V1 - Drop a Running Environment Without `stop()` (Reproduction)

- Goal: observe whether dropping a started environment releases its HTTP
  binding.
- Original symptom: found by source review on 2026-10-07, not by a failure.
  The HTTP environment's running state holds a plain `CancellationToken` and
  no `DropGuard`, while PR #2459 F1 showed that the UDP environment's server
  kept running after a drop without `stop()` until it got a `DropGuard`.
- Hypothesis: dropping a `CancellationToken` does not cancel it, so dropping a
  started HTTP environment without `stop()` leaves its server task running and
  its address bound. SI-16 (PR #2439) replaced the halt sender, whose drop did
  end the server, with this token.
- Initial state: a started environment; no `stop()` call.
- Status: `DONE` (outcome: **Reproduced**)

The wrong outcome is internal to the test fixture, so the nearest observation
seam is a temporary test in the environment's test module. It was added to
`packages/axum-http-server/src/testing/environment.rs`, run once, and reverted
with `git checkout --`; it was never committed. Its code, verbatim:

```rust
#[tokio::test]
async fn temporary_reproduction_http_binding_after_drop_without_stop() {
    let environment = start_within_deadline(unstarted_environment().await).await;
    let binding: SocketAddr = *environment.bind_address();

    drop(environment);

    tokio::time::timeout(TEST_DEADLINE, async {
        while TcpListener::bind(binding).is_err() {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the HTTP binding should be released within the test deadline after the environment is dropped");
}
```

`TEST_DEADLINE` is the module's existing 10-second constant.

#### Steps Performed

1. Added the test above after `it_should_release_the_http_binding_when_stopped`.
2. `cargo test -p torrust-tracker-axum-http-server --lib -- testing::environment::tests::temporary_reproduction`
   (nightly Rust toolchain).
3. Reverted the file.

#### Observed Result

```text
running 1 test
test testing::environment::tests::temporary_reproduction_http_binding_after_drop_without_stop ... FAILED

---- testing::environment::tests::temporary_reproduction_http_binding_after_drop_without_stop stdout ----

thread 'testing::environment::tests::temporary_reproduction_http_binding_after_drop_without_stop' (1556734) panicked at packages/axum-http-server/src/testing/environment.rs:390:10:
the HTTP binding should be released within the test deadline after the environment is dropped: Elapsed(())

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 43 filtered out; finished in 10.02s
```

#### Conclusion

Reproduced: the binding was still taken 10 s after the drop, so the HTTP
server kept running.

Pre-regression behavior, from source (not run): before PR #2439
(`29afe946c^1`), the running state held `halt_task:
oneshot::Sender<Halted>`. `torrust-server-lib` 0.3.0's `shutdown_signal`
awaits the receiver and panics on `Err`, which a dropped sender produces, so
the server task ended when the environment was dropped. PR #2459 F1 recorded
the same mechanism for the UDP environment.

## Red, Fix, Green and Recheck

### V2 - Maintained Regression Test Is Red Before the Fix

- Boundary: a unit test in the environment's own test module. The environment
  decides what dropping it does, so this is the causal seam; it mirrors the UDP
  test `it_should_release_the_udp_socket_when_dropped_without_being_stopped`.
- Toolchain: `rustc 1.99.0 (b940084d7 2026-09-28)`, `cargo 1.99.0 (5f94df478
  2026-08-27)`, on `develop` plus only the new test (the plain `CancellationToken`
  still in place).
- Command: `cargo test -p torrust-tracker-axum-http-server --lib -- testing::environment::tests::it_should_release_the_http_binding_when_dropped`

```text
running 1 test
test testing::environment::tests::it_should_release_the_http_binding_when_dropped_without_being_stopped ... FAILED

---- testing::environment::tests::it_should_release_the_http_binding_when_dropped_without_being_stopped stdout ----

thread 'testing::environment::tests::it_should_release_the_http_binding_when_dropped_without_being_stopped' (1190028) panicked at packages/axum-http-server/src/testing/environment.rs:394:10:
the HTTP binding should be released within the test deadline after the environment is dropped: Elapsed(())

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 43 filtered out; finished in 10.02s
```

### V3 - Green After the Fix, and Like-for-Like Recheck of V1

The fix: `Running` holds the token as a `DropGuard` (`cancel_on_drop`);
`stop()` disarms it, then cancels and joins exactly as before.

- Command: `cargo test -p torrust-tracker-axum-http-server --lib -- testing::environment`

```text
running 7 tests
test testing::environment::tests::it_should_report_a_drain_timeout_even_when_every_task_joins ... ok
test testing::environment::tests::it_should_join_the_event_listener_before_reporting_an_http_server_task_failure ... ok
test testing::environment::tests::it_should_release_the_http_binding_when_stopped ... ok
test testing::environment::tests::it_should_not_leave_the_statistics_listener_running_when_the_http_server_fails_to_start ... ok
test testing::environment::tests::it_should_stop_without_a_join_failure_when_every_owned_task_finishes_through_cancellation ... ok
test testing::environment::tests::it_should_release_the_http_binding_when_dropped_without_being_stopped ... ok
test testing::environment::tests::it_should_serve_requests_after_being_stopped_and_started_again ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 1.03s
```

- Like-for-like recheck of V1: the maintained test has the same body as the
  V1 temporary test (start, read the binding, drop, poll
  `TcpListener::bind` for up to `TEST_DEADLINE`). V1 timed out after 10.02 s;
  the maintained test now passes. The artifact under test is the test
  environment itself, so the test run is the real-artifact recheck.
- Package suite: `cargo test -p torrust-tracker-axum-http-server`: 44 unit
  tests and 62 integration tests passed, 0 failed.
- `cargo clippy -p torrust-tracker-axum-http-server --all-targets --all-features -- -D warnings`: clean.
- Code state: the working tree for the commit
  `fix(axum-http-server): [#2471] stop the HTTP test environment when dropped without stop()`.

## Failures and Follow-up

None.
