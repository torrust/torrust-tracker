---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2840 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2840.

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

F1 to F3 are in da2ce7's review 5875801804 and are inline threads; each carries his own finding ID. R1 and R2 are assertions in the summary of Copilot's review 5875883208, which has no inline thread, so their source is the review and their thread state is `NON_RESOLVABLE`; one PR conversation response names both with their dispositions.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2840-f1 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2840-f2 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2840-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2840-r1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| R2 | review-finding:pr-2840-r2 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The ticked checkpoint contradicts the unticked `linter all` criterion

- PR number: 2840
- Source review ID: 5875801804
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551481295
- Concern: `linter all` was not recorded and its criterion was unticked while the checkpoint was ticked.
- Solution: `linter all` was run (exit 0) and recorded in the evidence, and the criterion is ticked.
- Current-tree verification: The "Linters" section of the evidence and the criterion in the spec.
- Resolution reference: docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551561813

### F2 - The completion review says no deviation but AC5 and D2 were narrowed

- PR number: 2840
- Source review ID: 5875801804
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551481302
- Concern: The review said the implementation followed the merged spec with no deviation, but AC5 and D2's test bullet were narrowed in PR #2835.
- Solution: The review states the narrowing as a deviation with the reason for no separate retrospective; the PR description is corrected.
- Current-tree verification: Implementation Completion Review in the spec; `gh pr view 2840 --json body`.
- Resolution reference: docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551562531

### F3 - The reason for no retrospective is not in the progress log

- PR number: 2840
- Source review ID: 5875801804
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551481312
- Concern: The reason was only in the completion review.
- Solution: A new progress-log entry carries the reason.
- Current-tree verification: The last progress-log entry of the spec.
- Resolution reference: docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#discussion_r4551563276

### R1 - The archived issue lacks the required issue-local retrospective

- PR number: 2840
- Source review ID: 5875883208
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#pullrequestreview-5875883208
- Concern: The summary of Copilot's review says the archived issue has no issue-local retrospective.
- Solution: `implementation-retrospective.md` is added from the template and the completion review points to it.
- Current-tree verification: `implementation-retrospective.md` exists in the closed folder; the Implementation Completion Review names it.
- Resolution reference: docs(issues): [#2822] add the implementation retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#issuecomment-6826373230

### R2 - No recorded `linter all` verification evidence

- PR number: 2840
- Source review ID: 5875883208
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2840#pullrequestreview-5875883208
- Concern: The summary says the `linter all` verification is not recorded.
- Solution: `linter all` was run (exit 0) and recorded in the evidence; its criterion is ticked.
- Current-tree verification: The "Linters" section of the evidence and the criterion in the spec.
- Resolution reference: docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2840#issuecomment-6826373230

## Processing Log

- 2026-11-12 14:06 UTC - Review 5875801804 (da2ce7, `CHANGES_REQUESTED`) submitted on the head of the close-out PR: F1, F2, F3.
- 2026-11-12 14:13 UTC - Replied to and resolved the three threads after the commit `docs(issues): [#2822] record the linter run and state the AC5 deviation in the close-out` was pushed; the reply URLs are in the entries.
- 2026-11-12 14:30 UTC - Review 5875883208 (Copilot, `COMMENTED`) had been submitted at 14:12 UTC on the first head of the close-out PR: R1 and R2 in its summary, no inline threads. The retrospective commit was pushed at 14:28 UTC (push log time), and the PR conversation response that names the review and both findings with their dispositions and resolution subjects was posted at 14:29 UTC (its URL is in the entries). One entry carries these events because the 14:13 entry above was written before the review was read.

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
