---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-container-test-gating/ISSUE.md
branch: "{issue-number}-container-test-gating"
related-pr: null
last-updated-utc: 2026-05-27 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Evaluate test execution policy in container image build

## Goal

Decide whether tests should continue running inside the container image build path, and if not, define a safer and faster workflow policy that separates validation from packaging while preserving quality.

## Background

The current [Containerfile](../../../../Containerfile) runs tests during image build stages. At the same time, test verification is already executed in [testing.yaml](../../../../.github/workflows/testing.yaml). This may duplicate expensive work and increase runtime in both [container.yaml](../../../../.github/workflows/container.yaml) and [testing.yaml](../../../../.github/workflows/testing.yaml) paths.

This coupling also scales poorly when packaging targets grow. If the same source revision is packaged in multiple forms (for example multi-architecture container images, Linux distribution packages, or other release artifacts), embedding test execution in each packaging path can repeat the same validation work many times.

Two policy ideas need explicit evaluation:

1. Quality gate alternative: do not run test execution in container build, but enforce image publication or release flow only after testing workflow passes.
2. Debugging flexibility: optionally allow building an image from commits that fail tests, so maintainers can reproduce failures in external environments.

This issue is analysis-first and baseline-driven. Any policy change must preserve trust in merge and release checks.

## Scope

### In Scope

- Measure how much time test execution inside container build adds.
- Verify whether this work is materially duplicated by testing workflow coverage.
- Evaluate a pipeline model where validation is executed once and packaging jobs consume validated inputs.
- Evaluate workflow-gating alternatives that preserve quality guarantees.
- Evaluate a controlled path for building debug images from failing commits for investigation.
- Propose a recommendation with explicit trade-offs and safeguards.

### Out of Scope

- Weakening required quality gates for merge to protected branches.
- Publishing production images from unverified commits.
- Unrelated refactors of container or testing workflows.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                            | Notes / Expected Output                                                                                                                                    |
| --- | ------ | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Quantify duplicate test cost                    | Baseline-aligned timing evidence showing cost of test execution inside container build path.                                                               |
| T2  | TODO   | Map coverage overlap                            | Clear comparison of tests run in container build versus testing workflow.                                                                                  |
| T3  | TODO   | Evaluate validation-versus-packaging separation | Candidate CI design where validation runs once and packaging jobs (multi-arch images, distribution packages, and similar artifacts) depend on that result. |
| T4  | TODO   | Evaluate gating alternatives                    | Candidate workflow designs to keep image quality checks while reducing duplicate test execution.                                                           |
| T5  | TODO   | Evaluate debug-image path                       | Safe policy proposal for optional non-green test images used only for failure reproduction.                                                                |
| T6  | TODO   | Recommendation and decision record              | Chosen policy with rationale, safeguards, and expected performance impact.                                                                                 |

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

- 2026-05-27 00:00 UTC - GitHub Copilot - Drafted issue to evaluate container-build test execution policy and alternatives - draft file created
- 2026-05-27 00:00 UTC - GitHub Copilot - Expanded the issue to evaluate separation of validation from packaging targets - draft updated

## Acceptance Criteria

- [ ] AC1: The report quantifies runtime cost of test execution in the container build path.
- [ ] AC2: Duplicate versus unique test coverage is documented for container and testing workflows.
- [ ] AC3: At least one policy option separates validation from packaging and preserves strict quality gates.
- [ ] AC4: A safe and explicit debug-image policy is defined for failure reproduction use cases.
- [ ] AC5: Recommended policy is justified with performance and risk evidence.
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

| ID  | Scenario                          | Command/Steps                                                                                 | Expected Result                                                                   | Status | Evidence                 |
| --- | --------------------------------- | --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------ | ------------------------ |
| M1  | Duplicate-cost measurement        | Compare baseline timings for container build path with and without test execution stages.     | Measured cost of in-container test execution is documented.                       | TODO   | {log/output/path}        |
| M2  | Coverage overlap review           | Map test commands and effective coverage in container and testing workflows.                  | Overlap and any unique coverage gaps are explicit.                                | TODO   | {analysis link}          |
| M3  | Validation-packaging split review | Propose and review a pipeline where validation executes once and packaging jobs depend on it. | Duplicate validation across packaging targets is reduced without weakening gates. | TODO   | {workflow proposal link} |
| M4  | Gating design review              | Propose and review a policy where image release/publish depends on testing workflow success.  | Quality gate remains strong while redundant work can be reduced.                  | TODO   | {workflow proposal link} |
| M5  | Debug-image policy review         | Define restricted path for creating investigation images from failing commits.                | Reproduction path is available without weakening production publish policy.       | TODO   | {policy doc link}        |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                   |
| ----- | ---------------------- | -------------------------- |
| AC1   | TODO                   | {benchmark/log link}       |
| AC2   | TODO                   | {coverage comparison link} |
| AC3   | TODO                   | {workflow design link}     |
| AC4   | TODO                   | {policy link}              |
| AC5   | TODO                   | {decision summary link}    |

## Risks and Trade-offs

- Risk: removing in-container tests could hide failures if gating is weak. Mitigation: keep strict dependency on testing workflow status for protected branches and publish paths.
- Risk: splitting validation and packaging can introduce coordination complexity across workflows. Mitigation: use explicit job dependencies and required checks.
- Risk: debug-image path could be misused as a production channel. Mitigation: clearly scope it to manual troubleshooting and non-release tags.
- Risk: overlap analysis misses subtle differences in execution context. Mitigation: document context gaps explicitly before changing policy.

## References

- Related issues: #TBD
- Related PRs: #TBD
- Related ADRs: #TBD
