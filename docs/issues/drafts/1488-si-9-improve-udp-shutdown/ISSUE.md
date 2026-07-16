---
doc-type: issue
issue-type: task
status: draft
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1488-si-9-improve-udp-shutdown/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-07-16
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

# Draft SI-9 — Improve UDP Server Shutdown

> **EPIC position**: SI-9 of #1488. Lowest priority. Independent.

## Goal

Improve the UDP server shutdown to be more observable and, where practical, to
avoid dropping in-flight UDP requests during shutdown.

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

See [analysis §5.2 and §7.8](../../analysis/20260716-shutdown-process/README.md).

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

## Implementation Options

### Option A: Log in-flight requests on shutdown (minimal)

Before aborting the main loop, log the number of requests in the buffer:

```rust
tracing::info!(
    "UDP server shutting down with {} requests in flight",
    active_requests.len()
);
stop.abort();
```

### Option B: Drain the request buffer before stopping (more complete)

Process all requests already in the socket buffer before stopping. This requires:

1. Stop accepting new UDP packets (close the socket for reads).
2. Process any already-buffered packets.
3. Then stop.

This is more complex and may not be worth the effort given UDP retry behavior.

## Recommendation

Implement **Option A** first as it is low risk and adds observability. Revisit
Option B if operator feedback indicates that in-flight UDP request drops are
causing measurable peer-tracking inconsistencies.

## Acceptance Criteria

- [ ] On shutdown, the UDP server logs the number of in-flight requests (if any).
- [ ] The abort approach is documented as intentional with a note about UDP
      retry semantics.
- [ ] `linter all` passes.

## Dependencies

- No hard prerequisites. Can land independently.
- Lower priority than SI-1 through SI-6.

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
