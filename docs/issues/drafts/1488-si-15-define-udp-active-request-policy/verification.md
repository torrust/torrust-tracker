# Verification Evidence — UDP Active-Request Shutdown Policy

> **Status**: Not started — capture controlled request-lifecycle evidence for
> this policy slice after UDP receive-loop ownership migration.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Deterministic Tests

### Test 1: Processors completing before deadline

- [ ] Use controlled processor tasks that complete before the request deadline.
- [ ] Verify the UDP component awaits them.
- [ ] Verify the shutdown summary records the completed count.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Processors deliberately aborted at deadline

- [ ] Use controlled processor tasks that remain blocked beyond the deadline.
- [ ] Verify the UDP component deliberately aborts and awaits them.
- [ ] Verify the shutdown summary records aborted separately from failed.

**Evidence:**

```text
(paste focused test output)
```

### Test 3: Normal-operation capacity compatibility

- [ ] Exercise the bounded `ActiveRequests` capacity behavior outside shutdown.
- [ ] Verify overload-induced aborts remain distinct from shutdown-induced
      aborts in observed outcomes.

**Evidence:**

```text
(paste focused test output)
```

## Manual UDP Traffic Evidence

- [ ] After SI-1, run the tracker with UDP enabled and send a bounded request
      burst before SIGTERM.
- [ ] Record the `main()` signal event and UDP completed/failed/aborted summary.
- [ ] Confirm the UDP component does not complete before active request work is
      joined or deliberately aborted.

**Evidence:**

```text
(paste shutdown log and command output)
```

## Summary

| Check                           | Result  | Evidence link or note |
| ------------------------------- | ------- | --------------------- |
| Completed processors counted    | Pending |                       |
| Deadline aborts counted         | Pending |                       |
| Failed processors distinguished | Pending |                       |
| Capacity compatibility          | Pending |                       |
| Manual UDP outcome summary      | Pending |                       |
