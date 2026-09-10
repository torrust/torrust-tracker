---
doc-type: issue
issue-type: bug
status: draft
priority: p3
epic: 2190
github-issue: null
spec-path: docs/issues/drafts/2190-narrow-container-workflow-release-branch-trigger/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/workflows/container.yaml
    - .github/workflows/deployment.yaml
    - .github/workflows/deployment-packages.yaml
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Narrow the container workflow release-branch trigger

**Parent EPIC:** #2190 - Repository maintenance frictions clean-up

## Goal

Stop the container workflow from starting on package release branches, so a push to `releases/pkg/**` no longer produces a workflow run that exists only to refuse itself.

## Background

Verified at revision `f6b73e29` on 2026-09-09. `.github/workflows/container.yaml:16` triggers pushes on `releases/**/*`, while `.github/workflows/deployment.yaml:15` was narrowed to `releases/v*`. Package release branches match `releases/pkg/**`, per `.github/workflows/deployment-packages.yaml:35`.

The consequence is that every package release push also starts the container workflow. The workflow extracts `pkg/<crate>/v<semver>` as its version, fails the semver test at `container.yaml:165`, prints `Not a valid release branch semver. Will Not Continue`, and exits 0. The run is wasted, and worse, it trains readers to scroll past container-workflow runs on release branches.

The narrowing that `deployment.yaml` already received is the same narrowing this workflow needs.

## Scope

### In Scope

- Narrow the push trigger at `.github/workflows/container.yaml:16` to `releases/v*`, matching `deployment.yaml`.
- Keep the semver guard at `container.yaml:165` as the second line of defence.

### Out of Scope

- Removing or relaxing the semver guard.
- Changing `deployment.yaml` or `deployment-packages.yaml`.
- Any other trigger, job, or step in the container workflow.
- Reducing the container workflow's runtime, which is EPIC #1840.

## Architectural Decisions

No architectural decision is expected. The change aligns one workflow's trigger predicate with the convention another workflow already follows.

- Related ADRs: `None`
- ADRs to create: `None known`

## Design and Ownership Review

`Not applicable`. No child processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures are involved.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Narrow the push trigger | `container.yaml:16` reads `releases/v*`; the workflow file validates. |
| T2 | TODO | Verify on a throwaway release-package branch | A push to a branch matching `releases/pkg/**` starts no run of the container workflow. |
| T3 | TODO | Final verification and acceptance review | `linter all` exits 0 and every acceptance criterion is re-reviewed against observed behaviour. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | The one-line trigger narrowing. | Commit on its own after workflow validation. |
| T2, T3 | Verification evidence. | No repository change. Record the observation in `manual-verification-evidence.md` and the progress log. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2190-narrow-container-workflow-release-branch-trigger/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created, linked as a subissue of the parent EPIC, and issue number added to this spec
- [ ] Specification moved from `docs/issues/drafts/` to `docs/issues/open/` under its assigned number
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

- 2026-09-10 09:30 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; cluster C3, friction F3 - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: A push to a branch matching `releases/pkg/**` starts no run of the container workflow.
- [ ] AC2: A push to a branch matching `releases/v*` still starts the container workflow and it still publishes as before.
- [ ] AC3: The semver guard at `container.yaml:165` is unchanged.
- [ ] AC4: No file outside `.github/workflows/container.yaml` changes.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Workflow file validation as part of the repository's existing checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Package release branches stay quiet | Push a throwaway branch named `releases/pkg/scratch/v0.0.1`, then list workflow runs for its head, then delete the branch | No container-workflow run appears. Before the fix, one appears and exits 0 at the semver guard | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Version release branches still build | Push a throwaway branch named `releases/v0.0.1-scratch`, then list workflow runs for its head, then delete the branch | The container workflow starts and reaches its build steps | TODO | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

- Both scenarios push and delete a branch in a repository whose workflows react to `releases/**`. Run them on a fork unless a maintainer agrees otherwise, and record which repository was used.

### Disposable Verification Scripts

None is planned. Both scenarios are direct pushes and a read of the resulting run list.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M1 |
| AC2 | TODO | M2 |
| AC3 | TODO | {PR link} |
| AC4 | TODO | {PR link} |

## Risks and Trade-offs

- Narrowing a trigger can silence a run somebody depends on. Mitigation: M2 confirms that version release branches still build, and the semver guard shows that package branches were never able to produce a release anyway.
- The verification requires pushing branches that match a release pattern. Mitigation: run it on a fork, use an obviously throwaway name, and delete the branch immediately.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if the trigger narrowing turns out to affect a release path that the semver guard was silently protecting.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2190 (parent EPIC), #1840 (container workflow performance, unrelated to this trigger defect)
- Related PRs: #2193 (the EPIC specification that produced this draft)
- Related ADRs: `None`
