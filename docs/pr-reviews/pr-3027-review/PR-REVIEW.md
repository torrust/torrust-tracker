---
pr-number: 3027
pr-url: https://github.com/torrust/torrust-tracker/pull/3027
last-updated-utc: "2026-11-24 15:58"
---

<!-- skill-link: process-pr-review -->

# PR #3027 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3027.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the finding was fixed before the merge).

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

Round 1, on the first head: CodeRabbit review 6040148469 (one thread). It says that at this head D1 of the #2387 decisions and the #2387 EPIC still require SI-1 to SI-20 before #2766, so this PR must land with the companion decision PR. That PR is #3024, owned by the #2387 coordinator. The fix is a merge-order line in this PR's description; no file of this PR changes.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3027-f1 | Unknown | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The #2387 records still give the old order

- PR number: 3027
- Source review ID: 6040148469
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3027#discussion_r4680751541
- Concern: At this head, D1 in `docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` and the #2387 EPIC still require SI-1 to SI-20 before #2766, while this PR requires only SI-1. Merged alone, the two records give opposite prerequisites for SI-2 to SI-20.
- Solution: The PR description now says to merge this PR together with or after #3024, which records order B′ in D1 and updates the #2387 EPIC. The #2387 files stay with their coordinator.
- Current-tree verification: `gh-bot pr view 3027 --json body` contains the `Merge order` line, and the diff of #3024 adds "Under order B′ (D1) #2766 comes after this issue" to the SI-1 draft.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3027#discussion_r4680911698
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3027#discussion_r4680911698

## Processing Log

- 2026-11-24 13:36 UTC - Round 1: CodeRabbit review 6040148469 (13:36:16) with one thread.
- 2026-11-24 13:48 UTC - Added the merge-order line to the PR description, replied on the thread and resolved it (13:48:18).
- 2026-11-24 13:49 UTC - Created this audit record after the reply and the resolution.
- 2026-11-24 15:58 UTC - Rebased on `develop` to resolve a conflict with the #3004 close-out (PR #3021) in the EPIC's `last-updated-utc`; kept the later stamp. PR #3024 had merged by then, so the merge order of F1 is met.

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
