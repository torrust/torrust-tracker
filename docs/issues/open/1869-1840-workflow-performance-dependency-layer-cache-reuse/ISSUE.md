---
doc-type: issue
issue-type: task
status: open
priority: p1
github-issue: 1869
spec-path: docs/issues/open/1869-1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md
branch: "{issue-number}-dependency-layer-cache-reuse"
related-pr: null
last-updated-utc: 2026-06-03 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
---

<!-- skill-link: create-issue -->

# Issue #1869 - Improve dependency-layer cache reuse within each workflow

## Goal

Reduce repeated dependency build time by ensuring dependency-related container layers are reused when Cargo dependencies are unchanged inside each workflow run sequence.

## Background

A quick analysis suggests dependency-heavy container build layers are often rebuilt even when dependency inputs do not change. In principle, when only application code changes and Cargo dependency metadata remains the same, dependency cook layers should be reusable.

Current workflows use isolated cache scopes to avoid conflicts and race conditions when multiple jobs write cache data concurrently. This issue treats that isolation as a current constraint and focuses first on making cache reuse reliable within each workflow.

This issue should determine whether current cache misses are caused by layer invalidation inputs, cache configuration, or both, and then propose a safe strategy to improve reuse within workflow boundaries.

A further concern emerged from post-#1853 CI analysis: in this repository, most logic lives in in-repo workspace packages (not external crates), and those packages change on nearly every PR. The `cargo-chef` cook stage can only pre-compile external dependencies; workspace members must always be compiled from source in the build stage. This raises the question of whether the cook/build split provides meaningful cache benefit at all given this churn pattern, or whether an alternative scoping strategy — for example, limiting the cook stage to external-only packages via `--package` selectors — would be more effective. This issue must include that evaluation as part of T3.

## Scope

### In Scope

- Measure dependency-layer cache hit and miss behavior for unchanged dependency inputs.
- Identify invalidation triggers for dependency stages in the Containerfile and workflow build configuration.
- Preserve current workflow concurrency while improving cache effectiveness.
- Evaluate whether the current `cargo-chef` cook/build split strategy delivers meaningful cache benefit given typical PR churn on workspace packages, and document findings with evidence. If the split is not effective, propose an alternative (for example, scoping the cook stage to external-only packages via `--package` selectors, or eliminating the split in favour of a single build step).
- Propose a practical cache policy and expected impact.
- Prepare follow-up scope for optional cross-workflow cache reuse only after in-workflow behavior is reliable.

### Out of Scope

- Unsafe cache sharing that can corrupt or poison cache data.
- Implementing cross-workflow cache reuse in this issue.
- Forcing workflows to execute sequentially as part of this issue.
- Broad workflow redesign unrelated to dependency cache reuse.
- Changes that weaken CI correctness guarantees.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                               | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                           |
| --- | ------ | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Reproduce current cache behavior   | Demonstrate dependency-layer misses when dependencies are unchanged and only app code differs.                                                                                                                                                                                                                                                                                                    |
| T2  | TODO   | Identify invalidation inputs       | Document which files, build args, or stage structure invalidate dependency layers.                                                                                                                                                                                                                                                                                                                |
| T3  | TODO   | Propose in-workflow reuse strategy | Recommendation for container and testing workflows independently, keeping current cache-scope isolation and concurrency. The strategy must also assess whether the `cargo-chef` cook/build split is appropriate given workspace-package churn: if the split provides little reuse benefit, propose an alternative (for example, scoping cook to external-only packages or eliminating the split). |
| T4  | TODO   | Validate impact on PR wait time    | Before/after evidence for dependency-stage reuse and effect on end-to-end check completion time.                                                                                                                                                                                                                                                                                                  |
| T5  | TODO   | Draft follow-up scope              | Outline a separate follow-up issue for optional cross-workflow cache reuse, including race and sequencing trade-offs.                                                                                                                                                                                                                                                                             |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
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

- 2026-05-27 00:00 UTC - GitHub Copilot - Drafted dependency-layer cache reuse issue from EPIC discussion - draft file created
- 2026-05-27 00:00 UTC - GitHub Copilot - Refocused this issue on in-workflow cache reuse first and moved cross-workflow sharing to follow-up scope - draft updated
- 2026-06-03 00:00 UTC - GitHub Copilot - Added workspace-churn angle: T3 now requires evaluating whether the cook/build split itself is effective, not only whether cache config is correct - draft updated
- 2026-06-03 00:00 UTC - GitHub Copilot - Created GitHub issue #1869 and promoted spec to `docs/issues/open/`

## Acceptance Criteria

- [ ] AC1: Current cache miss behavior for unchanged dependency inputs is reproduced and documented.
- [ ] AC2: Dependency-layer invalidation triggers are identified with concrete evidence.
- [ ] AC3: At least one strategy improves dependency-layer reuse within each workflow while preserving current concurrency.
- [ ] AC4: Impact is measured on end-to-end PR check wait time, not only summed workflow runtime.
- [ ] AC5: Follow-up scope for optional cross-workflow cache reuse is documented with explicit race and sequencing trade-offs.
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

| ID  | Scenario                        | Command/Steps                                                                                                            | Expected Result                                                                        | Status | Evidence          |
| --- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- | ------ | ----------------- |
| M1  | Unchanged-dependency rerun      | Run container build twice with unchanged Cargo dependency inputs and app-code-only changes between runs.                 | Dependency stages show expected cache reuse behavior and are measurable.               | TODO   | {log/output/path} |
| M2  | Invalidation trigger inspection | Trace which dependency-related layers are invalidated and why.                                                           | Root causes for misses are explicit and actionable.                                    | TODO   | {analysis link}   |
| M3  | In-workflow strategy review     | Evaluate cache strategy changes independently inside container and testing workflows without cross-workflow sharing.     | Safe in-workflow strategy is selected with maintainable configuration.                 | TODO   | {proposal link}   |
| M4  | Critical-path impact check      | Compare before/after end-to-end wait time until all required checks finish.                                              | Improvement is documented on user-facing wait time while keeping workflow concurrency. | TODO   | {benchmark link}  |
| M5  | Follow-up definition            | Capture candidate cross-workflow reuse options, including optional sequential orchestration, in a follow-up issue draft. | Follow-up scope is explicit and does not block this issue.                             | TODO   | {draft link}      |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                     |
| ----- | ---------------------- | ---------------------------- |
| AC1   | TODO                   | {benchmark/log link}         |
| AC2   | TODO                   | {invalidation analysis link} |
| AC3   | TODO                   | {cache strategy link}        |
| AC4   | TODO                   | {policy decision link}       |
| AC5   | TODO                   | {timing comparison link}     |

## Risks and Trade-offs

- Risk: aggressive cache sharing can introduce write races or inconsistent state. Mitigation: design explicit ownership and write policy per scope.
- Risk: reducing per-workflow runtime may still not improve total wait time if critical-path behavior is ignored. Mitigation: measure and optimize end-to-end wait until all required checks complete.
- Risk: forcing sequential workflows for cache reuse can increase total wait time despite lower compute usage. Mitigation: keep this issue focused on in-workflow reuse and evaluate sequential orchestration only in follow-up.
- Risk: measured gains may be lower than expected if invalidation is driven by unavoidable inputs. Mitigation: validate root causes before implementation.
- Risk: even with correct cache configuration, workspace-package churn on most PRs may mean the cook stage provides little reuse benefit, making the overall optimization marginal. Mitigation: T3 explicitly evaluates this and proposes an alternative strategy if the current split is not effective.

## References

- Related issues: #TBD
- Related PRs: #TBD
- Related ADRs: #TBD
