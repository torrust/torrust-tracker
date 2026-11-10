---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p1
epic: 1488
github-issue: null
spec-path: docs/issues/drafts/1488-si-19-remove-legacy-shutdown-api/ISSUE.md
branch: "{issue-number}-remove-legacy-shutdown-api"
related-pr: null
last-updated-utc: "2026-11-10 14:00"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/signals.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - src/bootstrap/jobs/manager.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/closed/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
    - docs/issues/closed/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-19 — Remove Legacy Shutdown API and Library OS Signals

> **EPIC position**: Roadmap step 19. Breaking removal after all declared
> compatibility, migration, deprecation, and release gates have been satisfied.

## Goal

Remove the deprecated `Halted`-based shutdown API and library-level OS-signal
subscriptions. After this task, normal server shutdown uses injected
`CancellationToken` propagation, and only executable entry points subscribe to
SIGINT or SIGTERM.

The separate `Started` oneshot startup-notification API remains unchanged.

## Start Gate

Evidence checked on 2026-11-10 against `torrust/develop` at `1df1ecdf0`. The full
evidence goes in this issue's `verification.md` (SI-18's gate evidence is its
`manual-verification-evidence.md`).

- [x] SI-18 (#2751) is done (PRs #2752, #2793, close-out #2797). `torrust-server-lib`
      0.5.0 (published 2026-11-06, torrust-server-lib#3 / PR 4) deprecates the
      legacy symbols and removes them in 0.6.0.
- [x] The end of the `torrust-server-lib` 0.5.x window. It has no fixed date: it
      ends when the maintainer approves publishing 0.6.0 (SI-18, Versioning
      Decisions). The maintainer gave that approval in chat on 2026-11-10 and
      confirmed 0.6.0 as the `torrust-server-lib` removal release and 4.0.0 as the
      root release target. The maintainer publishes 0.6.0 as part of this task.
      The unpublished package crates and the root `HaltSignal`/`NestedServerTask`
      have never been in a published release and have no public window.
- [x] HTTP tracker, REST API, health-check API and UDP tracker application
      components use only token-aware shutdown paths (`src/bootstrap/jobs/*`).
- [x] Standalone HTTP and UDP examples use only token-aware shutdown paths
      (SI-16 #2412, SI-17 #2448).
- [x] REST API and health-check API test environments use only token-aware
      shutdown paths (SI-23 #2449, SI-24 #2450).
- [x] #1588 (closed) revalidated the task inventory. The remaining in-workspace
      users of the legacy API are the deprecated symbols themselves, their own
      tests and compat tests, each marked `#[allow(deprecated)]`.
- [x] Q5's process-wrapper rule is resolved (`docs/containers.md#stopping-the-tracker`).
- [ ] `torrust-server-lib` 0.6.0 (removal release) is ready: companion issue and
      PR in that repository (session `server-lib-060`), release notes in its
      `CHANGELOG.md`. The maintainer publishes it to crates.io after merge.
      The implementation PR here cannot go green before publication.

## Scope

### In scope

- Remove deprecated `Halted` shutdown channels, legacy server stop triggers,
  and `shutdown_signal`/`shutdown_signal_with_message` helpers.
- Remove `global_shutdown_signal()` and any library-level SIGINT/SIGTERM
  subscription from `torrust-server-lib` and server packages.
- Remove legacy-only compatibility tests and documentation.
- Update Rust documentation, package release notes, and migration guidance to
  state that executables own OS signals and servers accept in-process
  cancellation.
- Prove the tracker and standalone binaries retain deterministic graceful
  shutdown through the token lifecycle API.
- Remove the legacy UDP launcher adapter (`Launcher::run_with_graceful_shutdown`,
  `Spawner::spawn_launcher`, `LaunchRequest`, and `Running`) added or kept by
  SI-14 (#2342), and the bootstrap `NestedServerTask` and `HaltSignal` owner
  types, which have no production consumer after SI-14.
- Bump `torrust-server-lib` to 0.6.0 and remove the `Halted`/`global_shutdown_signal`
  imports and the `#[allow(deprecated)]` markers that guarded them.
- Remove `axum-server`'s `graceful_shutdown` and the legacy `start`/`stop`,
  `Running.halt_task` and `rx_halt` entry points of the HTTP, REST API and
  health-check API servers (the list SI-18 deprecated).

### Out of scope

- Consolidating the token-aware component supervisors and reviewing the owner
  types (`OwnedTask`, `TokenAwareServerTask`, `OwnedReceiveLoop`), and adopting
  the health-check join order. This is a refactor, not a removal of a deprecated
  symbol, so it would widen the breaking surface (Removal Constraint 1). The
  maintainer agreed on 2026-11-10 to split it into its own follow-up issue.
- Publishing `torrust-server-lib` 0.6.0 (the maintainer does it; its removal PR
  lives in the `torrust-server-lib` repository).

- Changing cancellation-tree behavior, component child ownership, timeout
  policy, exit-code policy, or readiness behavior.
- Removing the `Started` oneshot startup-notification API.
- Implementing new features for unknown external consumers that missed the
  declared migration window.
- Altering normal UDP request policy or Axum drain behavior.

## Removal Constraints

1. Remove only symbols documented as deprecated in SI-18; do not widen the
   breaking surface opportunistically.
2. The build must have no remaining in-workspace reference to the removed
   shutdown types, helpers, or OS-signal subscriptions outside executable
   entry points.
3. Each server's token-aware lifecycle path must retain and join its owned
   children before reporting completion. Removal cannot reintroduce detached
   drain, receive, reset, or request tasks.
4. Standalone examples must subscribe to signals only in their executable
   files, then request cancellation through their in-process lifecycle API.
5. A same-process SIGKILL ends the runtime and cannot leave Tokio tasks alive;
   supported `cargo run`, container, and service-manager wrapper behavior is
   documented according to the resolved Q5 policy.

## Acceptance Criteria

- [ ] All start-gate evidence is complete and independently reviewed.
- [ ] `Halted` shutdown API and legacy shutdown helpers are removed from their
      declared packages; `Started` remains available and tested.
- [ ] Server-library code contains no OS-signal subscription.
- [ ] Workspace searches find no legacy shutdown API reference outside archived
      documentation describing the migration history.
- [ ] The tracker binary and standalone HTTP/UDP examples handle SIGINT and
      Unix SIGTERM only at their executable boundaries.
- [ ] Deterministic tests request cancellation through tokens/lifecycle APIs and
      verify owned child completion without OS signals.
- [ ] End-to-end SIGINT and SIGTERM tests verify one orderly shutdown sequence
      per component with no duplicate library signal handling.
- [ ] Container and service-manager verification follows the deadline policy
      defined by Q4 and the deployment guidance defined by the final policy task.
- [ ] `linter all` passes.

## Dependencies

- SI-18 is complete and the `torrust-server-lib` 0.5.x support window has ended (approved 2026-11-10); the
  unpublished package crates and the root types need no window (SI-18, Versioning Decisions).
- SI-11 through SI-17, SI-23, and SI-24 token-lifecycle migrations are complete.
- `torrust-server-lib` 0.6.0 published to crates.io (companion issue and PR in that repository).
- #1588 final inventory evidence is complete (closed).
- Q4 is resolved. Q5's process-wrapper verification rule is required for final
  end-to-end removal verification.

## Rollback

This is a breaking removal. Revert the complete removal commit/release to the
last compatible version if a supported consumer needs the legacy path. Do not
attempt a partial runtime rollback by restoring individual signal branches; that
would risk reintroducing mixed signal authority. Publish an urgent compatible
patch or restore the deprecated API in a new compatible release if needed.

## Manual Verification

Record commands, output, and logs in issue-local `manual-verification-evidence.md` before closing this issue. Follow the EPIC executable-boundary protocol for the tracker and both standalone binaries: signal each direct PID, capture bounded completion and exit status, and prove affected bindings are released.

1. Link all start-gate evidence, including the external deprecation window and
   maintainer approval.
2. Run workspace searches proving legacy shutdown symbols and library-level OS
   signal subscriptions were removed.
3. Run deterministic component and standalone tests using injected tokens or
   lifecycle APIs; no test should require OS signals for cancellation proof.
4. Run end-to-end SIGINT and SIGTERM verification for the tracker and both
   standalone examples. Record exactly one application-owned shutdown sequence.
5. Run container/service-manager verification using the Q4 deployment deadline,
   and record process result according to Q3.
