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

Assign each new row an immutable repository reference in the form
`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, with the finding ID lowercased. For example, audit
finding `F1` for PR #2230 is `review-finding:pr-2230-f1`. Use this reference to link the finding
from ADRs, issue specifications, and other repository artifacts. GitHub identifiers remain source
metadata, not the canonical finding reference. Never change a reference after assigning it.

This convention applies to new audits only. Historical audit records remain unchanged and do not
need review-finding references.

Record each finding in two coordinated places: one compact tracking row below, and one matching
entry in `## Finding Details` for the narrative fields. The pair shares the finding ID and
together records every required audit field.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| <FINDING_ID> | <REVIEW_FINDING_REFERENCE> | <AUTHOR_CLASS> | <SEVERITY> | <CATEGORY> | <RELATIONSHIP> | <DISPOSITION> | <THREAD_STATE> |

## Finding Details

Create one entry per tracking row, in the same order. Keep the concern and the applied solution
readable as prose; this section carries the source metadata and verification evidence.

### <FINDING_ID> - <SUMMARY>

- PR number: <PR_NUMBER>
- Source review ID: <REVIEW_ID>
- Source URL: <SOURCE_URL>
- Concern: <WHAT_THE_REVIEWER_REPORTED>
- Solution: <WHAT_WAS_DONE_AND_WHY_OR_WHY_NOT>
- Current-tree verification: <COMMAND_OR_INSPECTION_AND_RESULT>
- Resolution reference: <UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL>
- Reply URL: <REPLY_URL_OR_NA>

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
