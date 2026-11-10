# Verification Evidence — Legacy Shutdown API Removal

> **Status**: Not started - do not populate implementation evidence until every
> SI-19 start-gate condition and the declared external compatibility window are met.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:
- `torrust-server-lib` breaking release/version: 0.6.0

## Start-Gate Evidence

- [ ] Link completed SI-18 deprecation evidence (#2751, PRs 2752, 2793, 2797).
- [ ] Link the completed token-lifecycle migrations (SI-11 through SI-17, SI-23 #2449, SI-24 #2450).
- [ ] Link #1588 final task inventory.
- [ ] Link the maintainer approval ending the 0.5.x window (no fixed end date) and the `torrust-server-lib` 0.6.0 release notes (torrust/torrust-server-lib#5, PR 6).
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
