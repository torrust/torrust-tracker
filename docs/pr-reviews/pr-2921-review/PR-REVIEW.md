---
pr-number: 2921
pr-url: https://github.com/torrust/torrust-tracker/pull/2921
last-updated-utc: "2026-11-17 18:39"
---

<!-- skill-link: process-pr-review -->

# PR #2921 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2921.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1: da2ce7 review 5943744061 (APPROVED, later shown as dismissed) with findings F1 to F6 (two Minor, four Nit), and Copilot review 5943800775 (COMMENTED) with three threads that carry the reviewer IDs F1, F2 and F1 (no severity tags, so Minor is inferred); they are F7 to F9 here.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2921-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2921-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2921-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2921-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2921-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2921-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2921-f7 | Copilot | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F8 | review-finding:pr-2921-f8 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2921-f9 | Copilot | Minor | documentation | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F10 | review-finding:pr-2921-f10 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2921-f11 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The F28 row has no post-merge approval record

- PR number: 2921
- Source review ID: 5943744061
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603589558
- Concern: F28 entered the PR 2901 audit after the merge with no durable approval comment URL in the Ownership section or the Resolution reference.
- Solution: A comment on this PR records the maintainer's chat decision and when it was written; the Ownership line and the F28 entry cite it.
- Current-tree verification: Read the Ownership line and the F28 entry of `docs/pr-reviews/pr-2901-review/PR-REVIEW.md`; the validator returns ok for PR 2901 (28 rows).
- Resolution reference: `docs(issues): [#2765] address the review of the close-out PR`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603680860

### F2 - The AC4 evidence names its CI head by round number and attributes later changes to the wrong rounds

- PR number: 2921
- Source review ID: 5943744061
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603589586
- Concern: The line called the passing head "the head of review round 6", which reads as the head whose Docker job failed, and attributed the later changes to the wrong rounds.
- Solution: The line names the passing head by its commit subject, the failed earlier head, the fixing commit and the two later commits.
- Current-tree verification: Read the AC4 line of the spec in `docs/issues/closed/`.
- Resolution reference: `docs(issues): [#2765] address the review of the close-out PR`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603681360

### F3 - The progress log gives the wrong finding sets to the reviewers

- PR number: 2921
- Source review ID: 5943744061
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603589597
- Concern: F1 to F24 were attributed to the human reviewer although F10 to F12 and F15 belong to Copilot.
- Solution: The entry lists the human set (F1 to F9, F13, F14, F16 to F24, F28) and the Copilot set (F10 to F12, F15, F25 to F27).
- Current-tree verification: Read the progress-log entry and the author-class column of the PR 2901 audit.
- Resolution reference: `docs(issues): [#2765] address the review of the close-out PR`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603681862

### F4 - A log entry is stamped after the events it records

- PR number: 2921
- Source review ID: 5943744061
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603589619
- Concern: The 17:36 entry recorded a reply and resolution that happened before the merge.
- Solution: The entry is split by event time: 17:25 reply and resolution, 17:33 merge, 17:36 row added, and an entry for this review.
- Current-tree verification: Read the Processing Log of the PR 2901 audit; the validator checks the order.
- Resolution reference: `docs(issues): [#2765] address the review of the close-out PR`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603682352

### F5 - The commit body does not describe the whole diff

- PR number: 2921
- Source review ID: 5943744061
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603589637
- Concern: The first commit body named only the archive, AC4 and the EPIC row.
- Solution: The follow-up commit body describes the whole diff of the PR; the first commit was not rewritten mid-review.
- Current-tree verification: Read the body of the follow-up commit.
- Resolution reference: `docs(issues): [#2765] address the review of the close-out PR`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603682773

### F6 - The title keeps the number of an already closed issue

- PR number: 2921
- Source review ID: 5943744061
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603589655
- Concern: The PR title carried `[#2765]`, an issue that was already closed.
- Solution: The title is `docs(issues): close out the deprecated tracker clients issue`, set with `gh pr edit`.
- Current-tree verification: `gh pr view 2921` shows the new title.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603683247
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603683247

### F7 - Correct the revision identified by the AC4 evidence

- PR number: 2921
- Source review ID: 5943800775
- Reviewer finding ID: Copilot F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603628590
- Concern: The AC4 evidence called the passing head (job 134153598863, head `3c9a1ce97`) the head of round 6, but it is the round-7 head.
- Solution: The AC4 line names the passing head by its commit subject and the earlier failing head separately.
- Current-tree verification: Read the AC4 line of the spec in `docs/issues/closed/`.
- Resolution reference: `docs(issues): [#2765] address the review of the close-out PR`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603722168

### F8 - Use the defined thread state for a no-change disposition

- PR number: 2921
- Source review ID: 5943800775
- Reviewer finding ID: Copilot F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603628654
- Concern: The F28 row of the PR 2901 audit had `NO_ACTION` with `RESOLVED`; the rules and the F25 row use `SUPERSEDED`.
- Solution: The row has `NO_ACTION` / `SUPERSEDED`.
- Current-tree verification: `validate-audit-record.py --pr-number 2901` returns ok (28 rows).
- Resolution reference: `docs(pr-reviews): [#2765] audit review round 1 of PR 2921`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603722613

### F9 - Distinguish the audit-update time from the reply time

- PR number: 2921
- Source review ID: 5943800775
- Reviewer finding ID: Copilot F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603628708
- Concern: The F28 log entry stamped 17:36 recorded a reply made at 17:25:09, before the merge.
- Solution: The entry is split by event time: reply 17:25, merge 17:33, row added 17:36.
- Current-tree verification: Read the Processing Log of the PR 2901 audit.
- Resolution reference: `docs(issues): [#2765] address the review of the close-out PR`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603723026

### F10 - Mark F7 as a re-raise of F2

- PR number: 2921
- Source review ID: 5944168304
- Reviewer finding ID: Copilot F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603882874
- Concern: Row F7 repeated F2 (correct the revision the AC4 evidence identifies) but had the relationship `ORIGINAL`.
- Solution: The row has `RE_RAISE_OF:F2`; the row and its details are kept.
- Current-tree verification: `validate-audit-record.py --pr-number 2921` returns ok; the relationship column of the rows F7 and F9 reads `RE_RAISE_OF:F2` and `RE_RAISE_OF:F4`.
- Resolution reference: `docs(pr-reviews): [#2765] mark the duplicated findings of PR 2921 as re-raises`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4604092354

### F11 - Mark F9 as a re-raise of F4

- PR number: 2921
- Source review ID: 5944168304
- Reviewer finding ID: Copilot F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4603882953
- Concern: Row F9 repeated F4 (separate the reply time from the audit-update time) but had the relationship `ORIGINAL`.
- Solution: The row has `RE_RAISE_OF:F4`; the row and its details are kept.
- Current-tree verification: `validate-audit-record.py --pr-number 2921` returns ok; the relationship column of the rows F7 and F9 reads `RE_RAISE_OF:F2` and `RE_RAISE_OF:F4`.
- Resolution reference: `docs(pr-reviews): [#2765] mark the duplicated findings of PR 2921 as re-raises`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2921#discussion_r4604092761

## Processing Log

- 2026-11-17 17:53 UTC - Review 5943744061 (da2ce7, APPROVED) arrived on the first head (submitted 17:53:43).
- 2026-11-17 18:02 UTC - Fixed all six in one commit and a title edit, replied on the six threads and resolved them; created this audit after the replies.
- 2026-11-17 18:05 UTC - Copilot review 5943800775 (three threads) was handled together with the above; its threads were resolved by my batch command before they were replied to (a process error, as in PR 2901), and the replies were posted afterwards on the resolved threads. F8 needed one more commit (`docs(pr-reviews): [#2765] audit review round 1 of PR 2921`).
- 2026-11-17 18:19 UTC - Round 2: Copilot review 5944168304 (COMMENTED, submitted 18:19:50) with two Nits (reviewer IDs F3 and F4, recorded as F10 and F11).
- 2026-11-17 18:39 UTC - Fixed in one commit, replied on both threads and resolved them.

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
