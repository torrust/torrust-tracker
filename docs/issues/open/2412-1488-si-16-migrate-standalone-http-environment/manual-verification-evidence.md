---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md
last-updated-utc: "2026-10-04 10:34"
---

# Verification Evidence — Standalone HTTP Environment and Example

<!-- cspell:ignore ltnp -->

> **Status**: In progress — T0 baseline recorded; implementation evidence
> pending.

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

### Test 1: `stop()` cancels and joins owned work

- [ ] Start the HTTP environment with controllable listener/server tasks.
- [ ] Call `Environment::stop()` without delivering an OS signal.
- [ ] Verify it cancels its token and awaits listener, server, and drain work.
- [ ] Verify it returns only after owned work completes or returns a defined
      failure result.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Listener is not aborted

- [ ] Verify `event_listener_job.abort()` is absent from the HTTP environment.
- [ ] Use a controllable listener to prove cancellation, rather than abort,
      causes its normal completion.

**Evidence:**

```text
(paste focused test output or source-review evidence)
```

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

- [ ] Run `http_only_public_tracker` and send SIGTERM to the example binary.
- [ ] Record the signal-boundary output, orderly stop, and final process result.

### V3: SIGINT

- [ ] Repeat with Ctrl+C and record the same lifecycle path.

### V4: Library Boundary and Compatibility

- [ ] Confirm no HTTP library module gains an OS-signal subscription.
- [ ] Confirm legacy HTTP start/stop callers still compile and behave as before.

**Evidence:**

```text
(paste commands and output)
```

## Summary

| Check                            | Result  | Evidence link or note |
| -------------------------------- | ------- | --------------------- |
| Environment token cancellation   | Pending |                       |
| Owned tasks joined               | Pending |                       |
| Listener cancellation, not abort | Pending |                       |
| Example SIGTERM                  | Baseline recorded (V1): library catches it, process stays up | V1 |
| Example SIGINT                   | Pending |                       |
| Legacy compatibility             | Pending |                       |
