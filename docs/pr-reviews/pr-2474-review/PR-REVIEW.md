---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2473-2003-asynchronous-discussion-rounds/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2474 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2474>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Copilot review 5445379625 (16:37 UTC) supplied the finding ID `F1` with a `[Minor]` severity; it is
kept. The review body summarizes that one thread and adds no other assertion.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2474-f1` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - M1 deletes the scratch discussion that M2 needs

- PR number: 2474
- Source review ID: 5445379625
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2474#discussion_r4209465217>
- Concern: Manual scenario M1 ended by deleting the scratch discussion folder, while M2 continues
  in that folder, so the mandatory scenarios could not be followed in order.
- Solution: M1 now keeps the scratch folder for M2, and M2 deletes it after V2 is recorded. A
  follow-up commit restored the scenario table's ID column width, which the first commit had
  narrowed.
- Current-tree verification: in the specification, M1 (:225) ends "Keep the scratch folder for
  M2." and M2 (:226) ends "After recording V2, delete the scratch folder."; `grep -c "then delete
  the scratch folder"` returns 0; `linter markdown` exits 0.
- Resolution reference: docs(issues): [#2473] keep the scratch discussion until the second manual scenario
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2474#discussion_r4209515217>

## Processing Log

- 2026-10-07 16:39 UTC - Started audit. Fetched the one review thread with GraphQL (unresolved,
  not outdated) and the review body; normalized them into F1.
- 2026-10-07 16:42 UTC - Pushed the F1 fix and the table-width correction after the pre-commit
  gate passed, re-checked the claim against the pushed tree, and replied on the thread.

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
