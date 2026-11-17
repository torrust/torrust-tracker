---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - .claude/skills/cleanup-completed-issues/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2895 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2895.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2895-f1 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2895-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2895-f3 | Human | Nit | other | ORIGINAL | NO_ACTION | RESOLVED |
| F4 | review-finding:pr-2895-f4 | Copilot | Minor | link-integrity | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2895-f6 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2895-f7 | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2895-f8 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The closed #2824 spec still points the F5 owner at the removed open path

- PR number: 2895
- Source review ID: 5934509844
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596538727
- Concern: The F5 owner cell of the #2824 Findings Register and the F5 row of its `lessons.md`
  name `docs/issues/open/2876-1347-peer-id-test-helper/ISSUE.md`, which this PR removes. The
  audit of #2878 (F3) classified these two cells as live owner pointers, so step 4 of
  `cleanup-completed-issues` asks for them to follow the move; the PR description had filed them
  as historical records.
- Solution: Both cells now name `docs/issues/closed/2876-1347-peer-id-test-helper/ISSUE.md`, and
  the #2824 spec's `last-updated-utc` is bumped. The PR description no longer lists them as
  historical.
- Current-tree verification: `git grep -n 'open/2876' -- ':!docs/pr-reviews'` finds only the
  #2876 progress-log entry that records the path the spec had when it was filed.
- Resolution reference: docs(issues): [#2876] point the #2824 F5 owner cells at the archived spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596697112

### F2 - "Breaking for `primitives` 3.0.0" reads against the in-tree 3.1.0

- PR number: 2895
- Source review ID: 5934509844
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596538751
- Concern: `packages/primitives/Cargo.toml` is at 3.1.0; 3.0.0 is the published version, so row
  13 of the EPIC should say so.
- Solution: Row 13 now reads "Breaking for the published `primitives` 3.0.0".
- Current-tree verification: `grep -n 'published .primitives. 3.0.0' docs/issues/open/1347-overhaul-packages-testing/EPIC.md`
  finds row 13.
- Resolution reference: docs(issues): [#2876] point the #2824 F5 owner cells at the archived spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596697425

### F3 - The folded commit-message scan matches `closed #2876`

- PR number: 2895
- Source review ID: 5934509844
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596538762
- Concern: Folding the commit message joins the subject's last word (`docs/issues/closed`) with
  the body's first reference (`#2876`), which the closing-keyword scan reads as a closing keyword.
- Solution: No change, as the reviewer suggests: #2876 is already closed and this PR is its
  close-out, so the match closes nothing. Rewording would need a force push of an approved commit.
- Current-tree verification: `gh-bot issue view 2876 --repo torrust/torrust-tracker --json state`
  prints `CLOSED`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596697701
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596697701

### F4 - Copilot: update the stale F5 owner links to the closed path

- PR number: 2895
- Source review ID: 5934524992
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596550546
- Concern: The same two cells as F1.
- Solution: Fixed with F1.
- Current-tree verification: as F1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596698017
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596698017

### F6 - The audit validator fails on F3's `Resolution reference: N/A`

- PR number: 2895
- Source review ID: 5934845195
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596803972
- Concern: `validate-audit-record.py` checks `N/A` as a commit subject and fails; the skill asks for
  the durable reply URL on a `NO_ACTION` row, which applies to F3 and F4.
- Solution: F3 and F4 give their own Reply URL as the resolution reference.
- Current-tree verification: `python3 .claude/skills/process-pr-review/scripts/validate-audit-record.py --pr-number 2895`
  exits 0.
- Resolution reference: docs(issues): [#2876] address review round 2 of PR 2895
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596817256

### F7 - The audit's `related-artifacts` name the old-path redirect stubs

- PR number: 2895
- Source review ID: 5934845195
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596803980
- Concern: The two skill paths in the frontmatter are the "Moved" stubs under `.github/skills/`;
  the template names the skills' current location.
- Solution: The frontmatter names `.claude/skills/process-pr-review/SKILL.md` and
  `.claude/skills/cleanup-completed-issues/SKILL.md`.
- Current-tree verification: `head -8 docs/pr-reviews/pr-2895-review/PR-REVIEW.md`
- Resolution reference: docs(issues): [#2876] address review round 2 of PR 2895
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596817649

### F8 - The #2824 F5 owner cell still says the design questions are open

- PR number: 2895
- Source review ID: 5934845195
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596803984
- Concern: The cell is a live pointer but still says the maintainer answers the design questions
  before implementation, which #2888 has done.
- Solution: The cell says the questions were answered on 2026-11-16 and #2888 implemented the
  outcome; the status column stays HANDED OFF.
- Current-tree verification: `grep -n 'implemented the outcome' docs/issues/closed/2824-1347-add-tests-primitives/ISSUE.md`
- Resolution reference: docs(issues): [#2876] address review round 2 of PR 2895
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2895#discussion_r4596818003

## Processing Log

- 2026-11-17 07:47 UTC - Started audit for review round 1: review 5934509844 (da2ce7, APPROVED,
  one Minor F1 and two Nits F2 and F3, none blocking) and review 5934524992 (Copilot, COMMENTED,
  one comment, the same concern as F1). F1 and F2 fixed by commit; F3 needs no change; the Copilot
  comment is recorded as F4, superseded by F1. The four thread replies posted at 07:45 UTC, before the
  push of the fix commit.
- 2026-11-17 08:02 UTC - Review round 2: review 5934845195 (da2ce7, COMMENTED, one Minor F6, one
  Nit F7 and one Suggestion F8; APPROVE withheld only because the validator failed). All three fixed
  by commit; the three thread replies posted at 08:01 UTC, before the push.

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
