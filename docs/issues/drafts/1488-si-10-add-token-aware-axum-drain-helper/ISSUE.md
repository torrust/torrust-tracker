---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/signals.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/server.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-10 — Add Token-Aware, Joinable Axum Drain Helper

> **EPIC position**: Roadmap step 6. Additive shared-helper task. It preserves
> the existing `Halted`-based helper while introducing a token-aware alternative.

## Goal

Add a new shared Axum graceful-shutdown helper that accepts an injected
`CancellationToken`, starts connection draining on cancellation, and can be
awaited by the server task that owns it. The existing
`graceful_shutdown(handle, rx_halt, message, address)` helper remains unchanged
in this task so all existing consumers continue to compile and behave as before.

This task creates a reusable building block; it does not migrate the HTTP
tracker, REST API, or health-check API to the new helper.

## Background

`packages/axum-server/src/signals.rs` currently spawns the shutdown path from
server implementations with `tokio::task::spawn(graceful_shutdown(...))` and
discards the returned `JoinHandle`. The helper waits on a shutdown `Halted`
channel or library-level OS signals, starts `Handle::graceful_shutdown`, and
polls connection count.

Under the Q2 target architecture, cancellation flows from the owner to a child
component token and completion flows upward through awaited handles. A detached
drain helper prevents the owner from proving that drain completed before its
server task reports completion.

## Scope

### In scope

- Add a token-aware helper beside the existing helper.
- Accept an injected `CancellationToken`, Axum `Handle`, address, message, and
  a deadline/budget input suitable for later Q4 configuration.
- Have the helper await token cancellation, request Axum graceful shutdown, and
  return a typed result that distinguishes drained versus deadline-reached.
- Make the helper usable as a future that a server task can await or spawn and
  retain as a `JoinHandle`.
- Add deterministic tests that cancel a token without delivering OS signals.
- Document the ownership contract for future HTTP, REST API, and health-check
  component migrations.

### Out of scope

- Migrating any existing Axum server consumer to the new helper.
- Removing or changing `Halted`, `shutdown_signal_with_message`, or
  `global_shutdown_signal()`.
- Selecting production deadline values or configuration schema (Q4).
- Exit-code behavior (Q3) and readiness behavior (Q6).

## Proposed API Shape

The exact type names are implementation decisions. The API must preserve this
shape of responsibility:

```rust
pub async fn graceful_shutdown_on_cancellation(
    handle: axum_server::Handle<SocketAddr>,
    cancellation_token: CancellationToken,
    message: String,
    address: SocketAddr,
    deadline: Duration,
) -> GracefulShutdownOutcome
```

The caller owns the returned future or its spawned `JoinHandle`. The helper must
not spawn an unowned background task internally. A later component migration
uses a `tokio::select!` between its server future and this owned drain future,
then awaits any remaining owned child before returning the component outcome.

## Acceptance Criteria

- [ ] Existing `graceful_shutdown` behavior and public signature are unchanged.
- [ ] A new token-aware helper accepts injected cancellation without subscribing
      to an OS signal or receiving a shutdown `Halted` channel.
- [ ] The helper starts `Handle::graceful_shutdown` only after token
      cancellation.
- [ ] The helper returns an outcome that distinguishes all connections drained
      from the drain deadline reached.
- [ ] The helper does not create an unowned task. Its caller can await it or
      retain its join handle.
- [ ] Deterministic tests cancel an injected token and cover both drained and
      deadline outcomes without OS signals.
- [ ] Existing HTTP tracker, REST API, and health-check server tests still pass
      unchanged against the legacy helper.
- [ ] `linter all` passes.

## Dependencies

- Follows the additive server lifecycle API from SI-2.
- Does not depend on migration of any Axum server consumer.
- Q4 later defines final deadline relationships and configuration; this task
  only provides an input for the budget.

## Rollback

This is additive. Reverting it removes only the unused new helper and tests;
all existing Axum consumers keep using the unchanged legacy helper.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run the focused deterministic tests and record their output.
2. Confirm existing server packages compile and their existing tests pass
   without changing their call sites.
3. Confirm the new helper has no OS-signal subscription or shutdown `Halted`
   channel parameter.
4. Confirm no `tokio::spawn` inside the new helper discards a join handle.
