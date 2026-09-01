# Verification Evidence — HTTP Tracker Token Lifecycle Migration

> **Status**: Not started — capture deterministic and manual evidence for this
> HTTP-only vertical slice.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Deterministic Tests

### Test 1: Injected cancellation drains HTTP component

- [ ] Start one HTTP tracker component with an injected `CancellationToken`.
- [ ] Cancel the token without delivering `SIGINT` or `SIGTERM`.
- [ ] Verify the component awaits its server and drain-controller children.
- [ ] Verify the component reports a named completion outcome.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Unexpected server completion is reported

- [ ] Cause or simulate server-task completion/failure without cancellation.
- [ ] Verify the component reports an explicit outcome and does not leave the
      drain-controller task detached.

**Evidence:**

```text
(paste focused test output)
```

### Test 3: Bootstrap wiring

- [ ] Start the tracker bootstrap with an HTTP binding.
- [ ] Request root-token cancellation without an OS signal.
- [ ] Verify the `http_instance_<index>_<address>` managed component completes.

**Evidence:**

```text
(paste focused test output)
```

## Compatibility and Manual Evidence

- [ ] Existing legacy HTTP start/stop tests pass without changing their call
      sites.
- [ ] After SI-1, SIGTERM sent to the tracker binary reaches `main()` and the
      migrated HTTP component records token-driven drain completion.
- [ ] The token-aware HTTP path has no OS-signal subscription.
- [ ] Every drain-controller handle created by the new path is retained and
      awaited by its HTTP component owner.

**Evidence:**

```text
(paste test output, source-review notes, and relevant shutdown logs)
```

## Summary

| Check                     | Result  | Evidence link or note |
| ------------------------- | ------- | --------------------- |
| Token-driven HTTP drain   | Pending |                       |
| Joined child tasks        | Pending |                       |
| Unexpected server outcome | Pending |                       |
| Bootstrap propagation     | Pending |                       |
| Legacy API compatibility  | Pending |                       |
| Manual SIGTERM path       | Pending |                       |
