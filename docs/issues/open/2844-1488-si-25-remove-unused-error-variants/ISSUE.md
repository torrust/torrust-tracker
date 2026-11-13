---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p3
epic: 1488
github-issue: 2844
spec-path: docs/issues/open/2844-1488-si-25-remove-unused-error-variants/ISSUE.md
branch: "2844-remove-unused-error-variants"
related-pr: 2858
last-updated-utc: "2026-11-13 22:30"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/udp-server/src/server/mod.rs
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - docs/issues/closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2844 - Remove the Unused Public Shutdown Error Variants (SI-25)

> **EPIC position**: Roadmap row 22, sub-issue SI-25 of [EPIC #1488](../1488-overhaul-tracker-shutdown/ISSUE.md).
> A follow-up of [SI-19](../../closed/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md).
> Implementation order 1 of 4 (SI-25, SI-26, SI-27, SI-28). Priority p3, but it must land before the
> first publish of the workspace crates (#2583, see Dependencies).

## Goal

SI-19 removed the legacy `Halted`-based shutdown API and the `Started` startup notification from
this workspace. It left public error variants that nothing constructs, because they had not been
deprecated and Removal Constraint 1 of SI-19 forbade widening the breaking surface. Remove them
now, while the crates that own them have never been published.

## Background

Checked on 2026-11-12 against `torrust/develop` at `388faf59a`. A search of the tree (`*.rs`) finds
each variant only at its definition (and in one doc comment); nothing constructs or matches it:

| Package (version, unpublished)         | Enum         | Variants to remove                                                          |
| -------------------------------------- | ------------ | --------------------------------------------------------------------------- |
| `axum-http-server` 0.1.0               | `Error`      | `StartupNotificationDropped`, `StartupNotification`, `Stop`                 |
| `axum-rest-api-server` 0.1.0           | `Error`      | `StartupNotificationDropped`, `StartupNotification`, `Stop`                 |
| `udp-server` 0.1.0                     | `UdpError`   | `StartupNotification`, `Launcher`, `FailedToStartOrStopServer`              |

All three enums are `#[non_exhaustive]`, so a downstream `match` already needs a wildcard arm;
removing a variant still breaks code that names it. The `udp-server` enum carries a doc comment
(`packages/udp-server/src/server/mod.rs`) that says the three variants stay "until the next
breaking cleanup"; it goes with them.

`Launcher` is also the name of three unrelated, live structs: one each in `axum-http-server` and
`axum-rest-api-server` (`server.rs`, holding the bind address and TLS config), and `udp-server`'s
own UDP server instance launcher (`packages/udp-server/src/server/launcher.rs`, used by
`states.rs`). A bare-name search for that one variant matches all three structs, so every check
below scopes it to `UdpError::Launcher` (the qualified construction form) or to the
`enum UdpError` definition in `packages/udp-server/src/server/mod.rs`, never to the bare
identifier `Launcher`.

`Stop` has the same problem in the other direction: a whole-word search for it also matches the
`Stopped` state structs and their aliases in `axum-http-server`/`axum-rest-api-server`
(`server.rs`), plus comments and skill steps across `*.rs` and `*.md` outside `docs/issues` (for
example "Stop the tracker"). Every check below scopes `Stop` to `Error::Stop` (the qualified
construction form) or to the `enum Error` definition in each package's `server.rs`, never to the
bare identifier `Stop`.

The health-check API server has no such variants (checked), so it is not touched.

## Scope

### In Scope

- Remove the nine variants listed above, their `#[error(...)]` messages, and the stale doc comment.
  Verify the `Launcher` removal against `UdpError::Launcher` and the `enum UdpError` definition, and
  the `Stop` removal against `Error::Stop` and each package's `enum Error` definition, never a
  bare-name search (see Background).
- Remove imports and dependencies that become unused (for example the `oneshot::error::RecvError`
  path), and keep `cargo check --workspace --all-targets` warning-free.
- Update documentation that mentions a removed variant (`git grep` over `docs/` and the package
  READMEs), except archived specs and history.
- Record the versioning decision for the three packages (see Architectural Decisions).

### Out of Scope

- Any other error variant, including variants that are only constructed on rare paths.
- Behavior changes: the removed variants are never constructed, so no runtime path changes.
- Removing the `Registration` or `Bind` variants, or reshaping the enums.
- Consolidating the supervisors (SI-26).

## Dependencies

- SI-19 (#2801) is done and merged; no other task blocks this one.
- Maintainer decision of 2026-11-12: SI-25, SI-26 and SI-27 must land before the first publish of
  the workspace crates (#2583), because this change breaks the public enums of crates that are not
  yet published. Once published, the same removal needs a breaking release. #2583 is blocked on
  EPIC #1488 as a whole.

## Architectural Decisions

- Related ADR: `docs/adrs/20260629000000_adopt_independent_package_versioning.md`.
- Versioning: the three packages are unpublished, so the removal needs no support window (SI-18,
  Versioning Decisions, and SI-19 Risks). The implementation records in the progress log whether it
  bumps their versions (the ADR decides how an unpublished crate versions its first release) and
  ships the change under the release notes of the first publish. Do not publish in this task.
- ADRs to create: none.

## Design and Ownership Review

Not applicable: the task removes dead enum variants. It adds no task, process, readiness wait or
resource owner.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE   | Re-run the variant search on the implementation base and record it. | A search for each of the nine variant names in `*.rs` and `*.md` outside `docs/issues` finds only the definitions; for `Launcher`, scope the search to `UdpError::Launcher` and the `enum UdpError` definition; for `Stop`, scope it to `Error::Stop` and each package's `enum Error` definition; never the bare identifier (see Background). |
| T2 | DONE   | Remove the `Error` variants of `axum-http-server` and `axum-rest-api-server`. | Warning-free build; unused imports gone. |
| T3 | DONE   | Remove the `UdpError` variants and the stale doc comment of `udp-server`. | Warning-free build. |
| T4 | DONE   | Update docs, record the versioning decision, and re-run the search. | A search for the nine variant names finds nothing outside archived specs and history; for `Launcher`, scope to `UdpError::Launcher` and the enum definition; for `Stop`, scope to `Error::Stop` and each package's enum definition; never the bare identifier. |
| T5 | DONE   | Verify and record evidence. | `manual-verification-evidence.md`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1   | None (search only)  | No commit; recorded in the evidence. |
| T2   | HTTP and REST variants | Commit after focused validation. |
| T3   | `udp-server` variants | Commit after focused validation. |
| T4   | Documentation and versioning note | Commit after the last production change. |
| T5   | Verification evidence | Commit after the last production change. |

Sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/`, moved to `docs/issues/open/`
- [x] Spec reviewed and approved by the maintainer (decisions of 2026-11-12, in chat)
- [x] GitHub issue created and issue number added to this spec (#2844)
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, pre-push checks)
- [x] Manual verification scenarios executed and recorded in `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-11 09:40 UTC - AI agent (Claude Sonnet 5.5) - Draft created as a follow-up of SI-19; registered as SI-25 of EPIC 1488 at 16:30 UTC.
- 2026-11-12 14:30 UTC - AI agent (Claude Sonnet 5.5) - Refined into a full spec on `torrust/develop` at `388faf59a`: verified no code constructs the nine variants, added the plan, verification and rollback, and recorded the maintainer's decisions of 2026-11-12 (SI-25 to SI-28 all open; SI-25, SI-26 and SI-27 land before #2583).
- 2026-11-13 22:30 UTC - AI agent (Claude Sonnet 5.5) - Implemented on `2844-remove-unused-error-variants`. Versioning decision (maintainer, in chat): no version bump; the three packages stay at 0.1.0 and the removal ships in the release notes of the first publish (#2583). No changelog or migration note elsewhere. Evidence in `manual-verification-evidence.md`.
- 2026-11-13 22:55 UTC - AI agent (Claude Sonnet 5.5) - PR #2858 review round 1: fixed the two stale `Error` doc comments, the PR title, `related-pr`, and the M2/M3 evidence (commands, stamps, six listeners).

## Acceptance Criteria

- [x] The nine variants and the stale `udp-server` doc comment are removed.
- [x] A search for each variant name in the tree finds no use outside archived specs and history; the
      `Launcher` check is scoped to `UdpError::Launcher` and the enum definition, not the bare identifier,
      because `axum-http-server`, `axum-rest-api-server` and `udp-server` itself each define an
      unrelated `Launcher` struct; the `Stop` check is scoped to `Error::Stop` and each package's
      enum definition, not the bare identifier, because it also matches the `Stopped` state structs,
      their aliases, and unrelated comments and skill steps.
- [x] `cargo check --workspace --all-targets` is warning-free and `cargo test --workspace` passes.
- [x] The versioning decision for the three packages is recorded.
- [x] `linter all` passes.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets` warning-free; `cargo test --workspace` and the focused tests of the three packages
- Pre-push checks before the implementation push

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

The change cannot alter runtime behavior, so one executable-boundary run is enough; it follows the EPIC protocol (signal the direct PID, bounded wait, exit status, bindings released).

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Variants are gone | `git grep` for seven of the nine variant names (`*.rs`, `*.md`); for `Launcher`, `git grep 'UdpError::Launcher'` plus a read of the `enum UdpError` definition, not a bare-name `git grep 'Launcher'`; for `Stop`, `git grep 'Error::Stop'` plus a read of each package's `enum Error` definition, not a bare-name `git grep -w 'Stop'` (see Background for the unrelated structs and identifiers). | No match outside archived specs and history. | DONE | `manual-verification-evidence.md` |
| M2 | Startup and shutdown unchanged | Start the tracker with HTTP, REST, UDP and health check enabled, send `SIGTERM` to the binary PID, then restart on the same ports. | Exit code 0 within the deadline; the restart binds the same ports. | DONE | `manual-verification-evidence.md` |
| M3 | Startup errors still reported | Start with the REST port already bound by another process. | The same bind error as before the change (`Bind`), exit code 1. | DONE | `manual-verification-evidence.md` |

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC | Status | Evidence |
| -- | ------ | -------- |
| Variants and doc comment removed | DONE | M1 |
| No use outside history | DONE | M1 |
| Build and tests pass | DONE | Automatic checks |
| Versioning decision recorded | DONE | Progress log |
| `linter all` passes | DONE | Automatic checks |

## Risks and Trade-offs

- A breaking change for any external code that names a removed variant. The packages are
  unpublished, so the only consumers are in this repository; the search proves there are none.
- Doing it after the first publish would need a breaking release; this task exists to avoid that.

## Implementation Completion Review

Before closing, review what the implementation discovered. If it found anything material, create
`implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this
directory; otherwise add a progress-log entry saying why none was needed.

## References

- Parent EPIC: #1488 (roadmap row 22)
- Related: #2801 (SI-19), #2583 (first publish), EPIC #1669

## Rollback

Revert the removal commit. The variants have no constructor, so restoring them changes no behavior.
After a publish, restoring a removed variant is a new minor release of the affected package.
