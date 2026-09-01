---
doc-type: issue
issue-type: task
status: superseded
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-3-fix-environment-stop/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-http-server/src/testing/environment.rs
    - packages/udp-server/src/testing/environment.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - packages/udp-server/examples/udp_only_public_tracker.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/features/shutdown-process/questions.md
---

<!-- skill-link: create-issue -->

# Superseded Draft SI-3 — Split Standalone Environment Migration

> **Status**: Superseded for implementation planning. [SI-16](../1488-si-16-migrate-standalone-http-environment/ISSUE.md)
> replaces the standalone HTTP environment/example and [SI-17](../1488-si-17-migrate-standalone-udp-environment/ISSUE.md)
> replaces the standalone UDP environment/example, after the additive server
> lifecycle API is available.

## Why This Draft Is Superseded

The goal remains valid, but this draft changes two independently releasable
package consumers in one task. The EPIC roadmap requires one standalone
consumer per migration: HTTP first, then UDP. Each replacement issue must make
its environment's `stop()` cancellation-driven, join every task it owns, and
update only that environment's executable example.

Do not implement this combined draft.

## Original Goal

Make standalone environments implement the same cancellation and ownership
contract as the tracker application:

1. **Event listeners are `abort()`ed instead of gracefully stopped** — the
   `CancellationToken` in `Environment` exists but is never `cancel()`led during
   shutdown. In-flight statistics events are silently lost.
2. **Server completion is not fully owned** — `stop()` must await every task
   owned by the environment, including server graceful-stop work.
3. **Only `SIGINT` is handled** — example binaries must translate both SIGINT
   and Unix SIGTERM at their executable boundary, then invoke `stop()`.

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

### Fix 1: Use `CancellationToken` and join all owned tasks in `Environment::stop()`

Change event listener shutdown from `abort()` to graceful cancel + await:

```rust
pub async fn stop(self) -> Environment<Stopped> {
    // Cancel all event listeners via the shared token
    self.cancellation_token.cancel();

    // Wait for each listener to finish (instead of abort)
    if let Some(job) = self.event_listener_job {
        let _ = job.await;
    }

    // Request component shutdown and await the server plus its owned children.
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
- [ ] `stop()` does not return until every environment-owned server and
      listener task has completed or its documented deliberate-abort policy ran.
- [ ] The `TODO` comments about graceful event listener shutdown are resolved.
- [ ] Both example binaries handle `SIGTERM` in addition to `SIGINT`.
- [ ] `kill <pid>` against a running example binary shuts it down cleanly.
- [ ] `linter all` passes.

## Dependencies

- SI-2 must define the token-based server lifecycle contract first.
- Example signal handling may follow once `Environment::stop()` is deterministic.

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
