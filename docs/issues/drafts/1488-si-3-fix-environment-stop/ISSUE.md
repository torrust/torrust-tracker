---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-3-fix-environment-stop/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-07-16
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-http-server/src/testing/environment.rs
    - packages/udp-server/src/testing/environment.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - packages/udp-server/examples/udp_only_public_tracker.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/features/shutdown-process/open-questions.md
---

<!-- skill-link: create-issue -->

# Draft SI-3 — Fix `Environment::stop()` in Standalone Library Examples

> **EPIC position**: SI-3 of #1488. Depends on Q1 resolution.
> **Blocked**: Q1 must be resolved — the fix strategy for `Environment::stop()`
> depends on the decision about `global_shutdown_signal()` in SI-2.

## Goal

Fix two problems in the standalone library usage examples:

1. **Event listeners are `abort()`ed instead of gracefully stopped** — the
   `CancellationToken` in `Environment` exists but is never `cancel()`led during
   shutdown. In-flight statistics events are silently lost.
2. **Only `SIGINT` is handled** — both examples only call `ctrl_c()`, not
   `SIGTERM`. `docker stop` and `kill` are ignored.

## Background

Both example binaries follow this pattern:

```rust
tokio::signal::ctrl_c().await.expect("failed to install Ctrl-C handler");
env.stop().await;
```

`Environment::stop()` currently aborts event listeners:

```rust
// todo: send a message to the event listener to stop and wait for it to finish
event_listener_job.abort();
```

The `CancellationToken` stored in `Environment` is created but `cancel()` is
never called on it. This is an explicit known issue (the `TODO` comments call it out).

The affected files are:

- `packages/axum-http-server/src/testing/environment.rs`
- `packages/udp-server/src/testing/environment.rs`
- `packages/axum-http-server/examples/http_only_public_tracker.rs`
- `packages/udp-server/examples/udp_only_public_tracker.rs`

## Implementation

### Fix 1: Use `CancellationToken` in `Environment::stop()`

Change event listener shutdown from `abort()` to graceful cancel + await:

```rust
pub async fn stop(self) -> Environment<Stopped> {
    // Cancel all event listeners via the shared token
    self.cancellation_token.cancel();

    // Wait for each listener to finish (instead of abort)
    if let Some(job) = self.event_listener_job {
        let _ = job.await;
    }

    let server = self.server.stop().await.expect("...");
    ...
}
```

### Fix 2: Handle `SIGTERM` in the example binaries

```rust
#[cfg(unix)]
let mut sigterm = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate()
).expect("failed to install SIGTERM handler");

tokio::select! {
    _ = tokio::signal::ctrl_c() => {}
    #[cfg(unix)]
    _ = sigterm.recv() => {}
}

env.stop().await;
```

## Acceptance Criteria

- [ ] `event_listener_job.abort()` is replaced with `cancel()` + `await` in both
      `axum-http-server` and `udp-server` environment `stop()` methods.
- [ ] The `TODO` comments about graceful event listener shutdown are resolved.
- [ ] Both example binaries handle `SIGTERM` in addition to `SIGINT`.
- [ ] `kill <pid>` against a running example binary shuts it down cleanly.
- [ ] `linter all` passes.

## Dependencies

- Q1 must be resolved (the `global_shutdown_signal()` decision affects whether
  servers in these examples also need explicit halt-sender changes).
- Can land independently of SI-1 and SI-2 if Q1 is resolved first.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Setup

Build and run each example binary:

```bash
# HTTP example
cargo run -p torrust-tracker-axum-http-server --example http_only_public_tracker

# UDP example
cargo run -p torrust-tracker-udp-server --example udp_only_public_tracker
```

Note the PID of the **example binary** (not cargo).

### Test 1: `kill <pid>` shuts down HTTP example gracefully (SIGTERM)

Run the HTTP example and send `kill <pid>`.

**Expected**:

- The example logs a shutdown message.
- The process exits cleanly (exit code 0).
- No "Killed" message from the OS.

**Record in `verification.md`**: full output including shutdown messages.

### Test 2: `kill <pid>` shuts down UDP example gracefully (SIGTERM)

Repeat Test 1 for the UDP example.

### Test 3: Event listeners are cancelled, not aborted

Verify that the statistics event listener shutdown message is logged (not just
silently killed). If event listeners log a shutdown message, they were cancelled
gracefully. If there is no log message, they were aborted.

**Expected**: a log line like `Stopping ... event listener` or `... receiver closed`
appears during `env.stop()`.

### Test 4: `TODO` comments are removed

Search the codebase for the `TODO` comments about graceful event listener shutdown
and confirm they are removed:

```bash
grep -rn 'todo: send a message to the event listener' packages/
```

Output should be empty.
