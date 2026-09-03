---
doc-type: issue
issue-type: <task|bug|feature|enhancement>
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/{short-description}/ISSUE.md
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

## Architectural Decisions

Record architectural decisions that are already known when this specification is
drafted. Link existing ADRs and identify ADRs this issue is expected to create.

- Related ADRs: `docs/adrs/...`
- ADRs to create: {decision title, or `None known`}

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

For work involving child processes, asynchronous I/O, network readiness,
resource cleanup, or reusable test fixtures, define before implementation:

- the narrow public interface and each collaborator's responsibility;
- normal, failure, and drop-path ownership/lifetime invariants;
- the absolute deadline that bounds every awaited readiness operation; and
- a post-vertical-slice design-review checkpoint.

Write `Not applicable` when these concerns do not apply. Do not prescribe
private types without evidence; the objective is clear responsibility and
ownership boundaries, not speculative abstraction.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task         | Notes / Expected Output           |
| --- | ------ | ------------ | --------------------------------- |
| T1  | TODO   | {Task title} | {What "done" means for this task} |
| T2  | TODO   | {Task title} | {What "done" means for this task} |

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/{short-description}/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
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
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Relevant tests for changed components
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario          | Command/Steps                        | Expected Result     | Status | Evidence                     |
| --- | ----------------- | ------------------------------------ | ------------------- | ------ | ---------------------------- |
| M1  | {Manual scenario} | {Exact command or interaction steps} | {Expected behavior} | TODO   | {log/output/screenshot/path} |
| M2  | {Manual scenario} | {Exact command or interaction steps} | {Expected behavior} | TODO   | {log/output/screenshot/path} |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- Risk 1 and mitigation
- Risk 2 and mitigation

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.

## References

- Related issues: #{number}
- Related PRs: #{number}
- Related ADRs: `docs/adrs/...`
