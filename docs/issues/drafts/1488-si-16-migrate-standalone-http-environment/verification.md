# Verification Evidence — Standalone HTTP Environment and Example

> **Status**: Not started — collect deterministic environment and executable
> signal-boundary evidence for this HTTP-only consumer migration.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

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

- [ ] Run `http_only_public_tracker` and send SIGTERM to the example binary.
- [ ] Record the signal-boundary output, orderly stop, and final process result.
- [ ] Repeat with Ctrl+C and record the same lifecycle path.
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
| Example SIGTERM                  | Pending |                       |
| Example SIGINT                   | Pending |                       |
| Legacy compatibility             | Pending |                       |
