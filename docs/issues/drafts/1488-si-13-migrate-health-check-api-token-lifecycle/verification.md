# Verification Evidence — Health-Check API Token Lifecycle Migration

> **Status**: Not started — capture deterministic and manual evidence for this
> health-check API-only vertical slice.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Deterministic Tests

### Test 1: Injected cancellation drains health-check API

- [ ] Start the health-check API with an injected `CancellationToken`.
- [ ] Cancel the token without delivering `SIGINT` or `SIGTERM`.
- [ ] Verify the component awaits its server and drain-controller children.
- [ ] Verify the component reports the named `health_check_api` outcome.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Unexpected server completion is reported

- [ ] Cause or simulate health-check server-task completion/failure without
      cancellation.
- [ ] Verify an explicit outcome is reported without detaching the drain
      controller.

**Evidence:**

```text
(paste focused test output)
```

### Test 3: Bootstrap wiring

- [ ] Start bootstrap with the health-check API enabled.
- [ ] Request root-token cancellation without an OS signal.
- [ ] Verify the `health_check_api` managed component completes.

**Evidence:**

```text
(paste focused test output)
```

## Compatibility and Manual Evidence

- [ ] Existing legacy health-check start/stop tests pass without call-site
      changes.
- [ ] Existing health-check responses and readiness semantics are unchanged.
- [ ] After SI-1, SIGTERM sent to the tracker binary reaches `main()` and the
      migrated health-check API records token-driven drain completion.
- [ ] The token-aware path has no OS-signal subscription.
- [ ] Every new drain-controller handle is retained and awaited by its
      health-check API component owner.
- [ ] Do not claim unhealthy-on-shutdown behavior in this migration; SI-21 owns
      Q6's approved readiness-before-drain behavior.

**Evidence:**

```text
(paste test output, source-review notes, and relevant shutdown logs)
```

## Summary

| Check                           | Result  | Evidence link or note |
| ------------------------------- | ------- | --------------------- |
| Token-driven health-check drain | Pending |                       |
| Joined child tasks              | Pending |                       |
| Unexpected server outcome       | Pending |                       |
| Bootstrap propagation           | Pending |                       |
| Legacy API compatibility        | Pending |                       |
| Unchanged readiness behavior    | Pending |                       |
| Manual SIGTERM path             | Pending |                       |
