---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1488
github-issue: 2751
spec-path: docs/issues/open/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
branch: "2751-deprecate-legacy-shutdown-api-spec"
related-pr: null
last-updated-utc: "2026-11-09 11:01"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/signals.rs
    - packages/axum-health-check-api-server/src/server.rs
    - packages/udp-server/src/server/spawner.rs
    - src/bootstrap/jobs/manager.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/src/server/states.rs
    - docs/features/shutdown-process/README.md
    - docs/features/shutdown-process/questions.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/issues/closed/2234-1488-si-2-remove-global-shutdown-signal/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2751 - Deprecate Legacy Shutdown API (SI-18)

> **EPIC position**: Roadmap step 18. Compatibility-preserving deprecation after
> every supported in-workspace and standalone consumer uses token lifecycle APIs.

## Goal

Deprecate the legacy shutdown API without changing runtime behavior or removing
symbols. Mark `Halted`-based server start/stop entry points and library-level
OS-signal shutdown helpers as deprecated, point consumers to the token-aware
lifecycle API, and publish migration guidance with a removal release target.

This task must not remove `Halted`, `shutdown_signal`,
`shutdown_signal_with_message`, `global_shutdown_signal`, or legacy server
start/stop methods. Existing consumers must compile and retain their present
behavior after deprecation warnings are addressed or explicitly allowed.

## Eligibility Gate

Evidence checked on 2026-11-06 against `torrust/develop` at `da8bd15d1`:

- [x] HTTP tracker, REST API, health-check API, and UDP tracker application
      components use token-aware lifecycle paths (`src/bootstrap/jobs/*` call
      `start_with_cancellation`).
- [x] Standalone HTTP and UDP environments/examples use token-aware lifecycle
      paths (SI-16 #2412, SI-17 #2448).
- [x] REST API and health-check API test environments use token-aware
      lifecycle paths (SI-23 #2449, SI-24 #2450).
- [x] The task inventory confirms no supported in-workspace consumer depends on
      the legacy shutdown path. The remaining callers are unit tests that
      exercise the legacy path on purpose.
- [x] The external `torrust-server-lib` release notes identify external consumer
      migration guidance and a compatible deprecation version: 0.5.0
      (published 2026-11-06) deprecates the legacy symbols, and its
      `CHANGELOG.md` gives the migration steps and the support window (the
      items stay for all 0.5.x releases and are removed in 0.6.0). See
      [Split with the external crate](#split-with-the-external-crate).
- [x] Process-wrapper documentation follows Q5 (`questions.md` Q5 is resolved;
      the deployment contract is in `docs/containers.md#stopping-the-tracker`).

## Split with the External Crate

The maintainer decided on 2026-11-06 to deprecate in the library first. The
symbols below are defined in the external `torrust-server-lib` crate, so this
repository cannot deprecate them.

| Symbol (external, `torrust_server_lib::signals`) | Replacement                                      |
| ------------------------------------------------ | ------------------------------------------------ |
| `Halted`                                         | `CancellationToken` (injected by the caller)     |
| `shutdown_signal`                                | `cancellation_signal`                            |
| `shutdown_signal_with_message`                   | `cancellation_signal` plus a log line            |
| `global_shutdown_signal`                         | an OS-signal subscription in the executable only |

Order of work:

1. **Library release** (separate repository and issue): add `#[deprecated]`
   attributes naming the replacement and the removal release, a changelog and
   release notes, and publish a new version (the "compatible deprecation
   version"). `Started` and `cancellation_signal` stay.
2. **This issue**: bump `torrust-server-lib` in the workspace to that version,
   then deprecate the in-repo symbols listed under Scope.

Step 1 is [torrust-server-lib#3](https://github.com/torrust/torrust-server-lib/issues/3),
implemented by [torrust-server-lib PR 4](https://github.com/torrust/torrust-server-lib/pull/4) (merged): the
symbols are deprecated since 0.5.0 and the crate removes them in 0.6.0 (the maintainer's choice).
The crate's removal version is therefore 0.6.0.

`torrust-server-lib` 0.5.0 was published on 2026-11-06 (merge commit 0ad8ca0), so this blocker is cleared.

## Versioning Decisions

Decided by the maintainer on 2026-11-06, using the EPIC #1669 coordinator's facts
([#2583](../2583-1669-first-publish-of-workspace-crates/ISSUE.md) publishes each unpublished crate at its
current version at the end of #1669; the root `torrust-tracker` next releases as 4.0.0):

- Deprecate and then remove, even for the unpublished crates, to stay consistent with the EPIC and with the
  published `torrust-server-lib` deprecation.
- `since` for a package crate is the version at which the crate is first published (its `Cargo.toml` version at
  that time, normally 0.1.0); for the root `torrust-tracker` types `HaltSignal` and `NestedServerTask`,
  `since = "4.0.0"`. The removal release is worded as "the next breaking release of this package".
- Support window (maintainer decision of 2026-11-06, resolving a release-order cycle: #2583 publishes the
  unpublished crates only after EPIC #1488 closes, and SI-19 is part of that EPIC): the five unpublished
  package crates have no external consumers, so no public support window exists for them. SI-18 still deprecates
  their symbols as in-repo guidance, and SI-19 may remove them without waiting for a window. The window applies
  only to what is already public: `torrust-server-lib` (deprecated since 0.5.0, removed in 0.6.0), which is
  released independently of #2583 and of the tracker release. #2583 needs no change.
- End of the `torrust-server-lib` 0.5.x window (proposed from SI-19's existing start-gate item "a maintainer
  explicitly approves the breaking-release timing"; confirmed by the maintainer on 2026-11-09): there is no fixed
  date. The window ends when the maintainer approves publishing 0.6.0, and the maintainer publishes 0.6.0 from the
  `torrust-server-lib` repository as part of SI-19, after that approval. The SI-19 start gate therefore waits for
  the approval, not for the 0.6.0 publication, which SI-19 itself produces.
- `HaltSignal` and `NestedServerTask` are deprecated only, not deleted.

Derived from the tree, then confirmed by the maintainer on 2026-11-09 ("we do not need a support window"): the root
`HaltSignal` and `NestedServerTask` were added after the published 3.0.0 (commit `3ee28b723`, "adopt JoinSet
supervision"; `git grep` finds neither in tag `v3.0.0`), so they have never been in a published release and need
no support window either. Their deprecation is in-repo guidance, with `since = "4.0.0"` (the next root release, which the next tracker application release
publishes after the libraries; #2583 does not publish the root crate),
and SI-19 may remove them inside EPIC #1488, before that release. The maintainer's Option A decision named the
root types as inside the window; this conclusion narrows it with the same principle (a window only for symbols
that are already public).

Proposed by the author in answer to review finding F3 of PR 2752, not yet confirmed by the maintainer (it follows
from the two decisions above and the maintainer's Option A decision): the attribute and release-note text per
symbol group.

- `torrust-server-lib` symbols (already public): the library's own `since = "0.5.0"` and `CHANGELOG.md` apply;
  SI-18 adds nothing there beyond the dependency bump.
- Symbols of the five unpublished package crates, which SI-19 removes before their first publication: `since` is
  the crate's current `Cargo.toml` version, and the note names the replacement and says the symbol is removed by
  SI-19 (EPIC #1488) before the crate's first publication, so it is in-repo guidance only. No release note is
  written for them, because no release ever contains them; the only release note is the `torrust-server-lib` one.
- The root `HaltSignal` and `NestedServerTask` (their crate is already published at 3.0.0, but the symbols are not
  in it): `since = "4.0.0"`, and the note names the replacement and says the symbol is removed by SI-19 before the
  next root release. No release note is written for them either.
- If SI-19 slips past a package crate's first publication, or past the next root release, the wording becomes
  "removed in the next breaking release of this package" and a release note with a support window is then required
  for that crate.

Other decisions of 2026-11-06:

- The `torrust-server-lib` 0.5.0 dependency bump lands in the implementation PR, not the spec PR.
- Ordering: #2482 row 6 (`axum-rest-api-server`, #2745) is merged on `develop`; only row 7
  (`axum-health-check-api-server`, #2757) remains open. The implementation PR must be coordinated with the #2482
  coordinator for row 7, to avoid conflicts in `Cargo.toml` and test environment files.

## Scope

### In scope

- Add Rust `#[deprecated]` attributes and documentation to legacy public APIs
  defined in this repository:
  - `axum-http-server`: `HttpServer<Stopped>::start`, `start_with_health_check`,
    `Running.halt_task`, `Running::stop`, `Launcher::start` (with `rx_halt`).
  - `axum-rest-api-server`: `ApiServer::start`, `Running.halt_task`,
    `Running::new` (takes a halt sender), `Running::stop`, `Launcher::start` (with `rx_halt`).
  - `axum-health-check-api-server`: `server::start` (with `rx_halt`) and the
    `RunningServer` type that only it returns.
  - `udp-server`: `Server<Stopped>::start`, `Running` (the legacy state type, its
    `halt_task` field, its derived `Running::new` and `Running::stop`),
    `Launcher::run_with_graceful_shutdown`, `Spawner::spawn_launcher`, and
    `LaunchRequest` (the type and its `rx_halt` field).
  - `axum-server`: `graceful_shutdown`.
  - `src/bootstrap/jobs/manager.rs`: `HaltSignal`, `NestedServerTask` (public,
    used only by their own tests).
- Document the token-aware replacement API and required migration steps.
- Update in-workspace call sites that remain only in tests, examples, or
  compatibility checks to suppress/allow the warning locally with a rationale.
- Add compiler/test coverage proving legacy API consumers still compile.
- Write the deprecation text defined in Versioning Decisions. The only release note with a support
  window is the published `torrust-server-lib` one; SI-18 writes no release note for symbols removed before
  their first publication or before the next root release.

### Out of scope

- Removing legacy shutdown APIs or library-level OS-signal subscriptions.
- Changing any server shutdown behavior, signal routing, deadline, or exit-code
  policy.
- Migrating a component or standalone consumer not already migrated before this
  eligibility gate.
- Requiring unknown external consumers to upgrade immediately.

## Deprecation Requirements

1. A deprecation message must name the token-aware replacement and explain that
   executable entry points, not libraries, own OS-signal subscriptions.
2. The message must identify the planned removal: for symbols that will have been
   published, the next breaking release according to the package versioning policy; for
   symbols SI-19 removes before their first publication, SI-19 itself (see Versioning
   Decisions).
3. Legacy API behavior stays unchanged. Deprecation is source guidance, not a
   behavioral migration.
4. The `torrust-server-lib` release notes must state the support window and the evidence
   required before removal. Symbols removed before their first publication need no
   release note (see Versioning Decisions).
5. The final removal task must not proceed merely because in-workspace code has
   migrated; for the already-public symbols (see Versioning Decisions) it also requires the
   declared external compatibility period to end. Symbols of crates that have never been
   published need no such period.

## Acceptance Criteria

- [ ] All eligibility-gate evidence is present and linked from this issue's
      `manual-verification-evidence.md`.
- [ ] Each deprecated legacy public API identifies its token-aware replacement
      and removal release target.
- [ ] Deprecated APIs remain source-compatible and preserve runtime behavior.
- [ ] In-workspace compatibility coverage compiles representative legacy
      server-library consumers.
- [ ] No new production code introduces a legacy shutdown API dependency.
- [ ] The `torrust-server-lib` release notes document migration, support window, and final
      removal prerequisites, and the in-repo deprecation messages follow the attribute text
      defined in Versioning Decisions.
- [ ] The workspace depends on `torrust-server-lib` 0.5.0 and every use of its
      deprecated symbols is either migrated or carries a documented
      `#[allow(deprecated)]`.
- [ ] `linter all` passes.

## Dependencies

- SI-11 through SI-17, SI-23, and SI-24 are complete (verified 2026-11-06).
- `torrust-server-lib` 0.5.0 (published 2026-11-06), which deprecates its legacy
  signal API (see the split above).
- SI-2's additive server lifecycle API is released and documented.
- #1588 revalidates the final supported-consumer inventory.
- Q5's process-wrapper verification rule is reflected in release notes and
  removal planning.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md`
  (independent package versions; no deprecation policy) and
  `docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md`
  (the token lifecycle that replaces `Halted`).
- ADRs to create: none known. The versioning decisions above follow the ADR
  and the EPIC #1669 coordinator's facts. If a lasting deprecation policy is
  wanted, record it in an ADR at implementation time.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                           | Notes / Expected Output                                                                         |
| --- | ------ | -------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| T1  | TODO   | Bump the workspace dependency to `torrust-server-lib` 0.5.0 and list every use of its deprecated symbols.    | Build output of the warnings; coordinate ordering with the #2482 coordinator first.             |
| T2  | TODO   | Deprecate the legacy public APIs of the five server packages and the root `HaltSignal`/`NestedServerTask`.   | Each message names the replacement and uses the `since` and removal text defined in Versioning Decisions (SI-19 removes them before publication, not "next breaking release", unless SI-19 slips). Behavior unchanged. |
| T3  | TODO   | Allow the warning locally, with a rationale, at the remaining legacy call sites (tests, adapters).            | `cargo check --workspace --all-targets` warning-free; migrated paths compile with `-D deprecated`. |
| T4  | TODO   | Add compile-compat tests for representative legacy consumers.                                                 | Tests compile with `#[allow(deprecated)]` and pass; legacy behavior unchanged.                  |
| T5  | TODO   | Write migration guidance for the deprecated symbols.                                                          | Replacement table; Rust docs render the guidance. No release note (see Versioning Decisions).   |
| T6  | TODO   | Record verification evidence.                                                                                  | `manual-verification-evidence.md`: gate evidence, warning output, rendered docs review.         |

## Commit Points

| Task | Coherent change set                                  | Commit policy                                        |
| ---- | ---------------------------------------------------- | ---------------------------------------------------- |
| T1   | Dependency bump and its call-site notes              | Commit after focused validation and required review. |
| T2   | Deprecation attributes and messages per package      | Commit after focused validation and required review. |
| T3   | Local `#[allow(deprecated)]` with rationales         | Commit after focused validation and required review. |
| T4   | Compile-compat tests                                 | Commit after focused validation and required review. |
| T5   | Migration guidance                                   | Commit after focused validation and required review. |
| T6   | Verification evidence                                | Commit after the last production change.             |

Use the `write-unit-test` skill for every test increment, record the prose-first
Arrange-Act-Assert design review in task evidence, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md`, moved to `docs/issues/open/`
- [x] Spec reviewed and approved by user/maintainer (issue creation approved in the review chat, 2026-11-06)
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 09:51 UTC - AI agent (Claude Sonnet 5.5) - Checked the eligibility gate against `develop` (`da8bd15d1`): everything met except the external `torrust-server-lib` deprecation, which the maintainer chose to release first (torrust-server-lib#3, PR 4, 0.5.0 published the same day)
- 2026-11-06 13:32 UTC - AI agent (Claude Sonnet 5.5) - Recorded the maintainer's versioning decisions using the EPIC #1669 coordinator's facts, and added the template sections required by the `create-issue` skill
- 2026-11-06 16:36 UTC - AI agent (Claude Sonnet 5.5) - Resolved a reviewer-found release-order cycle (#2583 publishes after EPIC #1488 closes; SI-19 is inside it): the maintainer chose Option A, so the unpublished package crates have no support window and only `torrust-server-lib` has one; recorded it in SI-18 and the SI-19 draft
- 2026-11-06 18:11 UTC - AI agent (Claude Sonnet 5.5) - Narrowed the window further to `torrust-server-lib` only, because the root `HaltSignal` and `NestedServerTask` were never in a published release (derived from the tree; not yet confirmed by the maintainer)
- 2026-11-09 10:56 UTC - AI agent (Claude Sonnet 5.5) - Edited the spec for the 2026-11-09 review of PR 2752: marked the derived conclusion, listed the adapter types, defined the text for symbols removed before first publication and the end of the 0.5.x window, recorded the gate commands, and corrected paths and the #2482 status
- 2026-11-09 11:01 UTC - AI agent (Claude Sonnet 5.5) - Recorded the maintainer's confirmation (chat, 2026-11-09) that the root `HaltSignal` and `NestedServerTask` need no support window and that the 0.5.x window ends at the maintainer's approval of 0.6.0

## Rollback

Remove only the deprecation attributes, migration text, and release-note entry.
Because this task does not remove or alter legacy behavior, reverting it is
source-compatible and has no runtime impact. The dependency bump to
`torrust-server-lib` 0.5.0 can stay or be reverted independently.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets` with no new warnings
- `cargo test --doc --workspace` and the focused tests of the changed packages
- Pre-push checks before the implementation push

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Executable-boundary verification is non-applicable: this task changes source
annotations only and no supported runtime path. If the implementation changes a
runtime path, follow the EPIC executable-boundary protocol instead.

| ID  | Scenario                              | Human-oriented command/steps                                                                  | Expected Result                                                      | Status | Evidence                                          |
| --- | ------------------------------------- | --------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------ | ------------------------------------------------- |
| M1  | Gate evidence                         | Link the migration issues, the final task inventory, Q5 and the torrust-server-lib release.    | Every gate item has a link or command output.                        | DONE   | `manual-verification-evidence.md` section V1      |
| M2  | Legacy consumers still compile        | Compile a representative consumer of each deprecated API without `allow(deprecated)`.         | The expected deprecation warnings, naming replacement and removal.   | TODO   | `manual-verification-evidence.md` section V2      |
| M3  | Migrated paths have no legacy use     | `RUSTFLAGS="-D deprecated" cargo check --workspace --lib --bins` on migrated code.            | No deprecated-API error in production paths.                         | TODO   | `manual-verification-evidence.md` section V3      |
| M4  | Documentation review                  | `cargo doc` and read the deprecation text and migration guidance.                                  | Replacement and removal wording match Versioning Decisions.                         | TODO   | `manual-verification-evidence.md` section V4      |

Record every command result with its toolchain or runtime when it can affect
behavior. If a scenario fails, record the failure and diagnosis in the progress
log before proceeding.

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC ID                                  | Status | Evidence                               |
| -------------------------------------- | ------ | -------------------------------------- |
| Every criterion in Acceptance Criteria | TODO   | Filled in after implementation review. |

## Risks and Trade-offs

- #2482 row 7 (#2757) still edits one of the affected crates, `axum-health-check-api-server`; ordering is
  coordinated to avoid conflicts in `Cargo.toml` and test environment files. Row 6 has already landed.
- The five package crates are unpublished and #2583 publishes them only after
  EPIC #1488 closes, so SI-19 removes their legacy symbols before any public
  release and no support window applies to them (see Versioning Decisions).
  Their `since` values are the crate's `Cargo.toml` version at the time (for the root types `4.0.0`); re-check
  them when #2583 decides every version.
- Deprecating test-only call sites needs local `#[allow(deprecated)]` with a
  rationale, or the workspace stops building warning-free.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory; otherwise add a progress-log entry saying why none
  was needed.
- The independent reviewer records its result in `agent-review-reports.md`
  using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1488 (roadmap row 18)
- Related issues: #2234 (SI-2), #2449 (SI-23), #2450 (SI-24), #1588, EPIC #1669, #2583, #2482
- External: torrust-server-lib#3 and PR 4 (deprecation, 0.5.0)
- Q5: `docs/features/shutdown-process/questions.md`
