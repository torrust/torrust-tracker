---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2946 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2946.

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
| F1 | review-finding:pr-2946-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2946-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Rows become FIXED without a recorded check after the merge

- PR number: 2946
- Source review ID: 5958197328
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2946#discussion_r4615231373
- Concern: The Current-tree verification fields of F9-F11 in the PR 2930 audit still describe the state before PR #2936 merged, so neither they nor the new log entry show the check behind `FIXED`.
- Solution: Each of the three fields now adds a check after the merge: `305027e70` is an ancestor of `develop`, `develop` has not changed the checked files since, and the finding's own check (V1 output byte for byte, the four "since #2850" lines in the ADR, the T3 row).
- Current-tree verification: At `develop` `4497837e6`: `git merge-base --is-ancestor 305027e70 torrust/develop` exited 0; the three V1 commands' output and the pasted block compare equal with `cmp`; `git grep -c "since #2850"` on the ADR prints 4; line 115 of the archived SI-1 spec is the updated T3 row.
- Resolution reference: docs(pr-reviews): [#2913] record the post-merge checks behind F9 to F11 of the PR 2930 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2946#discussion_r4615308825

### F2 - Merge time does not name its clock

- PR number: 2946
- Source review ID: 5958197328
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2946#discussion_r4615231388
- Concern: 13:31:43 UTC is the forge's merge time, not the merge commit's committer time (13:28:40Z), and the record does not say so.
- Solution: The three Solution fields and the log entry now say "forge `mergedAt`", and the Solution fields add the merge commit's committer time.
- Current-tree verification: `gh-bot pr view 2936 --json mergedAt` prints `2026-11-18T13:31:43Z`; `git log -1 --format=%cI 305027e70` prints `2026-11-18T13:28:40Z`.
- Resolution reference: docs(pr-reviews): [#2913] record the post-merge checks behind F9 to F11 of the PR 2930 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2946#discussion_r4615309489

## Processing Log

- 2026-11-18 14:09 UTC - Started audit for da2ce7's review 5958197328 (APPROVED, submitted 14:06:18Z on the head commit "docs(pr-reviews): [#2913] mark F9 to F11 of the PR 2930 audit fixed after PR 2936 merged"), with one Minor and one Nit finding, recorded as F1 and F2 with the reviewer's IDs. Copilot's review 5957963421 on the same head had no findings. Fixed both in one commit.
- 2026-11-18 14:12 UTC - Pushed the fix, replied to F1 and F2 (replies created 14:11:56Z and 14:11:58Z), then resolved both threads by 14:12:03Z (a `date -u` run after the resolve commands). GraphQL refresh: zero unresolved threads on PR #2946.

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
