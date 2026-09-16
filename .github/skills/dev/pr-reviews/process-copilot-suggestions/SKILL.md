---
name: process-copilot-suggestions
description: Deprecated compatibility redirect for Copilot pull-request review suggestions. Use the unified process-pr-review skill instead.
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# Deprecated: Process Copilot Suggestions

This compatibility entry point is retained for one release. Use
[process-pr-review](../process-pr-review/SKILL.md) for all pull-request review findings, including
Copilot-authored threads. It owns the canonical audit record at
`docs/pr-reviews/pr-<PR_NUMBER>-review.md`.
