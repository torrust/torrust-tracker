# Verification Evidence — Token-Aware, Joinable Axum Drain Helper

> **Status**: Not started — collect deterministic test output and compatibility
> evidence when implementing this additive helper.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Deterministic Helper Tests

### Test 1: Cancellation starts graceful drain

- [ ] Inject a `CancellationToken` into the new helper.
- [ ] Cancel the token without sending `SIGINT` or `SIGTERM`.
- [ ] Verify the helper requests graceful shutdown and returns a drained outcome.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Drain deadline returns a timeout outcome

- [ ] Hold the helper's connection-count condition above zero using a controlled
      test double or fixture.
- [ ] Verify the helper returns its deadline-reached outcome.
- [ ] Verify the test does not deliver an OS signal.

**Evidence:**

```text
(paste focused test output)
```

## Compatibility

- [ ] Existing `graceful_shutdown` call sites remain unchanged.
- [ ] Existing HTTP tracker, REST API, and health-check server tests pass.
- [ ] The new helper has no OS-signal subscription or shutdown `Halted` channel
      parameter.
- [ ] The new helper creates no detached task.

**Evidence:**

```text
(paste test and source-review evidence)
```

## Summary

| Check                              | Result  | Evidence link or note |
| ---------------------------------- | ------- | --------------------- |
| Token cancellation initiates drain | Pending |                       |
| Drained outcome                    | Pending |                       |
| Deadline-reached outcome           | Pending |                       |
| Legacy helper compatibility        | Pending |                       |
| No OS-signal or detached task      | Pending |                       |
