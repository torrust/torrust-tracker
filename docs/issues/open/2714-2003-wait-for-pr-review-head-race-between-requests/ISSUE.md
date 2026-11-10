---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p3
epic: 2003
github-issue: 2714
spec-path: docs/issues/open/2714-2003-wait-for-pr-review-head-race-between-requests/ISSUE.md
branch: "2714-wait-for-pr-review-head-race"
related-pr: null
last-updated-utc: "2026-11-10 14:30"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/snapshot.rs
    - .github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/event.rs
    - docs/issues/closed/2581-2003-pr-review-wait-poller/ISSUE.md
    - docs/issues/open/2714-2003-wait-for-pr-review-head-race-between-requests/manual-verification-evidence.md
---

<!-- skill-link: create-issue -->

# Issue #2714 - `wait-for-pr-review` can report a review of a newer commit than the head it reports

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make a `review` or `comment` result never claim a head that is older than the feedback it reports, so the
review loop's stale-checkout check cannot pass on a checkout that does not have the commit the feedback is about.

## Background

`wait-for-pr-review` (issue #2581) takes a snapshot of a pull request with four sequential REST calls, in this order
(`take_snapshot` in `.github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/snapshot.rs`, on `develop` at
`f7fecdf14`): the pull request (which carries the head commit), then its reviews, then its conversation
comments, then its inline comments. The calls are not an atomic snapshot of GitHub's state.

A result carries `head_sha` (the head read in the first call), and a `review` or `comment` result also carries
`head_moved`, which is true when that head differs from the commit the agent says it has checked out
(`--since-head-sha`). The skill's review loop uses exactly these two fields to decide whether the checkout is
stale before it processes feedback: "If `head_moved` is `true`, or `head_sha` differs from `git rev-parse HEAD`,
run `git fetch` and bring the branch up to date".

What happens, step by step, when a push and a review of that push arrive while a snapshot is being taken:

1. The poller reads the pull request and gets head A.
2. Someone pushes commit B, and a reviewer reviews B.
3. The poller reads the reviews and gets the review of B.
4. `detect` returns a `review` result whose `head_sha` is A and whose `review_commit_id` is B. The head the
   poller compared with the agent's checkout (A) matches, so `head_moved` is false.
5. The loop's check passes, and the agent processes feedback about commit B on a checkout of A.

The feedback is about code the agent does not have. The agent can write a fix against the wrong tree, reply to a
thread with a claim that its tree does not support, and only fail later, when its push is rejected as a
non-fast-forward. Nothing is lost in GitHub itself, and the next poll would report the head change, but the
review result has already been consumed.

This was reproduced with the real binary and a scripted `gh` that returns exactly the interleaving above; the
timing is scripted, not observed on GitHub. See `manual-verification-evidence.md`.

The problem is the order of the reads, not the fields: any result whose head was read before its feedback can
claim a head that is older than the feedback.

## Scope

### In Scope

- Make the head in a result at least as new as the feedback in it.
- Maintained regression tests at the snapshot and event level.
- Update the skill's description of `head_sha` and `head_moved` if their meaning changes.

### Out of Scope

- Making the four calls an atomic snapshot, which the REST API does not offer.
- The 15-minute wait, the cursor, or the ordering of events.

## Architectural Decisions

- Related ADRs: `None known`.
- ADRs to create: `None known`.

## Design and Ownership Review

Not applicable: no child process, I/O lifetime or resource ownership change. The question is the order of reads
in one function.

Design question for the maintainer (settled: **option 1**, see the decision below):

- **Option 1, read the head again after the feedback.** After the three feedback calls, read the pull request
  a second time and use that head (and state) for the result. The invariant becomes: the head in a result was read
  after the feedback in it, so it is at least as new as any commit that was reviewed or commented on before the
  feedback was read. A push after the feedback read makes `head_moved` true, which is a safe false alarm. Cost: one
  extra request per snapshot.
- **Option 2, compare the commit the feedback names.** Set `head_moved` when a review's `review_commit_id` (and an
  inline comment's `commit_id`, which the poller does not read today) differs from the checkout. No extra request,
  but conversation comments name no commit, so it covers only part of the feedback.
- **Option 3, both.** Option 1 for every result, and option 2 as a second check on reviews.

Recommendation: option 1. It is the smallest change, covers every kind of feedback, and states a single invariant.

### Decision

2026-11-10, maintainer (josecelano), asked in the implementation session: **option 1**, as recommended. The head
and state of a result are read after the three feedback calls. The implementation moves the one pull-request read
to the end of the snapshot instead of adding a second read, so there is no extra request per snapshot.

## Bug-Fix Process

Use [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md). The sequence:

- Analysis: the order of reads above; the hypothesis is that reading the head first is the whole cause.
- Real-artifact reproduction: done while drafting; see `manual-verification-evidence.md` section V1. Outcome:
  **Reproduced**, with a scripted `gh` supplying the interleaving. A real race on GitHub was not observed and is
  not expected to be reproducible on demand.
- Regression test boundary and red run: T1.
- Production fix: T2.
- Green run and like-for-like recheck of V1, after the last production change: T3.

## Regression Test Strategy

The smallest deterministic maintained boundary is a unit test of `take_snapshot` and `wait` with the existing
scripted `FakeGh` (`snapshot.rs`, `wait.rs`), which already lets a test queue different responses for the same
endpoint on consecutive calls. The test queues head A for the first read of the pull request and a review of
commit B in the review list, and asserts that the result does not present head A as current for feedback about B:
with option 1, that `head_sha` is B and `head_moved` is true for a checkout of A. A second test covers a
conversation comment, which has no commit id, so it distinguishes option 1 from option 2. A third test keeps the
ordinary case green: a review of the head that was read, with `head_moved` false.

The tests must be run red against the unfixed code and green after. The integration-level V1 reproduction is the
like-for-like recheck, not the regression test.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                        | Notes / Expected Output                                                     |
| --- | ------ | --------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| T1  | DONE   | Add the three unit tests with the scripted `FakeGh`; run them red           | Red output recorded in `manual-verification-evidence.md`.                   |
| T2  | DONE   | Implement the chosen option                                                 | The head in a result is read after its feedback.                            |
| T3  | DONE   | Run the tests green, then repeat V1 on the final build                      | Green output and the recheck recorded, naming the commit subject it ran on. |
| T4  | DONE   | Update the skill's wording for `head_sha` and `head_moved` if it changed    | Wording only where the meaning changed.                                     |

## Commit Points

| Task   | Coherent change set                                   | Commit policy                                        |
| ------ | ----------------------------------------------------- | ---------------------------------------------------- |
| T1     | The failing tests (fix applied in the next commit)    | Commit with the fix if the repository requires green commits, otherwise commit red first and say so. |
| T2     | The new read order in the snapshot                    | Commit after focused validation and required review. |
| T3, T4 | Evidence and documentation                            | Commit after focused validation.                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2714-2003-wait-for-pr-review-head-race-between-requests/ISSUE.md`
- [x] Reproduction attempted and recorded before review (outcome: Reproduced, with scripted timing)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created (#2714) and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Final like-for-like recheck of V1 recorded after the last production change
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-04 12:20 UTC - OUT-OF-ORDER (the stamp is wrong, see the 13:07 correction below) - AI assistant (Copilot SDK in VS Code) - Drafted the specification and reproduced the
  defect on the real binary with a scripted `gh`: a review of commit `bbbb…` was returned with `head_sha`
  `aaaa…` and no `head_moved` - `manual-verification-evidence.md` V1.
- 2026-11-04 12:12 UTC - AI assistant (Copilot SDK in VS Code) - The maintainer approved the specification; GitHub issue #2714 created, labelled `Bug` and `Automation`, and linked as a sub-issue of EPIC #2003; the specification moved from `docs/issues/drafts/` to `docs/issues/open/` for a spec-only pull request.
- 2026-11-04 13:07 UTC - AI assistant (Copilot SDK in VS Code) - Correction: the drafting entry above (12:20) was stamped before the commands it
  describes ran. The reproduction and lint commands ran between 11:49 and 11:56 UTC, as the command output and
  `manual-verification-evidence.md` record, so the drafting stamp is wrong. The earlier entries are kept unchanged,
  and `last-updated-utc` now holds the time of this edit.
- 2026-11-04 13:41 UTC - AI assistant (Copilot SDK in VS Code) - The drafting entry is now marked `OUT-OF-ORDER`: the
  file order matches the order of the events, and only its stamp is wrong, as the 13:07 correction states. The
  entry text is unchanged.
- 2026-11-04 14:35 UTC - AI assistant (Copilot SDK in VS Code) - The reproduction script now reports head `bbbb…`
  for a pull-request read made after the second review-list response, so a fix that reads the head again after
  the feedback calls can pass M2. A rerun of the unfixed build at 14:33 UTC gave the same result as V1.
- 2026-11-10 12:00 UTC - AI assistant (Claude Code) - The maintainer chose option 1 in the implementation session.
- 2026-11-10 12:30 UTC - AI assistant (Claude Code) - Implemented option 1; tests red then green; V2 recorded.
- 2026-11-10 14:30 UTC - AI assistant (Claude Code) - Addressed the review of the pull request: spec risks, evidence, an ordinary-case wait test, comment-test trigger, wording.

## Acceptance Criteria

- [x] AC1: When the pull request head changes between the read of the head and the read of the feedback, a
      `review` or `comment` result does not report the older head as current: its `head_sha` is the newer head,
      and `head_moved` is true for a checkout of the older one.
- [x] AC2: The ordinary case is unchanged: feedback on the head that was read yields the same result as before.
- [x] AC3: The skill describes `head_sha` and `head_moved` as they behave after the fix.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] The final recheck of V1 is recorded in `manual-verification-evidence.md`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p wait-for-pr-review`
- `cargo clippy -p wait-for-pr-review --all-targets`
- Pre-push checks

### Manual Verification Scenarios

| ID | Scenario                                         | Human-oriented command/steps                                                                                                  | Expected Result                                                           | Status | Evidence                                     |
| -- | ------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1 | A review of a newer commit than the head (before) | `evidence/fake-gh-head-race.sh` as `gh` on `PATH`, `wait-for-pr-review --pr-number 7 --poll-interval-seconds 1 --timeout-seconds 30` | `head_sha` `aaaa…`, `review_commit_id` `bbbb…`, no `head_moved`          | DONE   | `manual-verification-evidence.md` section V1 |
| M2 | The same, after the fix                          | The same command on the final build                                                                                           | `head_sha` is `bbbb…` or the result is `head_moved: true`, per the chosen option | DONE   | `manual-verification-evidence.md` section V2 |

Every recorded command result names the toolchain that produced it.

### Disposable Verification Scripts

`evidence/fake-gh-head-race.sh` is a disposable reproduction script, not a maintained test. A scripted `gh` is the
only practical way to play a specific interleaving of the poller's sequential API calls against the real binary,
because a real race on GitHub cannot be produced on demand. It lives in this folder; the durable check is the unit
tests of T1, which use the existing `FakeGh`, and the implementer removes or archives the script with the issue.
It is shell, not Python.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | Unit tests in `snapshot.rs` and `wait.rs`; V2 |
| AC2   | DONE                   | Ordinary-case tests in `snapshot.rs` and `wait.rs`; existing tests green |
| AC3   | DONE                   | `SKILL.md` paragraph after the result table |

## Risks and Trade-offs

- Option 1, as implemented, adds no request: the one pull-request read moves to the end of the snapshot.
- Option 1 narrows the window but does not close it: a push between the feedback reads and the head read
  makes `head_moved` true for feedback that was about the older head. That is a safe false alarm: the loop
  synchronizes and reads the pull request again.
- The reproduction is scripted. The spec does not claim to have seen the race on GitHub.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a concise progress-log entry explaining why.

## References

- Related issues: #2581 (closed, implemented the poller), #2003 (parent EPIC)
