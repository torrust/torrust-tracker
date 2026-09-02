# Verification Evidence — Shutdown Policy and Deployment Contract

> **Status**: Not started — record configuration, exit-result, and deployment
> evidence after all lifecycle consumers can accept the configured budgets.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:
- Container/service-manager environment:

## Configuration and Exit-Result Tests

### Test 1: Approved defaults

- [ ] Load configuration without a `[shutdown]` section.
- [ ] Verify defaults are 25s process, 20s HTTP drain, and 5s UDP request.
- [ ] Verify the process deadline is a single concurrent deadline.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Overrides and validation

- [ ] Verify valid configured overrides reach their component consumers.
- [ ] Verify zero values are rejected with actionable startup errors.
- [ ] Verify HTTP/UDP component budgets greater than or equal to process
      deadline are rejected.

**Evidence:**

```text
(paste focused test output)
```

### Test 3: Aggregate outcome to exit result

- [ ] Verify all completed top-level outcomes map to exit code 0.
- [ ] Verify failed, panicked, timed-out, and deliberately aborted outcomes map
      to exit code 1.
- [ ] Verify component tasks do not invoke `std::process::exit`.

**Evidence:**

```text
(paste focused test output)
```

## Deployment Verification

- [ ] Configure a Docker/Podman grace period of at least 30 seconds.
- [ ] Record SIGTERM, named component outcomes, and the resulting process exit.
- [ ] Verify documentation identifies the default 10-second Docker/Podman
      deadline as insufficient.
- [ ] Record Kubernetes/systemd configuration review showing a grace period of
      at least 30 seconds and preferably 35 seconds or more where possible.

**Evidence:**

```text
(paste commands, configuration, and raw logs)
```

## Summary

| Check                     | Result  | Evidence link or note |
| ------------------------- | ------- | --------------------- |
| Default policy            | Pending |                       |
| Override validation       | Pending |                       |
| Outcome-to-exit mapping   | Pending |                       |
| No component process exit | Pending |                       |
| Container verification    | Pending |                       |
| Deployment documentation  | Pending |                       |
