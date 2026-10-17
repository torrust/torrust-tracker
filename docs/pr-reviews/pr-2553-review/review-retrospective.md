---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/pr-reviews/pr-2553-review/PR-REVIEW.md
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/pr-reviews/pr-2484-review/review-retrospective.md
    - docs/issues/open/2503-2278-author-self-audit-gate/ISSUE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2553

Covers the six reviews up to da2ce7's approval 5548045897 of the head ending in
`docs(reviews): [#2413] record the PR 2553 review round 5 audit`, the first head he approved with
no finding. Written at the maintainer's request, because five of the six reviews
produced findings on a documentation-only PR. Any review of this retrospective is recorded in the
audit and not counted below.

## Purpose

Record a blameless, evidence-based review of how the pull request was reviewed and how its
findings were processed. This complements the audit record, which tracks what each finding was
and how it was resolved; the retrospective explains why the process cost what it did and what to
change. It does not replace the audit record and is not required for routine reviews.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 6 reviews (5 human, 1 bot); 5 produced findings |
| Audit findings | 24 |
| Re-raised findings (`RE_RAISE_OF`) | 0 |
| Findings about the audit record itself | 6 (F5, F19, F20, F22, F23, F24) |
| Findings about the other records (evidence, spec, stamps, a commit body) | 9 (F4, F7, F9, F11, F14, F15, F16, F18, F21) |
| Commits on the branch | 36 at the approved head (11 `docs(reviews)` audit-only) |
| First review to last thread resolution | 2026-10-16 10:41 UTC (Copilot) to about 12:41 UTC (round 5 threads resolved); approval with no finding at 12:54 UTC |
| AI models used | Claude Sonnet 5.5 (author: implementation and review processing, per the maintainer); unknown (da2ce7 and the Copilot review bot) |

Sources: `gh api repos/torrust/torrust-tracker/pulls/2553/reviews` for the reviews, their times
and heads; the Findings table and Finding Details of `PR-REVIEW.md` for counts and subjects;
`git rev-list --count` and the `docs(reviews)` subjects of `git log --format=%s`, both from
`torrust/develop` to that approved head, for commits. The author model comes from the
maintainer; the session was switched to Claude Opus 5.5 after the approval, and that model wrote
this retrospective.

## What Happened, Round by Round

| Round | Review | Findings | About the deliverable | About the author's records |
| --- | --- | --- | --- | --- |
| 1 (bot) | 5546625913 | F1, F2 | 2: glossary over-claim, inventory miss | 0 |
| 2 | 5547031766 | F3 to F13 | 6: ADR consequences, conditional listeners, tokio version, discard sites, exit code, test | 5: evidence location, audit severity, statuses, evidence time, stamp |
| 3 | 5547366758 | F14 to F17 | 1: test equality | 3: rerun order, M1 status, locator |
| 4 | 5547599741 | F18 to F21 | 0 | 4: clock reading, unnamed check, push time, commit body |
| 5 | 5547834689 | F22 to F24 | 0 | 3: check placement, push time again, dismissals |
| 6 | 5548045897 | none | 0 | 0 |

Rounds 3 to 5 were self-inflicted: 10 of their 11 findings were about text the author wrote while
fixing the previous round; the exception is F15, a status left at `TODO` from the start. From round
2 on, every fix push dismissed the approval just given, so each round cost a new review of the new
head.

## What Went Well

1. **Rounds 1 and 2 found real defects.** The inventory missed four UDP publish sites (F2), the
   glossary promised more than the server drains do (F1), and the ADR draft dropped the old ADR's
   Consequences while claiming to restate it (F3). These would have misled the implementation of
   the next sub-issues.
2. **The reviewer recomputes from the bytes and lists his checks.** Each review body names what was
   recomputed: stamps against commits, log times against the timeline, replies against threads,
   references against rows. That list is the check the author should run before pushing.
3. **The processing mechanics held.** Each finding had its own commit, every thread got a reply
   before resolution, the validator reported 0 failures before every audit commit, and there were no re-raises.
4. **Severity was honest.** Every human review approved with non-blocking findings and named the
   head, so the maintainer could have merged at any round.

## What Made the Review Costly

1. **The deliverable's own gaps (rounds 1 and 2).** The T1 inventory searched for `.send(` call
   text and so missed the receive loop, which publishes through a helper (F2). Statements claimed
   more than the text did: the glossary's drain entry (F1) and the ADR's "restates everything"
   (F3). A known wrong fact was reported to the coordinator but left in the design record (F8).
2. **Delivery requirements read after the PR opened.** The sub-EPIC's gates ask for issue-local
   evidence (F4), and the evidence template requires dependent steps to run one at a time (F14).
   Neither was read before the evidence was written.
3. **Records written from intent, not from output (rounds 3 to 5).** A log entry said "and pushed"
   before the push (F20); the evidence described an `equals HEAD` check without naming it (F19),
   and its placement was then stated three ways (F22); a clock was called non-monotonic on one
   stamp (F18); a commit body claimed a run on an older Git that never happened (F21).
4. **The same mistake twice.** F23 repeats F20 one round later: the round 4 log again stamped a
   push before it happened, because the author fixed the one entry F20 named without searching for
   the pattern.

## Root Causes

1. **No author sweep before the first push.** The `review-task` skill and the sub-EPIC's Shared
   Delivery Gates existed, but the PR was opened after `linter all` and the hooks only. Those
   checks cannot see wrong statements.
2. **Bookkeeping from memory.** Log entries, Solutions and replies were written while the fix was
   still in progress, then not re-derived from `git log`, the command output, or the GitHub
   timeline. The skill had no rule for this; issue #2503 plans one but had not landed.
3. **Each fix adds reviewable text.** The audit, evidence and log grow with every round, and the
   reviewer checks all of it against the bytes. Imprecise text therefore generates new rounds at
   the rate it is written.
4. **Search too narrow.** Both the inventory method (call text instead of sender fields) and the
   correction method (the named entry instead of every entry) looked for one form of a problem.

## Model Observation

The maintainer reports that, in other sessions, Claude Opus 5.5 needed far fewer review rounds
than this PR did with Claude Sonnet 5.5. No earlier session recorded its model, so the repository
has no data to test this, and nobody reran this PR with another model. The observation is recorded
here, not turned into a rule: every root cause above has a process fix that applies to any model,
and engineering policy 7 treats provider-specific choices as optional adapters, not repository
rules. From this PR on,
progress logs and review retrospectives record the model, so the comparison can be made later.

## What We Learnt

1. A reviewer who recomputes everything turns every imprecise sentence into a round; fewer, exact
   sentences cost less than more, careful ones.
2. Write a log entry about an event after the event, from the event's own time.
3. Name the command behind every state claim in evidence.
4. When a finding corrects one of your statements, look for the pattern, not the instance.
5. Read the gates and templates the spec cites before writing the evidence they govern.

## Improvements for Future Reviews

1. **Stamps and named commands for dependent evidence steps** — `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
   — `APPLIED_IN_THIS_PR` (`docs(templates): [#2413] require stamps and named commands for dependent evidence steps`).
2. **Record the AI model in progress logs and review retrospectives** — `docs/templates/ISSUE.md`,
   `docs/templates/EPIC.md`, `docs/templates/PR-REVIEW-RETROSPECTIVE.md`, `create-issue` skill —
   `APPLIED_IN_THIS_PR` (`docs(templates): [#2413] record the AI model in progress logs and review retrospectives`).
3. **Rules for statements in the author's own records** (stamp after the event, state only what
   ran, sweep for the same mistake) — `process-pr-review/SKILL.md`, now version 1.6 —
   `APPLIED_IN_THIS_PR` at the maintainer's request (`docs(skills): [#2413] add rules for statements in the author's own review records`).
   Issue #2503 plans the full author self-audit gate in the same skill and expects to bump it from
   1.5; its owner should start from 1.6 and fold these rules into the gate.
4. **Run `review-task` and read the spec's gates before opening a PR** — existing skill and spec
   sections — no change: the guidance existed and was not followed.

## Avoiding Overcorrection

- No rule naming a model: one PR and an unrecorded comparison do not justify it.
- No new checker: the three new rules are manual, and #2503 owns any automation.
- No limit on review rounds or on fixing non-blocking findings: the maintainer chose to fix all of
  them, and each fix was correct.
- The inventory search lesson is specific to this task and stays in the #2413 evidence.

## Evidence

- Audit record: `docs/pr-reviews/pr-2553-review/PR-REVIEW.md`
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2553>
- Reviews: 5546625913 (Copilot), 5547031766, 5547366758, 5547599741, 5547834689, 5548045897
  (da2ce7), from `gh api repos/torrust/torrust-tracker/pulls/2553/reviews`.
- Dismissal times: `gh api repos/torrust/torrust-tracker/issues/2553/timeline`, events
  `review_dismissed`.
