---
semantic-links:
  skill-links:
    - process-pr-review-feedback
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md
---

<!-- cspell:disable -->

# PR #<PR_NUMBER> Review Feedback Tracking

Source: pull-request reviews and inline review comments for <PR_URL>.

## Purpose

Track review feedback independently from Copilot review-thread audits. A
review summary can contain several independently actionable findings, so this
record has one **review** row and one or more **finding** rows per review.

A GitHub review ID identifies the submitted review. An inline review thread ID
identifies a resolvable suggestion thread. They are different resources and
must not be used interchangeably.

## Status Values

- Finding decision: `ACTION`, `NO_ACTION`, `FOLLOW_UP`
- Finding status: `OPEN`, `IN_PROGRESS`, `DONE`, `BLOCKED`
- Inline thread state: `NOT_APPLICABLE`, `OPEN`, `RESOLVED`
- Review response state: `PENDING`, `POSTED`

## Workflow

1. Record each submitted review by review ID and URL.
2. Decompose every review body and inline comment into independent findings.
3. Commit each independent action separately; do not mix workflow documentation
   with product fixes.
4. For an inline suggestion thread, reply with the outcome and resolve it after
   its action or no-action decision is complete.
5. For a review-level summary, post one consolidated PR comment after all of
   that review's findings are complete. Link that response from the review row.
6. Update this audit record after every decision, commit, reply, or resolution.

## Reviews

| Review ID | Submitted at (UTC) | Reviewer | State | URL | Reviewed commit | Consolidated response URL | Response state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <REVIEW_ID> | <TIMESTAMP> | <REVIEWER> | <REVIEW_STATE> | <REVIEW_URL> | <COMMIT> | <RESPONSE_URL_OR_PENDING> | <PENDING_OR_POSTED> |

## Findings

| ID | Review ID | Source | Comment / thread ID | URL | Summary | Decision | Independent fix commit | Validation | Reply URL | Inline thread state | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | <REVIEW_ID> | <REVIEW_BODY_OR_INLINE> | <COMMENT_OR_THREAD_ID> | <COMMENT_URL> | <SUMMARY> | <DECISION> | <COMMIT_OR_NA> | <VALIDATION> | <REPLY_URL_OR_NA> | <STATE> | <STATUS> |

## Processing Log

- <YYYY-MM-DD HH:MM UTC> - Started audit.

## Notes

- A `CHANGES_REQUESTED` review state is historical evidence of the reviewer's
  decision, not a per-finding completion state. Keep it unchanged in the table.
- A `DISMISSED` review remains an auditable review record; record why it was
  superseded in the processing log or finding notes.
- GitHub does not expose a dedicated reply resource for a submitted review
  summary. Use a normal PR conversation comment as the consolidated response
  and record its comment URL.
- Do not resolve an inline review thread without first replying on that thread.
- A review-level comment with no inline thread cannot be marked resolved in
  GitHub. The review ID plus this record's response state is the durable status.
