---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2690 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2690.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

F1 to F8 come from review 5712289311 (round 1) and F9 to F11 from review 5712767693 (round 2). F12 to F19 are the eight "Not yet" follow-up comments of reviews 5712768462 to 5712771679, each a re-raise of F1 to F8 against a head that predates the fixes. Each is an inline reply inside the thread of the finding it re-raises, so its `Source URL` is the submitted review that holds it (the inline comment id is named in the Concern) and its `Reply URL` is the reply posted on that same thread: the validation script can only match a reply to a thread root, and a reply comment is never one. The reviewer ID scheme (F1 to F11) was kept; no collision. All findings are by the human reviewer `da2ce7`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2690-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2690-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2690-f3 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2690-f4 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2690-f5 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2690-f6 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2690-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2690-f8 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2690-f9 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2690-f10 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2690-f11 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2690-f12 | Human | Major (inferred) | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F13 | review-finding:pr-2690-f13 | Human | Minor (inferred) | correctness | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F14 | review-finding:pr-2690-f14 | Human | Major (inferred) | documentation | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F15 | review-finding:pr-2690-f15 | Human | Major (inferred) | correctness | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F16 | review-finding:pr-2690-f16 | Human | Minor (inferred) | metadata | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F17 | review-finding:pr-2690-f17 | Human | Minor (inferred) | correctness | RE_RAISE_OF:F6 | FIXED | RESOLVED |
| F18 | review-finding:pr-2690-f18 | Human | Minor (inferred) | documentation | RE_RAISE_OF:F7 | FIXED | RESOLVED |
| F19 | review-finding:pr-2690-f19 | Human | Minor (inferred) | correctness | RE_RAISE_OF:F8 | FIXED | RESOLVED |

## Finding Details

### F1 - Separate the published baseline from the full-window recount

- PR number: 2690
- Source review ID: 5712289311
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423639954
- Concern: The full-window recount has 68 merges and a 3.78-hour median, not 4.0; section 1 wrongly said the recount reproduces 4.0, and sections 2 and 7 kept the old denominator.
- Solution: Separated the published 67-PR snapshot from the 68-PR recount in sections 1, 2, 8 and the conclusion.
- Current-tree verification: Section 1 and 2 tables show the three populations; 4.0086 (67 PRs without #2506) and 3.7828 (68) are stated apart; #2506 has no force-push event.
- Resolution reference: `docs(issues): [#2547] separate the published baseline from the full-window recount`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424365596

### F2 - Correct the per-day merge-volume multiplier

- PR number: 2690
- Source review ID: 5712289311
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423639964
- Concern: (121/14)/(68/13) is 1.65, not 1.8.
- Solution: Replaced 1.8 by 1.65 and 66% by 65%.
- Current-tree verification: `grep -c '1\.8 times'` on the evidence returns 0.
- Resolution reference: `docs(issues): [#2547] correct the per-day merge volume multiplier to 1.65`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424365954

### F3 - Record the transformations needed to reproduce the tables

- PR number: 2690
- Source review ID: 5712289311
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423639969
- Concern: The recorded commands stop at API retrieval; the aggregation and per-run totals are absent.
- Solution: Added Appendix A with the aggregation commands and a table of the 53 per-run job-minute totals.
- Current-tree verification: Appendix A.1 to A.6 exist and the per-run table has 53 rows.
- Resolution reference: `docs(issues): [#2547] record the aggregation commands and per-run CI data`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424366282

### F4 - Account for workload mix before calling per-PR costs comparable

- PR number: 2690
- Source review ID: 5712289311
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423639973
- Concern: The documentation/code split promised by the specification is not reported; per-PR normalization leaves the mix confounder.
- Solution: Added section 8 (49/19 against 62/59) and qualified the CI conclusion and caveat.
- Current-tree verification: Section 8 table and A.6 command present; the split reproduced independently before use.
- Resolution reference: `docs(issues): [#2547] report the documentation and code split of the merges`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424366579

### F5 - Finish the close-out review and update the specification state

- PR number: 2690
- Source review ID: 5712289311
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423639977
- Concern: Acceptance verification, completion review, metadata and log order were not reconciled with the closing body.
- Solution: Completed AC1 to AC4, added the retrospective, set status and log order, and recorded the API-only M1/M2 deviation.
- Current-tree verification: Spec status is in-review, AC rows DONE, retrospective linked.
- Resolution reference: `docs(issues): [#2547] complete the close-out state of the specification`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424366878

### F6 - Distinguish workflow merges, files, and merge clocks

- PR number: 2690
- Source review ID: 5712289311
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423639984
- Concern: Four merges touched seven distinct files, and the displayed times were committer times, not merge times.
- Solution: Rewrote section 6 with per-merge files and merged_at times.
- Current-tree verification: Section 6 table lists 4 merges and 7 files with merged_at times.
- Resolution reference: `docs(issues): [#2547] distinguish the workflow merges, files and merge clocks`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424367212

### F7 - Make the body use the documented populations and sample exception

- PR number: 2690
- Source review ID: 5712289311
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423639990
- Concern: The PR body said 11 force-pushes in the window (one is dated 14 October) and six runs per workflow without the five-run exception.
- Solution: Edited the PR body to state lifetime events of merged PRs, 10 inside the window, and 53 sampled runs.
- Current-tree verification: `gh pr view 2690 --json body` shows the corrected statements.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2690#issuecomment-6524493920
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424367635

### F8 - Separate non-rebase citation repairs from rebase fallout

- PR number: 2690
- Source review ID: 5712289311
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423639998
- Concern: The #2630 audit repair follows a non-rebase rewrite and was counted as rebase fallout; other counted edits are preventive.
- Solution: Excluded that commit, split the rest into kind A and kind B, and reported one definition for both periods.
- Current-tree verification: Section 5 lists kinds A and B and the exclusion; the comparison shows 13 against 11.
- Resolution reference: `docs(issues): [#2547] separate non-rebase citation repairs from rebase fallout`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424367971

### F9 - Retain the CI-to-merge inputs and compute the stated median

- PR number: 2690
- Source review ID: 5712767693
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423998190
- Concern: The CI-to-merge input and its construction were absent and q(.5) selected the lower middle observation.
- Solution: Kept the per-merge input in evidence/ci-to-merge.tsv, recorded the loop, and used statistics.median (15.4 and 46.3 min) with a labelled percentile.
- Current-tree verification: Section 7 table shows 15.4/46.3 and 274.8/605.3; the TSV has 189 rows.
- Resolution reference: `docs(issues): [#2547] retain the CI-to-merge inputs and compute the stated median`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424368472

### F10 - Correct the exclusion of two baseline merges

- PR number: 2690
- Source review ID: 5712767693
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423998196
- Concern: All 68 baseline PRs have a first-parent merge commit, so the table's 66 was wrong.
- Solution: Counted all 68; the omission came from `git log --since` stopping at an older commit (#2397 and #2400).
- Current-tree verification: Section 7 row reads 68; A.3 states the cause.
- Resolution reference: `docs(issues): [#2547] separate the published baseline from the full-window recount`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424368829

### F11 - Replace future timestamps with the actual update times

- PR number: 2690
- Source review ID: 5712767693
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4423998206
- Concern: The 13:30 and 13:00 log stamps and last-updated were later than the commits that introduced them.
- Solution: Restamped entries with commit or event times and fixed last-updated-utc and the evidence header.
- Current-tree verification: Log entries carry 11:06, 11:49, 12:45, 12:57 and 13:45 UTC; last-updated is not later than its commit.
- Resolution reference: `docs(issues): [#2547] stamp the spec metadata and progress log with the actual times`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424369134

### F12 - Re-check of F1 at an earlier head

- PR number: 2690
- Source review ID: 5712768462
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#pullrequestreview-5712768462
- Concern: A "Not yet" follow-up (inline comment r4423998788) in the thread of F1, written against a head that predates the fix: it states the F1 concern was still present there.
- Solution: The fix of F1 is on the branch; see that finding.
- Current-tree verification: Same current-tree verification as F1: Section 1 and 2 tables show the three populations; 4.0086 (67 PRs without #2506) and 3.7828 (68) are stated apart; #2506 has no force-push event.
- Resolution reference: `docs(issues): [#2547] separate the published baseline from the full-window recount`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424365596

### F13 - Re-check of F2 at an earlier head

- PR number: 2690
- Source review ID: 5712768920
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#pullrequestreview-5712768920
- Concern: A "Not yet" follow-up (inline comment r4423999148) in the thread of F2, written against a head that predates the fix: it states the F2 concern was still present there.
- Solution: The fix of F2 is on the branch; see that finding.
- Current-tree verification: Same current-tree verification as F2: `grep -c '1\.8 times'` on the evidence returns 0.
- Resolution reference: `docs(issues): [#2547] correct the per-day merge volume multiplier to 1.65`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424365954

### F14 - Re-check of F3 at an earlier head

- PR number: 2690
- Source review ID: 5712769402
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#pullrequestreview-5712769402
- Concern: A "Not yet" follow-up (inline comment r4423999516) in the thread of F3, written against a head that predates the fix: it states the F3 concern was still present there.
- Solution: The fix of F3 is on the branch; see that finding.
- Current-tree verification: Same current-tree verification as F3: Appendix A.1 to A.6 exist and the per-run table has 53 rows.
- Resolution reference: `docs(issues): [#2547] record the aggregation commands and per-run CI data`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424366282

### F15 - Re-check of F4 at an earlier head

- PR number: 2690
- Source review ID: 5712769841
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#pullrequestreview-5712769841
- Concern: A "Not yet" follow-up (inline comment r4423999833) in the thread of F4, written against a head that predates the fix: it states the F4 concern was still present there.
- Solution: The fix of F4 is on the branch; see that finding.
- Current-tree verification: Same current-tree verification as F4: Section 8 table and A.6 command present; the split reproduced independently before use.
- Resolution reference: `docs(issues): [#2547] report the documentation and code split of the merges`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424366579

### F16 - Re-check of F5 at an earlier head

- PR number: 2690
- Source review ID: 5712770404
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#pullrequestreview-5712770404
- Concern: A "Not yet" follow-up (inline comment r4424000175) in the thread of F5, written against a head that predates the fix: it states the F5 concern was still present there.
- Solution: The fix of F5 is on the branch; see that finding.
- Current-tree verification: Same current-tree verification as F5: Spec status is in-review, AC rows DONE, retrospective linked.
- Resolution reference: `docs(issues): [#2547] complete the close-out state of the specification`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424366878

### F17 - Re-check of F6 at an earlier head

- PR number: 2690
- Source review ID: 5712770836
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#pullrequestreview-5712770836
- Concern: A "Not yet" follow-up (inline comment r4424000467) in the thread of F6, written against a head that predates the fix: it states the F6 concern was still present there.
- Solution: The fix of F6 is on the branch; see that finding.
- Current-tree verification: Same current-tree verification as F6: Section 6 table lists 4 merges and 7 files with merged_at times.
- Resolution reference: `docs(issues): [#2547] distinguish the workflow merges, files and merge clocks`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424367212

### F18 - Re-check of F7 at an earlier head

- PR number: 2690
- Source review ID: 5712771254
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#pullrequestreview-5712771254
- Concern: A "Not yet" follow-up (inline comment r4424000774) in the thread of F7, written against a head that predates the fix: it states the F7 concern was still present there.
- Solution: The fix of F7 is on the branch; see that finding.
- Current-tree verification: Same current-tree verification as F7: `gh pr view 2690 --json body` shows the corrected statements.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2690#issuecomment-6524493920
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424367635

### F19 - Re-check of F8 at an earlier head

- PR number: 2690
- Source review ID: 5712771679
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2690#pullrequestreview-5712771679
- Concern: A "Not yet" follow-up (inline comment r4424001105) in the thread of F8, written against a head that predates the fix: it states the F8 concern was still present there.
- Solution: The fix of F8 is on the branch; see that finding.
- Current-tree verification: Same current-tree verification as F8: Section 5 lists kinds A and B and the exclusion; the comparison shows 13 against 11.
- Resolution reference: `docs(issues): [#2547] separate non-rebase citation repairs from rebase fallout`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2690#discussion_r4424367971

## Processing Log

- 2026-10-30 13:53 UTC - Started audit after the consolidated response (https://github.com/torrust/torrust-tracker/pull/2690#issuecomment-6524493920) and the 11 thread replies had been posted (13:53:51 to 13:54:34 UTC).
- 2026-10-30 13:56 UTC - Resolved the 11 threads (GraphQL `resolveReviewThread`, 13:56:39 to 13:56:50 UTC); a GraphQL refresh at 13:56:50 UTC reported 0 unresolved threads. `validate-audit-record.py --pr-number 2690` exited 0 (19 rows, 0 failures) before this entry.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
