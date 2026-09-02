# Verification Evidence — UDP Receive Token Lifecycle Migration

> **Status**: Not started — capture deterministic and manual evidence for this
> UDP receive-loop ownership slice.

## Environment

- Date:
- OS:
- Rust version (`rustc --version`):
- Tracker commit/branch:

## Deterministic Tests

### Test 1: Cancellation joins the receive task

- [ ] Start one UDP component with an injected `CancellationToken`.
- [ ] Cancel the token without delivering `SIGINT` or `SIGTERM`.
- [ ] Verify new packet admission stops.
- [ ] Verify the receive loop is awaited.
- [ ] Verify the separate application-level UDP IP-ban cleanup job remains
      manager-owned and token-cancellable.
- [ ] Verify the component reports its named UDP outcome.

**Evidence:**

```text
(paste focused test output)
```

### Test 2: Unexpected receive-loop completion is reported

- [ ] Cause or simulate receive-loop completion/failure without cancellation.
- [ ] Verify an explicit UDP component outcome is reported.

**Evidence:**

```text
(paste focused test output)
```

### Test 3: Active-request compatibility

- [ ] Verify existing `ActiveRequests` capacity and deliberate abort behavior
      remain unchanged.
- [ ] Do not add request deadlines, drain behavior, or outcome metrics here.

**Evidence:**

```text
(paste focused test output or source-review evidence)
```

### Test 4: Bootstrap wiring

- [ ] Start bootstrap with one UDP binding.
- [ ] Request root-token cancellation without an OS signal.
- [ ] Verify the `udp_instance_<index>_<address>` component completes.

**Evidence:**

```text
(paste focused test output)
```

## Compatibility and Manual Evidence

- [ ] Existing legacy UDP start/stop tests pass without call-site changes.
- [ ] After SI-1, SIGTERM sent to the tracker binary reaches `main()` and the
      migrated UDP component completes through the token path.
- [ ] The token-aware UDP path has no OS-signal subscription.
- [ ] The receive-loop handle is retained and awaited by the UDP component
      owner; UDP IP-ban cleanup remains separately manager-owned.

**Evidence:**

```text
(paste test output, source-review notes, and relevant shutdown logs)
```

## Summary

| Check                        | Result  | Evidence link or note |
| ---------------------------- | ------- | --------------------- |
| Token-driven UDP stop        | Pending |                       |
| Joined receive loop          | Pending |                       |
| Managed UDP IP-ban cleanup   | Pending |                       |
| Unexpected receive outcome   | Pending |                       |
| Active-request compatibility | Pending |                       |
| Bootstrap propagation        | Pending |                       |
| Legacy API compatibility     | Pending |                       |
| Manual SIGTERM path          | Pending |                       |
