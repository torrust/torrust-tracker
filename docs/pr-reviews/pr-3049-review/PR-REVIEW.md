---
pr-number: 3049
pr-url: https://github.com/torrust/torrust-tracker/pull/3049
last-updated-utc: "2026-11-25 13:30"
---

<!-- skill-link: process-pr-review -->

# PR #3049 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3049.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (pre-merge review).

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1, on the first head of the pull request (`fa4c27842`): Greptile review 6045345701 (F1, F2)
and CodeRabbit review 6045401167 (F3, F4), both on that head. F1, F2 and F4 are about the
"Review-Service Check Runs" section this PR adds to `watch-pull-request-checks`; F4 repeats F1.
This is the first audit record with the `Reviewer` column the PR introduces.

Round 2, on head `e57378df3`: Greptile review 6045462816 (F5), about the wait loop added for F2.
Round 3, on head `13b96f0cf`: CodeRabbit review 6045569854 with one inline thread (F6) and one
review-body finding (F7) that repeats F5.
Round 4, on head `9d1946125`: CodeRabbit review 6045717795, its answer in the F6 thread (F8).
Round 5, on head `146179e79`: CodeRabbit review 6045878013 with one thread (F9). It came after the
fifth round of fixes; the contributor allowed a sixth round on 2026-11-25.
Round 6, on head `9e743e54f`: CodeRabbit review 6053945210 with one thread (F10) and one
outside-diff finding in its body (F11). On head `2b9de50df`: CodeRabbit review 6054013798, a
follow-up in the F9 thread (F12), and da2ce7's review 6054114265 (`APPROVED`, findings F1 to F7,
recorded as F13 to F19 because their ids collide with this record's). The contributor allowed one
seventh round for all of them on 2026-11-25.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3049-f1 | coderabbitai[bot] | Major | correctness | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-3049-f2 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - Define the overall result of `review-task` when a criterion is pending

- PR number: 3049
- Source review ID: 6058202792
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695361559
- Concern: "Output" item 6 of `.claude/skills/review-task/SKILL.md` defined `REVIEW PASSED` only as every required criterion passing, which contradicts the task-reviewer practice of leaving a criterion `PENDING` with a reason and still returning `REVIEW_PASSED`.
- Solution: Item 6 now says `REVIEW PASSED` when no required criterion fails and no blocking issue remains, and that a criterion left `PENDING` for evidence from a later task is listed with the reason and does not by itself fail the review; step 9 reports the overall result instead of pass/fail.
- Current-tree verification: read the skill after the edit; `grep` for the old wording in `.claude/agents/task-reviewer.md` and `contrib/dev-tools/checks` found no copy (the agent file already says "the overall result"); the commit passed the pre-commit hook.
- Resolution reference: docs(skills): [#2904] define the review-task result when a criterion is pending
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695609915

### F2 - AC10 ticked done although scenario M5 has no reply files

- PR number: 3049
- Source review ID: 6058202792
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695361573
- Concern: AC10 of `ISSUE.md` was ticked and its acceptance-verification row was DONE, but it also covers scenario M5 (task T9), which has not produced its reply files.
- Solution: The checkbox is back to `[ ]` and the row to `TODO`, with a note that the M7 evidence (section V7 of `manual-verification-evidence.md`) is recorded and M5 is pending. AC11 is unchanged; a Progress Log entry records the fix.
- Current-tree verification: read AC10 and its row in `ISSUE.md` after the edit; the commit passed the pre-commit hook.
- Resolution reference: docs(issues): [#2904] reopen AC10 until scenario M5 has its reply files
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3049#discussion_r4695610470

## Processing Log

- 2026-11-25 13:25 UTC - Started audit for review `6058202792` of `coderabbitai[bot]`; the review `6058085652` of `claude[bot]` (review disabled for forks) and the CodeRabbit walkthrough comment `7163196060` carry no finding.
- 2026-11-25 13:25 UTC - Verified both findings against the current tree and committed the two fixes.
- 2026-11-25 13:30 UTC - Pushed the fixes and replied on both threads, then recorded the reply URLs.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
