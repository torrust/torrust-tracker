---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-container-workflow-build-deduplication/ISSUE.md
branch: "{issue-number}-container-workflow-build-deduplication"
related-pr: null
last-updated-utc: 2026-05-27 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - Containerfile
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Evaluate removing duplicate container build from container workflow

## Goal

Determine whether PR-time container build execution in container workflow can be removed or reduced because testing workflow already builds a tracker image for Docker E2E, while preserving release and publish guarantees.

## Background

Today, container workflow builds Docker images in the test job for pull requests. Testing workflow also builds a tracker image for Docker E2E execution. This may duplicate expensive container build work.

A candidate optimization is to avoid the PR-time build in container workflow and keep container builds only where they are needed for publishing (publish_development and publish_release paths). If this is done, we need to preserve confidence in image correctness and avoid breaking required-check policies.

This issue is analysis-first and must be baseline-driven.

## Scope

### In Scope

- Quantify duplicated container build cost between container and testing workflows.
- Verify which checks would be lost if PR-time build is removed from container workflow.
- Evaluate policy options:
  - keep current behavior,
  - reduce container workflow PR build scope,
  - remove PR build from container workflow and rely on testing workflow build plus publish-path builds.
- Verify that publish_development and publish_release jobs remain correct and unaffected for push/release events.
- Recommend the option that reduces end-to-end PR wait time without weakening required verification.

### Out of Scope

- Removing publish-time container build jobs.
- Weakening branch protection or required checks.
- Broad CI redesign unrelated to duplicate container builds.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                           | Notes / Expected Output                                                                                           |
| --- | ------ | ------------------------------ | ----------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Measure duplicated build cost  | Evidence for overlap between container workflow test build and testing workflow Docker E2E build.                 |
| T2  | TODO   | Map verification dependency    | Explicit list of checks provided by container workflow PR build and whether testing workflow already covers them. |
| T3  | TODO   | Evaluate workflow options      | Compare keep/reduce/remove options with risk and critical-path wait-time impact.                                  |
| T4  | TODO   | Validate publish-path behavior | Confirm publish_development and publish_release logic remains correct under candidate changes.                    |
| T5  | TODO   | Recommend decision             | Chosen option with rationale, safeguards, and expected wait-time impact.                                          |

## Decision Matrix

Use this table to compare policy options before selecting the final recommendation.

Scoring guidance:

- Verification coverage: `equivalent`, `partial`, `insufficient`
- PR wait-time impact: `better`, `neutral`, `worse`
- Publish-path safety: `safe`, `needs-guards`, `risky`
- Implementation complexity: `low`, `medium`, `high`

| Option | Description                                                                               | Verification Coverage | PR Wait-Time Impact | Publish-Path Safety | Implementation Complexity | Notes                                                                       | Decision |
| ------ | ----------------------------------------------------------------------------------------- | --------------------- | ------------------- | ------------------- | ------------------------- | --------------------------------------------------------------------------- | -------- |
| A      | Keep current behavior                                                                     | TODO                  | TODO                | TODO                | TODO                      | Baseline reference option.                                                  | TODO     |
| B      | Reduce PR build scope in container workflow                                               | TODO                  | TODO                | TODO                | TODO                      | Keep a smaller PR build signal in container workflow.                       | TODO     |
| C      | Remove PR build from container workflow and rely on testing workflow build + publish jobs | TODO                  | TODO                | TODO                | TODO                      | Candidate for strongest deduplication if required checks remain equivalent. | TODO     |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-05-27 00:00 UTC - GitHub Copilot - Drafted issue to evaluate deduplicating container builds between container and testing workflows - draft file created
- 2026-05-27 00:00 UTC - GitHub Copilot - Added decision matrix template for keep/reduce/remove policy comparison - draft updated

## Acceptance Criteria

- [ ] AC1: Duplicate container build cost is measured and documented.
- [ ] AC2: Coverage/check differences between container and testing workflows are explicit.
- [ ] AC3: At least one option reduces PR critical-path wait time without weakening required checks.
- [ ] AC4: Publish-path behavior for development/release remains correct in the chosen option.
- [ ] AC5: Final recommendation includes explicit trade-offs and rollback plan.
- [ ] `linter all` exits with code `0`
- [ ] Relevant checks pass for changed workflow/spec files
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Workflow syntax and CI checks pass for changed files
- Benchmark/report artifacts remain lint-clean

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                  | Command/Steps                                                                                                       | Expected Result                                              | Status | Evidence                 |
| --- | ------------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ | ------ | ------------------------ |
| M1  | Build overlap measurement | Compare build timings and logs for container workflow test job and testing workflow docker-e2e image build.         | Duplicate container build cost is quantified.                | TODO   | {log/output/path}        |
| M2  | Required-check review     | Map branch protection/required checks to candidate workflow behavior.                                               | No required verification is silently removed.                | TODO   | {analysis link}          |
| M3  | Publish-path validation   | Confirm publish_development and publish_release still run only in intended contexts and still build/push correctly. | Publish behavior remains correct under selected option.      | TODO   | {workflow analysis link} |
| M4  | Critical-path comparison  | Compare end-to-end wait time until all required checks finish for current and candidate workflow designs.           | Selected option improves or preserves user-facing wait time. | TODO   | {benchmark link}         |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                        |
| ----- | ---------------------- | ------------------------------- |
| AC1   | TODO                   | {benchmark/log link}            |
| AC2   | TODO                   | {coverage/check map link}       |
| AC3   | TODO                   | {critical-path comparison link} |
| AC4   | TODO                   | {publish validation link}       |
| AC5   | TODO                   | {decision summary link}         |

## Risks and Trade-offs

- Risk: removing PR-time build from container workflow may hide issues not caught elsewhere. Mitigation: verify exact check coverage and keep equivalent gates.
- Risk: reducing total compute does not guarantee better user wait time. Mitigation: use critical-path completion time as decision metric.
- Risk: workflow changes can accidentally impact publish behavior. Mitigation: validate publish job triggers and dependencies before rollout.

## References

- Related issues: #TBD
- Related PRs: #TBD
- Related ADRs: #TBD
