---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:disable -->

## Advisory Finding Format

This format is advisory. A review is never rejected because it omits this format; authors must
process free-form feedback with the same rigor.

Use one independent finding per inline review thread. Start each finding with exactly:

```text
[<Severity>][<FindingId>] <summary>
```

`<Severity>` is one of `Blocker`, `Major`, `Minor`, `Nit`, or `Suggestion`. Use the original
`<FindingId>` when re-raising an earlier finding, and state that it is a re-raise in the body.

Keep review bodies to the round verdict or summary. Put detailed, independently actionable
findings in their own inline threads instead of duplicating them in the review body.

## Re-pushed Heads and Checklists

After a re-push, review the current PR head. Re-raise an earlier finding only when the current
tree still contains the concern or the re-push regressed it, retaining the original `<FindingId>`.

Mark a review checklist item `N/A` only after determining that it does not apply. Leave it
unchecked when it was not assessed.
