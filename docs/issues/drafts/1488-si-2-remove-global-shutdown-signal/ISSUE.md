---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
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
    - docs/features/shutdown-process/questions.md
---

<!-- skill-link: create-issue -->

# Draft SI-2 — Establish Token-Based Server Lifecycle and Remove OS Signals

> **EPIC position**: Roadmap step 5. Additive server-lifecycle foundation for
> #1488; legacy shutdown behavior remains available to existing consumers.

## Goal

Establish a token-based lifecycle contract for server libraries. A server
receives an in-process `CancellationToken`, owns the tasks it spawns, and does
not subscribe to OS signals. On cancellation, it performs its protocol-specific
graceful stop and joins its owned children before its top-level task completes.

`main.rs` remains the only tracker OS-signal boundary. `JobManager` cancels its
root token; component child tokens carry that request into the server tree.
The `Started` oneshot remains a startup notification. The shutdown `Halted`
oneshot is removed from the target lifecycle API; any temporary forwarding is a
strictly bounded migration bridge, not the final design.

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

The tracker application already has a transitional bridge: its HTTP, REST API,
health-check API, and UDP job wrappers receive the manager token, forward
cancellation to private `Halted::Normal` channels, and await their server
tasks. This task replaces that bridge with the additive direct token-aware
lifecycle API; it must not describe manager cancellation wiring as new.

## Implementation

1. Define and release an **additive** `torrust-server-lib` lifecycle API based
   on injected cancellation, while retaining `global_shutdown_signal()` and
   shutdown `Halted` channel compatibility for existing consumers.
2. Make server start APIs accept an injected cancellation token or a component
   lifecycle context that owns one.
3. Require server implementations to retain and join their graceful-stop
   controller tasks rather than discarding their handles.
4. Document the required component-consumer migrations; SI-11 through SI-17
   derive component child tokens and await server completion through managed
   component tasks.
5. Document migration and deprecation criteria; do not remove the legacy path
   in this issue.

See [Q2 decision](../../../features/shutdown-process/questions.md#q2) for full
rationale.

## Important Considerations

### External package dependency

`torrust_server_lib` is an external standalone crate, not part of this workspace.
Changes to `shutdown_signal()` require a coordinated release of `torrust-server-lib`
and a version bump in this workspace's `Cargo.toml`.

### Process-wrapper signal targeting

Q5 is resolved: a `SIGKILL` of the tracker process cannot leave its Tokio
tasks or ports alive. A `cargo run` child process is a separate operational
concern; manual tests must signal the actual tracker binary or a deliberately
selected process group.

### Impact on standalone binary consumers

The examples `http_only_public_tracker.rs` and `udp_only_public_tracker.rs` must
be migrated to token lifecycle APIs in their own later drafts. Their executable
entry points, not server libraries, handle OS signals.

## Acceptance Criteria

- [ ] An additive lifecycle API accepts injected cancellation without requiring
      an OS-signal subscription.
- [ ] Existing `global_shutdown_signal()` and shutdown `Halted` channel users
      remain source- and behavior-compatible.
- [ ] The target lifecycle API does not expose a shutdown `Halted` channel;
      `Started` startup signaling remains unaffected.
- [ ] Server components receive cancellation through an injected token or
      lifecycle context, not through OS signals.
- [ ] Migration documentation states that each eventual server component must
      join its graceful-stop controller before reporting completion to its
      parent.
- [ ] All existing server start/stop tests pass.
- [ ] Compatibility tests prove existing legacy server consumers preserve their
      current shutdown behavior while the additive API remains unused.

## Dependencies

- Requires `torrust-server-lib` to be updated and released.
- Q5's process-wrapper premise must be corrected before legacy removal, not
  before this additive API release.
- HTTP, REST, health-check, UDP, and standalone consumers migrate separately
  before legacy deprecation and removal.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Setup

```bash
cargo build --release
RUST_LOG=info ./target/release/torrust-tracker
```

### Test 1: Additive API preserves legacy consumers

Compile and run representative legacy server consumers without changing their
call sites. Record that their shutdown behavior remains available while the new
token-aware lifecycle API is introduced.

### Test 2: New lifecycle API has no OS-signal dependency

Run focused deterministic tests that cancel an injected token and await the
new lifecycle API outcome. Do not use SIGINT or SIGTERM in this test.

### Test 3: Process targeting validation

Run the tracker through `cargo run`, record the process tree, and demonstrate
that a direct test targets the actual tracker binary rather than its Cargo
launcher. If testing a process group, document that intention explicitly.

**Expected**: Signal delivery and observed shutdown behavior match the selected
target. Do not describe a separate Cargo child process as a Tokio task orphan.

### Test 4: Restart succeeds immediately after clean shutdown

After a graceful shutdown (SIGTERM or SIGINT), restart the tracker immediately.

**Expected**: All services bind to their ports without "address already in use" errors.
