---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2878 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2878.

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
| F1 | review-finding:pr-2878-f1 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2878-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2878-f3 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2878-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2878-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2878-f6 | Copilot | Minor | link-integrity | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2878-f7 | Copilot | Minor | metadata | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F8 | review-finding:pr-2878-f8 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2878-f9 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2878-f10 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The spec `branch:` names the implementation branch, not the `-spec` branch

- PR number: 2878
- Source review ID: 5917568324
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4583875661
- Concern: The PR head branch is `2876-peer-id-test-helper-spec`, but the spec frontmatter said
  `branch: "2876-peer-id-test-helper"`; the create-issue skill says a spec-only PR records its
  `-spec` branch and keeps the base name for the implementation branch.
- Solution: Set `branch: "2876-peer-id-test-helper-spec"`.
- Current-tree verification: line 10 of `docs/issues/open/2876-1347-peer-id-test-helper/ISSUE.md`
  reads `branch: "2876-peer-id-test-helper-spec"`, the PR head branch name.
- Resolution reference: docs(issues): [#2876] use the -spec branch name in the spec frontmatter
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584078322

### F2 - The spec has `epic: 1347` but no parent line under its title

- PR number: 2878
- Source review ID: 5917568324
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4583875669
- Concern: The create-issue skill asks an EPIC subissue spec to name its parent directly below
  the title; the other #1347 subissue specs carry `Parent EPIC: #1347 - Overhaul: Packages Testing`.
- Solution: Added that line between the title and `## Goal`.
- Current-tree verification: `grep -n "Parent EPIC" docs/issues/open/2876-1347-peer-id-test-helper/ISSUE.md`
  shows line 31, two lines after the title at line 29.
- Resolution reference: docs(issues): [#2876] name the parent EPIC under the spec title
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584078708

### F3 - Pointers to the F5 owner still name the draft and its removed path

- PR number: 2878
- Source review ID: 5917568324
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4583875673
- Concern: EPIC row 11 still gave F5's owner as "draft spec PR #2865", and the closed #2824
  Findings Register F5 row and `lessons.md` row cited the removed draft path and said no issue
  existed yet.
- Solution: EPIC row 11 now says "F5 (#2876, drafted through #2865)"; the two closed #2824 owner
  cells name #2876 and `docs/issues/open/2876-1347-peer-id-test-helper/ISSUE.md`. Historical
  records (the #2824 progress log, its AC9 evidence row, the review audits, this spec's own
  checkpoint and log) keep the draft path.
- Current-tree verification: `git grep -n "drafts/peer-id-test-helper" -- docs` outside
  `docs/pr-reviews` matches only `2824-1347-add-tests-primitives/ISSUE.md` lines 340 (progress log)
  and 401 (AC9 row), and this spec's checkpoint and progress-log lines.
- Resolution reference: docs(issues): [#2876] point the F5 owner cells at #2876 and its open spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584079025

### F4 - The checkpoints record the issue as created before the spec was approved

- PR number: 2878
- Source review ID: 5917568324
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4583875682
- Concern: The issue checkpoint was ticked while "Spec reviewed and approved by user/maintainer
  (Q1-Q7 answered)" stayed open, which reads as a skipped step; the box joined two states.
- Solution: Split it into "Draft spec reviewed and merged through #2865 (filing approved by the
  maintainer in chat, 2026-11-16)", ticked, and "Open design questions Q1-Q7 answered by the
  maintainer (before T0)", open.
- Current-tree verification: lines 370 and 371 of the spec carry the two boxes, ticked and open.
- Resolution reference: docs(issues): [#2876] split the spec approval checkpoint from the Q1-Q7 answers
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584079359

### F5 - The PR description lacks the validation section and the spec-only link form

- PR number: 2878
- Source review ID: 5917568324
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4583875691
- Concern: The open-pull-request skill asks for a validation statement and `Related to #N` for a
  spec-only PR; the body used `Refs #2876` and said only a progress-log entry changed in the spec.
- Solution: Edited the PR description: it opens with `Related to #2876`, has a Validation section,
  and names the heading, parent EPIC line, checkpoint and progress-log edits.
- Current-tree verification: `gh-bot pr view 2878 --json body` shows the new first line and the
  `## Validation` heading.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2878#issuecomment-6912817188
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584079817

### F6 - Repair the live F5 handoff references when moving the spec

- PR number: 2878
- Source review ID: 5917625331
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4583918650
- Concern: The closed #2824 `ISSUE.md` F5 row and `lessons.md` row point at the removed draft path
  and say there is no GitHub issue yet.
- Solution: The same current-tree change as F3, which fixed it.
- Current-tree verification: as for F3.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584080272
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584080272

### F7 - Use the spec-only branch name in frontmatter

- PR number: 2878
- Source review ID: 5917625331
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4583918721
- Concern: The spec `branch:` should be `2876-peer-id-test-helper-spec` for a spec-only PR.
- Solution: The same current-tree change as F1, which fixed it.
- Current-tree verification: as for F1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584080662
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584080662

### F8 - The edited description carries a closing keyword for #2824

- PR number: 2878
- Source review ID: 5918010504
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584209561
- Concern: The F5 edit of the description wrote "the closed #2824 Findings Register", and
  `closed #2824` matches the closing-keyword pattern of the open-pull-request skill.
- Solution: Edited the description: the bullet now reads "the Findings Register, AC9 row and
  `lessons.md` of the closed spec of #2824".
- Current-tree verification: `grep -niE '\b(close[sd]?|fix(e[sd])?|resolve[sd]?)\b:?\s+#[0-9]+'`
  on the new description body printed nothing.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2878#issuecomment-6913176963
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584242781

### F9 - The #2824 AC9 row cites a `lessons.md` row that no longer says what it claims

- PR number: 2878
- Source review ID: 5918010504
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584209574
- Concern: The AC9 row said F5 had no GitHub issue and that "its `lessons.md` row says so", while
  this PR rewrote that `lessons.md` row to name #2876.
- Solution: The AC9 cell now ends "with no GitHub issue yet at closure (filed later as #2876; see
  the Findings Register)"; the draft path stays as evidence of the time.
- Current-tree verification: `grep -n "filed later as #2876" docs/issues/closed/2824-1347-add-tests-primitives/ISSUE.md`
  shows the AC9 row at line 401.
- Resolution reference: docs(issues): [#2876] note in the #2824 AC9 row that F5 was filed later as #2876
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584243304

### F10 - The update stamps of the files the F3 commit touched do not match its time

- PR number: 2878
- Source review ID: 5918010504
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584209583
- Concern: The F3 commit was made at 09:38:58 UTC, but the #2824 spec said 09:40 and the EPIC
  kept 08:45.
- Solution: Both stamps set to `2026-11-16 09:38`. The F9 commit then edited the #2824 spec again
  at 09:55:51 UTC and set its stamp to `2026-11-16 09:55`.
- Current-tree verification: `git log --format=%cI` gives 09:38:58Z for the F3 commit and
  09:55:51Z for the F9 commit; line 9 of the EPIC reads `2026-11-16 09:38` and line 12 of the #2824
  spec reads `2026-11-16 09:55`.
- Resolution reference: docs(issues): [#2876] match the update stamps of the EPIC and the #2824 spec to their edit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2878#discussion_r4584243807

## Processing Log

- 2026-11-16 09:41 UTC - Started audit for review round 1: review 5917568324 (da2ce7, APPROVED,
  five Minor findings F1-F5) and review 5917625331 (Copilot, COMMENTED, two Minor findings that
  re-raise F3 and F1, recorded as F6 and F7). Fixes F1-F4 committed; PR description edited for F5;
  the PR conversation response and the seven thread replies posted at 09:39-09:40 UTC, before the
  push of the fix commits.
- 2026-11-16 09:57 UTC - Review round 2: review 5918010504 (da2ce7, APPROVED) with
  reviewer findings F6, F7 and F8, recorded as F8, F9 and F10 because F6 and F7 were already taken.
  F9 and F10 committed; description edited for F8; the PR conversation response and the three
  thread replies posted at 09:56-09:57 UTC, before the push of the fix commits.

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
