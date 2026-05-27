---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-containerfile-target-scope/ISSUE.md
branch: "{issue-number}-containerfile-target-scope"
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
    - docs/issues/open/1726-reduce-build-times-sccache/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Narrow Containerfile build targets to tracker image needs

## Goal

Reduce container image build time by avoiding compilation and linking of workspace targets that are not required to produce and validate the tracker runtime image.

## Background

The current [Containerfile](../../../../Containerfile) builds and archives a very broad target set (`--tests --benches --examples --workspace --all-targets --all-features`) across multiple stages. A quick maintainer analysis suggests some of that work is unrelated to the final tracker image, including targets from packages such as `packages/torrent-repository-benchmarking`.

This issue should only proceed after the baseline subissue confirms both of these points:

1. Unneeded target compilation/linking is materially present in the container build path.
2. That work has significant impact on workflow runtime.

If confirmed, narrowing target scope can speed up [container.yaml](../../../../.github/workflows/container.yaml) directly, and can also improve [testing.yaml](../../../../.github/workflows/testing.yaml) because Docker E2E builds and uses the tracker image there.

## Scope

### In Scope

- Identify which binaries, examples, benches, and packages are truly required for the tracker image build and test path.
- Propose the minimal safe target set for relevant `cargo chef` and `cargo nextest archive` commands in the Containerfile.
- Validate that the produced release image still contains required executables and passes existing container and E2E checks.
- Quantify runtime impact in container and testing workflows before and after the change.

### Out of Scope

- Broad test policy changes unrelated to container image scope.
- Removing mandatory runtime checks from CI.
- Refactoring unrelated workflow jobs.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                   | Notes / Expected Output                                                                |
| --- | ------ | -------------------------------------- | -------------------------------------------------------------------------------------- |
| T1  | TODO   | Confirm eligibility from baseline data | Evidence shows meaningful time spent on targets not needed by tracker runtime image.   |
| T2  | TODO   | Define required target inventory       | Explicit list of required binaries and test artifacts for container build and E2E use. |
| T3  | TODO   | Narrow Containerfile target selection  | Update cargo commands to avoid unnecessary targets while preserving expected behavior. |
| T4  | TODO   | Measure workflow impact                | Before/after timing comparison for container and testing workflows.                    |

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

- 2026-05-27 00:00 UTC - GitHub Copilot - Drafted Containerfile target-scope optimization issue from EPIC discussion - draft file created

## Acceptance Criteria

- [ ] AC1: Baseline evidence confirms that unnecessary target compilation/linking is a significant bottleneck.
- [ ] AC2: Containerfile target scope is reduced without removing artifacts required by the runtime image.
- [ ] AC3: Container workflow runtime improves measurably after the change.
- [ ] AC4: Testing workflow Docker E2E path remains valid and does not regress.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests and container checks pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Container workflow-equivalent build command(s) complete successfully
- Docker E2E command path used by testing workflow still passes

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                       | Command/Steps                                                                              | Expected Result                                            | Status | Evidence          |
| --- | ------------------------------ | ------------------------------------------------------------------------------------------ | ---------------------------------------------------------- | ------ | ----------------- |
| M1  | Bottleneck confirmation        | Use baseline report to compare phase timings and identify unneeded target build/link cost. | Decision to proceed is backed by measured data.            | TODO   | {log/output/path} |
| M2  | Reduced-scope build validation | Build tracker image with narrowed Containerfile target scope.                              | Required executables are present and image build succeeds. | TODO   | {log/output/path} |
| M3  | E2E compatibility check        | Run Docker E2E flow against the reduced-scope image.                                       | E2E tests pass with no functional regression.              | TODO   | {log/output/path} |
| M4  | Performance comparison         | Compare before/after container and testing workflow runtimes.                              | Improvement is measurable and documented.                  | TODO   | {log/output/path} |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                     |
| ----- | ---------------------- | ---------------------------- |
| AC1   | TODO                   | {benchmark/log link}         |
| AC2   | TODO                   | {build/image evidence}       |
| AC3   | TODO                   | {workflow timing comparison} |
| AC4   | TODO                   | {e2e results link}           |

## Risks and Trade-offs

- Risk: removing targets too aggressively can break test coverage or E2E expectations. Mitigation: define required target inventory first and validate with E2E.
- Risk: performance gain may be small if linking of required targets dominates. Mitigation: gate implementation on baseline evidence.
- Risk: target selection complexity can reduce maintainability. Mitigation: document rationale near modified commands.

## References

- Related issues: #TBD, #1726
- Related PRs: #TBD
- Related ADRs: #TBD
