---
pr-number: 2948
pr-url: https://github.com/torrust/torrust-tracker/pull/2948
last-updated-utc: "2026-11-18 14:21"
---

<!-- skill-link: process-pr-review -->

# PR #2948 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2948.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

Round 1 is the approving review `5958211073` of `da2ce7` (F1 and F2, reviewer-provided IDs) on the first head; Copilot's
review `5958205525` on the same head recommends approval with no findings and has no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2948-f1` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2948-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - F16 cites the reword commit, not the commit that made the change

- PR number: 2948
- Source review ID: 5958211073
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2948#discussion_r4615242881
- Concern: The change F16 asked for landed in `docs(pr-reviews): [#2904] record the late review of PR #2920 and correct the verification of F15`; the cited reword commit only reorders that line, so "each with its fixing commit's subject" did not hold for F16.
- Solution: The row of F16 cites both subjects, the correction first, separated by a semicolon, and a correction entry in that record's log says so.
- Current-tree verification: The Resolution reference of F16 in `docs/pr-reviews/pr-2920-review/PR-REVIEW.md` names both subjects; `validate-audit-record.py --pr-number 2920` exited 0 (19 rows).
- Resolution reference: `docs(pr-reviews): [#2904] cite the correcting commit of F16 in the review audit of PR #2920`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2948#discussion_r4615429797

### F2 - The record stops before the end of step 6

- PR number: 2948
- Source review ID: 5958211073
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2948#discussion_r4615242891
- Concern: Step 6 of Reviews Submitted After Merge ends with recording zero unresolved threads after a GraphQL refresh; the record said only that the threads would be resolved after the push.
- Solution: A new entry records the push time, the resolution of the four threads by 13:49:23Z and the refresh with 0 unresolved threads.
- Current-tree verification: A GraphQL refresh on pull request #2920 shows 0 unresolved threads; the new entry gives the times.
- Resolution reference: `docs(pr-reviews): [#2904] record the resolution of F16 to F19 in the review audit of PR #2920`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2948#discussion_r4615430354

## Processing Log

- 2026-11-18 14:17 UTC - Started audit, before the replies of round 1.
- 2026-11-18 14:21 UTC - Round 1: the approving review `5958211073` of `da2ce7` (submitted 14:07:07Z, forge `submitted_at`) with two findings. The fixes were committed at 14:14:30Z and 14:16:14Z (`git log --format=%cI`, in UTC) and pushed; `git ls-remote` showed the fork branch at the local head at 14:20:24Z (`date -u`). The two replies were posted at 14:20:34Z and 14:20:36Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2948 --base torrust/develop`, with its `gh` calls made through `gh-bot`, exited 0 (2 rows, 0 failures) at 14:20:51Z; the two threads were then resolved by their thread IDs by 14:20:55Z, and a GraphQL refresh showed 0 unresolved threads.

<!-- Copied verbatim into each audit record. -->

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
