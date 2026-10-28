---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/manual-verification-evidence.md
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
---

# Implementation Retrospective — Configure Shutdown Policy and Deployment Contract (#2668)

## Purpose

Record evidence-based process improvements discovered while implementing issue #2668 (SI-20 of
EPIC #1488). This is a blameless review of the implementation approach; it does not replace the
acceptance verification, which stays in the specification and the evidence file.

## Outcome

The tracker reads four shutdown budgets from `[application.shutdown]` (25 s process deadline, 15 s
HTTP-family drain, 5 s UDP request drain, 5 s listener drain), rejects a configuration whose chain
`max(HTTP, UDP) + listener` is not below the deadline, injects each value into the component that
applies it, and exits `0` only after a complete stop and `1` otherwise. The deployment contract
(orchestrator waits at least the deadline plus 5 s) is documented and was demonstrated with Docker and
systemd. The evidence is in `manual-verification-evidence.md` (V1 to V9 and the per-task sections); the
decisions are in two root ADRs dated 2026-10-27 and the Q4 amendment.

## What Went Well

1. Reproducing the defect before review (V1) made the exit-code fix testable: the unmodified binary
   could not be driven into a failed shutdown, so the fault-injection patch was recorded honestly as
   "reproduced with a temporary patch", and the same patch was re-applied after the fix (V5).
2. Mutating each new test and restoring with `cmp` found real gaps, for example two surviving
   mutations in T2 and one in T4 (a listener site that discarded the helper's result), each closed with
   a better test.
3. Deciding the injection design with the maintainer before wiring it (D5) and re-evaluating cohesion
   and coupling afterwards found one worthwhile change (named accessors at 31 call sites).

## What Changed During Implementation

1. **The design changed after the first schema step (D5).** The first proposal kept the crate
   constants and added builders; the maintainer decided to inject the values top to bottom and change
   the signatures, because 3.0.0 is unpublished. That superseded an earlier plan and touched four
   server crates, their test environments and the bootstrap. Evidence: the T4 and T5 sections.
2. **The shared native-tracker fixture could not see a failed shutdown.** `NativeTracker::shutdown`
   ignored the exit status, so no lifecycle test could have failed on exit code `1`. With the new exit
   contract that was a hole; `shutdown()` now requires status `0` and a separate `wait_for_exit`
   returns the status. Evidence: the T6 section.
3. **A second `Duration` overflow.** The drain helper added the caller's `Duration` to the clock and
   panicked on an oversized value; it now clamps to one year (a test passes `Duration::MAX`). The
   independent review found that `JobManager::wait_for_all` has the same pattern and relies only on the
   3600 s configuration bound. It was left unchanged here and is a follow-up candidate.
4. **The chain rule is a bound, not a prediction.** The Q4 numbers (25/20/5) plus the 5 s listener
   stage give `20 + 5 = 25`, which is not below 25, so the default HTTP drain became 15 s. In V3 the
   stop took 2.0 s, not the 3 s the bound allows, because the listeners had nothing queued.
5. **The e2e Compose stacks stopped the tracker with Docker's 10 s default**, shorter than the new
   deadline; they now set `stop_grace_period: 35s`.
6. **The composition root has no maintained test for which key goes to which crate.** `H` and `U` are
   visible in the V2 and V9 logs only. The independent review recorded this as an open AC5 gap for the
   maintainer.
7. **Evidence and document edits slipped.** A documentation commit deleted a heading of the evidence
   file by mistake (found two steps later, while writing the next section), the evidence text became
   stale after the script was removed, and dependent tool calls were batched in parallel five times
   (each time the result was discarded and the step redone).

## Root Cause

1. The specification fixed the number of components and the per-crate constants but not how a value
   reaches its consumer, so the injection design was decided after the schema existed. Deciding it in
   the specification would have avoided the superseded proposal.
2. The fixture contract was never stated as "the test sees the process outcome"; the old fixture
   returned only the output, which was enough while every shutdown exited `0`.
3. Time-handling helpers were treated case by case; the same overflow pattern recurred in a sibling
   helper nobody had listed as in scope.
4. The acceptance criterion for `U` and `L` ("covered by a test that fails with the default") assumed a
   test seam at the composition root, but the composition root is wiring with no unit seam, and the
   executable boundary exposes only some of the values in its logs.

## Improvements for Future Work

1. When a specification introduces configuration, state in it how each value reaches its consumer
   (parameter, shared container, default constant) and who owns the default, before the schema step.
2. When an exit code or process outcome becomes a contract, review the shared test fixtures that
   observe the process for the same contract in the same issue.
3. After fixing an overflow or bound in one helper, search the workspace for the same pattern
   (`Instant::now() +` with a caller-supplied `Duration`) and list the sites in the issue.
4. Write the executable-boundary check for each configured value in the plan, and log the effective
   value of every budget at startup so that check is possible (the listener drain is not logged).
5. Run the evidence-file consistency pass (stale statements, stamps, counts) after the last
   structural change, not only after the last test change.

## Avoiding Overcorrection

The evidence does not justify a general configuration-injection framework, a trait for budgets, or a
repository-wide lint against `Instant::now() +`. The named accessors and the one clamp were enough.
It also does not justify blocking every future issue on an executable-boundary check for values that
leave no observable effect.

## Evidence

- `docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/ISSUE.md` (decisions D1 to D10, progress log).
- `docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/manual-verification-evidence.md`
  (V1 to V9, T2 to T7 and the D5 follow-up with their mutation proofs).
- `docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/agent-review-reports.md` (the independent review).
- The commits `test(lifecycle): add native-binary tests for the shutdown policy of #2668`,
  `fix(axum-server): bound the drain timeout so an oversized value cannot overflow the clock` and
  `refactor(configuration): read each shutdown budget through a named accessor`.
