---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2965 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2965.

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

Both rows come from da2ce7's review 5973079104 (APPROVED on the first head); the reviewer's IDs are
kept.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2965-f1 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2965-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The state list's date does not cover the updated primitives bullet

- PR number: 2965
- Source review ID: 5973079104
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2965#discussion_r4627237835
- Concern: The `primitives` bullet of the first-publish plan now records a merge of 2026-11-19 under a list introduced as "State on 2026-11-18".
- Solution: The introduction keeps its date and adds that only the `primitives` bullet was updated since, on 2026-11-19, for EPIC #2912.
- Current-tree verification: `git log --since='2026-11-18 10:30'` on the plan lists the #1488 close-out commits, whose diffs change only `last-updated-utc` and a link near line 545, outside the list, and the #2914 commits, of which only the state-line commit touches the list (the `primitives` bullet).
- Resolution reference: docs(issues): [#2914] say which bullet of the publish plan's state list changed after 2026-11-18
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2965#discussion_r4627307928

### F2 - The description misstates why the EPIC link changed

- PR number: 2965
- Source review ID: 5973079104
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2965#discussion_r4627237856
- Concern: The PR description said the parent EPIC link was repaired "for the new folder depth", but the depth did not change; the link needed `../../open/` because the EPIC stays in `open/` while the spec moved to `closed/`. The description becomes the merge commit's message.
- Solution: Edited the PR description (no commit): it now says the link points back into `open/` because the EPIC stays there while the spec moved to `closed/`.
- Current-tree verification: `gh-bot pr view 2965 --json body` contains "points back into"; the closing-keyword scan of the description matches nothing.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2965#discussion_r4627308384
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2965#discussion_r4627308384

## Processing Log

- 2026-11-19 10:19 UTC - Started audit for da2ce7's review 5973079104 (APPROVED at 10:15:44Z on the first head, "chore(issues): archive closed issue #2914 spec to docs/issues/closed"), with F1 and F2, both Nits, recorded with the reviewer's IDs. F1 fixed in a commit; F2 fixed by editing the PR description at 10:19 UTC.
- 2026-11-19 10:22 UTC - Pushed "docs(issues): [#2914] say which bullet of the publish plan's state list changed after 2026-11-18" as the new head. Replied on F1 and F2 (replies created 10:21:59Z and 10:22:01Z), recorded the reply URLs, ran `validate-audit-record.py --pr-number 2965 --base torrust/develop` through a scratch copy that calls `gh-bot` (status ok, 2 rows, 0 failures), then resolved both threads by 10:22:25 UTC (a `date -u` run after the resolve commands). GraphQL refresh: zero unresolved threads on PR #2965.

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
