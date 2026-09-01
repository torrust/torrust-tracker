---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1488-si-19-remove-legacy-shutdown-api/ISSUE.md
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
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/drafts/1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Draft SI-19 — Remove Legacy Shutdown API and Library OS Signals

> **EPIC position**: Roadmap step 15. Breaking removal after all declared
> compatibility, migration, deprecation, and release gates have been satisfied.

## Goal

Remove the deprecated `Halted`-based shutdown API and library-level OS-signal
subscriptions. After this task, normal server shutdown uses injected
`CancellationToken` propagation, and only executable entry points subscribe to
SIGINT or SIGTERM.

The separate `Started` oneshot startup-notification API remains unchanged.

## Start Gate

Do not start implementation until every item below is complete and linked from
`verification.md`:

- [ ] SI-18 deprecation evidence confirms every declared deprecation condition
      and the public support window has ended.
- [ ] HTTP tracker, REST API, health-check API, and UDP tracker application
      components use only token-aware shutdown paths.
- [ ] Standalone HTTP and UDP environments/examples use only token-aware
      shutdown paths.
- [ ] #1588 revalidates the final task inventory and records no supported
      in-workspace legacy shutdown consumer.
- [ ] `torrust-server-lib` release notes confirm the planned breaking release
      and communicate the migration deadline to external consumers.
- [ ] Q5's process-wrapper verification rule is documented: same-process Tokio
      tasks do not survive SIGKILL, and tests target the tracker process or a
      deliberately selected process group.
- [ ] A maintainer explicitly approves the breaking-release timing.

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

### Out of scope

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

- SI-18 is complete and the declared external compatibility window has ended.
- SI-11 through SI-17 token-lifecycle migrations are complete.
- #1588 completes final inventory evidence.
- Q4 is resolved. Q5's process-wrapper verification rule is required for final
  end-to-end removal verification.

## Rollback

This is a breaking removal. Revert the complete removal commit/release to the
last compatible version if a supported consumer needs the legacy path. Do not
attempt a partial runtime rollback by restoring individual signal branches; that
would risk reintroducing mixed signal authority. Publish an urgent compatible
patch or restore the deprecated API in a new compatible release if needed.

## Manual Verification

Record evidence in `verification.md` before closing this issue.

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
