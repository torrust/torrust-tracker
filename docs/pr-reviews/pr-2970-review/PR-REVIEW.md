---
pr-number: 2970
pr-url: https://github.com/torrust/torrust-tracker/pull/2970
last-updated-utc: "2026-11-19 18:59"
---

<!-- skill-link: process-pr-review -->

# PR #2970 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2970.

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

Round 1 is da2ce7's review `5981287649` (APPROVED) on the first head. It left one inline finding with reviewer ID F1, kept as the audit ID. The review body summarizes it and has no row of its own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2970-f1` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The archived spec left its acceptance criteria unticked

- PR number: 2970
- Source review ID: 5981287649
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2970#discussion_r4633530905
- Concern: The archived spec has `status: done` and an Acceptance Verification table with AC1 to AC6 `DONE`, but all eleven acceptance-criteria boxes stayed unticked, while Step 3 of `cleanup-completed-issues` asks the archive to tick what the spec body supports and recent archives did.
- Solution: All eleven boxes are ticked, each backed by the verification table, the T6 entry, evidence V1 to V7 or the 10:21 UTC re-review entry; a new progress-log entry records this as a correction to the archival entry. The reviewer checkpoints stay unticked.
- Current-tree verification: the Acceptance Criteria section of the archived `ISSUE.md` has 11 `- [x]` items and no `- [ ]` item.
- Resolution reference: `chore(issues): [#2953] tick the acceptance criteria of the archived spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2970#discussion_r4633592063

## Processing Log

- 2026-11-19 18:58 UTC - Started audit. Round 1: da2ce7 review `5981287649` (APPROVED, submitted 18:53:19Z, poller `cursor`) on the first head, with one inline finding (F1). The fix was committed at 18:54:34Z (`git log --format=%cI`); `git ls-remote` showed the fork branch at the local head at 18:58:31Z (`date -u`). The reply was posted at 18:58:37Z (forge `created_at`), after the push.
- 2026-11-19 18:59 UTC - Round 1, continued: `validate-audit-record.py --pr-number 2970 --base torrust/develop`, with the bot token in `GH_TOKEN`, exited 0 (1 row, 0 failures) at 18:58:54Z. A GraphQL query then listed only that thread as unresolved; it was resolved by its thread ID, and a refresh showed 0 unresolved threads.

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
