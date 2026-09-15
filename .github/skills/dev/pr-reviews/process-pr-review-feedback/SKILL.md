---
name: process-pr-review-feedback
description: Deprecated compatibility redirect for pull-request review feedback. Use the unified process-pr-review skill instead.
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# Deprecated: Process PR Review Feedback

This compatibility entry point is retained for one release. Use
[process-pr-review](../process-pr-review/SKILL.md) for all pull-request review findings, regardless
of reviewer identity. It owns the canonical audit record at
`docs/pr-reviews/pr-<PR_NUMBER>-review.md`.
