---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p1
epic: 2410
github-issue: 2413
spec-path: docs/issues/open/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md
branch: "2410-process-queued-events-before-listeners-stop-spec"
related-pr: null
last-updated-utc: "2026-10-02 17:28"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - docs/application-jobs.md
    - docs/features/shutdown-process/task-inventory.md
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
---

<!-- skill-link: create-issue -->

# Issue #2413 - Document Event Flows and Draft the Shutdown-Order ADR

Parent: [EPIC #2410 - Process queued events before event listeners stop](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md)
(`epic` is set to the sub-EPIC's issue number once it exists),
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
  it in `docs/features/shutdown-process/task-inventory.md`.
- T2: move `docs/application-jobs.md` to `docs/architecture/` and update all
  links; create `docs/architecture/glossary.md` (D12); write the ADR draft as
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
| T1 | TODO | Producer-consumer inventory | Every producer, consumer, and listener chain recorded; differences from the design record reported to the maintainer (D14). |
| T2a | TODO | Move the jobs doc | `docs/architecture/application-jobs.md`; every link updated; `linter lychee` passes. |
| T2b | TODO | Glossary | `docs/architecture/glossary.md` from the design record's glossary, merged with the jobs doc's "Terms" (one definition per term). |
| T2c | TODO | ADR draft | `adr-draft.md` in the sub-EPIC folder, `create-adr` structure, covering D1, D7, D8, D9, D13 and the rules in the design record's Architectural Decisions; written to fully supersede the cancellation-tree ADR (D11). |

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
      component and the buses between them, checked against the code. It
      also rechecks the facts of the sub-EPIC's record entry 27 (who owns
      each sender and when it is dropped, where each receiver is created,
      and that no producer sends from a detached task) and reports any
      difference to the maintainer.
- [ ] AC2 (sub-EPIC AC9): the glossary is at `docs/architecture/glossary.md`,
      the jobs doc at `docs/architecture/application-jobs.md`, all links to
      the moved file are updated, and the sub-EPIC links to the glossary.
- [ ] AC3: `adr-draft.md` exists in the sub-EPIC folder and the maintainer
      has approved it.
- [ ] `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `linter all` (lychee covers the moved file's links).

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status |
| --- | --- | --- | --- | --- |
| M1 | Inventory against the code | For each bus, search for `send`/`publish` calls and listener `subscribe` calls; compare with the inventory | No unlisted producer or consumer | TODO |

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

### Progress Log

- 2026-10-02 11:03 UTC - GitHub Copilot - Drafted from SI-22 T1-T2 (D17).
- 2026-10-02 13:30 UTC - GitHub Copilot - AC1 now rechecks the channel-lifetime facts of the sub-EPIC's record entry 27.
