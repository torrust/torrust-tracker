---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p1
epic: 2410
github-issue: 2415
spec-path: docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/ISSUE.md
branch: "2415-per-component-cancellation-tokens"
related-pr: 2591
last-updated-utc: "2026-10-21 09:23"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - src/console/profiling.rs
    - docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/manual-verification-evidence.md
    - docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/agent-review-reports.md
    - docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/implementation-retrospective.md
---

<!-- skill-link: create-issue -->

# Issue #2415 - Give Each Application Component Its Own Cancellation Token

Parent: [EPIC #2410 - Process queued events before event listeners stop](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md),
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
| T6a | DONE | `reserve` and reservation-based `spawn` | Unit tests: a reservation's token is cancelled by `cancel()`; a dropped reservation is released. Seven lifecycle tests and a `compile_fail` doctest (evidence V1); design agreed at the T6a review (sub-EPIC record entry 30). |
| T6b | DONE | Convert `src/app.rs` | Every component uses only its reservation token; `.child_token()` calls on server sites removed; `udp_ban_cleanup` keeps the root token. A new app-level test guards AC1 (evidence V2); agreed at the T6-complete review. |
| T6c | DONE | Update direct `JobManager` users | Tests in `manager.rs`, `src/app.rs`, and `src/console/profiling.rs`. Evidence V2; agreed at the T6-complete review. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T6a-T6c | `JobManager` API and every caller | One compiling change set after unit tests, the full suite, and `cargo test --test lifecycle-signals` pass; split only with a compiling additive adapter. |

Sign every commit with GPG.

## Acceptance Criteria

- [x] AC1: every component spawned through `JobManager` uses only the token
      from its reservation.
- [x] AC2: shutdown behavior is unchanged: all existing tests pass, including
      `cargo test --test lifecycle-signals`.
- [x] AC3: the new `JobManager` API has unit tests.
- [x] AC4 (keeps sub-EPIC AC13): listeners are still registered before any
  producer in `start_jobs_with_manager`, their subscriptions exist before
  publication, and sufficient bus ownership is retained through their join.
  Dropping individual sender clones is allowed (sub-EPIC entry 27).
- [x] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace` and `cargo test --test lifecycle-signals`
- Pre-push checks before opening the PR

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status |
| --- | --- | --- | --- | --- |
| M1 | Shutdown unchanged | Run the tracker with all services, SIGTERM; compare shutdown logs with `develop` | Same components stop; exit code unchanged | DONE (SIGTERM and SIGINT, evidence V3) |

Not applicable: bug-fix process and regression tests (no behavior change).

## Architectural Decisions

Follow D7 and the issue-local ADR draft. This prepares ordering without
promoting the ADR; sub-issue 4 owns final supersession.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | DONE | Caller inventory and focused tests; evidence V2 (13 `reserve` sites, no `.child_token()`, the app-level test and its 14 mutations) |
| AC2 | DONE | Existing suite and manual baseline comparison; evidence V2, V3, and V4 (full stable workspace suite, `lifecycle-signals`, M1 against `develop`) |
| AC3 | DONE | Reservation lifecycle and cancellation tests; evidence V1 and V4 (seven lifecycle tests, the id-exhaustion test added in review, a `compile_fail` doctest, and their mutations) |
| AC4 | DONE | Bootstrap subscription/ownership check; evidence V2 and the Task Reviewer report: registration order in `start_jobs_with_manager` is unchanged, read from source, not run |

## Implementation Completion Review

Follow the sub-EPIC's [Shared Delivery Gates](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#shared-delivery-gates),
including issue-local manual evidence, acceptance re-review, retrospective
assessment, and independent Task Reviewer report.

- Task Reviewer report: [agent-review-reports.md](agent-review-reports.md).
- Retrospective: [implementation-retrospective.md](implementation-retrospective.md).

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted
- [x] Spec reviewed and approved by the maintainer
- [x] GitHub issue created and linked to the sub-EPIC
- [x] Design-review checkpoint recorded
- [x] Implementation completed and acceptance criteria reviewed

### Progress Log

- 2026-10-02 11:03 UTC - GitHub Copilot - Drafted from SI-22 T6 (D17).
- 2026-10-02 13:30 UTC - GitHub Copilot - Added AC4: the refactor keeps the listeners-first startup order and the bus lifetimes (sub-EPIC AC13).
- 2026-10-19 17:08 UTC - GitHub Copilot (Claude Opus 5.5) - Refreshed: `branch:` names the planned implementation branch `2415-per-component-cancellation-tokens`; removed the obsolete note about setting `epic`.
- 2026-10-20 20:40 UTC - GitHub Copilot (Claude Opus 5.5) - Started on branch `2415-per-component-cancellation-tokens` from `develop` after the #2414 archive merge. Recount in `src/app.rs`: 13 `spawn` sites, 14 `new_cancellation_token()` calls, 4 `.child_token()` calls, 1 `register_legacy`, and six test-side `JobManager::new()` sites (the sub-EPIC's T6 step says four); `src/bootstrap/jobs/manager.rs` has 10 `JobManager::new()`, `src/console/profiling.rs` one direct user, and `tests/common/workspace.rs` only calls `cancel` and `wait_for_all`.
- 2026-10-20 20:52 UTC - GitHub Copilot (Claude Opus 5.5) - T6a implemented in the working tree, not committed: `reserve`, reservation-based `spawn` returning `SpawnError::ForeignReservation` for a reservation from another manager, released dropped reservations, and `cancel()` cancelling every component token and the legacy root token. Seven new unit tests and a `compile_fail` doctest pass; six mutations are each caught (evidence V1). Existing callers use a temporary `spawn_unreserved` adapter until T6b. Stopped for the design-review checkpoint (D14). No design change.
- 2026-10-20 20:59 UTC - GitHub Copilot (Claude Opus 5.5) - T6a design review: the maintainer approved the API, the lifecycle rules, and the tests, and decided three details the Chosen Design left to T6, recorded as sub-EPIC record entry 30 with `adr-draft.md` section 3 updated: `SpawnError` is propagated by bootstrap as a new `app::Error::ComponentRegistration`; `new_cancellation_token()` is renamed `legacy_cancellation_token()`; and the work lands as two code commits, the API with the temporary `spawn_unreserved` adapter, then the caller conversion that removes it. T6a committed; T6b and T6c are next.
- 2026-10-20 21:16 UTC - GitHub Copilot (Claude Opus 5.5) - T6b and T6c done in the working tree, not committed: all 13 components reserve and spawn with their own token, the four `.child_token()` calls are gone, `udp_ban_cleanup` uses `legacy_cancellation_token()`, the adapter is removed, and bootstrap propagates `SpawnError` as `app::Error::ComponentRegistration`. Finding: no existing test guarded AC1 (a listener on an unrelated token still passed every test, aborted at the deadline); a new app-level test now does, and fails for each of 14 token mutations (evidence V2). The jobs doc, glossary, and task inventory describe the per-component tokens. M1 matches `develop` for SIGTERM and SIGINT (evidence V3). Full root suite and `lifecycle-signals` pass. Stopped for the T6-complete design review. No design change.
- 2026-10-21 07:02 UTC - GitHub Copilot (Claude Opus 5.5) - T6-complete design review: the maintainer approved T6b, T6c, the documentation updates, and the new app-level AC1 test (its prose-first specification was reviewed, then removed from the code). Committed as `refactor(bootstrap): [#2415] give every application component its own reservation token` and `docs(architecture): [#2415] describe per-component cancellation tokens`. Both design-review checkpoints (T6a, and T6 complete) are recorded, sub-EPIC record entry 30 holds the decisions. Next: Task Reviewer, retrospective, pre-push checks. No design change.
- 2026-10-21 07:20 UTC - GitHub Copilot (Claude Opus 5.5) - Independent Task Reviewer report recorded in `agent-review-reports.md`: REVIEW PASSED (with findings), AC1 to AC4 and `linter all` pass, no design deviation the maintainer had not agreed. Its should-fix findings are addressed: the removal of the duplicate test `it_should_report_cooperatively_cancelled_jobs` is recorded (S1); the release test now checks that a drop leaves other components uncancelled, proven by a mutation the old test missed (S2, evidence V4); the final full suite and an M1 recheck follow with the pre-push checks (S3). Sub-EPIC record entry 30 is restamped 20:53, the minute the maintainer's answer arrived (N1). `implementation-retrospective.md` added. No design change.
- 2026-10-21 07:22 UTC - GitHub Copilot (Claude Opus 5.5) - S3 closed: on the final code commit the full stable workspace suite passes (77 targets, 3337 passed) and an M1 recheck on a binary built from it matches `develop` (evidence V4). Next: the maintainer's decision on push and the pull request. No design change.
- 2026-10-21 07:25 UTC - GitHub Copilot (Claude Opus 5.5) - Maintainer decisions before push: push and open the pull request with `Closes #2415`, without rebasing onto the newer `develop` (its new commits touch none of this branch's files); the maintainer's review and merge of the pull request is the acceptance review, as for #2414, so AC1 to AC4 and the linter item are ticked after the merge, in the archive pull request. No design change.
- 2026-10-21 08:18 UTC - GitHub Copilot (Claude Opus 5.5) - Opened PR #2591. Copilot review round 1 (three findings) processed with the maintainer's approval, audit in `docs/pr-reviews/pr-2591-review/PR-REVIEW.md`: the component id counter no longer wraps in release builds and a test proves it (`review-finding:pr-2591-f1`; `review-finding:pr-2591-f2` is the same concern), and `mark_spawned` asserts its invariant (`review-finding:pr-2591-f3`). No design change.
- 2026-10-21 09:23 UTC - GitHub Copilot (Claude Sonnet 5.5) - PR #2591 merged on 2026-10-21 at 09:19 UTC after 17 passing checks (3 skipped: the two publish jobs and Docker E2E), and issue #2415 is closed on GitHub. The maintainer approved the head of the PR (`ACK` comment) and, asked how the acceptance criteria are reviewed, chose to count his review and merge as the acceptance review, so AC1 to AC4 and the `linter all` criterion are ticked here at his instruction, as for #2414. Qualifications: AC4 was read from source and not run: this issue adds no test of subscription before the first publication, which the sub-EPIC lists under its AC13 for #2416; and AC1 is proven by the application-level test and its mutations, because the existing suite could not detect a component on the wrong token. The three Copilot review threads of PR #2591 are resolved. The spec is archived to `docs/issues/closed/`, and the delivery gates are done: evidence V1 to V4, the Task Reviewer report, and the retrospective.
