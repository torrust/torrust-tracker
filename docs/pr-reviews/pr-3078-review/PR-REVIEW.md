---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3078 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3078.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Reviewer: the author's GitHub login
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`, `maintainability`, `security`, `other`

## Findings

F1 to F3 are da2ce7's three inline comments in review 6078692302 (an approval on the head whose last commit is `docs(issues): [#3060] record the green run, the V1 recheck and the AC3 scan on the fixed code`); the three findings of its body are the same three. F4 is the "Out of Scope Changes" pre-merge check in the body of CodeRabbit's review 6079403289 (no inline thread). The claude[bot] review 6077829784 (automated review is disabled for fork PRs), and the following issue comments contain no actionable assertion and have no rows: CodeRabbit's status comment 7198155624, the maintainer's `ACK` comments 7198979430 and 7200974779, the reviewer's `ACK` comments 7199898165 and 7201656233, and my own conversation reply 7201388614. I listed every review, issue comment and inline comment of the PR by hand before recording this round.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3078-f1 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3078-f2 | da2ce7 | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3078-f3 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3078-f4 | coderabbitai[bot] | Suggestion (inferred) | other | ORIGINAL | NO_ACTION | NON_RESOLVABLE |

## Finding Details

### F1 - The acceptance criteria are ticked by the author of the work

- PR number: 3078
- Source review ID: 6078692302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3078#discussion_r4711971987
- Concern: AC1 to AC4 were `[x]` in the checklist and `DONE` in the status table, but `implement-task` and `commit-changes` reserve ticking a criterion for the reviewer, and the folder has no task-review record; the checklist item "Reviewer validated acceptance criteria" stayed open, so the spec contradicted itself.
- Solution: The ticks and the `DONE` cells are reverted to `[ ]` and `TODO`; the evidence cells keep the pointers to the recorded runs and say the reviewer ticks the criterion.
- Current-tree verification: `grep -n "^- \[.\] AC" ISSUE.md` shows four `[ ]`; the Acceptance Verification table shows `TODO` for AC1 to AC4.
- Resolution reference: docs(issues): [#3060] leave the acceptance criteria to the reviewer to tick and name both AC4 evidence sections
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3078#discussion_r4712469254

### F2 - No prose-first Arrange-Act-Assert record for the two new tests

- PR number: 3078
- Source review ID: 6078692302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3078#discussion_r4711971998
- Concern: The spec's Commit Points and `write-unit-test` require the completed prose-first comparison in the task evidence before review and commit; no file of the folder records it.
- Solution: The evidence file has a section with the Arrange, Act and Assert prose and its comparison with the committed code; it says it was written after this review, and no refactor followed.
- Current-tree verification: `grep -n "Prose-first Arrange-Act-Assert record" manual-verification-evidence.md` finds the section; the committed tests are unchanged.
- Resolution reference: docs(issues): [#3060] record the prose-first Arrange-Act-Assert comparison of the two new tests
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3078#discussion_r4712469538

### F3 - The AC4 evidence cell names only the after-fix section

- PR number: 3078
- Source review ID: 6078692302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3078#discussion_r4711972009
- Concern: AC4 is red before and green after, but the cell pointed only at the after-fix section.
- Solution: The AC4 cell names both the red section and the after-fix section.
- Current-tree verification: The AC4 row of the Acceptance Verification table names "T2 - Regression tests, red before the fix" and "T2 green run and V1 final recheck, after the fix".
- Resolution reference: docs(issues): [#3060] leave the acceptance criteria to the reviewer to tick and name both AC4 evidence sections
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3078#discussion_r4712469814

### F4 - The audit record is out of scope for this pull request

- PR number: 3078
- Source review ID: 6079403289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3078#pullrequestreview-6079403289
- Concern: CodeRabbit's "Out of Scope Changes" pre-merge check failed and asks to remove `docs/pr-reviews/pr-3078-review/PR-REVIEW.md` from the pull request, keep the audit outside the tracked repository, or move it to a separate pull request.
- Solution: No action. `process-pr-review` makes the PR author own the tracked audit record `docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md`, committed separately from product fixes, in the same pull request; the merged pull request 3062 carried its audit the same way. The review body has no inline thread, so the reply is a PR conversation comment.
- Current-tree verification: `grep -n "docs/pr-reviews" .claude/skills/process-pr-review/SKILL.md` shows the rule (the record path is stated at the top of the skill and in its workflow).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3078#issuecomment-7201388614
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3078#issuecomment-7201388614

## Processing Log

- 2026-11-26 17:57 UTC - da2ce7's review 6078692302 arrived with F1 to F3 (approval).
- 2026-11-26 18:55 UTC - Found it after the maintainer's message, and committed the F1 and F3 fix (`docs(issues): [#3060] leave the acceptance criteria to the reviewer to tick and name both AC4 evidence sections`).
- 2026-11-26 18:57 UTC - Committed the F2 fix (`docs(issues): [#3060] record the prose-first Arrange-Act-Assert comparison of the two new tests`).
- 2026-11-26 19:01 UTC - Pushed both, then replied on the three threads.
- 2026-11-26 19:01 UTC - Created this record from the template. It was started after the replies were posted, not before the first reply as `process-pr-review` requires; the late start is logged here. The maintainer's `ACK` of the previous head (the commit `docs(issues): [#3060] record the green run, the V1 recheck and the AC3 scan on the fixed code`) is stale after this push.
- 2026-11-26 19:08 UTC - Correction: the commit-id check printed the id of the previous head, which the Findings introduction and the 19:01 UTC entry above cited; I replaced it there with the commit subject (commit `docs(pr-reviews): [#3060] cite the audited head by commit subject, not by id`), so those two texts differ from what was first pushed.
- 2026-11-26 19:15 UTC - CodeRabbit's review 6079403289 arrived on the head with a failed pre-merge check (F4).
- 2026-11-26 19:16 UTC - Replied in a PR conversation comment (no action).
- 2026-11-26 19:16 UTC - Added F4 to this record.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
