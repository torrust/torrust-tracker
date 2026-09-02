# Verification Evidence — Health Check Unhealthy During Shutdown

> **Status**: Not started — capture deterministic readiness ordering and manual
> shutdown evidence after the health-check token lifecycle migration.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:
- Deployment/readiness-probe configuration:

## Deterministic Readiness Tests

### Test 1: Healthy response before shutdown

- [ ] Start the application in its ready state.
- [ ] Request `/health_check`.
- [ ] Verify current healthy response status/body and downstream probe fan-out.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Not-ready response does not probe services

- [ ] Set application readiness to not ready without delivering an OS signal.
- [ ] Request `/health_check`.
- [ ] Verify HTTP 503.
- [ ] Verify registered-service probe tasks were not spawned or executed.

**Evidence:**

```text
(paste focused test output)
```

### Test 3: Ordering before cancellation

- [ ] Initiate normal application shutdown with controllable readiness and root
      cancellation test doubles.
- [ ] Verify readiness changes to not ready before root cancellation.
- [ ] Verify readiness cannot return to ready in the same process lifecycle.

**Evidence:**

```text
(paste focused test output)
```

## Manual Verification

- [ ] After SI-1, run the tracker with health-check API enabled and send SIGTERM
      to the tracker binary.
- [ ] Poll `/health_check` and record a 503 response before component drain
      completion and process exit.
- [ ] Verify the same endpoint remains healthy before shutdown.
- [ ] Record readiness-probe configuration showing infrastructure consumes the
      503 result when routing traffic.

**Evidence:**

```text
(paste raw commands, responses, and logs)
```

## Summary

| Check                            | Result  | Evidence link or note |
| -------------------------------- | ------- | --------------------- |
| Healthy response unchanged       | Pending |                       |
| Not-ready response is 503        | Pending |                       |
| No probe fan-out while not ready | Pending |                       |
| Readiness precedes cancellation  | Pending |                       |
| One-way readiness                | Pending |                       |
| Manual drain ordering            | Pending |                       |
