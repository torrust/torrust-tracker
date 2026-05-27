---
doc-type: issue
issue-type: task
status: open
priority: p1
github-issue: 1841
spec-path: docs/issues/open/1841-1840-workflow-performance-baseline-analysis/ISSUE.md
branch: "1841-1840-workflow-performance-baseline-analysis"
related-pr: null
last-updated-utc: 2026-05-27 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #1841 - Baseline workflow profiling and bottleneck analysis

## Goal

Measure where time is spent in [`.github/workflows/container.yaml`](../../../../.github/workflows/container.yaml) and [`.github/workflows/testing.yaml`](../../../../.github/workflows/testing.yaml), then record a baseline that can be reused to compare future workflow optimizations.

## Background

The two workflows are critical PR checks and currently take long enough to slow down merges and encourage batching unrelated changes. Before changing the workflows, we need a repeatable baseline that answers two questions:

1. How long does each workflow take on a clean run with no meaningful local cache?
2. How much faster is the second run when the local cache is already populated?

The baseline should emulate shared-runner constraints as closely as practical on a local machine. That means clearing relevant local caches before the cold run, then running the same commands again to capture the warm-cache case. The resulting report must remain in the subissue folder so later optimization work can compare against it.

## Scope

### In Scope

- Measure total wall time for the container and testing workflows.
- Measure the major parts inside each job so the bottleneck is visible, not just the total runtime.
- Identify linker-heavy targets that are not required for the final tracker runtime image.
- Capture both a no-cache first run and a second run with local caches available.
- Clear local Rust and Docker-related caches where needed to approximate a shared runner first run.
- Store the benchmark report in this subissue folder and update it after later workflow improvements.

### Out of Scope

- Changing workflow logic as part of the baseline work.
- Optimizing any step before the measurements are captured.
- Replacing critical checks or lowering verification quality.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                     | Notes / Expected Output                                                                                                 |
| --- | ------ | ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Define the benchmark procedure           | A reproducible command sequence for cold and warm runs, including cache reset steps.                                    |
| T2  | TODO   | Capture baseline timings                 | A measured comparison for both workflows with and without local caches.                                                 |
| T3  | TODO   | Profile linker-heavy non-runtime targets | List the highest-cost targets in the container build path and flag which are not required by the tracker runtime image. |
| T4  | TODO   | Write the benchmark report               | A durable report in this folder with workflow totals, sub-step timings, linker hotspot findings, and comparison notes.  |

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

- 2026-05-27 00:00 UTC - GitHub Copilot - Drafted the baseline workflow profiling subissue for the performance EPIC - draft file created
- 2026-05-27 00:00 UTC - GitHub Copilot - Expanded baseline scope to include linker-heavy target analysis and runtime relevance classification - draft updated
- 2026-05-27 00:00 UTC - GitHub Copilot - Created GitHub issue #1841 and linked it as a child issue of EPIC #1840 - draft updated

## Acceptance Criteria

- [ ] AC1: The baseline report records a no-cache and warm-cache run for both target workflows.
- [ ] AC2: The baseline report identifies the dominant bottleneck inside each workflow.
- [ ] AC3: The baseline report identifies linker-heavy targets and explicitly marks which are not required by the tracker runtime image.
- [ ] AC4: The report is stored in this subissue folder and can be reused for later comparisons.
- [ ] AC5: The benchmark procedure is explicit enough to rerun on the same machine later.
- [ ] `linter all` exits with code `0`
- [ ] Relevant measurement commands are run and documented
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- The benchmark command sequence completes without errors
- If the report format changes, `linter markdown` and `linter cspell` still pass

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                | Command/Steps                                                                                                                                                   | Expected Result                                                                              | Status | Evidence                     |
| --- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | ------ | ---------------------------- |
| M1  | Cold baseline capture   | Clear local Rust caches and any relevant Docker layer cache, then run the workflow-equivalent commands once for container and testing.                          | The report records no-cache wall times and the measured bottleneck for each workflow.        | TODO   | {log/output/screenshot/path} |
| M2  | Warm baseline capture   | Re-run the same benchmark commands immediately after M1 without clearing caches.                                                                                | The report records warm-cache wall times for both workflows and shows the expected speed-up. | TODO   | {log/output/screenshot/path} |
| M3  | Linker hotspot capture  | Capture per-target compilation and linking timings for the container build path and classify targets as runtime-required or not-required for the tracker image. | The report includes a ranked linker-heavy target list with runtime relevance classification. | TODO   | {log/output/screenshot/path} |
| M4  | Persistent report check | Update the benchmark artifact in this folder and verify it still reflects the latest measured baseline.                                                         | The report stays versioned alongside the issue and is ready for future comparison runs.      | TODO   | {log/output/screenshot/path} |

Notes:

- Manual verification is mandatory even when automated checks pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- A local machine will never be identical to GitHub-hosted runners. Mitigation: record the cache-reset procedure and run the same commands each time.
- Different stages may dominate on different machines. Mitigation: measure both total runtime and the major internal phases.
- The report can drift out of date after later changes. Mitigation: keep the artifact in the same subissue folder and refresh it after each improvement.

## References

- Related issues: #1840
- Related PRs: #TBD
- Related ADRs: #TBD
