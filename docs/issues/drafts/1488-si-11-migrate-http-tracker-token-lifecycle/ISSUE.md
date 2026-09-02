---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1488-si-11-migrate-http-tracker-token-lifecycle/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/app.rs
    - src/bootstrap/jobs/http_tracker.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-server/src/signals.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/issues/drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md
      - docs/issues/drafts/1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-11 — Migrate HTTP Tracker to Token Lifecycle

> **EPIC position**: Roadmap step 7. One independently releasable HTTP tracker
> vertical slice after the additive server lifecycle API and Axum drain helper.

## Goal

Migrate only the HTTP tracker component to the supervised cancellation tree.
The tracker bootstrap derives a component child `CancellationToken` from
`JobManager`; the HTTP tracker receives it, starts graceful Axum draining on
cancellation through the new helper, joins its server and drain-controller
children, and reports one named component outcome to `JobManager`.

This migration does not change REST API, health-check API, UDP, or standalone
HTTP environment consumers. Their legacy lifecycle paths remain supported.

## Current State

`src/bootstrap/jobs/http_tracker.rs` starts `HttpServer` then returns a wrapper
`JoinHandle<()>` to `JobManager`. The wrapper already receives the manager
token, forwards cancellation to its private `Halted::Normal` sender, and awaits
the server task. In `packages/axum-http-server`, the server spawns
`graceful_shutdown(...)` and discards that drain controller's handle. The
legacy helper observes a `Halted` channel or library-level OS signals.

Consequently, the existing bridge requests HTTP tracker shutdown but does not
give the component a direct token-aware lifecycle API or prove that the HTTP
drain controller completed before the wrapper task ends.

## Scope

### In scope

- Add an HTTP tracker start path that accepts an injected component
  `CancellationToken`.
- Derive one child token per configured HTTP tracker instance in `src/app.rs`.
- Use the token-aware Axum drain helper introduced by the preceding shared-helper
  task.
- Retain and join the HTTP server task and its drain-controller task inside the
  HTTP component's owned task tree.
- Report one named `http_instance_<index>_<address>` outcome to `JobManager`.
- Add deterministic tests that cancel an injected token and await the HTTP
  component completion without an OS signal.
- Add focused manual verification using the tracker binary after SI-1 is
  available, confirming SIGTERM reaches `main()` and the migrated HTTP
  component drains through its token path.

### Out of scope

- Changes to REST API, health-check API, UDP server, or their consumers.
- Removal or deprecation of legacy `Halted`-based HTTP start/stop APIs.
- Removal of `global_shutdown_signal()` or other shared legacy APIs.
- Final component/process deadline values, configuration, and exit codes.

## Implementation Constraints

1. The existing `HttpServer::start` / `HttpServer::stop` lifecycle remains
   source- and behavior-compatible for consumers that have not migrated.
2. The token-aware path must not subscribe to `SIGINT` or `SIGTERM` inside the
   HTTP server package.
3. The HTTP component owns its direct children. It must await both the server
   future and drain-controller future before it returns its outcome.
4. `JobManager` receives only the HTTP component's top-level handle and outcome;
   it does not receive nested HTTP handles.
5. If cancellation races with unexpected server completion, the component must
   return an explicit completed or failed outcome rather than panic or silently
   dropping the drain controller.

## Acceptance Criteria

- [ ] One configured HTTP tracker instance receives one component child
      `CancellationToken` derived from the `JobManager` root token.
- [ ] Token cancellation starts HTTP graceful draining through the new Axum
      helper without a library-level OS-signal subscription.
- [ ] The HTTP component awaits its server and drain-controller tasks before
      reporting its named outcome to `JobManager`.
- [ ] Legacy HTTP start/stop API consumers still compile and preserve behavior.
- [ ] HTTP component tests deterministically cancel an injected token and cover
      normal drain completion and unexpected server-task completion/failure.
- [ ] A focused integration test proves a cancellation request reaches the HTTP
      tracker through bootstrap wiring without delivering an OS signal.
- [ ] Manual SIGTERM verification confirms the migrated HTTP component logs one
      token-driven shutdown path; legacy server signal logs are not required to
      disappear until all consumers migrate and the legacy API is removed.
- [ ] `linter all` passes.

## Dependencies

- Additive token-aware server lifecycle API (SI-2) is available and released.
- Token-aware, joinable Axum drain helper is available.
- SI-1 is required only for the manual SIGTERM check; deterministic tests do
  not require it.

## Rollback

The migration is reversible without an API rollback: restore the HTTP tracker
bootstrap and server call sites to the unchanged legacy lifecycle path. The
additive token-aware APIs remain available but unused; REST, health-check, UDP,
and standalone HTTP consumers are unaffected.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run focused HTTP component and bootstrap integration tests that cancel an
   injected token, recording their output.
2. Run the tracker with one HTTP binding, then send SIGTERM to the tracker
   binary after SI-1. Record the `main()` signal-boundary log and HTTP drain
   completion in the correct order.
3. Start an HTTP tracker through an unchanged legacy start/stop call path and
   confirm it still compiles and stops using its legacy behavior.
4. Review the migrated token-aware path to confirm it has no OS-signal listener
   and retains every drain-controller handle it creates.
