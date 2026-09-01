# Verification Evidence — Legacy Shutdown API Removal

> **Status**: Not started — do not populate implementation evidence until every
> SI-19 start-gate condition and the declared external compatibility window are met.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:
- `torrust-server-lib` breaking release/version:

## Start-Gate Evidence

- [ ] Link completed SI-18 deprecation evidence and support-window end date.
- [ ] Link every completed token-lifecycle migration (SI-11 through SI-17).
- [ ] Link #1588 final task inventory.
- [ ] Link external breaking-release notes and maintainer timing approval.
- [ ] Link Q4/Q5 decisions and deployment/process-wrapper policy.

## Source and Deterministic Tests

### Test 1: Legacy symbols and library signal handling are gone

- [ ] Search the workspace for removed `Halted` shutdown symbols and legacy
      signal helpers.
- [ ] Verify remaining matches are only archived migration history.
- [ ] Verify no server-library module subscribes to SIGINT or SIGTERM.

**Evidence:**

```text
(paste searches and review output)
```

### Test 2: Token lifecycle owns completion

- [ ] Run deterministic component tests using injected cancellation tokens.
- [ ] Verify every component joins or deliberately aborts its owned child tasks.
- [ ] Verify no test needs an OS signal to prove component cancellation.

**Evidence:**

```text
(paste focused test output)
```

## End-to-End Evidence

- [ ] Tracker binary: SIGINT and SIGTERM produce one application-owned shutdown
      sequence with no duplicate library signal handling.
- [ ] Standalone HTTP example: SIGINT and SIGTERM use the in-process stop path.
- [ ] Standalone UDP example: SIGINT and SIGTERM use the in-process stop path.
- [ ] Container/service-manager evidence uses the deadlines specified by Q4.
- [ ] Process results match the Q3 exit-code policy.

**Evidence:**

```text
(paste raw logs and command output)
```

## Summary

| Check                           | Result  | Evidence link or note |
| ------------------------------- | ------- | --------------------- |
| Start gate                      | Pending |                       |
| Legacy shutdown symbols removed | Pending |                       |
| No library OS signals           | Pending |                       |
| Deterministic lifecycle tests   | Pending |                       |
| Tracker signal handling         | Pending |                       |
| Standalone signals              | Pending |                       |
| Deployment verification         | Pending |                       |
