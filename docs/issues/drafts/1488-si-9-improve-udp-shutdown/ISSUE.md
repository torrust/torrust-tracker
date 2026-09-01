---
doc-type: issue
issue-type: task
status: superseded
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1488-si-9-improve-udp-shutdown/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/analysis/20260716-shutdown-process/README.md
---

<!-- skill-link: create-issue -->

# Superseded Draft SI-9 — Split UDP Lifecycle Migration

> **Status**: Superseded for implementation planning. [SI-14](../1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md)
> replaces the UDP receive/reset-loop migration, followed by [SI-15](../1488-si-15-define-udp-active-request-policy/ISSUE.md)
> for the separate active-request policy change.

## Why This Draft Is Superseded

This draft mixes two ownership boundaries. The first replacement must introduce
a token-aware UDP stop path and make the receive loop plus IP-ban reset loop
owned, cancellable, and joined. It retains the current safe, deliberate active
request abort behavior as a compatibility fallback. A later replacement can
then change only the active-request deadline, drain, abort metrics, and policy.

Do not implement this combined draft.

## Original Goal

Replace the UDP server's shutdown `Halted` oneshot and OS-signal wait with a
component `CancellationToken`. The UDP server must own, stop, and join its
receive loop and IP-ban reset loop, and apply a documented, observable policy
to active request processors before reporting its component outcome upward.

## Background

The UDP server (`packages/udp-server/src/server/launcher.rs`) shuts down by
aborting its main loop:

```rust
select! {
    _ = running => { ... },
    _ = halt_task => { ... }
}
stop.abort();  // Force-abort the main loop task
tokio::task::yield_now().await;  // Give other tasks a chance to run
```

There is no connection draining mechanism — in-flight UDP requests are dropped
silently.

See [analysis §5.2 and §7.8](../../../analysis/20260716-shutdown-process/README.md).

## Important Context: UDP is Stateless

UDP is fire-and-forget at the protocol level. BitTorrent UDP clients:

- Expect responses on a best-effort basis.
- Retry automatically if no response arrives.
- Do not hold long-lived connections.

This means a dropped UDP request during shutdown is **much less severe** than
a dropped HTTP connection. The client will retry on the next tracker (if multiple
trackers are configured) or on the next announce interval.

The actual improvement is therefore primarily about **observability**, not about
preventing data loss.

## Required Lifecycle Policy

1. Cancellation stops admission of new UDP packets.
2. The UDP server cancels and joins its IP-ban reset loop; it cannot remain a
   detached, indefinite task.
3. Active request processors may finish only until the component deadline. The
   server deliberately aborts any remaining processors, records their count,
   and then completes.
4. The UDP component joins its receive loop and reports one named outcome to
   its parent. `JobManager` does not receive every per-request handle.

## Implementation Notes

During shutdown, log completed and deliberately aborted active request counts:

```rust
tracing::info!(
    "UDP server shutting down with {} requests in flight",
    active_requests.len()
);
```

## Acceptance Criteria

- [ ] UDP server shutdown is driven by an injected `CancellationToken`, not a
      shutdown `Halted` oneshot or OS-signal listener.
- [ ] The receive loop and IP-ban reset loop are retained, cancelled, and joined
      by their UDP component owner.
- [ ] Active request processors complete before the component deadline or are
      deliberately aborted; the outcome counts are logged.
- [ ] The UDP component reports a single outcome to its parent after all owned
      child tasks have completed or been aborted.
- [ ] `linter all` passes.

## Dependencies

- SI-2 must define the token-based server lifecycle contract first.
- Q4 must define component deadlines before the active-request policy is final.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Test 1: In-flight request count is logged on shutdown

Start the tracker with UDP enabled. Use the tracker client to send a burst of
UDP requests, then immediately send Ctrl+C.

```bash
# Send several UDP announce requests rapidly
for i in $(seq 1 10); do
  cargo run -p torrust-tracker-client -- udp announce \
    udp://127.0.0.1:6969 aabbccddeeff00112233445566778899aabbccdd &
done

# Immediately shut down
kill -INT <tracker-pid>
```

**Expected**: a log line like:

```text
INFO  UDP server shutting down with N requests in flight
```

(even if N is 0 in practice, the log line must be present)

**Record in `verification.md`**: the log line.

### Test 2: Abort is documented as intentional

Search the code for the `stop.abort()` call and confirm there is a comment
explaining that UDP abort is intentional due to protocol retry semantics:

```bash
grep -n 'abort' packages/udp-server/src/server/launcher.rs
```

**Expected**: the `abort()` call has an adjacent comment explaining the decision.

### Test 3: UDP clients retry after tracker restart

Shut down the tracker during active UDP traffic and confirm that clients
(BitTorrent peers) reconnect and resume normal operation after the tracker
restarts. Use the checker/monitor tool if available, or observe from logs
after restart.

**Note**: This is a best-effort test. UDP retry behavior is client-dependent.
