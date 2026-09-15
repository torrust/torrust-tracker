---
name: "Process Copilot Suggestions"
description: "Process Copilot-authored pull-request review findings through the unified PR review workflow"
argument-hint: "Optional PR number; defaults to the active pull request"
agent: "Copilot Suggestions Handler"
---

Process Copilot-authored findings for the target pull request through the canonical
[process PR review skill](../skills/dev/pr-reviews/process-pr-review/SKILL.md) and all applicable
repository instructions.

Target pull request: ${input:PR number (leave empty for the active PR):}

When no PR number is supplied, identify the active pull request. This entry point selects
Copilot-authored findings only; the canonical skill exclusively defines audit fields, finding
normalization, dispositions, current-tree verification, commit-subject citation, replies,
resolution order, and completion checks. Record this work in
`docs/pr-reviews/pr-<PR_NUMBER>-review.md`.
