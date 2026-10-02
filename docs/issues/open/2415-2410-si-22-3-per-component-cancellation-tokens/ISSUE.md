---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p1
epic: 2410
github-issue: 2415
spec-path: docs/issues/open/2415-2410-si-22-3-per-component-cancellation-tokens/ISSUE.md
branch: "2410-process-queued-events-before-listeners-stop-spec"
related-pr: null
last-updated-utc: "2026-10-02 17:28"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - src/console/profiling.rs
---

<!-- skill-link: create-issue -->

# Issue #2415 - Give Each Application Component Its Own Cancellation Token

Parent: [EPIC #2410 - Process queued events before event listeners stop](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md)
(`epic` is set to the sub-EPIC's issue number once it exists),
under EPIC #1488.

> Sub-issue 3 of 4 (SI-22 task T6). A refactor with no behavior change; it
> prepares the stop order in sub-issue 4.

## Goal

Each top-level component registered with `JobManager` gets its own
cancellation token through a reservation, so a later change can cancel
components at different times. `cancel()` still cancels everything at once.

## Background

Today every component uses a child of one root token, so they can only be
cancelled together. The chosen design (D7) registers components in two steps:
`reserve(name)` returns a component id and that component's token, then
`spawn(reservation, runner, ...)` starts it. Doing this alone, with no
ordering, means a mistake shows up in the existing tests before any ordering
is added (sub-EPIC risk "Registration API change touches every component").

## Scope

### In Scope

- `JobManager::reserve`; `spawn` takes the reservation; dropped reservations
  are released; `register_legacy` keeps the root token.
- Convert every `spawn` site and token site in `src/app.rs` (13 and 14 when
  counted on 2026-10-01; recount before starting).
- Update the tests that build a `JobManager` directly.

### Out of Scope

- Stop-before edges and `EventFlows` (sub-issue 4).
- Any deliberate change to shutdown ordering policy or deadline values;
  identical scheduler timing is not a refactor guarantee.

## Design and Ownership Review

`JobManager` owns every component token; a component only uses its own.
Design-review checkpoint after this issue (sub-EPIC D14).

Preserve cancellation before registration and between reserve and spawn, as
well as idempotent `cancel()`. A dropped unspawned reservation must release
its state without cancelling unrelated components. A reservation cannot be
used by another manager or spawned twice. Test these lifecycle boundaries.

## Implementation Plan

Detailed steps: sub-EPIC
[Implementation Steps](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#implementation-steps),
item 7 (T6).

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T6a | TODO | `reserve` and reservation-based `spawn` | Unit tests: a reservation's token is cancelled by `cancel()`; a dropped reservation is released. |
| T6b | TODO | Convert `src/app.rs` | Every component uses only its reservation token; `.child_token()` calls on server sites removed; `udp_ban_cleanup` keeps the root token. |
| T6c | TODO | Update direct `JobManager` users | Tests in `manager.rs`, `src/app.rs`, and `src/console/profiling.rs`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T6a-T6c | `JobManager` API and every caller | One compiling change set after unit tests, the full suite, and `cargo test --test lifecycle-signals` pass; split only with a compiling additive adapter. |

Sign every commit with GPG.

## Acceptance Criteria

- [ ] AC1: every component spawned through `JobManager` uses only the token
      from its reservation.
- [ ] AC2: shutdown behavior is unchanged: all existing tests pass, including
      `cargo test --test lifecycle-signals`.
- [ ] AC3: the new `JobManager` API has unit tests.
- [ ] AC4 (keeps sub-EPIC AC13): listeners are still registered before any
  producer in `start_jobs_with_manager`, their subscriptions exist before
  publication, and sufficient bus ownership is retained through their join.
  Dropping individual sender clones is allowed (sub-EPIC entry 27).
- [ ] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace` and `cargo test --test lifecycle-signals`
- Pre-push checks before opening the PR

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status |
| --- | --- | --- | --- | --- |
| M1 | Shutdown unchanged | Run the tracker with all services, SIGTERM; compare shutdown logs with `develop` | Same components stop; exit code unchanged | TODO |

Not applicable: bug-fix process and regression tests (no behavior change).

## Architectural Decisions

Follow D7 and the issue-local ADR draft. This prepares ordering without
promoting the ADR; sub-issue 4 owns final supersession.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | TODO | Caller inventory and focused tests |
| AC2 | TODO | Existing suite and manual baseline comparison |
| AC3 | TODO | Reservation lifecycle and cancellation tests |
| AC4 | TODO | Bootstrap subscription/ownership check |

## Implementation Completion Review

Follow the sub-EPIC's [Shared Delivery Gates](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#shared-delivery-gates),
including issue-local manual evidence, acceptance re-review, retrospective
assessment, and independent Task Reviewer report.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted
- [x] Spec reviewed and approved by the maintainer
- [x] GitHub issue created and linked to the sub-EPIC
- [ ] Design-review checkpoint recorded
- [ ] Implementation completed and acceptance criteria reviewed

### Progress Log

- 2026-10-02 11:03 UTC - GitHub Copilot - Drafted from SI-22 T6 (D17).
- 2026-10-02 13:30 UTC - GitHub Copilot - Added AC4: the refactor keeps the listeners-first startup order and the bus lifetimes (sub-EPIC AC13).
