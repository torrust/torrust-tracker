# Verification Evidence — REST API Token Lifecycle Migration

> **Status**: Not started — capture deterministic and manual evidence for this
> REST API-only vertical slice.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Deterministic Tests

### Test 1: Injected cancellation drains REST API

- [ ] Start the REST API component with an injected `CancellationToken`.
- [ ] Cancel the token without delivering `SIGINT` or `SIGTERM`.
- [ ] Verify the component awaits its server and drain-controller children.
- [ ] Verify the component reports the named `http_api` outcome.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Unexpected server completion is reported

- [ ] Cause or simulate REST API server-task completion/failure without
      cancellation.
- [ ] Verify an explicit outcome is reported without detaching the drain
      controller.

**Evidence:**

```text
(paste focused test output)
```

### Test 3: Bootstrap wiring

- [ ] Start bootstrap with the REST API enabled.
- [ ] Request root-token cancellation without an OS signal.
- [ ] Verify the `http_api` managed component completes.

**Evidence:**

```text
(paste focused test output)
```

## Compatibility and Manual Evidence

- [ ] Existing legacy REST API start/stop tests pass without call-site changes.
- [ ] After SI-1, SIGTERM sent to the tracker binary reaches `main()` and the
      migrated REST API records token-driven drain completion.
- [ ] The token-aware REST API path has no OS-signal subscription.
- [ ] Every drain-controller handle created by the new path is retained and
      awaited by its REST API component owner.

**Evidence:**

```text
(paste test output, source-review notes, and relevant shutdown logs)
```

## Summary

| Check                       | Result  | Evidence link or note |
| --------------------------- | ------- | --------------------- |
| Token-driven REST API drain | Pending |                       |
| Joined child tasks          | Pending |                       |
| Unexpected server outcome   | Pending |                       |
| Bootstrap propagation       | Pending |                       |
| Legacy API compatibility    | Pending |                       |
| Manual SIGTERM path         | Pending |                       |
