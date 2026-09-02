---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/app.rs
    - src/bootstrap/jobs/health_check_api.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/handlers.rs
    - packages/axum-server/src/signals.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md
      - docs/issues/drafts/1488-si-10-add-token-aware-axum-drain-helper/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-13 — Migrate Health-Check API to Token Lifecycle

> **EPIC position**: Roadmap step 9. One independently releasable health-check
> API vertical slice after the additive server lifecycle API and Axum drain helper.

## Goal

Migrate only the health-check API component to the supervised cancellation tree.
The bootstrap derives a health-check API child `CancellationToken` from
`JobManager`; the health-check API receives it, begins Axum connection draining
through the token-aware helper, joins its server and drain-controller children,
and reports one named `health_check_api` outcome to `JobManager`.

This migration does not change HTTP tracker, REST API, UDP server, or standalone
consumers. Their legacy lifecycle paths remain supported.

## Current State

`src/bootstrap/jobs/health_check_api.rs` creates startup and shutdown oneshot
channels, spawns the server, then returns a wrapper `JoinHandle<()>` to
`JobManager`. The wrapper already receives the manager token, forwards
cancellation to its private `Halted::Normal` sender, and awaits the server task.
`packages/axum-health-check-api-server/src/server.rs` spawns
`graceful_shutdown(...)` and drops the drain-controller handle. The legacy
helper observes a `Halted` channel or library-level OS signals.

Consequently, `JobManager` reaches the health-check API through a transitional
bridge but cannot verify drain-controller completion before `health_check_api`
reports completion.

## Scope

### In scope

- Add a health-check API start path accepting an injected component
  `CancellationToken`.
- Derive one health-check API child token from the `JobManager` root token in
  `src/app.rs`.
- Use the token-aware, joinable Axum drain helper.
- Retain and join the health-check API server and drain-controller tasks within
  the component's owned task tree.
- Report one named `health_check_api` outcome to `JobManager`.
- Add deterministic tests for injected-token cancellation, normal drain,
  unexpected server-task completion/failure, and bootstrap propagation.
- Add focused manual SIGTERM evidence after SI-1 verifies the tracker signal
  boundary and the health-check API token-driven drain path.

### Out of scope

- Returning unhealthy or HTTP 503 responses before or during shutdown. SI-21
  owns the Q6-approved readiness behavior as a separate vertical slice.
- HTTP tracker, REST API, UDP server, and standalone consumer migrations.
- Removal or deprecation of legacy `Halted`-based health-check start/stop APIs.
- Removal of `global_shutdown_signal()`, deadline configuration, and exit codes.

## Implementation Constraints

1. Existing health-check API start/stop callers remain source- and
   behavior-compatible until the separate deprecation/removal phase.
2. The token-aware path has no `SIGINT` or `SIGTERM` subscription inside the
   health-check server package.
3. The component joins its server and drain-controller children before it
   returns the top-level `health_check_api` outcome.
4. `JobManager` receives only the health-check component handle and outcome;
   it does not receive server or drain-controller handles.
5. Existing health-check request and probe behavior remains unchanged. SI-21,
   after this migration, alters shutdown readiness, response status, and probe
   fan-out behavior.
6. A cancellation race or unexpected server completion returns an explicit
   outcome; it must not panic or silently drop a drain-controller task.

## Acceptance Criteria

- [ ] The health-check API receives a component child `CancellationToken`
      derived from the `JobManager` root token.
- [ ] Token cancellation starts health-check API graceful draining through the
      new Axum helper without a library-level OS-signal subscription.
- [ ] The component awaits its server and drain-controller children before
      reporting the named `health_check_api` outcome to `JobManager`.
- [ ] Legacy health-check API start/stop callers compile and preserve behavior.
- [ ] Deterministic tests cover injected-token cancellation, normal drain, and
      unexpected server-task completion/failure without OS signals.
- [ ] A focused bootstrap integration test proves root-token cancellation
      reaches the health-check API without delivering an OS signal.
- [ ] Existing health-check response and readiness semantics are unchanged in
      this migration; SI-21 applies the separately approved shutdown behavior.
- [ ] Manual SIGTERM verification records the `main()` signal event followed
      by the health-check API's token-driven drain completion.
- [ ] `linter all` passes.

## Dependencies

- The additive token-aware server lifecycle API from SI-2 is available and
  released.
- The token-aware, joinable Axum drain helper is available.
- SI-1 is required only for manual SIGTERM verification.
- SI-21 follows this migration to apply Q6's readiness-before-drain behavior.

## Rollback

Restore only the health-check API bootstrap and server call sites to the legacy
lifecycle path. The additive lifecycle API and helper remain available but
unused; HTTP, REST, UDP, and standalone consumers are unaffected.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run focused health-check API component and bootstrap tests that cancel an
   injected token, recording their output.
2. Run the tracker with the health-check API enabled. After SI-1, send SIGTERM
   to the tracker binary and record the `main()` signal event followed by the
   health-check API drain completion.
3. Verify that health-check responses remain unchanged during normal operation;
   do not assert a shutdown readiness response because SI-21 owns Q6's approved
   readiness-before-drain behavior.
4. Confirm a legacy health-check API start/stop call path still compiles and
   retains current behavior.
5. Review the token-aware path to confirm it has no OS-signal listener and
   retains every drain-controller handle it creates.
