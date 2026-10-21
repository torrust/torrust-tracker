# PR #2599 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2599>.

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

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| AUDIT-001 | review-finding:pr-2599-audit-001 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### AUDIT-001 - Closure event is outside the Processing Log

- PR number: 2599
- Source review ID: 5597905213
- Reviewer finding ID: AUDIT-001
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2599#discussion_r4335386291>
- Concern: The dated WAIT-001 closure entry was appended under `## Completion Rules` in the PR
  #2584 audit, so readers of `## Processing Log` miss it. The reviewer asked to move it after the
  existing log entries and before the append-only notice, keeping earlier entries.
- Solution: Moved the entry into the Processing Log, after the 2026-10-21 07:25 UTC entry and before
  the append-only notice. No earlier entry was changed and no text of the moved entry was altered.
- Current-tree verification: Read `docs/pr-reviews/pr-2584-review/PR-REVIEW.md` after the fix; the
  09:34 UTC entry is the last bullet before "Append entries only", and `## Completion Rules` ends
  with its template text. `git show --stat` of the fix commit shows 4 insertions and 4 deletions in
  that file only.
- Resolution reference: docs(pr-reviews): move the WAIT-001 closure entry into the Processing Log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2599#discussion_r4335499171>

## Processing Log

- 2026-10-21 10:05 UTC - Processed Copilot review 5597905213 (head 46f2c63dd): one Nit finding,
  AUDIT-001. Fixed in the commit named above, pushed, replied on the thread at
  <https://github.com/torrust/torrust-tracker/pull/2599#discussion_r4335499171>, and resolved the
  thread. A GraphQL refresh then showed 0 unresolved threads, and no pending review exists.

Append entries only. If an entry was rewritten in place, restore the prior entries, then append a
correction that names the rewritten content; do not rewrite the record again.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
