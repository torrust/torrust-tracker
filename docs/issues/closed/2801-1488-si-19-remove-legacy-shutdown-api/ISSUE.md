---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p1
epic: 1488
github-issue: 2801
spec-path: docs/issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md
branch: "2801-remove-legacy-shutdown-api"
related-pr: 2807
last-updated-utc: "2026-11-11 10:19"
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

# Issue #2801 - Remove Legacy Shutdown API and Library OS Signals

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
evidence goes in this issue's `manual-verification-evidence.md` (SI-18's gate evidence is its
`manual-verification-evidence.md`).

- [x] SI-18 (#2751) is done (PRs #2752, #2793, close-out #2797). `torrust-server-lib`
      0.5.0 (published 2026-11-06, torrust-server-lib#3 / PR 4) deprecates the
      legacy symbols and removes them in 0.6.0.
- [x] The end of the `torrust-server-lib` 0.5.x window. It has no fixed date: it
      ends when the maintainer approves publishing 0.6.0 (SI-18, Versioning
      Decisions). The maintainer gave that approval in chat on 2026-11-10 and
      confirmed 0.6.0 as the `torrust-server-lib` removal release and 4.0.0 as the
      root release target. SI-18 counted the publication of 0.6.0 as part of SI-19; it was completed in
      the `torrust-server-lib` repository before implementation, so this task only
      waits for it (below).
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
- [x] `torrust-server-lib` 0.6.0, the removal release, is published: issue
      torrust/torrust-server-lib#5, PR torrust/torrust-server-lib/pull/6 (merged
      into `main` at `2dd04f4`), release notes in its `CHANGELOG.md`; crates.io
      reports `max_version` 0.6.0 (checked 2026-11-10). This is an external
      dependency this task waits for; its publication is the step SI-18 assigned to
      SI-19.

## Scope

### In scope

- Remove deprecated `Halted` shutdown channels, legacy server stop triggers,
  and `shutdown_signal`/`shutdown_signal_with_message` helpers.
- Remove `global_shutdown_signal()` (removed upstream in `torrust-server-lib`
  0.6.0; here, its last use in `udp-server`) and any library-level SIGINT/SIGTERM
  subscription from the server packages.
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
- Remove every item SI-18 deprecated in this repository. The complete list is the
  27 shutdown-related `#[deprecated]` attributes in the tree, and T2 to T4 name each
  one. SI-18's in-scope list
  (`docs/issues/closed/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md`) is a
  starting point, not the complete list: it omits the aliases and the HTTP and REST `Running`
  structs as a whole, and it names the private HTTP `Launcher::start`, which carries
  `#[allow(deprecated)]`, not `#[deprecated]`. The set includes `axum-server`'s `graceful_shutdown`; the legacy `start`, `start_with_health_check`,
  `stop`, `Running` state, `Running::new`, `halt_task` and `rx_halt` of the HTTP,
  REST API, health-check API and UDP servers; and the aliases that name the legacy
  states (`RunningHttpServer`, `RunningApiServer`, `RunningUdpServer`). The workspace
  search in Acceptance Criteria catches any residue.

### Out of scope

- Consolidating the token-aware component supervisors and reviewing the owner
  types (`OwnedTask`, `TokenAwareServerTask`, `OwnedReceiveLoop`), and adopting
  the health-check join order. This is a refactor, not a removal of a deprecated
  symbol, so it would widen the breaking surface (Removal Constraint 1). The
  maintainer agreed on 2026-11-10 to split it into its own follow-up issue.
- The `torrust-server-lib` 0.6.0 removal and its publication: SI-18 assigned the
  publication to SI-19, but it was done in the `torrust-server-lib` repository
  (torrust/torrust-server-lib#5, PR 6) and published before
  implementation, so it is an external dependency this task only waits for.
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

- [x] All start-gate evidence is complete and independently reviewed (the reviews of PR 2807).
- [x] `Halted` shutdown API and legacy shutdown helpers are removed from their
      declared packages; `Started` remains available in `torrust-server-lib` 0.6.0 and
      is no longer used in this workspace.
- [x] Server-library code contains no OS-signal subscription.
- [x] Workspace searches find no use of the legacy shutdown API. Descriptions of it
      remain only in archived documentation and migration history, and in five lines
      of live specs of other issues (see the evidence, Test 1).
- [x] The tracker binary and standalone HTTP/UDP examples handle SIGINT and
      Unix SIGTERM only at their executable boundaries.
- [x] Deterministic tests request cancellation through tokens/lifecycle APIs and
      verify owned child completion without OS signals.
- [x] End-to-end SIGINT and SIGTERM tests verify one orderly shutdown sequence
      per component with no duplicate library signal handling.
- [x] Container and service-manager verification follows the deadline policy
      defined by Q4 and the deployment guidance defined by the final policy task.
- [x] `linter all` passes.

## Dependencies

- SI-18 is complete and the `torrust-server-lib` 0.5.x support window has ended (approved 2026-11-10); the
  unpublished package crates and the root types need no window (SI-18, Versioning Decisions).
- SI-11 through SI-17, SI-23, and SI-24 token-lifecycle migrations are complete.
- `torrust-server-lib` 0.6.0 published to crates.io (external dependency; done 2026-11-10, torrust/torrust-server-lib#5 and PR 6).
- #1588 final inventory evidence is complete (closed).
- Q4 is resolved. Q5's process-wrapper verification rule is required for final
  end-to-end removal verification.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`
  (the token lifecycle that replaces `Halted`) and
  `docs/adrs/20260629000000_adopt_independent_package_versioning.md`.
- ADRs to create: none known.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | DONE | Wait for `torrust-server-lib` 0.6.0 (done) and branch from the merged spec. No change and no commit. | Recorded in the progress log only. |
| T2  | DONE | `udp-server`: remove `Server<Stopped>::start`, legacy `Running` and `RunningUdpServer`, `Server<Running>::stop`, `Launcher::run_with_graceful_shutdown`, `Spawner::spawn_launcher`, `LaunchRequest`, `legacy_stop_requested`, `halt_requested`, and the `global_shutdown_signal` use. | Legacy-only tests removed; token lifecycle tests stay. |
| T3  | DONE | `axum-http-server`, `axum-rest-api-server`, `axum-health-check-api-server`, `axum-server`: remove the legacy `start`, `start_with_health_check`, `stop`, `Running` structs and aliases, `Running::new`, `halt_task`, `rx_halt`, `RunningServer` and `graceful_shutdown`. | Token-aware paths keep their child ownership and join behavior. |
| T4  | DONE | Root: remove `HaltSignal` and `NestedServerTask` and their tests; bump `torrust-server-lib` to 0.6.0 in every `Cargo.toml`; drop every `#[allow(deprecated)]` added by SI-18 and the compat tests that exist only for removed symbols. | The bump lands with the removals, since 0.6.0 removes `Halted` and an earlier bump would not build. `cargo check --workspace --all-targets` warning-free. |
| T5 | DONE | Update docs: the migration guide `docs/features/shutdown-process/legacy-api-migration.md` (removed, with history), and the shutdown-process docs. The EPIC row 19 is flipped to Done by the close-out archive step, as for SI-18 row 18. | Workspace search finds no use of the legacy symbols; descriptions remain in archived docs and the listed live specs. |
| T6 | DONE | Verify and record evidence: searches, deterministic tests, SIGINT/SIGTERM for the tracker and both standalone examples, container check. | `manual-verification-evidence.md`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | None (wait only) | No commit. |
| T2 | `udp-server` removal | Commit after focused validation and required review. |
| T3 | Axum packages removal | Commit after focused validation and required review. |
| T4 | Root types, allowances, compat tests and the `torrust-server-lib` 0.6.0 bump | Commit after focused validation and required review. Bumping in the same commit keeps the tree building. |
| T5 | Documentation | Commit after the last production change. |
| T6 | Verification evidence | Commit after the last production change. |

Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/`, moved to `docs/issues/open/`
- [x] Spec reviewed and approved by the maintainer (issue creation and spec-only PR approved in chat, 2026-11-10)
- [x] GitHub issue created and issue number added to this spec (#2801)
- [x] Spec-only PR merged into `develop` before implementation (PR 2803)
- [x] `torrust-server-lib` 0.6.0 published (external dependency, 2026-11-10)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, pre-push checks)
- [x] Manual verification scenarios executed and recorded in `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: an issue-local `implementation-retrospective.md` for material discoveries, or a progress-log entry that says why none was needed
- [x] Independent reviewer reports recorded (the reviews of PR 2807 and the audit `docs/pr-reviews/pr-2807-review/PR-REVIEW.md`)
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-10 12:52 UTC - AI agent (Claude Sonnet 5.5) - Refreshed the draft after SI-18: recorded the start-gate evidence and the maintainer's approval (chat, 2026-11-10) ending the `torrust-server-lib` 0.5.x window with 0.6.0 as the removal release and 4.0.0 as the root target; split the supervisor consolidation into a follow-up; companion removal in torrust/torrust-server-lib#5 (PR 6).
- 2026-11-10 13:04 UTC - AI agent (Claude Sonnet 5.5) - Created issue #2801 and moved the spec to `docs/issues/open/`.
- 2026-11-10 13:30 UTC - AI agent (Claude Sonnet 5.5) - Addressed the review of PR 2803 (F1 to F7): full removal list by reference to SI-18, T1 as a wait-only task with the bump in T4, one wording for the 0.6.0 dependency, real timestamps, the verification sections, and the evidence file matched to the gate. `torrust-server-lib` 0.6.0 was published meanwhile (crates.io `max_version` 0.6.0).
- 2026-11-10 13:06 UTC - AI agent (Claude Sonnet 5.5) - Opened the spec-only PR 2803 (created at 13:06:40 UTC).
- 2026-11-10 13:36 UTC - AI agent (Claude Sonnet 5.5) - Added the implementation-completion-review checkpoint and requirement for Copilot's finding R2 (commit at 13:36:46 UTC).
- 2026-11-10 13:59 UTC - AI agent (Claude Sonnet 5.5) - Edited the spec for the second review of PR 2803: the 0.6.0 wording reconciled with SI-18 (F11), the removal list stated as the 27 `#[deprecated]` attributes (F12), and these log entries and the stamp corrected (F4). The audit record `docs/pr-reviews/pr-2803-review/PR-REVIEW.md` is added in the same round (F8).
- 2026-11-10 14:25 UTC - AI agent (Claude Sonnet 5.5) - Edited the spec for the third review of PR 2803: the stamps now match the commit times (F4) and the removal bullet says the HTTP and REST `Running` structs (F15).
- 2026-11-10 15:47 UTC - AI agent (Claude Sonnet 5.5) - Implemented T2 to T6 on branch `2801-remove-legacy-shutdown-api` after the spec PR 2803 merged: removed the legacy API from `udp-server` and the three axum servers and `axum-server`, removed `HaltSignal` and `NestedServerTask`, bumped `torrust-server-lib` to 0.6.0, and updated the migration guide. `Started` is no longer used or tested inside this workspace; it stays available in `torrust-server-lib` 0.6.0. The public `Error::Stop`, `StartupNotification` and `StartupNotificationDropped` variants of the HTTP and REST servers are now unused but were not deprecated, so they stay (candidate follow-up). Evidence in `manual-verification-evidence.md`.
- 2026-11-10 16:18 UTC - AI agent (Claude Sonnet 5.5) - Addressed the first review of PR 2807 (F1 to F11): wording for `Started`, the T5 note, the evidence (raw search output, the script, the approval link), the status and frontmatter, one removed-API line in a live draft, the token-aware owner test, and the docs left by the removal. The audit is `docs/pr-reviews/pr-2807-review/PR-REVIEW.md`.
- 2026-11-10 16:55 UTC - AI agent (Claude Sonnet 5.5) - Addressed the second review of PR 2807 (F12 to F16): the live-spec search pasted as run, the ten signal invocations and the tracker configuration recorded, the criterion, row and evidence item reworded to match the five live-spec matches, and `branch` pointed at the implementation branch.
- 2026-11-10 16:57 UTC - AI agent (Claude Sonnet 5.5) - Removed the statements that the maintainer published `torrust-server-lib` 0.6.0 from the gate item, the scope note and the evidence, because the evidence only shows that 0.6.0 is published on crates.io.
- 2026-11-10 17:27 UTC - AI agent (Claude Sonnet 5.5) - Addressed the third review of PR 2807 (F17 to F19): the evidence cites the merge of the `torrust-server-lib` PR and describes the verification configuration as a reduced copy, and the second-round entry above is stamped at its commit minute.
- 2026-11-11 07:17 UTC - AI agent (Claude Sonnet 5.5) - Archived the spec: the GitHub issue is CLOSED (closed by PR 2807, merged 2026-11-11 with `torrust-server-lib` 0.6.0), so the spec moved from `docs/issues/open/` to `docs/issues/closed/`, `status` is `done`, and EPIC row 19 is Done. No retrospective was needed: the change is a removal with no design surprise. Left for follow-ups: the now-unused public error variants (`Error::Stop`, `StartupNotification` and `StartupNotificationDropped` of the HTTP and REST servers, and three `UdpError` variants) and the supervisor consolidation split out of scope. A flaky `axum-health-check-api-server` test failed once in a container build and passed on rerun. The completion review was recorded after the issue closed (PR 2807 merged at 07:08 UTC), so the order the checklist asks for was not kept. Follow-ups are tracked in the drafts `docs/issues/drafts/1488-si-25-remove-unused-error-variants/` and `docs/issues/drafts/1488-si-26-consolidate-shutdown-supervisor/`.

## Design and Ownership Review

Not applicable: this task only removes deprecated entry points and their tests. It
adds no task, process, readiness wait or resource owner. Removal Constraint 3
requires each token-aware lifecycle path to keep owning and joining its children;
the deterministic tests in the Verification Plan prove it.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets` warning-free, with no `#[allow(deprecated)]` left
- `cargo test --workspace` and the focused tests of the changed packages
- Pre-push checks before the implementation push

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Executable-boundary verification follows the EPIC protocol (signal each direct PID, bounded wait, exit status, bindings released).

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Start-gate evidence | Link SI-18 evidence, the migration issues, #1588, Q5, the approval and the 0.6.0 release. | Every gate item has a link or command output. | DONE | `manual-verification-evidence.md` |
| M2 | Legacy symbols removed | `git grep` for `Halted`, `shutdown_signal`, `global_shutdown_signal`, `HaltSignal`, `NestedServerTask`, `run_with_graceful_shutdown`, `spawn_launcher`, `LaunchRequest` and `tokio::signal` outside executables. | No use remains; descriptions match only in archived history and the live specs the evidence lists. | DONE | `manual-verification-evidence.md` |
| M3 | Token lifecycle owns completion | Run the component tests that cancel an injected token. | Owned children joined; no OS signal needed. | DONE | `manual-verification-evidence.md` |
| M4 | Tracker and standalone binaries | SIGINT and SIGTERM to the tracker and both standalone examples. | One application-owned shutdown sequence each; bindings released. | DONE | `manual-verification-evidence.md` |
| M5 | Container / service manager | Stop the container per `docs/containers.md#stopping-the-tracker`. | Orderly shutdown within the Q4 deadline; result per Q3. | DONE | `manual-verification-evidence.md` |

Record every command result with its toolchain or runtime. If a scenario fails,
record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC | Status | Evidence |
| -- | ------ | -------- |
| Start-gate evidence complete and reviewed | DONE | M1, and the reviews of PR 2807 (eight rounds by da2ce7; the audit records rounds 1 to 7, and the final approval, review 5849873024, is on the forge) |
| Legacy API removed; `Started` remains available in `torrust-server-lib` 0.6.0 and is no longer used here | DONE | M2 |
| No OS-signal subscription in server libraries | DONE | M2 |
| No use of the removed API remains; descriptions remain in archived docs and five lines of live specs | DONE | M2 |
| Binaries handle SIGINT/SIGTERM only at their boundaries | DONE | M4 |
| Deterministic token tests | DONE | M3 |
| End-to-end SIGINT/SIGTERM, one sequence | DONE | M4 |
| Container and service-manager verification | DONE | M5 |
| `linter all` passes | DONE | `linter all` run on 2026-11-10 after the evidence edit (below) |

## Risks and Trade-offs

- A breaking removal: external consumers of `torrust-server-lib` 0.5.x that did not
  migrate stop compiling against 0.6.0. The 0.5.0 changelog told them.
- Removing the legacy-only tests removes coverage of paths that no longer exist; the
  token lifecycle tests keep the coverage of the supported paths.
- The five package crates are unpublished, so their removal needs no support window
  (SI-18, Versioning Decisions); re-check if #2583 publishes before SI-19 lands.

## Implementation Completion Review

- Before closing #2801, review what the implementation discovered. If it found
  anything material, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add
  a progress-log entry saying why none was needed. The issue is not closed until
  one of the two exists.
- The independent reviews are the reviews of PR 2807 (eight rounds by da2ce7 and a Copilot summary; the audit records the summary and rounds 1 to 7, the eighth being the final approval after the last commit), recorded in `docs/pr-reviews/pr-2807-review/PR-REVIEW.md`; the audit of the spec PR 2803 is `docs/pr-reviews/pr-2803-review/PR-REVIEW.md`.
- Retrospective: none needed. The change removes deprecated symbols and their tests with no design surprise. The discoveries are recorded in the progress log: a flaky health-check test that passes on rerun, and unused public error variants left in place.

## References

- Parent EPIC: #1488 (roadmap row 19)
- Related issues: #2751 (SI-18), #1588, #2482, #2583
- External: torrust/torrust-server-lib#5 and PR 6 (0.6.0 removal)
- Q4 and Q5: `docs/features/shutdown-process/questions.md`

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
