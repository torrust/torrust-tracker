---
pr-number: 3070
pr-url: https://github.com/torrust/torrust-tracker/pull/3070
last-updated-utc: "2026-11-26 13:39"
---

<!-- skill-link: process-pr-review -->

# PR #3070 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3070.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and do not own this record. A reply file that an issue-linked
review task names is a separate artifact, which the reviewer writes (see the `review-pr` skill).

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

Round 1: CodeRabbit review 6074263774 on the head of commit `docs(issues): [#2904] correct the finding count of the T7 review and record its second round` (F1). The review of `claude[bot]` (6074206099) says that automated review is disabled for fork pull requests and carries no finding.

Round 2: CodeRabbit review 6074765233 on the head of commit `docs(pr-reviews): [#2904] record the first review round on pull request 3070` (F2). The reply URL of this round is recorded by the commit that follows the reply. `da2ce7`'s review 6074951727 on the head of commit `docs(pr-reviews): [#2904] record the first review round on pull request 3070` (F3 to F6); F6 is its body finding that the record read `Reply URL: pending` for F1, which the local commit with the reply of round 1 already answers.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3070-f1 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3070-f2 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3070-f3 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3070-f4 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3070-f5 | da2ce7 | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3070-f6 | da2ce7 | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The reply file of `review-pr` does not require the review context

- PR number: 3070
- Source review ID: 6074263774
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4708461352
- Concern: The JSON instructions did not require the base branch, the reviewed files or the follow-up owner and action, which the procedure for a legacy report and the inventory say are preserved, so a reviewer could omit them and still follow every field instruction.
- Solution: The `summary` carries the base branch, and the `evidence` carries the reviewed files and the follow-up owner and action, with `none` when no follow-up applies. The schema is unchanged.
- Current-tree verification: Read "A Reply File" in `.claude/skills/review-pr/SKILL.md` after the edit.
- Resolution reference: docs(skills): [#2904] require the review context in the reply file of review-pr
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4708850770

### F2 - The ownership statement of this record contradicts the reply file of `review-pr`

- PR number: 3070
- Source review ID: 6074765233
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4708873507
- Concern: The Ownership section said that reviewers have no repository-artifact obligation, while `review-pr` now requires a reviewer to write the reply file that an issue-linked review task names, so a reviewer could omit it.
- Solution: In this record the statement says that reviewers do not own this audit record, and names the reply file as a separate artifact that the reviewer writes. The sentence comes from the shared template `docs/templates/PR-REVIEW-TEMPLATE.md`, which carries it for every record; the template is not changed here, and the reply says so.
- Current-tree verification: Read the Ownership section of this record after the edit and "A Reply File" in `.claude/skills/review-pr/SKILL.md`.
- Resolution reference: docs(pr-reviews): [#2904] limit the ownership statement of the record of pull request 3070 to the record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4709150390

### F3 - The skill does not say which severities block

- PR number: 3070
- Source review ID: 6074951727
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4709030731
- Concern: `review-pr` depended on the word "blocker" without saying which of the five severities blocks, so a `COMMENT` with a `Major` finding could map to `REVIEW_PASSED`, the field the orchestrator reads.
- Solution: The Output says that a `Blocker` or a `Major` finding is a blocker, that `COMMENT` is for `Minor`, `Nit` and `Suggestion` findings, and the reply mapping repeats it.
- Current-tree verification: Read the Output and "A Reply File" of `.claude/skills/review-pr/SKILL.md` after the edit.
- Resolution reference: docs(skills): [#2904] say which severities block in review-pr and fix a link
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4709150804

### F4 - The progress log of T7 ends at the failed first review

- PR number: 3070
- Source review ID: 6074951727
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4709030739
- Concern: The last T7 entry ended at `REVIEW_FAILED`; nothing recorded the passed second review, its two nits and their fixes.
- Solution: A new entry records the second review, its verdict, the two nits and their fixes, names both reply files, and records this round.
- Current-tree verification: Read the entries of T7 after the edit.
- Resolution reference: docs(issues): [#2904] record the second task review of T7 and the first review round of pull request 3070
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4709151257

### F5 - A link names a subsection but points to its parent

- PR number: 3070
- Source review ID: 6074951727
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4709030745
- Concern: The sentence cites "A Branch Without The `task-reviewer` Agent" but the anchor was `#recording-the-review`.
- Solution: The anchor is `#a-branch-without-the-task-reviewer-agent`.
- Current-tree verification: Checked that the heading exists in `review-task` and that the link resolves.
- Resolution reference: docs(skills): [#2904] say which severities block in review-pr and fix a link
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3070#discussion_r4709151749

### F6 - The record read `Reply URL: https://github.com/torrust/torrust-tracker/pull/3070#issuecomment-7193642603` at the reviewed head

- PR number: 3070
- Source review ID: 6074951727
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3070#pullrequestreview-6074951727
- Concern: The detail of F1 still read `Reply URL: pending` and `OPEN` at the head under review, although the reply and the resolution existed.
- Solution: The commit that records the reply and the resolution of round 1 is on the branch with this push.
- Current-tree verification: Ran the audit validator after the edit.
- Resolution reference: docs(pr-reviews): [#2904] record the reply and the resolved thread of the first review round on pull request 3070
- Follow-up PR URL: N/A
- Reply URL: pending

## Processing Log

- 2026-11-26 12:40 UTC - Round 1: started on CodeRabbit review `6074263774` (one inline comment); the record was created before the first reply, and the fix is committed locally. The reply follows the push.
- 2026-11-26 13:13 UTC - Pushed round 1 (head commit `docs(pr-reviews): [#2904] record the first review round on pull request 3070`), replied on the thread (F1) and resolved it with the GraphQL mutation, which returned `isResolved: true`. The reply URL is recorded by this commit, which goes out with the next push.
- 2026-11-26 13:17 UTC - Round 2: started on CodeRabbit review `6074765233` (one inline comment, F2), after the push of round 1; the fix is committed locally. The reply follows the next push.
- 2026-11-26 13:29 UTC - Round 2 (continued): `da2ce7`'s review `6074951727` arrived before the push; F3 to F5 are fixed and committed locally, and F6 is answered by the commit of the replies of round 1. Replies follow the push.
- 2026-11-26 13:39 UTC - Pushed round 2 (head commit `docs(pr-reviews): [#2904] record the second review round on pull request 3070`), replied on the four threads (F2 to F5; GitHub `created_at` 13:39 UTC) and with one conversation comment for the body finding F6, then resolved the four threads with the GraphQL mutation, each returning `isResolved: true`. The reply URLs are recorded by this commit, which goes out with the next push.

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
