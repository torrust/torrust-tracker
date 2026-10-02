---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: 2410
github-issue: 2416
spec-path: docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md
branch: "2410-process-queued-events-before-listeners-stop-spec"
related-pr: null
last-updated-utc: "2026-10-02 17:28"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
    - create-adr
  related-artifacts:
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - packages/axum-http-server/src/testing/environment.rs
    - packages/udp-server/src/testing/environment.rs
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
---

<!-- skill-link: create-issue -->

# Issue #2416 - Stop Event Producers Before Event Listeners

Parent: [EPIC #2410 - Process queued events before event listeners stop](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md)
(`epic` is set to the sub-EPIC's issue number once it exists),
under EPIC #1488.

> Sub-issue 4 of 4 (SI-22 tasks T7 to T10). Closes loss window (b) and
> finishes the sub-EPIC's documentation.

## Goal

On shutdown, listener components are cancelled only after every component
that publishes to their buses has completed, or the shared deadline has
expired. The application and the HTTP and UDP test environments follow this
order, and the ADR and diagrams describe the final design.

## Bug in Plain Terms

Servers keep answering requests for a while after shutdown starts, and those
requests still publish events. Listeners are cancelled at the same moment as
the servers, so events published during that time have nobody left to
process them.

## Scope

### In Scope

- Stop-before edges in `JobManager`, enforced by `cancel()` and
  `wait_for_all` (D7).
- `EventFlows` and `Bus` in `src/bootstrap/`, wired in `src/app.rs` (Chosen
  Design).
- The HTTP and UDP test environments stop servers before listeners (needs
  SI-16 and SI-17 merged).
- Documentation: move the ADR draft into `docs/adrs/` and supersede the
  cancellation-tree ADR (D11, D15); move the diagrams (D16); update the jobs
  doc, glossary, shutdown feature document, and task inventory.

### Out of Scope

- The listener drain (sub-issue 2, a prerequisite).
- Configurable deadlines (SI-20).

## Design and Ownership Review

See the sub-EPIC's
[Design and Ownership Review](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#design-and-ownership-review)
and [Chosen Design](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#chosen-design-d7-d8).
Design-review checkpoint after T8.

## Bug-Fix Process

Follows `.github/skills/dev/debugging/fix-bug/SKILL.md`. Sub-EPIC T0 reproduced
shutdown loss but did not isolate window (b). Before fixing it, T7 captures an
event published after the consumer stopped using controlled producer completion
and the real event bus at the nearest observable boundary. Record that
reproduction and the maintained stop-order test's red output, then fix (T8,
T9), rerun both, and recheck the real-binary `race` scenario (M3).

## Regression Test Strategy

Sub-EPIC
[Regression Test Strategy](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#regression-test-strategy):
a `JobManager` unit test with fake components (red while edges are stored
but not enforced), and option 2, the in-process application test.

## Implementation Plan

Detailed steps: sub-EPIC
[Implementation Steps](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#implementation-steps),
items 8 (T7) to 11 (T10).

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T7 | TODO | Reproduce window (b), then red stop-order test | Controlled late publication proves the lost effect; consumer must remain live while any of two producers runs; red output recorded. |
| T8 | TODO | Stop order | Edges enforced with waiting and cancellation logs; `EventFlows` with unit tests; `src/app.rs` wired; in-process application test. Design-review checkpoint. |
| T9 | TODO | Test environments | HTTP and UDP environments stop servers first; test that an event from a request completed just before `stop()` is counted. |
| T10 | TODO | Documentation and verification | ADR moved and old ADR superseded; diagrams moved; docs updated; M1-M3; AC review; completion review for the sub-EPIC. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T7-T8 | Red test, then the stop order | Commit after focused validation and review; do not commit a failing test. |
| T9 | Test environments | Commit after package tests pass. |
| T10 | ADR move and supersession | Separate commit. |
| T10 | Diagrams and other docs | Commit after `linter all`. |

Use the `write-unit-test` skill and the prose-first Arrange-Act-Assert design
review for every test increment. Sign every commit with GPG.

T8 tests normal ordering, multiple producers, already-finished producers,
producer failure/panic, and the shared deadline with a handler in progress.
Completion alone before `cancel()` must not cancel consumers. On deadline,
join all aborted work and retain named outcomes even for consumers never
cooperatively cancelled. Test invalid/released/foreign IDs and late edges.
A bootstrap collaboration test verifies actual receiver subscription before
publication, not only registry order. T9 defines normal/failure/drop cleanup
with bounded waits and no detached listener on an early server error.

## Acceptance Criteria

- [ ] AC1 (sub-EPIC AC2): no listener component is cancelled before every
      event-producing component has completed or the shared deadline has
      expired.
- [ ] AC2 (sub-EPIC AC5): the HTTP and UDP test environments stop producers
      before listeners.
- [ ] AC3 (sub-EPIC AC7): the T0 scenarios, rerun after the fix, show every
  control completion persisted within budgets. Race evidence records
  counts and observed lag, leftovers, failures, and aborts; an unexplained
  deficit blocks acceptance. An abort may leave an in-progress outcome
  unknown, which cannot be counted using receiver length.
- [ ] AC4 (sub-EPIC AC8): the ADR is in `docs/adrs/`, restates every
      still-valid point of the cancellation-tree ADR, and that ADR is marked
      `Superseded by` it.
- [ ] AC5 (sub-EPIC AC10): the five diagrams describe the final
      implementation and live under `docs/architecture/`; the shutdown
      diagrams they replace are updated or removed.
- [ ] AC6 (sub-EPIC AC13): `EventFlows` rejects a consumer registered after
  a producer of its bus (unit test). Bootstrap proves subscription before
  publication and retains bus ownership through listener join; individual
  sender clones may drop. The ADR states these rules and the drain rule,
  including shared-deadline abort exceptions verified by T8 (AC12).
- [ ] `linter all` exits with code `0`; relevant tests pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace` and `cargo test --test lifecycle-signals`
- Pre-push checks before opening the PR

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status |
| --- | --- | --- | --- | --- |
| M1 | Stop-order logs | Run the tracker with HTTP and UDP trackers, SIGTERM during load | Logs show producers completing before listeners are cancelled | TODO |
| M2 | Recheck `control` | `reproduce-lost-completions.sh control` with the T0 parameters | No loss | TODO |
| M3 | Recheck `race` | `reproduce-lost-completions.sh race` with the T0 parameters | Counts and observed loss/failure signals recorded under AC3; no unexplained deficit | TODO |

Record evidence in an issue-local `manual-verification-evidence.md`. After
M3, remove the disposable script from the sub-EPIC folder unless the
maintainer decides to keep it.

## Risks and Trade-offs

- A missing `EventFlows` declaration silently reopens window (b); mitigated by
  the inventory from sub-issue 1 and the stop-order logs.
- Shutdown takes longer; bounded by the shared deadline.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted
- [x] Spec reviewed and approved by the maintainer
- [x] GitHub issue created and linked to the sub-EPIC
- [ ] Design-review checkpoint after T8 recorded
- [ ] Implementation completed and acceptance criteria reviewed

### Progress Log

- 2026-10-02 11:03 UTC - GitHub Copilot - Drafted from SI-22 T7-T10 (D17).
- 2026-10-02 13:30 UTC - GitHub Copilot - Added AC6 (sub-EPIC AC13): listeners-first registration, retained bus ownership through join, and ADR coverage; the review qualifies clone drops and hard aborts.

## Architectural Decisions

This issue owns D11/D15's final ADR promotion and supersession. Follow the
sub-EPIC's Architectural Decisions section, including the corrected delivery
exceptions; do not edit away the superseded ADR's original decision text.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1-AC2 | TODO | Ordering, failure/deadline, and environment tests |
| AC3 | TODO | T0 like-for-like rechecks and controlled window-(b) proof |
| AC4-AC5 | TODO | Final ADR, diagrams, and local link checks |
| AC6 | TODO | Registry and bootstrap subscription tests; in-flight abort test |

## Implementation Completion Review

Follow the sub-EPIC's [Shared Delivery Gates](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#shared-delivery-gates).
This issue also closes the shared design and EPIC acceptance review; record
what was verified here and link earlier children's evidence rather than
claiming their tests were rerun without results.
