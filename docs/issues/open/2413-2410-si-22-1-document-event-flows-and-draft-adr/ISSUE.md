---
schema-version: 1
doc-type: issue
issue-type: task
status: in-review
priority: p1
epic: 2410
github-issue: 2413
spec-path: docs/issues/open/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md
branch: "2413-document-event-flows-and-draft-adr"
related-pr: null
last-updated-utc: "2026-10-19 11:29"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - docs/architecture/application-jobs.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
---

<!-- skill-link: create-issue -->

# Issue #2413 - Document Event Flows and Draft the Shutdown-Order ADR

Parent: [EPIC #2410 - Process queued events before event listeners stop](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md),
under EPIC #1488.

> Sub-issue 1 of 4 (SI-22 tasks T1 and T2). Documentation only; no code
> changes.

## Goal

Record who produces and who consumes each event bus, give the shutdown terms
one shared definition, and draft the ADR that the code sub-issues implement.

## Background

The sub-EPIC's design record explains the bug, the chosen design (D7, D8),
and the decisions this work writes down. This issue turns the parts that
outlive the fix into stable documentation before any code changes, so later
reviews can point to them.

## Scope

### In Scope

- T1: revalidate the producer-consumer inventory against the code and record
  it in `docs/features/shutdown-process/task-inventory.md`, including the
  listener chains the HTTP and UDP test environments run.
- T2: move `docs/application-jobs.md` to `docs/architecture/` and update all
  live links; create `docs/architecture/glossary.md` (D12); write the ADR draft as
  `adr-draft.md` in the sub-EPIC folder (D15).

### Out of Scope

- Moving the ADR into `docs/adrs/` and superseding the cancellation-tree ADR
  (sub-issue 4, D15).
- Moving the diagrams (sub-issue 4, D16).
- Any code change.

## Implementation Plan

Detailed steps: sub-EPIC
[Implementation Steps](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#implementation-steps),
items 2 (T1) and 3 (T2). Revalidate paths before starting.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Producer-consumer inventory | Every producer, consumer, and listener chain recorded, including the test environments' chains (see below); differences from the design record reported to the maintainer (D14). |
| T2a | DONE | Move the jobs doc | `docs/architecture/application-jobs.md`; every live link updated (see below); `linter lychee` passes. |
| T2b | DONE | Glossary | `docs/architecture/glossary.md` from the design record's glossary, merged with the jobs doc's "Terms" (one definition per term). The "Stop request" entry notes that dropping a `CancellationToken` does not cancel it, so an owner that can be dropped without `stop()` holds a `DropGuard` (#2459, #2471). |
| T2c | DONE | ADR draft | `adr-draft.md` in the sub-EPIC folder, `create-adr` structure, covering D1, D7, D8, D9, D13 and the rules in the design record's Architectural Decisions; written to fully supersede the cancellation-tree ADR (D11). |

**Test environments in the T1 inventory.** SI-16 and SI-17 gave both
migrated test environments their own listener chains, and T9 (#2416) changes
them, so the inventory records them:

- `packages/axum-http-server/src/testing/environment.rs`: one listener, http-core
  statistics.
- `packages/udp-server/src/testing/environment.rs`: three listeners, udp-core
  statistics, udp-server statistics, and banning.

Both subscribe before the server starts, so they already meet the
subscribe-before-publish rule (sub-EPIC AC13). The REST API and health-check
API environments (SI-23, SI-24) run no event listeners.

**Finding the links for T2a.** `lychee.toml` skips `docs/issues/closed/`, so
`linter lychee` does not find every link to the moved file. Search with
`rg 'application-jobs\.md'` instead. On 2026-10-07 the live references were
`docs/index.md`, `docs/architecture/README.md`,
`docs/architecture/tracker-instance-architecture.md`, and the SI-22 specs (the
sub-EPIC and this issue). Historical records keep the old path unchanged, as
the maintainer decided on 2026-10-07: the closed specs
`1453-1978-ip-bans-reset-interval-configurable` and
`2095-organize-runtime-architecture-documentation`, and the plain-text mention
in the PR #2126 review record.

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Inventory | Commit after `linter all`. |
| T2a | File move and link updates only | Separate commit, so the move is easy to review. |
| T2b | Glossary | Commit after `linter all`. |
| T2c | ADR draft | Commit after maintainer approval. |

Sign every commit with GPG.

## Acceptance Criteria

- [ ] AC1: The task inventory lists every event producer and consumer
      component and the buses between them, checked against the code,
      including the listener chains of the HTTP and UDP test environments. It
      also rechecks the facts of the sub-EPIC's record entry 27 (who owns
      each sender and when it is dropped, where each receiver is created,
      and that no producer sends from a detached task) and reports any
      difference to the maintainer.
- [ ] AC2 (sub-EPIC AC9): the glossary is at `docs/architecture/glossary.md`,
      the jobs doc at `docs/architecture/application-jobs.md`, every live link
      to the moved file is updated (historical records keep the old path), and
      the sub-EPIC links to the glossary.
- [ ] AC3: `adr-draft.md` exists in the sub-EPIC folder and the maintainer
      has approved it.
- [ ] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `linter all`. Its lychee step skips `docs/issues/closed/`, so also run
  `rg 'application-jobs\.md'` after the move and check that only the
  historical records listed in T2a still use the old path.

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status |
| --- | --- | --- | --- | --- |
| M1 | Inventory against the code | For each bus, search for `send`/`publish` calls and listener `subscribe` calls, including in the test environments; compare with the inventory | No unlisted producer or consumer | DONE |

Not applicable: bug-fix process and regression tests (documentation only).

## Architectural Decisions

Follow the sub-EPIC's D11/D15 and Architectural Decisions section: draft the
replacement ADR here, but sub-issue 4 promotes it and marks the old ADR
superseded. Distinguish current implementation from proposed invariants in
the glossary and moved jobs document; no runtime behavior changes here.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1 | TODO | Source-checked inventory and manual review record |
| AC2 | TODO | Moved documents, updated links, and lychee result |
| AC3 | TODO | ADR draft and maintainer approval |

## Implementation Completion Review

Follow the sub-EPIC's [Shared Delivery Gates](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#shared-delivery-gates),
including local manual evidence for the source review, acceptance re-review,
retrospective assessment, and independent Task Reviewer report. Required
pre-push checks still apply to this documentation-only PR.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted
- [x] Spec reviewed and approved by the maintainer
- [x] GitHub issue created and linked to the sub-EPIC
- [ ] Implementation completed and acceptance criteria reviewed
- [ ] Independent Task Reviewer report recorded in `agent-review-reports.md` (sub-EPIC Shared Delivery Gates)
- [ ] Retrospective recorded, or the reason none is needed (sub-EPIC Shared Delivery Gates)

### Progress Log

- 2026-10-02 11:03 UTC - GitHub Copilot - Drafted from SI-22 T1-T2 (D17).
- 2026-10-02 13:30 UTC - GitHub Copilot - AC1 now rechecks the channel-lifetime facts of the sub-EPIC's record entry 27.
- 2026-10-07 13:00 UTC - GitHub Copilot - Refreshed with lessons from SI-17 (#2448, #2459) and the archive PRs (#2457, #2465): T1 and AC1 cover the test environments' listener chains; T2a finds links with `rg` because lychee skips closed specs, and historical records keep the old path (maintainer decision); T2b records the `DropGuard` rule; `branch:` names the implementation branch.
- 2026-10-16 07:56 UTC - GitHub Copilot - T1 done: producer-consumer inventory added to `docs/features/shutdown-process/task-inventory.md` (read from source, not run). M1 source search recorded in the sub-EPIC's `manual-verification-evidence.md`. Findings for the coordinator: `docs/architecture/events.md` omits the swarm bus; entry 27 differs on torrent cleanup, which publishes on the swarm bus indirectly; tokio in `Cargo.lock` is 1.53.3, not 1.53.1.
- 2026-10-16 08:04 UTC - GitHub Copilot - T2a done: moved `docs/application-jobs.md` to `docs/architecture/application-jobs.md` in its own commit and updated the live links (`docs/index.md`, `docs/architecture/README.md`, `docs/architecture/tracker-instance-architecture.md`, this spec, and the sub-EPIC). The moved file has no relative links to fix. Historical records keep the old path: closed specs `1453-...` and `2095-...` and the PR #2126 review record.
- 2026-10-16 08:07 UTC - GitHub Copilot - T2b done: `docs/architecture/glossary.md` created (one definition per term, merged with the jobs doc "Terms", planned SI-22 terms marked **Planned**, no event topology copied from `events.md`). The jobs doc "Terms" section now links to the glossary. The glossary is linked from the architecture README, `docs/index.md`, and the sub-EPIC.
- 2026-10-16 08:27 UTC - GitHub Copilot - T2c done: `adr-draft.md` written in the sub-EPIC folder and approved by the maintainer before this commit. It restates the cancellation-tree ADR points that stay valid (D11) and cites the tokio 1.53.3 `try_recv` and `len` semantics (read from source, not run). The old ADR budgets (25 s, 20 s, 5 s) differ from the code (10 s, 90 s, 5 s); the draft records this and leaves the alignment to SI-20.
- 2026-10-16 10:19 UTC - GitHub Copilot - The first push was blocked: the pre-push tests of `clippy-allow-reasons` ran their fixture Git commands against the real repository because Git exports `GIT_DIR` to hooks. The damage (a stray test commit on this branch, a `feature` branch, `user.name` and `user.email` in `.git/config`) was reverted by hand, and nothing was pushed. Fixed in a separate commit (`fix(dev-tools): [#2413] isolate clippy-allow-reasons fixture Git commands from hook environment`); evidence is in `manual-verification-evidence.md` of the sub-EPIC. No separate issue exists for it.
- 2026-10-16 10:48 UTC - GitHub Copilot - Review round 1 of PR #2553: Copilot raised two findings (`review-finding:pr-2553-f1`, `review-finding:pr-2553-f2`), both valid and fixed in separate commits. F2 showed that the T1 inventory missed the UDP receive loop publish sites in `launcher.rs` (the search was for `.send(`); the M1 evidence now carries a dated correction. Audit: `docs/pr-reviews/pr-2553-review/PR-REVIEW.md`.
- 2026-10-16 11:33 UTC - GitHub Copilot - Review round 2 of PR #2553: da2ce7 approved with eleven non-blocking findings (`review-finding:pr-2553-f3` to `review-finding:pr-2553-f13`), all valid and fixed in separate commits. The #2413 evidence moved to `manual-verification-evidence.md` in this folder (the Task Reviewer report and retrospective gates are listed below as unchecked checkpoints), the ADR draft restates the old ADR consequences and the exit-result gap, and the inventory marks the conditional listeners. Audit: `docs/pr-reviews/pr-2553-review/PR-REVIEW.md`.
- 2026-10-16 11:57 UTC - GitHub Copilot - Review round 3 of PR #2553: da2ce7 approved the head after the round 2 fixes with four non-blocking findings (`review-finding:pr-2553-f14` to `review-finding:pr-2553-f17`), all valid and fixed in separate commits. The isolation evidence was rerun one step at a time on the final test file, M1 is `DONE`, and the fixture-environment test is now a subset check. Audit: `docs/pr-reviews/pr-2553-review/PR-REVIEW.md`.
- 2026-10-16 12:21 UTC - GitHub Copilot - Review round 4 of PR #2553: da2ce7 approved the head after the round 3 fixes with four Nit findings (`review-finding:pr-2553-f18` to `review-finding:pr-2553-f21`), all valid and fixed in separate commits. The fixes only correct statements in the isolation evidence and the audit that outran the record. Audit: `docs/pr-reviews/pr-2553-review/PR-REVIEW.md`.
- 2026-10-16 12:42 UTC - GitHub Copilot - Review round 5 of PR #2553: da2ce7 approved the head after the round 4 fixes with three findings (`review-finding:pr-2553-f22` to `review-finding:pr-2553-f24`), all valid and fixed in separate commits. The fixes only make statements in the audit and the evidence agree with what the commands and the timeline show. Audit: `docs/pr-reviews/pr-2553-review/PR-REVIEW.md`.
- 2026-10-16 15:41 UTC - GitHub Copilot (Claude Opus 5.5) - After da2ce7 approved PR #2553 with no finding, the maintainer asked for a PR review retrospective: `docs/pr-reviews/pr-2553-review/review-retrospective.md`. It records that 15 of the 24 findings were about the author's own records, and the maintainer's observation that the earlier entries here were written with Claude Sonnet 5.5. The evidence template, the issue and EPIC templates (model in progress logs), the review retrospective template and `process-pr-review` (version 1.6) were updated in their own commits.
- 2026-10-19 11:13 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 7 of PR #2553: da2ce7's re-review of the head after the retrospective (posted 2026-10-18) had three Nit findings (`review-finding:pr-2553-f25` to `review-finding:pr-2553-f27`), all valid and fixed in separate commits: the retrospective's audit-only count, the round count in the `process-pr-review` section, and a log time range. Audit: `docs/pr-reviews/pr-2553-review/PR-REVIEW.md`.
- 2026-10-19 11:29 UTC - GitHub Copilot (Claude Opus 5.5) - Renamed the 11:13 entry above from "round 6" to "round 7" (`review-finding:pr-2553-f28`): the retrospective's round table uses round 6 for the approval review 5548045897, so review 5562034258 is round 7.
