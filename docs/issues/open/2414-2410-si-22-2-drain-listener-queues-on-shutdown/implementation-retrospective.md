---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md
    - docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/manual-verification-evidence.md
    - docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/agent-review-reports.md
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
---

# Implementation Retrospective — Drain Listener Queues on Shutdown

## Purpose

Record evidence-based process improvements discovered while implementing issue #2414 (SI-22.2). This is a
blameless review of the implementation approach; it does not replace acceptance-criteria verification.
Written before the pull request, as the sub-EPIC's Shared Delivery Gates require (#2413 did it after merge).

## Outcome

All seven event listeners now handle the events already queued for them when they are cancelled, within an
injected drain timeout, log what they did with one shared format, and return `Cancelled`. The shared policy
is `packages/events`: `Receiver::try_recv`, `len`, a default `is_empty`, `TryRecvError`, `drain`, and the
`log_drain_report!` macro. On the real binary, with the T0 parameters, the control run persists 500 of 500
completions (on `develop`: 467) and both race runs lose nothing (790 and 775 lost on `develop`); a drain
cut short logs a warning after 5 s with the counts (V10, V11). The regression tests of the bug were red before
the fix and green after it (V3, V4, V7), and every new test was shown to fail under a mutation of the code it
guards. The branch has 11 commits at the independent review and 3 code commits and the documentation after it.

The design record changed twice at the maintainer's agreement (entries 28 and 29): the default timeout is a
policy bound by a measurement and not derived from it, the shared log format is a macro, the drain yields
between events and tolerates a huge timeout, and the 5 s default is named in `packages/events`.

## What Went Well

1. **The measurement came before the default, and it overturned a premise.** D9 said to choose the default
   from the time to drain a full buffer. The `release` measurement showed 12 to 14 s on tmpfs and 158 to 188 s
   on NVMe (V6), more than the 10 s shared deadline, so no timeout could drain a full buffer. Finding that at
   T4b, not after the PR, let the maintainer decide the policy (D14) with data.
2. **Red-first with a control.** T3a and T3c were written, run red, and recorded before the fix, and a
   temporary uncancelled control persisted all three completions, so the red was caused by the cancellation
   and not by the setup (V4).
3. **Mutation testing found a gap and kept the evidence honest.** The branch where the timeout expires with
   nothing unread had no test until a mutation passed every test (V5). Mutations that did not compile were
   recorded as non-results each time, and not as "no test failed".
4. **The independent review found what my tests missed.** The Task Reviewer, before push, confirmed the
   structure and raised the cooperative-scheduling requirement and the unrecorded dependency edge. It also
   found the `Instant` overflow in `drain` (its finding N2), which I then confirmed with a scratch program.

## What Changed During Implementation

1. **A function cannot carry the `tracing` target.** The approved design was "one shared function formats the
   drain log per listener". A function taking the target gave `error[E0435]` (V7). The format is a macro
   expanded in each package (entry 28).
2. **The scripted test receivers popped eagerly.** `tokio::select!` creates the `recv()` future even when the
   cancellation branch wins, so an eager pop consumed the queued event before the drain, and T3a stayed red
   after the fix. A debug print of the drain report (`processed: 0`) showed it. The real receiver is lazy.
3. **Counts are events, not completions.** The persistent listener receives every swarm event and ignores all
   but `PeerDownloadCompleted`. The race run's 2,804 processed events and 6,556 unread events are not
   completions (entry 28).
4. **The yield broke the T3b drop-path test.** The drain now yields after each event. The test's helper
   published a probe on every turn, so a draining listener kept finding a new event until its timeout. The
   behavior is the policy and the observer was wrong (V11).
5. **The drain's public API had an edge case I did not look at.** `Instant::now() + timeout` panics for a huge
   `Duration`. SI-20 will make the timeout configurable, so it was a latent shutdown panic.

## Root Cause

1. **I designed tests from the task list and not from the spec's test notes.** The #2414 spec says the
   slow-handler test must drive time explicitly and that cooperative scheduling must be tested. My T4 test
   list came from the sub-EPIC's step 5, and the cooperative-scheduling note was missed until the reviewer
   cited its line. Nothing prompted me to re-read the notes at the start of each test increment.
2. **A new public function got no "what is the largest input" pass.** The tests covered the semantic cases
   (empty, closed, timeout, lag, slow handler) and none looked at the boundaries of the arguments. A
   `Duration` parameter that goes through clock arithmetic has a boundary.
3. **A fan-out change was applied six times before one reviewer looked.** T5 repeated one pattern over six
   packages. The pattern's gaps (the receiver doubles, the import placement, the assertion messages) were
   replicated, and the reviewer found them once, at the end.
4. **A dependency edge looked like a test-support detail.** Adding `torrust-tracker-events` to
   `axum-http-server` was made inside a test-environment change and recorded in a log line and the evidence,
   but not as a design change, so D14 was not triggered until the reviewer asked.
5. **Shared Git state.** Stashes belong to the repository, not to a worktree. A `git stash pop` merged
   already-committed lines a second time into a file, and a later pop conflicted; a stash of another agent's
   work was beside mine. Nothing was lost, but I only noticed the duplicate through a compile error.

## Improvements for Future Work

1. **Before each test increment, list the spec's test notes next to the task row** and tick each in the
   evidence. A candidate addition to the `write-unit-test` skill's Quick Checklist; no change is made here.
2. **For a new public function, record the boundary inputs of every argument** (zero, the maximum, empty) in the
   test inventory, and write the red test when one can fail. A candidate for the same checklist.
3. **Test doubles for an async trait must do their work when polled, not when called**, because `select!` and
   similar constructs create futures they never poll. A candidate note for the `write-unit-test` skill.
4. **When a pattern fans out across packages, review the first package and the pattern before the rest.** The
   maintainer approved the T5 test design once, which was right; a quick reviewer pass on the first package
   would have caught the import and message nits six times earlier.
5. **Treat a new dependency edge as a design change.** Record it in the Design Discussion Record when it is made,
   with the alternative (a re-export here).
6. **Do not use `git stash` in this repository's linked checkouts** while others exist; commit a work-in-progress
   commit or stage by blob instead. This session's evidence records the cost.

## Avoiding Overcorrection

- No rule that every function gets a boundary-value test; the improvement is to look at boundaries of new public
  functions and record the answer, because here one looked at once found a real bug.
- No new automation for dependency edges or mutation testing: the review and the evidence caught both here, and
  one occurrence each does not justify a checker.
- No ban on parameterized fan-out changes: one commit per package kept each tree testable alone and made the
  review easy to follow.
- The measurement was cheap (a temporary Rust test, a few runs) and worth it; that does not make a release
  measurement a requirement for every default.
- A throughput regression from the per-event yield was not measured (V11); the reason to expect none is an
  inference, so the claim is not made.

## Evidence

- [Issue specification](ISSUE.md): the tasks, the acceptance criteria, and the progress log
- [Manual verification evidence](manual-verification-evidence.md): V1 to V11
- [Agent review reports](agent-review-reports.md): the Task Reviewer report before push
- Sub-EPIC [`EPIC.md`](../2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md): Design Discussion Record
  entries 27 to 29, D9, D13, and D14
- The PR for issue #2414: the code commits cited by their Conventional Commit subjects in the evidence
