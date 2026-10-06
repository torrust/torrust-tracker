---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md
last-updated-utc: "2026-10-05 17:40"
---

# Verification Evidence — Standalone HTTP Environment and Example

<!-- cspell:ignore ltnp -->

> **Status**: Complete — T0 baseline (V1) and post-implementation runs (V2-V4)
> recorded. The deterministic `stop()` guarantees are automated tests, listed
> below rather than repeated as manual evidence.

## Environment

- Date: 2026-10-04
- OS: Linux 7.0.0-34-generic
- Rust version (`rustc --version`): `rustc 1.101.0-nightly (0abfedbc7 2026-10-02)`
- Tracker commit/branch: `135ec2ab` (`develop`), branch
  `2412-migrate-standalone-http-environment` before any change

## T0 Baseline Test Results

Before any change, both affected packages pass:

```text
$ cargo test -p torrust-tracker-axum-http-server
test result: ok. 36 passed; 0 failed   (unit)
test result: ok. 61 passed; 0 failed   (integration)

$ cargo test -p torrust-tracker-axum-health-check-api-server
test result: ok. 3 passed; 0 failed
test result: ok. 8 passed; 0 failed
```

## Deterministic Environment Tests

These guarantees are covered by automated tests in
`packages/axum-http-server/src/testing/environment.rs` (no OS signals). Each
was proven by a mutation recorded in the spec's progress log.

| Guarantee | Test |
| --- | --- |
| `stop()` cancels once and joins every owned task without a join failure | `it_should_stop_without_a_join_failure_when_every_owned_task_finishes_through_cancellation` |
| The listener is not aborted (aborting it fails the stop-path tests) | Same test, plus the two below |
| The HTTP binding is free when `stop()` returns | `it_should_release_the_http_binding_when_stopped` |
| A stopped environment can start again and serve requests | `it_should_serve_requests_after_being_stopped_and_started_again` |
| An early server failure is reported only after the listener joins | `it_should_join_the_event_listener_before_reporting_an_http_server_task_failure` |
| A drain timeout is reported by name | `it_should_report_a_drain_timeout_even_when_every_task_joins` |

```text
$ cargo test -p torrust-tracker-axum-http-server --lib testing::environment
test result: ok. 5 passed; 0 failed
```

Source review: `environment.rs` contains no `abort()` call.

## Example Executable Evidence

### V1: Baseline SIGTERM

- [x] Run the unmodified example directly, wait for readiness, signal its PID,
      and record output and exit status before changing the example.

Classification: the spec expected "not handled, killed by the default action".
The actual baseline is worse: the HTTP library catches SIGTERM itself, stops
only the server, and leaves the process running; a later SIGINT then makes
`Environment::stop()` panic (exit 101).

```text
$ cargo build -p torrust-tracker-axum-http-server --example http_only_public_tracker
$ target/debug/examples/http_only_public_tracker > m1.log 2>&1 &
# wait for "Listening on", then:
$ kill -TERM 3201652
# process still alive after SIGTERM; HTTP server gone:
$ curl -s -o /dev/null -w 'http_status=%{http_code}\n' --max-time 2 http://127.0.0.1:42739/health_check
http_status=000            (curl exit 7: connection refused)
$ ss -ltnp | grep 42739
port 42739 not listening
$ kill -INT 3201652
exit_status=101

m1.log:
Listening on 127.0.0.1:42739
Press Ctrl-C to stop.
Shutting down...
thread 'main' (3201652) panicked at packages/axum-http-server/src/testing/environment.rs:142:47:
Failed to stop the HTTP tracker server: Stop { message: "task killer channel was closed" }
```

Cause: the environment starts the server through the legacy `Halted` path,
whose `graceful_shutdown` (`packages/axum-server/src/signals.rs`) waits on
`torrust-server-lib` 0.3.0 `shutdown_signal_with_message`, which calls
`shutdown_signal`, which selects on `global_shutdown_signal()` (Ctrl-C or Unix
SIGTERM). SIGTERM therefore
reaches a library subscription, not the example's `main`. That task then ends,
so the later `HttpServer::stop()` fails to send `Halted` on the closed channel.

A first attempt ran the whole `cargo build && example` chain as one background
job, so its SIGTERM hit the wrapper shell and orphaned the example; that run was
discarded and the orphan stopped with SIGINT before the run above.

### V2: Announce, Then SIGTERM

- [x] Run `http_only_public_tracker` and send SIGTERM to the example binary.
- [x] Record the signal-boundary output, orderly stop, and final process result.

Run on 2026-10-05 against the commit `feat(axum-http-server): [#2412] migrate the
test environment to the token lifecycle` plus the then-uncommitted T4 example
change, same toolchain as above. The announce used the maintained client.

```text
$ target/debug/examples/http_only_public_tracker > m2.log 2>&1 &
# wait for "Listening on", then:
$ target/debug/tracker_client http announce http://127.0.0.1:53503/announce 9c38422213e30bff212b30c360d26f9a02136422
{"complete":1,"incomplete":0,"interval":120,"min interval":120,"peers":[]}
$ kill -TERM 2632630
exit_status=0

m2.log (tail):
Listening on 127.0.0.1:53503
Press Ctrl-C (or send SIGTERM on Unix) to stop.
Received SIGTERM. Shutting down...
Stopped.
```

### V3: SIGINT

- [x] Repeat with Ctrl+C and record the same lifecycle path.

```text
$ target/debug/examples/http_only_public_tracker > m3.log 2>&1 &
$ target/debug/tracker_client http announce http://127.0.0.1:49771/announce 9c38422213e30bff212b30c360d26f9a02136422
{"complete":1,"incomplete":0,"interval":120,"min interval":120,"peers":[]}
$ kill -INT 2639322
exit_status=0

m3.log (tail):
Listening on 127.0.0.1:49771
Press Ctrl-C (or send SIGTERM on Unix) to stop.
Received SIGINT. Shutting down...
Stopped.
```

No example process remained after either run.

### V4: Library Boundary and Compatibility

- [x] Confirm no HTTP library module gains an OS-signal subscription.
- [x] Confirm legacy HTTP start/stop callers still compile and behave as before.

**Evidence:**

```text
$ rg -n 'tokio::signal' packages/axum-http-server/src
(no matches; exit 1)
```

Scope of that claim: no module in `packages/axum-http-server/src` subscribes
to OS signals directly. The legacy `HttpServer::start`/`stop` path still does
so indirectly, through `axum-server` `graceful_shutdown` and
`torrust-server-lib` `global_shutdown_signal` (see V1). This issue keeps that
path for compatibility; the environment and the example no longer use it, and
SI-19 removes it. The legacy unit test
`it_should_preserve_the_launcher_bind_address_after_starting_and_stopping`
still passes.

## Summary

| Check                            | Result  | Evidence link or note |
| -------------------------------- | ------- | --------------------- |
| Environment token cancellation   | Automated test | Deterministic Environment Tests |
| Owned tasks joined               | Automated tests | Deterministic Environment Tests |
| Listener cancellation, not abort | Automated tests (mutation-proven); no `abort()` in source | Deterministic Environment Tests |
| Example SIGTERM                  | Baseline (V1): library catches it, process stays up. After T4 (V2): orderly stop, exit 0 | V1, V2 |
| Example SIGINT                   | Orderly stop, exit 0 | V3 |
| Legacy compatibility             | Legacy unit test passes; legacy path still subscribes indirectly until SI-19 | V4 |
