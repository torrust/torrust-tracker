---
semantic-links:
  skill-links:
    - write-markdown-docs
    - write-unit-test
  related-artifacts:
    - docs/issues/open/1586-evaluate-job-manager-join-set/ISSUE.md
    - docs/issues/open/1586-evaluate-job-manager-join-set/verification.md
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
---

# Implementation Retrospective

## Historical Context

On 2026-07-20, the initial #1586 local specification was drafted and approved
as a spec-only change. Implementation was deliberately deferred until the
shutdown architecture for EPIC #1488 settled. The original nested-wrapper
proposal was removed in favor of direct task ownership. After the EPIC adopted
the supervised cancellation tree, the issue was re-scoped as roadmap sequence
2 and this folder-style specification superseded the flat draft.

## Outcome

Issue #1586 adopts direct named `JoinSet` ownership for application components.
Each component returns a cross-package `ComponentResult` outcome: completed,
cooperatively cancelled, or contextual failure. Panic and deadline abort remain
named supervisor outcomes. One deadline covers every direct component and the
narrow legacy compatibility registry concurrently.

## What Changed During Implementation

The approved design exposed two constraints that required explicit treatment.
First, `JoinSet` cannot adopt an existing `JoinHandle` without a wrapper task;
the torrent-cleanup, activity-metrics, and UDP ban-cleanup handles therefore
remain a narrow pre-spawned exception. This preserves SI-4/SI-5 cancellation
and starter behavior until those migrations change their APIs. Second, aborting
an outer component while it awaits a nested server handle would otherwise drop
the handle and detach the child. The component runners now use drop-safe
signal-and-abort ownership to preserve their child boundary during escalation.

## Alternatives Rejected

Keeping all handles in a manual vector loses direct `JoinSet` completion
observation. Wrapping pre-spawned handles to insert them into `JoinSet` was
rejected because aborting the wrapper can detach its real child. Flattening
nested server tasks into the manager would violate component ownership. A
per-job timeout was rejected because it would multiply the process shutdown
deadline.

## Improvements for Future Work

Future server runners must make nested-task cleanup cancellation-safe before
being registered as direct components. SI-4/SI-5 should remove the legacy
registry by returning unspawned component futures rather than broadening this
exception.

## Evidence

- [Issue specification](ISSUE.md)
- [Verification evidence](verification.md)
- [`JobManager`](../../../../src/bootstrap/jobs/manager.rs)
- Focused manager tests cover concurrent completion, failure, panic,
  cooperative cancellation, shared-deadline escalation, nested-child abort,
  and shutdown-controller abort.
- Server-launcher tests preserve asynchronous startup readiness and
  startup-failure behavior after moving direct component ownership to the
  manager.
- `src/AGENTS.md` was updated and re-reviewed for the direct `JoinSet` and
  legacy compatibility ownership model.
- [Verification evidence](verification.md) records the completed final command
  results, including the passing `linter all` gate.
