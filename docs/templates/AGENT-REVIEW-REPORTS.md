---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

# Agent Review Reports - {Issue Title}

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### {YYYY-MM-DD HH:MM UTC} - {Reviewer}

- Invocation scope: {Changed functions, acceptance criteria, or PR number and files reviewed.}
- Inputs: {Issue specification, diff, PR metadata, tests, or other reviewed inputs.}
- Evidence: {Commands, relevant paths, CI state, or observable results. Do not include raw logs, secrets, or tokens.}
- Findings:
  - {Finding with severity, or `None`.}
- Verdict: {AUDIT PASSED|AUDIT WARNED|AUDIT FAILED|REVIEW PASSED|REVIEW FAILED|APPROVE|REQUEST_CHANGES|COMMENT}
- Follow-up actions:
  - {Required owner/action, or `None`.}
