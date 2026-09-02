# Verification Evidence — Standalone UDP Environment and Example

> **Status**: Not started — collect deterministic environment and executable
> signal-boundary evidence for this UDP-only consumer migration.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Deterministic Environment Tests

### Test 1: `stop()` cancels and joins owned work

- [ ] Start the UDP environment with controllable listener/server tasks.
- [ ] Call `Environment::stop()` without delivering an OS signal.
- [ ] Verify it cancels its token and awaits all three listeners plus UDP server
      work.
- [ ] Verify it returns only after owned work completes or returns a defined
      failure result.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Listeners are not aborted

- [ ] Verify listener `abort()` calls are absent from the UDP environment.
- [ ] Use controllable listeners to prove cancellation, rather than abort,
      causes normal completion.

**Evidence:**

```text
(paste focused test output or source-review evidence)
```

## Example Executable Evidence

- [ ] Run `udp_only_public_tracker` and send SIGTERM to the example binary.
- [ ] Record the signal-boundary output, orderly stop, and final process result.
- [ ] Repeat with Ctrl+C and record the same lifecycle path.
- [ ] Confirm no UDP library module gains an OS-signal subscription.
- [ ] Confirm legacy UDP start/stop callers still compile and behave as before.

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
