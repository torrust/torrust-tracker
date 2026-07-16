---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-07-16
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/signals.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/udp-server/src/server/launcher.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/features/shutdown-process/open-questions.md
---

<!-- skill-link: create-issue -->

# Draft SI-2 — Remove `global_shutdown_signal()` from Per-Server Shutdown

> **EPIC position**: SI-2 of #1488. Depends on SI-1.
> **Blocked**: Open questions Q1 and Q5 must be resolved before implementing.

## Goal

Remove the `global_shutdown_signal()` call from `shutdown_signal()` in
`torrust_server_lib::signals` so that servers only stop when explicitly told to
by `main.rs` via the halt channel — not by independently catching OS signals.

After SI-1 (`main.rs` catches `SIGTERM`), both `main.rs` and each server still
catch SIGINT/SIGTERM independently. This creates a double (or triple) signal
scenario. This sub-issue cleans that up so that:

- `main.rs` owns all signal handling.
- Servers shut down only when they receive a `Halted::Normal` message via
  their oneshot channel.

## Background

`torrust_server_lib::signals::shutdown_signal()` currently contains:

```rust
pub async fn shutdown_signal(rx_halt: tokio::sync::oneshot::Receiver<Halted>) {
    tokio::select! {
        signal = halt => { ... },
        () = global_shutdown_signal() => { ... }  // <-- catches SIGINT/SIGTERM directly
    }
}
```

This means every server independently catches Ctrl+C and SIGTERM. The servers
do not wait for `main.rs` to coordinate the shutdown order.

## Implementation

Change `shutdown_signal()` in `torrust_server_lib` to remove the
`global_shutdown_signal()` branch:

```rust
pub async fn shutdown_signal(rx_halt: tokio::sync::oneshot::Receiver<Halted>) {
    match rx_halt.await {
        Ok(signal) => tracing::debug!("Halt signal processed: {}", signal),
        Err(err) => panic!("Failed to install stop signal: {err}"),
    }
}
```

Servers then only stop when `main.rs` explicitly sends `Halted::Normal` via
the halt channel.

## Important Considerations

### External package dependency

`torrust_server_lib` is an external standalone crate, not part of this workspace.
Changes to `shutdown_signal()` require a coordinated release of `torrust-server-lib`
and a version bump in this workspace's `Cargo.toml`.

### Orphan risk on `main.rs` crash

If `main.rs` crashes or is SIGKILL'd before sending halt messages, servers
become orphaned (ports stay open, process appears dead). See Q5 for the
strategy decision on this risk.

### Impact on standalone binary consumers

The examples `http_only_public_tracker.rs` and `udp_only_public_tracker.rs` use
`global_shutdown_signal()` indirectly via the server's `shutdown_signal()`. After
this change, those examples must be updated to send halt signals explicitly — or
they can be updated independently in SI-3.

## Acceptance Criteria

- [ ] Q1 and Q5 are resolved before implementation begins.
- [ ] `shutdown_signal()` in `torrust-server-lib` no longer calls
      `global_shutdown_signal()`.
- [ ] Servers only stop when the halt channel receives `Halted::Normal`.
- [ ] `main.rs` sends halt messages to all servers before or during
      `jobs.wait_for_all()`.
- [ ] All existing server start/stop tests pass.
- [ ] Experimental validation: `kill <pid>` (after SI-1) still shuts down
      cleanly with a single shutdown sequence in the logs (no duplicate
      "halting" messages per server).

## Dependencies

- SI-1 must land first.
- Requires `torrust-server-lib` to be updated and released.
- Q1 and Q5 in open-questions.md must be resolved.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Setup

```bash
cargo build --release
RUST_LOG=info ./target/release/torrust-tracker
```

### Test 1: Single shutdown sequence in the logs (no duplicate "halting" messages)

Send `kill <pid>` (SIGTERM) to the tracker.

**Expected**: Each server logs exactly **one** halt message, not two:

```text
# CORRECT (one message per server):
INFO  Shutting down HTTP server on socket address: 0.0.0.0:7070

# WRONG (two messages per server — indicates double-signal still present):
WARN  caught interrupt signal (ctrl-c), halting...
INFO  Shutting down HTTP server on socket address: 0.0.0.0:7070
```

**Record in `verification.md`**: full shutdown log showing absence of
`global_shutdown_signal` messages.

### Test 2: Ctrl+C also produces a single shutdown sequence

Send SIGINT (Ctrl+C). Confirm the same — no duplicate halt messages per server.

### Test 3: Orphan risk validation

Send `kill -9 <pid>` to force-kill the main process.

**Expected**: All server ports (`6868`, `6969`, `7070`, `7171`, `1212`, `1313`)
are freed within a few seconds (the OS reclaims them when the process group exits).

```bash
sleep 2 && lsof -i :7070,6969,1212 | grep LISTEN
```

Output should be empty. **Record any ports that remain open.**

### Test 4: Restart succeeds immediately after clean shutdown

After a graceful shutdown (SIGTERM or SIGINT), restart the tracker immediately.

**Expected**: All services bind to their ports without "address already in use" errors.
