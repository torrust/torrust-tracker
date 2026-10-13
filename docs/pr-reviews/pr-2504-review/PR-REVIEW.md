---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2503-2278-author-self-audit-gate/ISSUE.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/pr-reviews/pr-2431-review/PR-REVIEW.md
---

<!-- skill-link: process-pr-review -->

# PR #2504 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2504>.

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

An automated review, 5503822504 (round 1, state `COMMENTED`, submitted 2026-10-13 12:55 UTC on head
`c1e8c7151`, <https://github.com/torrust/torrust-tracker/pull/2504#pullrequestreview-5503822504>),
recommends approval, reports no open finding, and leaves no inline comment, so it creates no row.

Review 5505346414 (round 1, josecelano, state `COMMENTED`, submitted 2026-10-13 14:44 UTC on head
`c1e8c7151`, <https://github.com/torrust/torrust-tracker/pull/2504#pullrequestreview-5505346414>)
left one inline comment, 4260112768, bracketed `[Minor][F1]`; the row keeps the reviewer's ID as the
audit-local ID. The rest of the body lists the reviewer's checks at that head, among them the title,
the `Related to` linking, the `-spec` branch and the frontmatter fields, EPIC row 5, the parent
issue, and a clean merge into `develop`, and offers approval once F1 is fixed or justified; none is
a further finding. F1 is `RESOLVED`, the state its thread takes when it is resolved after the reply,
and its Reply URL keeps the template's placeholder `<REPLY_URL_OR_NA>` until the posted reply's URL
is recorded.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2504-f1` | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The PR #2431 audit record links the deleted draft path

- PR number: 2504
- Source review ID: 5505346414
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2504#discussion_r4260112768>
- Concern: The move deletes `docs/issues/drafts/2278-author-self-audit-gate/ISSUE.md`, but the
  frontmatter of `docs/pr-reviews/pr-2431-review/PR-REVIEW.md` still listed that path under
  `semantic-links.related-artifacts`. A frontmatter semantic link is a live reference, which
  `create-issue` ("Migrating Legacy Specifications") says to repair when a specification moves, and
  the missing-path check of `frontmatter-validator --all` does not run on review records, so the
  pull request's validation did not cover the link. The comment states that this is the only
  `related-artifacts` entry under `docs/pr-reviews/` left pointing at a missing file, asks that it
  name the open specification or that the pull request state why it is historical, and asks that the
  body's `git grep` verification lines stay as written.
- Solution: The entry now names `docs/issues/open/2503-2278-author-self-audit-gate/ISSUE.md`, the
  specification's open path. The record's body keeps the draft path in its F1 to F5 verification
  lines, which describe the tree as it was when each check ran. A tree-wide search of
  `related-artifacts` and `spec-path` entries found no other entry naming a path this pull request
  deletes; the other entries that name paths absent from the tree predate this pull request and stay
  as they are, because repairing them is outside the order 5 move.
- Current-tree verification: a standard-library scan of the frontmatter of every Markdown file under
  `docs/` and `.github/`, resolving each `spec-path` value and each `related-artifacts` item against
  the commit's own tree, reports 158 absent entries at the fix commit, 159 at `c1e8c7151`, and 158
  at the merge base `7614ae5f3`; the extra entry at `c1e8c7151` is line 7 of
  `docs/pr-reviews/pr-2431-review/PR-REVIEW.md`, and no entry at the fix commit names
  `docs/issues/drafts/2278-author-self-audit-gate/`. Under `docs/pr-reviews/`, 24 absent entries in
  20 earlier records remain at both the merge base and the fix commit, so the comment's count holds
  for the entries this pull request breaks, not for the directory as a whole. Restricted to that
  record, `git grep -n -F docs/issues/drafts/2278-author-self-audit-gate HEAD` matches lines 70, 83,
  96, 109, and 122, all in the body.
- Resolution reference: `docs(pr-reviews): [#2431] point the audit record at the moved order 5 specification`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

## Processing Log

- 2026-10-13 12:55 UTC - An automated review, 5503822504, was submitted on head `c1e8c7151`,
  recommending approval with no open finding and no inline comment; it creates no row.
- 2026-10-13 14:44 UTC - Review 5505346414 (josecelano, `COMMENTED`, on head `c1e8c7151`) left
  inline comment 4260112768 `[Minor][F1]` (`created_at` 14:44:13Z). The thread capture shows its
  thread `resolved=false`.
- 2026-10-13 14:53 UTC - Committed the F1 fix
  `docs(pr-reviews): [#2431] point the audit record at the moved order 5 specification` (author date
  14:53:04Z).
- 2026-10-13 14:54 UTC - Ran the tree-wide frontmatter path search at the fix commit, at
  `c1e8c7151`, and at the merge base `7614ae5f3`: 158, 159, and 158 absent path entries, the one
  extra at `c1e8c7151` being the F1 entry.
- 2026-10-13 14:56 UTC - Recorded F1 `FIXED`/`RESOLVED`. The reply is drafted for posting after the
  push, and its URL replaces the placeholder then.
- 2026-10-13 14:56 UTC - Ran `validate-audit-record.py --pr-number 2504` on the uncommitted record
  with `--base` at the merge base `7614ae5f3`. With the review-comments capture it reports
  `{"status": "failed", "rows": 1, "log_entries": 5, "failures": 1}`, exit 1, the one failure being
  F1's pending Reply URL; with a probe whose Reply URL names a constructed reply on F1's thread it
  reports `{"status": "ok", "rows": 1, "log_entries": 5, "failures": 0}`, exit 0.

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
