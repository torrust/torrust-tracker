---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/index.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# Pull-Request Review Audits

This directory contains the canonical audit records for all pull-request review findings,
regardless of whether their author is Copilot, a maintainer, or another reviewer.

Create new records from [PR-REVIEW-TEMPLATE.md](../templates/PR-REVIEW-TEMPLATE.md), named
`pr-<PR_NUMBER>-review.md`. Process all findings through the
[`process-pr-review` skill](../../.github/skills/dev/pr-reviews/process-pr-review/SKILL.md).

Historical duplicate source audits for a PR use the `-copilot-suggestions-legacy.md` suffix.
They preserve completed evidence from before the unified workflow and are not templates for new
records.

See [EXAMPLE-COMPLETED.md](EXAMPLE-COMPLETED.md) for a completed historical audit.
