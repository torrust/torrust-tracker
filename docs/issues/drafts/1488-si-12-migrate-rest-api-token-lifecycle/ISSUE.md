---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1488-si-12-migrate-rest-api-token-lifecycle/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/app.rs
    - src/bootstrap/jobs/tracker_apis.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-server/src/signals.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/features/shutdown-process/shutdown-architecture-examples.md
    - docs/issues/drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md
      - docs/issues/drafts/1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-12 — Migrate REST API to Token Lifecycle

> **EPIC position**: Roadmap step 8. One independently releasable REST API
> vertical slice after the additive server lifecycle API and Axum drain helper.

## Goal

Migrate only the tracker management REST API component to the supervised
cancellation tree. The bootstrap derives a REST API component child
`CancellationToken` from `JobManager`; the REST API receives it, starts
connection draining through the token-aware Axum helper, joins its server and
drain-controller children, and reports one named `http_api` outcome to
`JobManager`.

This migration does not change the HTTP tracker, health-check API, UDP server,
or standalone consumers. Their legacy lifecycle paths remain supported.

## Current State

`src/bootstrap/jobs/tracker_apis.rs` starts `ApiServer` and returns a wrapper
`JoinHandle<()>` to `JobManager`. The wrapper already receives the manager
token, forwards cancellation to its private `Halted::Normal` sender, and awaits
the server task. In `packages/axum-rest-api-server`, the launcher spawns
`graceful_shutdown(...)` and discards the drain-controller handle. The legacy
helper observes a `Halted` channel or library-level OS signals.

Consequently, the application supervisor reaches the REST API through a
transitional bridge but cannot prove that the REST API drain controller
completed before the `http_api` wrapper completes.

## Scope

### In scope

- Add a REST API start path that accepts an injected component
  `CancellationToken`.
- Derive a REST API child token from the `JobManager` root token in `src/app.rs`.
- Use the token-aware, joinable Axum drain helper.
- Retain and join the REST API server task and drain-controller task within the
  REST API component's owned task tree.
- Report one named `http_api` outcome to `JobManager`.
- Add deterministic tests for injected-token cancellation and unexpected
  server-task completion/failure, without OS signals.
- Add focused manual SIGTERM evidence after SI-1 verifies the tracker signal
  boundary and the REST API token-driven drain path.

### Out of scope

- HTTP tracker, health-check API, UDP server, and standalone consumer changes.
- Readiness behavior during shutdown; SI-21 owns Q6's approved behavior after
  the health-check API lifecycle migration.
- Removal or deprecation of legacy `Halted`-based REST API start/stop APIs.
- Removal of `global_shutdown_signal()`, deadline configuration, and exit codes.

## Implementation Constraints

1. Existing `ApiServer::start` / `ApiServer::stop` callers remain source- and
   behavior-compatible until migration and deprecation are complete.
2. The new REST API path does not subscribe to `SIGINT` or `SIGTERM` in the
   server package.
3. The REST API component joins its server and drain-controller children before
   returning its top-level outcome.
4. `JobManager` receives only the `http_api` top-level handle and outcome, not
   internal REST server task handles.
5. A cancellation race or unexpected server completion yields an explicit
   outcome; it must not panic or silently discard the drain controller.

## Acceptance Criteria

- [ ] The REST API receives a component child `CancellationToken` derived from
      the `JobManager` root token.
- [ ] Token cancellation starts REST API graceful draining through the new Axum
      helper without a library-level OS-signal subscription.
- [ ] The REST API component joins its server and drain-controller children
      before reporting its named `http_api` outcome to `JobManager`.
- [ ] Legacy REST API start/stop callers compile and preserve their behavior.
- [ ] Deterministic REST API tests cover injected-token cancellation, normal
      drain completion, and unexpected server-task completion/failure.
- [ ] A focused bootstrap integration test proves root-token cancellation
      reaches the REST API without delivering an OS signal.
- [ ] Manual SIGTERM verification records the `main()` signal-boundary event
      followed by the REST API component's token-driven drain completion.
- [ ] `linter all` passes.

## Dependencies

- The additive token-aware server lifecycle API from SI-2 is available and
  released.
- The token-aware, joinable Axum drain helper is available.
- SI-1 is required only for manual SIGTERM verification.

## Rollback

Restore only REST API bootstrap and server call sites to the legacy lifecycle
path. The additive lifecycle API and helper remain available but unused; HTTP,
health-check, UDP, and standalone consumers are unaffected.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run focused REST API component and bootstrap tests that cancel an injected
   token, recording their output.
2. Run the tracker with the REST API enabled. After SI-1, send SIGTERM to the
   tracker binary and record the `main()` signal log followed by REST API drain
   completion.
3. Confirm a legacy REST API start/stop call path still compiles and retains
   its current behavior.
4. Review the token-aware path to confirm it has no OS-signal listener and
   retains every drain-controller handle it creates.
