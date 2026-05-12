---
doc-type: issue
issue-type: <task|bug|feature|enhancement>
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/{short-description}.md
branch: "{issue-number}-{short-description}"
related-pr: null
last-updated-utc: YYYY-MM-DD HH:MM
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - {Title}

## Goal

Describe the expected outcome in one or two sentences.

## Background

Describe the context, problem statement, and why this issue matters.

## Scope

### In Scope

- Item 1
- Item 2

### Out of Scope

- Item 1
- Item 2

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task         | Notes / Expected Output           |
| --- | ------ | ------------ | --------------------------------- |
| T1  | TODO   | {Task title} | {What "done" means for this task} |
| T2  | TODO   | {Task title} | {What "done" means for this task} |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- YYYY-MM-DD HH:MM UTC - {Role/Agent} - {Update summary} - {Links to evidence}

## Acceptance Criteria

- [ ] AC1: {Behavior/outcome that must be true}
- [ ] AC2: {Behavior/outcome that must be true}
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Documentation is updated when behavior/workflow changes

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- Risk 1 and mitigation
- Risk 2 and mitigation

## References

- Related issues: #{number}
- Related PRs: #{number}
- Related ADRs: `docs/adrs/...`
