---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #<PR_NUMBER> Review Audit

Source: pull-request reviews and inline review threads for <PR_URL>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Create one row for every independent concern. Split a review body into separate rows for separate
actionable assertions. Assign a reviewer-provided finding ID when available; otherwise assign
`F<ordinal>` in source-review and source-order order. Before action, map a later request for the
same current-tree change to `RE_RAISE_OF:<FindingId>`.

For every new audit row, record the source author's derived `Author class` and exactly one primary
`Category`. Classify the category from the concern, not the proposed fix; use `other` only when no
listed category fits. These fields make future audit records suitable for deterministic aggregation.
Historical records predate this schema and remain unchanged.

| PR number | Source review ID | Source URL | Author class | Finding ID | Severity | Category | Summary | Relationship | Disposition | Current-tree verification | Resolution reference | Reply URL | Thread state |
| --------- | ---------------- | ---------- | ------------ | ---------- | -------- | -------- | ------- | ------------ | ----------- | ------------------------- | -------------------- | --------- | ------------ |
| <PR_NUMBER> | <REVIEW_ID> | <SOURCE_URL> | <AUTHOR_CLASS> | <FINDING_ID> | <SEVERITY> | <CATEGORY> | <SUMMARY> | <RELATIONSHIP> | <DISPOSITION> | <COMMAND_OR_INSPECTION_AND_RESULT> | <UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL> | <REPLY_URL_OR_NA> | <THREAD_STATE> |

## Processing Log

- <YYYY-MM-DD HH:MM UTC> - Started audit.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated or superseded thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
