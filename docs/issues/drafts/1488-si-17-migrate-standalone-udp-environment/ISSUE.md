---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1488-si-17-migrate-standalone-udp-environment/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/udp-server/src/testing/environment.rs
    - packages/udp-server/examples/udp_only_public_tracker.rs
    - packages/udp-server/src/server/launcher.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-3-fix-environment-stop/ISSUE.md
    - docs/issues/drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-17 — Migrate Standalone UDP Environment and Example

> **EPIC position**: Roadmap step 13. One independently releasable standalone
> UDP consumer migration after the token-aware UDP lifecycle is complete.

## Goal

Make the UDP test environment and UDP-only example use the token-based UDP
lifecycle contract. `Environment::stop()` cancels its component token and does
not return until the UDP server and every event-listener task it owns has
completed or followed a documented deliberate-abort policy. The example
executable maps SIGINT and Unix SIGTERM to `Environment::stop()`; the UDP
library remains free of OS-signal subscriptions.

This task changes only the standalone UDP consumer. The standalone HTTP
environment/example and tracker application bootstrap are out of scope.

## Current State

`packages/udp-server/src/testing/environment.rs` creates a `CancellationToken`
and gives it to the UDP core, UDP server statistics, and UDP server banning
event listeners. Its `stop()` method aborts those three listener tasks instead
of cancelling and awaiting them. It stops the UDP server through the legacy
`Server::stop()` path. The `udp_only_public_tracker` example listens only for
Ctrl+C before calling `Environment::stop()`.

Therefore callers cannot use a deterministic stop-and-wait path for all
UDP-environment-owned work, and copied example applications do not handle the
standard Unix termination signal.

## Scope

### In scope

- Update UDP `Environment` startup to use the new token-aware UDP server path.
- Make `Environment::stop()` cancel its token and await all three owned event
  listeners plus UDP server work before returning.
- Remove the three listener-abort TODO comments once graceful cancellation and
  joining are implemented.
- Update only `udp_only_public_tracker.rs` to await SIGINT or Unix SIGTERM,
  then call `Environment::stop()`.
- Add deterministic environment tests that cancel or call `stop()` without OS
  signals and prove it awaits all owned tasks.
- Add manual verification of the example's SIGTERM path and clean exit.

### Out of scope

- Standalone HTTP environment/example changes.
- Tracker `main()` / `JobManager` changes and tracker application bootstrap.
- UDP receive-loop ownership or active-request policy work, which must already
  be supplied by SI-14 and SI-15. The application-level UDP IP-ban cleanup job
  remains separately manager-owned.
- Removal/deprecation of legacy UDP lifecycle APIs or library OS-signal paths.
- HTTP, REST API, health-check, deadline configuration, exit-code, and
  readiness changes.

## Implementation Constraints

1. `Environment::stop()` owns its three listener tasks and UDP server task. It
   requests cancellation top-down and awaits completion bottom-up.
2. The example is the OS-signal boundary. No UDP library module may introduce a
   SIGINT or SIGTERM listener.
3. Legacy `Server::start` / `Server::stop` APIs remain supported for consumers
   not migrated to the new token lifecycle.
4. If a listener or server fails while stopping, `stop()` must expose a defined
   failure result rather than silently dropping or aborting it. The error API
   may evolve with the token-aware UDP server contract.
5. The component obeys the established UDP active-request policy; this task does
  not change receive-loop or request behavior within the UDP server.

## Acceptance Criteria

- [ ] The UDP environment uses the token-aware UDP server lifecycle path.
- [ ] `Environment::stop()` cancels its token and awaits all three listener
      tasks plus UDP server-owned work before returning.
- [ ] Listener `abort()` calls and the related graceful-shutdown TODO comments
      are removed from the UDP environment.
- [ ] `udp_only_public_tracker.rs` maps SIGINT and Unix SIGTERM to `stop()`.
- [ ] UDP library modules contain no new OS-signal subscription.
- [ ] Deterministic tests prove that `stop()` waits for every owned listener and
      server task without delivering an OS signal.
- [ ] Manual SIGTERM verification against the example records graceful stop and
      the process result specified by the finalized exit-code policy.
- [ ] Existing legacy UDP lifecycle callers still compile and preserve behavior.
- [ ] `linter all` passes.

## Dependencies

- SI-14 (UDP receive-loop token lifecycle) is complete.
- SI-15 (UDP active-request policy) is complete.
- SI-20 later implements Q3's process exit-result mapping; that mapping is not
  required to migrate this standalone consumer's lifecycle.

## Rollback

Restore only the UDP environment and example to their legacy start/stop path.
The additive UDP lifecycle remains available for tracker consumers; standalone
HTTP and every other component remain unaffected.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

1. Run deterministic environment tests that call `stop()` or cancel its token
   without delivering an OS signal. Record proof that all listener and UDP
   server work is awaited.
2. Run `udp_only_public_tracker`, send SIGTERM to the example binary, and
   record its signal-boundary output, orderly stop, and exit result.
3. Repeat with Ctrl+C and confirm it follows the same lifecycle path.
4. Confirm no UDP library module introduces an OS-signal listener and legacy
   UDP start/stop callers remain compatible.
