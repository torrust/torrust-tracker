---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-16-migrate-standalone-http-environment/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-http-server/src/testing/environment.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - packages/axum-http-server/src/server.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-3-fix-environment-stop/ISSUE.md
    - docs/issues/drafts/1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/drafts/1488-si-11-migrate-http-tracker-token-lifecycle/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-16 — Migrate Standalone HTTP Environment and Example

> **EPIC position**: Roadmap step 12. One independently releasable standalone
> HTTP consumer migration after the token-aware HTTP server lifecycle exists.

## Goal

Make the HTTP test environment and HTTP-only example use the token-based server
lifecycle contract. `Environment::stop()` cancels its component token and does
not return until the HTTP server and every listener task it owns has completed
or followed a documented deliberate-abort policy. The example executable maps
SIGINT and Unix SIGTERM to `Environment::stop()`; the HTTP library remains free
of OS-signal subscriptions.

This task changes only the HTTP standalone consumer. The standalone UDP
environment/example and tracker application bootstrap are out of scope.

## Current State

`packages/axum-http-server/src/testing/environment.rs` creates a
`CancellationToken` and passes it to the statistics listener, but `stop()`
aborts the listener instead of cancelling and awaiting it. It stops the HTTP
server through the legacy `HttpServer::stop()` path. The
`http_only_public_tracker` example listens only for Ctrl+C before calling
`Environment::stop()`.

Therefore callers cannot use a deterministic stop-and-wait path for all
HTTP-environment-owned work, and copied example applications do not handle the
standard Unix termination signal.

## Scope

### In scope

- Update HTTP `Environment` startup to use the new token-aware HTTP server path.
- Make `Environment::stop()` cancel its token and await every owned listener and
  HTTP server task before it returns.
- Remove the listener-abort TODO once graceful cancellation and joining are
  implemented.
- Update only `http_only_public_tracker.rs` to await SIGINT or Unix SIGTERM,
  then call `Environment::stop()`.
- Add deterministic environment tests that cancel or call `stop()` without OS
  signals, and prove it awaits all owned tasks.
- Add manual verification of the example's SIGTERM path and clean exit.

### Out of scope

- Standalone UDP environment/example changes.
- Tracker `main()` / `JobManager` changes and tracker application bootstrap.
- Changing HTTP server library signal handling or legacy lifecycle API removal.
- REST API, health-check API, UDP, deadline configuration, exit-code, and
  readiness changes.

## Implementation Constraints

1. `Environment::stop()` owns the environment's listener and HTTP server tasks;
   it must request cancellation top-down and await completion bottom-up.
2. The example is the OS-signal boundary. No library module in the HTTP package
   may introduce a SIGINT or SIGTERM listener.
3. Legacy `HttpServer::start` / `HttpServer::stop` APIs remain supported for
   consumers not migrated to the new token lifecycle.
4. If a listener or server fails while stopping, `stop()` must expose a defined
   failure result rather than silently dropping or aborting it. The exact error
   API may evolve with the token-aware server contract.

## Acceptance Criteria

- [ ] The HTTP environment uses the token-aware HTTP server lifecycle path.
- [ ] `Environment::stop()` cancels its component token and awaits all owned
      listener, server, and drain-controller work before returning.
- [ ] `event_listener_job.abort()` and the related graceful-shutdown TODO are
      removed from the HTTP environment.
- [ ] `http_only_public_tracker.rs` maps SIGINT and Unix SIGTERM to `stop()`.
- [ ] HTTP library modules contain no new OS-signal subscription.
- [ ] Deterministic tests prove that `stop()` waits for its listener and server
      work without delivering an OS signal.
- [ ] Manual SIGTERM verification against the example shows graceful stop and
      records the process result specified by the finalized exit-code policy.
- [ ] Existing legacy HTTP lifecycle callers still compile and preserve behavior.
- [ ] `linter all` passes.

## Dependencies

- Additive token-aware server lifecycle API from SI-2 is released.
- Token-aware, joinable Axum drain helper and HTTP tracker lifecycle migration
  establish the supported token-driven HTTP server path.
- SI-20 later implements Q3's process exit-result mapping; that mapping is not
  required to migrate this standalone consumer's lifecycle.

## Rollback

Restore only the HTTP environment and example to their legacy start/stop path.
The additive HTTP server lifecycle remains available for tracker consumers; the
standalone UDP environment/example and other components are unaffected.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run deterministic environment tests that call `stop()` or cancel its token
   without delivering an OS signal. Record proof that listener and server work
   is awaited.
2. Run `http_only_public_tracker`, send SIGTERM to the example binary, and
   record its signal-boundary output, orderly stop, and exit result.
3. Repeat with Ctrl+C and confirm it follows the same lifecycle path.
4. Confirm no HTTP library module introduces an OS-signal listener and legacy
   HTTP start/stop callers remain compatible.
