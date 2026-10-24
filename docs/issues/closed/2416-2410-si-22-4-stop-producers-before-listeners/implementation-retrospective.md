---
semantic-links:
  skill-links:
    - write-markdown-docs
    - write-unit-test
    - fix-bug
  related-artifacts:
    - docs/issues/closed/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md
    - docs/issues/closed/2416-2410-si-22-4-stop-producers-before-listeners/manual-verification-evidence.md
    - docs/issues/closed/2416-2410-si-22-4-stop-producers-before-listeners/agent-review-reports.md
    - docs/issues/closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
---

# Implementation Retrospective - Stop Event Producers Before Event Listeners

## Purpose

Record evidence-based process improvements found while implementing #2416 (SI-22.4, T7 to T10 of
sub-EPIC #2410). This is a blameless review of the implementation approach; it does not replace the
acceptance-criteria verification.

## Outcome

`JobManager::spawn_with` takes `StopBefore` edges, and `cancel()` and `wait_for_all` enforce them: a
listener is cancelled only after every component that publishes on its buses has completed (in any
state) or the shared deadline has aborted it. `EventFlows` derives the edges from per-bus
declarations in `src/app.rs`. The HTTP and UDP test environments stop the server before the listeners.
The ADR is promoted to `docs/adrs/`, the cancellation-tree ADR is superseded, and the diagrams and
documents describe the final implementation. Evidence: V1 to V6 and the Task Reviewer report.

## What Went Well

1. **The red proof came from controlled tests, not from the race script.** The coordinator had warned
   that the real binary would not show window (b) after #2414. Two deterministic tests (fake
   components, and the real swarm bus with controlled producers) were red against a stub that stored
   the edges and did not enforce them, and V5 says plainly that the race runs are a recheck.
2. **The API review came before the edges were built.** The checkpoint on `StopBefore`, the error
   variants and where completion is tracked meant T8 was written once; the later changes were small
   additions (record entries 31 and 32).
3. **A mutation per lifecycle clause found real gaps.** Two wiring mutations (a dropped bus in the HTTP
   and UDP declarations) survived the first application test because completion order depends on
   timing. A deterministic assertion on the declared stop order (`waiting_components()`) now kills all
   seven (V3).
4. **A probe found a deterministic HTTP red.** An announce whose headers are not terminated stays in
   flight while the server drains, so the late event is lost with one token and counted with two
   (V4). Holding the listeners guard inside the join function also covers an abandoned `stop()`.

## What Changed During Implementation

- The additions beyond the Chosen Design were agreed and recorded: `spawn_with` beside `spawn`,
  `waiting_components()`, `ComponentId` naming its manager, the HTTP test helper and test binary
  (entry 31), and the listeners guard moving into the join (entry 32).
- The UDP environment first had no deterministic red for the late event, because I wrote that a UDP
  request cannot be held in flight. The reviewer showed that the same crate already holds one
  (`HoldAcceptedRequestsSender`), and review round 1 added the test (evidence V7).

## Improvements for Future Work

1. **After writing a mutation script, check that each replacement was applied.** Two of my mutations
   were wrong: one replacement silently did not match and reported "no failing test", and another did
   not change behavior. Both were noticed only because the result was implausible. A `diff` against the
   original before running each mutation would have said so at once.
2. **When a test depends on completion order, assert the declared relation as well.** Timing-dependent
   order passed with a missing edge.
3. **Write counts into the evidence from the final run.** The reviewer found two stale counts (5 tests
   where 4 exist, 144 where 146 run).
4. **Ask before applying a detail beyond the design record, even in an uncommitted tree.** I applied the
   guard detail of entry 32 before asking. It stayed uncommitted and the maintainer agreed, but the rule
   says to agree first.
5. **A bind failure ("Address in use") followed a race run twice** (here and in #2414's V10), more than
   40 seconds after the previous run, with no socket visible in `ss`. I did not investigate it. A
   separate issue could, if the maintainer wants one; this retrospective only records that it recurs.

## Improvements From Review Round 1

1. **Before writing that a test cannot be made, search the crate for an existing hook.** I stopped at
   "a UDP request cannot be held in flight" without searching `packages/udp-server` for the holding
   sender its own shutdown tests use. A search for `Hold` or `release` would have found it in a minute
   (review-finding:pr-2623-f2).

## Avoiding Overcorrection

- No mutation-testing tool: recorded, targeted mutations found each gap; the problem was applying them
  carefully.
- No rule that every test needs a red proof: the UDP environment cannot have one, and the evidence says
  so instead of reaching for a weaker claim.
- No change to the `create-adr` skill is proposed here without the maintainer's approval (see the
  checkpoint).

## Evidence

- [Issue specification](ISSUE.md), [manual verification evidence](manual-verification-evidence.md)
  (V1 to V6), and [agent review reports](agent-review-reports.md).
- Sub-EPIC [`EPIC.md`](../../closed/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md): record
  entries 31 and 32.
- The code commits, cited by their Conventional Commit subjects:
  `fix(bootstrap): [#2416] stop event producers before event listeners` and
  `fix(axum-http-server,udp-server): [#2416] stop the server before the listeners in the test environments`.
