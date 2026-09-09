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

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set                       | Commit policy                                        |
| ---- | ----------------------------------------- | ---------------------------------------------------- |
| T1   | {Narrow, independently reviewable change} | Commit after focused validation and required review. |
| T2   | {Narrow, independently reviewable change} | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. Record this review in task evidence
or a file-local test plan. Commit each reviewed test-design increment before starting the next planned
file or behavior area. Keep final verification and completion evidence separate when it improves
reviewability. Use a Conventional Commit message with the narrow affected scope, and sign every
commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [ ] Folder-style spec drafted in `docs/issues/drafts/{short-description}/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
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
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
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

| ID  | Scenario          | Human-oriented command/steps                      | Expected Result     | Status | Evidence                                     |
| --- | ----------------- | ------------------------------------------------- | ------------------- | ------ | -------------------------------------------- |
| M1  | {Manual scenario} | {Exact command or interaction actually performed} | {Expected behavior} | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | {Manual scenario} | {Exact command or interaction actually performed} | {Expected behavior} | TODO   | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a
  real human-oriented use of the feature or reproduction of the bug fix, not a
  simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these
  scenarios. Record actual prerequisites, actions, commands, program output,
  relevant tracker logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

Temporary scripts may automate an issue-local verification scenario, but they
are neither maintained automatic tests nor manual-verification evidence. Before
creating one, record in the issue specification:

- why temporary automation is better for this concrete scenario than a
  maintained Rust automatic test;
- the script's issue-local path, what it verifies, and its intended removal or
  retention owner; and
- when using Python instead of Rust, why Rust is not suitable for that specific
  script.

Keep the script in the issue-specification folder so later reviewers can inspect
the verification performed. Promote durable product-behavior checks into Rust
automatic tests when practical, then remove the disposable script.

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
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`. Do not create that artifact for a
  legacy standalone specification or when no folder-style specification was
  supplied.

## References

- Related issues: #{number}
- Related PRs: #{number}
- Related ADRs: `docs/adrs/...`
