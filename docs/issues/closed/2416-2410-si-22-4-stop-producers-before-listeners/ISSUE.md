---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p1
epic: 2410
github-issue: 2416
spec-path: docs/issues/closed/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md
branch: "2416-stop-producers-before-listeners"
related-pr: 2623
last-updated-utc: "2026-10-24 15:36"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
    - create-adr
  related-artifacts:
    - docs/issues/closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
    - src/bootstrap/jobs/manager.rs
    - src/app.rs
    - packages/axum-http-server/src/testing/environment.rs
    - packages/udp-server/src/testing/environment.rs
    - docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
---

<!-- skill-link: create-issue -->

# Issue #2416 - Stop Event Producers Before Event Listeners

Parent: [EPIC #2410 - Process queued events before event listeners stop](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md),
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
- The HTTP and UDP test environments stop servers before listeners (SI-16
  and SI-17 are merged; the #2471 prerequisite is satisfied, merged in PR
  #2544 on 2026-10-16).
- Documentation: move the ADR draft into `docs/adrs/` and supersede the
  cancellation-tree ADR (D11, D15); move the diagrams (D16); update the jobs
  doc, glossary, shutdown feature document, and task inventory; add the swarm
  bus, its producers, and its three consumers to `docs/architecture/events.md`,
  which omits them (finding from #2413 T1).

### Out of Scope

- The listener drain (sub-issue 2, a prerequisite).
- Configurable deadlines (SI-20).

## Design and Ownership Review

See the sub-EPIC's
[Design and Ownership Review](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#design-and-ownership-review)
and [Chosen Design](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#chosen-design-d7-d8).
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
[Regression Test Strategy](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#regression-test-strategy):
a `JobManager` unit test with fake components (red while edges are stored
but not enforced), and option 2, the in-process application test.

## Implementation Plan

Detailed steps: sub-EPIC
[Implementation Steps](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#implementation-steps),
items 8 (T7) to 11 (T10).

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T7 | DONE | Reproduce window (b), then red stop-order test | Controlled late publication proves the lost effect; consumer must remain live while any of two producers runs; red output recorded (evidence V1). |
| T8 | DONE | Stop order | Edges enforced with waiting and cancellation logs; `EventFlows` with unit tests; `src/app.rs` wired; in-process application test. Design-review checkpoint. Implemented and reviewed (evidence V2, V3; record entry 31). |
| T9 | DONE | Test environments | HTTP and UDP environments stop servers first, keeping the sub-EPIC's [Test environments](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#test-environments) guarantees (join-all failure reporting; independent server and listener tokens, each with its own `DropGuard`, D20); for each environment, a test that an event from a request completed just before `stop()` is counted. |
| T10 | DONE | Documentation and verification | ADR moved and old ADR superseded; diagrams moved; `docs/architecture/events.md` gains the swarm bus, its producers, and its three consumers (#2413 T1 finding); docs updated; M1-M3; AC review; completion review for the sub-EPIC. Done: the ADR move, the diagrams and docs, M1-M3 (evidence V5), the pre-push checks (V6), the independent review and the retrospective, and the maintainer's decision to keep the disposable script. Left for the maintainer: the acceptance-criteria review. |

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

- [x] AC1 (sub-EPIC AC2): no listener component is cancelled before every
      event-producing component has completed or the shared deadline has
      expired.
- [x] AC2 (sub-EPIC AC5): the HTTP and UDP test environments stop producers
      before listeners.
- [x] AC3 (sub-EPIC AC7): the T0 scenarios, rerun after the fix, show every
  control completion persisted within budgets. Race evidence records
  counts and observed lag, leftovers, failures, and aborts; an unexplained
  deficit blocks acceptance. An abort may leave an in-progress outcome
  unknown, which cannot be counted using receiver length.
- [x] AC4 (sub-EPIC AC8): the ADR is in `docs/adrs/`, restates every
      still-valid point of the cancellation-tree ADR, and that ADR is marked
      `Superseded by` it.
- [x] AC5 (sub-EPIC AC10): the five diagrams describe the final
      implementation and live under `docs/architecture/`; the shutdown
      diagrams they replace are updated or removed.
- [x] AC6 (sub-EPIC AC13): `EventFlows` rejects a consumer registered after
  a producer of its bus (unit test). Bootstrap proves subscription before
  publication and retains bus ownership through listener join; individual
  sender clones may drop. The ADR states these rules and the drain rule,
  including shared-deadline abort exceptions verified by T8 (AC12).
- [x] `linter all` exits with code `0`; relevant tests pass.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace` and `cargo test --test lifecycle-signals`
- Pre-push checks before opening the PR

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status |
| --- | --- | --- | --- | --- |
| M1 | Stop-order logs | Run the tracker with HTTP and UDP trackers, SIGTERM during load | Logs show producers completing before listeners are cancelled | DONE (evidence V5) |
| M2 | Recheck `control` | `reproduce-lost-completions.sh control` with the T0 parameters | No loss | DONE (evidence V5: 500 of 500 persisted, twice) |
| M3 | Recheck `race` | `reproduce-lost-completions.sh race` with the T0 parameters | Counts and observed loss/failure signals recorded under AC3; no unexplained deficit | DONE (evidence V5: two runs, no deficit, no lag, no abort, no failure) |

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
- [x] Design-review checkpoint after T8 recorded (sub-EPIC record entry 31)
- [x] Implementation completed and acceptance criteria reviewed

### Progress Log

- 2026-10-02 11:03 UTC - GitHub Copilot - Drafted from SI-22 T7-T10 (D17).
- 2026-10-02 13:30 UTC - GitHub Copilot - Added AC6 (sub-EPIC AC13): listeners-first registration, retained bus ownership through join, and ADR coverage; the review qualifies clone drops and hard aborts.
- 2026-10-07 13:05 UTC - GitHub Copilot - Refreshed T9 with the sub-EPIC: keep SI-17's join-all failure reporting, cancel both tokens on drop, add the late-event test for UDP as well as HTTP, and wait for #2471 (SI-16 and SI-17 are merged).
- 2026-10-08 06:58 UTC - GitHub Copilot - Maintainer decision D20 (sub-EPIC): T9 uses independent server and listener tokens, each with its own `DropGuard`, mirroring `JobManager`; the parent-child token option is rejected.
- 2026-10-19 17:08 UTC - GitHub Copilot (Claude Opus 5.5) - Refreshed: `branch:` names the planned implementation branch `2416-stop-producers-before-listeners`; removed the obsolete note about setting `epic`; the #2471 prerequisite of T9 is satisfied (merged in PR #2544 on 2026-10-16); T10 also adds the swarm bus, its producers, and its three consumers to `docs/architecture/events.md` (finding from #2413 T1).
- 2026-10-21 17:07 UTC - GitHub Copilot (Claude Opus 5.5) - Started on branch `2416-stop-producers-before-listeners` (cut from `torrust/develop` at the merge of PR #2615). T7 done: the controlled late-publication test on the real swarm event bus and the stop-order `JobManager` test were red against a stub that stored the edges and did not enforce them (evidence V1; the real-binary `race` script is not claimed as the proof). The maintainer approved the `StopBefore` API (additive `spawn_with`, three `SpawnError` variants, completion tracked at the join in `wait_for_all`) at the design-review checkpoint before the edges were implemented. T8 implemented in the working tree: edges enforced in `cancel()` and `wait_for_all`, `EventFlows`/`Bus`, `src/app.rs` wired, the application-level and in-process application tests, and a recorded mutation per lifecycle clause (V2, V3). Awaiting the design-review checkpoint after T8.
- 2026-10-21 17:09 UTC - GitHub Copilot (Claude Opus 5.5) - T8 design-review checkpoint passed; the maintainer approved T8 and four additions (public `waiting_components()`, `ComponentId` naming its manager, the HTTP test helper and its `[[test]]` binary, a `BTreeMap` registry), recorded as sub-EPIC record entry 31. T7-T8 committed (signed) after `linter all`. The recommendation to deliver one PR (D17) stands; the maintainer did not ask for a split. T9 next.
- 2026-10-21 18:47 UTC - GitHub Copilot (Claude Opus 5.5) - T9 done and approved by the maintainer: HTTP and UDP environments stop the server first; the listeners guard moves into the join (sub-EPIC record entry 32); HTTP late-event test red then green, UDP late-event test is regression coverage (evidence V4). Committed (signed). T10 next.
- 2026-10-21 19:07 UTC - GitHub Copilot (Claude Opus 5.5) - T10 in progress: the ADR was promoted to `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md` and the cancellation-tree ADR superseded (separate signed commit); the diagrams and documents were updated (second signed commit); M1 to M3 were run and recorded (evidence V5; the race runs show no deficit and are not the proof of window (b)). The acceptance criteria are not ticked: they await the maintainer's acceptance review. The pre-push script exits 0 (evidence V6). Pending: the disposable script decision, the independent Task Reviewer report, the retrospective.
- 2026-10-21 19:23 UTC - GitHub Copilot (Claude Opus 5.5) - The independent Task Reviewer report (REVIEW PASSED WITH FINDINGS, no blocking finding) and the implementation retrospective are recorded; the should-fix findings (two stale evidence counts, the open retrospective) are addressed. Pending before push: checkpoint (c) with the maintainer, the disposable-script decision, and the push approval.
- 2026-10-22 07:05 UTC - GitHub Copilot (Claude Opus 5.5) - Checkpoint (c) answered by the maintainer: keep `reproduce-lost-completions.sh` in the sub-EPIC folder, document the draft-ADR process in the `create-adr` skill (done, its own commit, as D15 asked), and push and open the pull request. Acceptance ticking is still to be decided by the maintainer.
- 2026-10-22 17:08 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 1 of PR #2623 started: the requested reviewer approved the pushed head and left six findings; the audit is `docs/pr-reviews/pr-2623-review/PR-REVIEW.md`. F2 (a UDP request can be held in flight) is fixed with a new deterministic UDP late-event test and corrected records (evidence V7).
- 2026-10-22 17:12 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 1, finding F1: the T10 row said the independent review, the retrospective and the script decision were pending, which the entries of 2026-10-21 19:23 and 2026-10-22 07:05 had recorded as done; it now reads `DONE` and leaves only the maintainer's acceptance review. The matching row of the sub-EPIC plan reads `DONE` too.
- 2026-10-22 17:13 UTC - GitHub Copilot (Claude Opus 5.5) - `related-pr` records PR #2623.
- 2026-10-22 17:18 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 1 of PR #2623 processed: six findings fixed in six signed commits, pushed, replied to and resolved (zero unresolved threads); audit `docs/pr-reviews/pr-2623-review/PR-REVIEW.md`. The reviewer approved the earlier head and acknowledged it; the acceptance-criteria review is still the maintainer's.
- 2026-10-23 07:09 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 2 of PR #2623, finding F8: the AC2 cell of the Acceptance Verification table now cites V7 (the UDP deterministic red and green) next to V4.
- 2026-10-23 07:10 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 2 of PR #2623, finding F9: V7 now names the working-tree state it ran on and states the true reason for using mutation U-M1 (it reproduces the one-token order; the test was not run on the tree before the T9 change), correcting the earlier "does not have the wrapper hooks" claim.
- 2026-10-23 07:13 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 2 of PR #2623 processed: three nits (F7 to F9) fixed in three signed commits, pushed, replied to and resolved (zero unresolved threads); the reviewer's ACK covers the head after round 1. The audit records both rounds.
- 2026-10-23 15:34 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 3 of PR #2623, finding F11: the AC2 cell said "the answered-request regression tests", but only the UDP environment has one; it now names the UDP test in the singular.
- 2026-10-23 15:39 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 3 of PR #2623 processed: F10 (the branch conflicted with `develop` in `docs/adrs/index.md`) fixed by a signed merge commit of `develop` keeping both ADR rows, F11 by a signed commit; both pushed, replied to and resolved (zero unresolved threads), and the PR body corrected. The audit records rounds 1 to 3.
- 2026-10-23 16:29 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 4 of PR #2623 processed: F12 (the PR body still carried two corrected claims) fixed by editing the body, F13 (a commit count) and F14 (the recheck logs were only under the git-ignored `.tmp/`; now retained in `evidence/`, V8) by signed commits, and the branch updated with the latest `develop` by a clean signed merge commit at the maintainer's request; the three threads are replied to and resolved (zero unresolved). The audit records rounds 1 to 4.
- 2026-10-23 17:07 UTC - GitHub Copilot (Claude Opus 5.5) - Review round 5 of PR #2623 processed: two nits (F15, the audit attributed its own figure of 23 to a reviewer; F16, the PR body cited V1 to V7 although V8 exists) fixed by a signed commit and a PR body edit, replied to and resolved (zero unresolved threads). The audit records rounds 1 to 5.
- 2026-10-23 17:50 UTC - GitHub Copilot (model not verifiable from the session; the maintainer states Claude Sonnet 5.5) - Correction of the model stamps in this log: the entries of the #2416 work that name `Claude Opus 5.5` copied that name from earlier entries without checking it, and the maintainer states that the session ran Claude Sonnet 5.5. The earlier entries are not rewritten; read them as `model unverified`. Appended in the same round as review round 6 of PR #2623 (finding F17), recorded in the audit `docs/pr-reviews/pr-2623-review/PR-REVIEW.md`.
- 2026-10-23 17:54 UTC - GitHub Copilot (model not verifiable from the session; the maintainer states Claude Sonnet 5.5) - Review round 6 of PR #2623 processed (F17, the audit row is committed) and the PR review retrospective written at `docs/pr-reviews/pr-2623-review/review-retrospective.md` at the maintainer's request: 17 findings, 8 of them in the text of the fix to an earlier finding; the improvements are `PROPOSED` and none is applied here.
- 2026-10-24 08:13 UTC - GitHub Copilot (model not verifiable from the session; the maintainer states Claude Sonnet 5.5) - Review round 7 of PR #2623 processed: one nit (F18, a reviewer finding ID that should read N/A in the F17 audit entry) fixed, replied and resolved; the audit row is recorded.
- 2026-10-24 08:34 UTC - GitHub Copilot (model not verifiable from the session; the maintainer states Claude Sonnet 5.5) - Review round 8 of PR #2623 processed: one finding (F19, the F18 verification matched its own heading) fixed, replied and resolved; the audit row is recorded.
- 2026-10-24 08:57 UTC - GitHub Copilot (model not verifiable from the session; the maintainer states Claude Sonnet 5.5) - Review round 9 of PR #2623 processed: two nits (F20, a review time that was 08:29 and not 08:30; F21, a count of edits of a reply) fixed by an appended correction in the audit log, replied and resolved; the audit rows are recorded.
- 2026-10-24 12:20 UTC - GitHub Copilot (model not verifiable from the session; the maintainer states Claude Sonnet 5.5) - PR #2623 was merged on 2026-10-24 at 12:19 UTC after nine review rounds, and issue #2416 is closed on GitHub. The maintainer chose to count his review and merge as the acceptance review, so AC1 to AC6, the `linter all` criterion and the last workflow checkpoint are ticked here at his instruction, and the evidence rows read ACCEPTED, as for #2414 and #2415. Qualifications that stand from the evidence: window (b) is proven by the controlled red and green tests (V1, V7), not by the real-binary race script (V5, a recheck), and the unstated limits are in the Implementation Completion Review. The spec is archived to `docs/issues/closed/`; the review audit and retrospective are in `docs/pr-reviews/pr-2623-review/`.

## Architectural Decisions

This issue owns D11/D15's final ADR promotion and supersession. Follow the
sub-EPIC's Architectural Decisions section, including the corrected delivery
exceptions; do not edit away the superseded ADR's original decision text.

### Acceptance Verification

| AC ID | Status | Evidence |
| --- | --- | --- |
| AC1-AC2 | ACCEPTED | AC1: V1 (red) and V2 (green): the `JobManager` stop-order test and the real-bus late-publication test; the application test asserts every listener waits for exactly the components that publish on its buses; 16 manager tests, one mutation per lifecycle clause (V3). AC2: V4 and V7: HTTP and UDP each red then green (a request held in flight while `stop()` drains; the UDP one added in review round 1), the UDP answered-request regression test, the staged-join tests, and the mutations of V4. |
| AC3 | ACCEPTED | V5: M2 control, 500 of 500 persisted, twice; M3 race, two runs, answered minus persisted 0, drain counts, no lag, no abort, no failure. The race runs do not prove window (b) (V1 does); no exact per-event attribution is claimed. |
| AC4-AC5 | ACCEPTED | The ADR `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md` restates the twelve still-valid points of the cancellation-tree ADR (section 9); that ADR has `- Status: Superseded by`; `docs/adrs/index.md` lists the new ADR. The five diagrams are in `docs/architecture/events.md` and `docs/architecture/application-jobs.md`; the shutdown diagrams they replace in the task inventory and the architecture examples are updated; `linter all` (including the local link and fragment check) exits 0. |
| AC6 | ACCEPTED | `EventFlows` unit tests (a consumer after a producer is rejected, the consumers of several buses are merged, only started listeners returned); the bootstrap subscription test (the swarm bus delivers to three receivers before any listener task can run, mutation: subscribe inside the task); bus retention: the extended application test sees every listener finish `Cancelled`, not `Completed`, while the container is alive, and `main` holds `_app_container` past `wait_for_all` (read from source, not run); the shared-deadline abort of a consumer cancelled late, which ignores its token (V3, C8 and the busy-consumer test). The ADR states the rules (sections 4, 6, 7). |

## Implementation Completion Review

Follow the sub-EPIC's [Shared Delivery Gates](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md#shared-delivery-gates).
This issue also closes the shared design and EPIC acceptance review; record
what was verified here and link earlier children's evidence rather than
claiming their tests were rerun without results.

### What was verified in this issue

- Window (b) is closed by the stop order: the controlled red and green pair (V1, V2), the application
  wiring (V2, V3), the test environments (V4), and the real binary under load (V5 M1: each listener
  was cancelled after the last producer it waited for completed, and every answered completion was
  persisted).
- The documentation describes the implementation: the ADR, the glossary, the jobs doc, the events doc,
  the task inventory, and the feature documents (commits `docs(adrs)` and `docs(architecture)`).
- Automatic checks: `linter all`, the package and root tests named in the evidence, and the pre-push
  script (evidence V6: the pre-push script exits 0 and `cargo test --test lifecycle-signals` passes).

### What this issue did not re-verify

- The listener drain of #2414 and the per-component tokens of #2415 are verified by their own evidence
  ([#2414](../2414-2410-si-22-2-drain-listener-queues-on-shutdown/manual-verification-evidence.md),
  [#2415](../2415-2410-si-22-3-per-component-cancellation-tokens/manual-verification-evidence.md)),
  and the inventory and ADR draft of #2413 by
  [its evidence](../2413-2410-si-22-1-document-event-flows-and-draft-adr/manual-verification-evidence.md).
  Here only the real-binary recheck (V5) and the workspace test suite exercised them again.
- Neither stop order nor drain guarantees lossless persistence under overflow, a failing handler, a
  panic, or the shared deadline's abort (ADR, Consequences). The exit-result policy (a timeout or an
  abort exits 1) and the configurable deadlines belong to SI-20.
- The UDP environment's deterministic red proof of the late-event behavior came in review round 1 (V7); V4 first said it was not possible (corrected there).
