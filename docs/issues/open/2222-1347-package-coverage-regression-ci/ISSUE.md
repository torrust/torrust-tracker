---
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 1347
github-issue: 2222
spec-path: docs/issues/open/2222-1347-package-coverage-regression-ci/ISSUE.md
branch: "2222-1347-package-coverage-regression-ci"
related-pr: null
last-updated-utc: 2026-09-15 10:42
semantic-links:
  skill-links:
    - create-issue
    - update-github-workflow-actions
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md
    - .github/workflows/generate_coverage_pr.yaml
    - .github/workflows/upload_coverage_pr.yaml
    - .github/workflows/coverage.yaml
    - codecov.yaml
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2222 - Prevent Per-Package Coverage Regressions in Pull Requests

Parent EPIC: #1347 - Overhaul: Packages Testing

## EPIC Guideline Applicability

This is a coverage-infrastructure subissue, not a per-package test-improvement subissue. The
EPIC's module inventory, property-test candidate, per-file decision-record, and module-level
approval-gate requirements are therefore not applicable. T1 instead inventories and prototypes
the candidate CI mechanisms; T2 is the maintainer approval gate before implementation; T6 is the
post-vertical-slice review. The remaining guidelines apply: toolchain-pinned validation, explicit
manual verification, a final reconciliation task, issue-local lessons, rebase-stable evidence,
and separate aggregate/unit-only/integration-only reporting where coverage is measured.

## Goal

Establish evidence for and introduce a low-overhead, per-package coverage-regression check for
pull requests. The check must compare each package against its actual base measurement, not an
absolute percentage target, and must not become required until a report-only rollout validates its
signal, operational cost, and tolerance.

## Background

The current `Generate Coverage Report (PR)` workflow produces one workspace-wide Codecov report.
`codecov.yaml` allows a 0.5 percentage-point decrease for project and patch status, but it neither
compares individual packages nor provides a repository-owned required check. The completed
`axum-http-server` work demonstrates why a package baseline is useful navigation evidence while
also showing why an absolute threshold is not: important combined configuration contracts were
missing at 95.26% source-line coverage.

The parent EPIC rejects uniform package thresholds and percentage-driven test selection. A
per-package regression check is compatible with that policy when it only reports or preserves the
measured baseline and leaves behavior-focused test selection to review.

The strategy in this draft is intentionally provisional. The implementation must revalidate the
current toolchain, Codecov capabilities, workflow behavior, artifact retention, package layout,
and cost before selecting an implementation. The preferred candidate is a one-build,
repository-owned comparison: publish a compact per-package summary from `develop` coverage runs,
then compare it with a summary derived from the PR's existing workspace coverage report. Codecov
components are a parallel report-only candidate. A base-worktree double-build is a documented
fallback, not the planned first implementation.

## Scope

### In Scope

- Revalidate this draft's candidate strategies through current documentation and a small proof of
  concept before changing a CI gate: (a) Codecov components/statuses, (b) a repository-owned
  one-build summary artifact comparison, and only if needed (c) a base-worktree double-build.
- Measure actual workflow runtime, summary/artifact availability for a PR base SHA, source-path
  semantics, and pass/fail behavior using a controlled package change.
- Select directly changed workspace packages using Cargo metadata and the pull request base and
  head SHAs, rather than a hard-coded `packages/*` path list.
- Produce per-package source-only line-coverage summaries from the existing workspace coverage
  reports. Count files below the package's `src/` directory and retain exact covered and
  instrumented line counts; do not parse rounded terminal percentages.
- Add a report-only `package-coverage-regression` job or step to the existing unprivileged
  `.github/workflows/generate_coverage_pr.yaml` pull-request workflow. It must show base, head,
  and delta for every comparable directly changed package without initially blocking merges.
- Publish the matching compact per-package baseline summary from `develop` coverage runs, with a
  retrieval strategy that works for pull requests from forks and has explicit retention behavior.
- Report new, deleted, and renamed packages without fabricating a comparison. A new or unmatched
  renamed package is baseline-unavailable and not gated; a removed package is reported and not
  gated.
- Based on observed report-only results, recommend whether the status should become required and
  what tolerance, if any, is justified. Do not make it required in this issue without explicit
  maintainer approval after that evidence is reviewed.
- Keep the existing workspace-wide Codecov report and artifact-upload flow unchanged.
- Add focused automated tests for changed-package discovery, report filtering, ratio comparison,
  and exceptional package cases.
- Document the contributor/CI behavior and the coverage scope in the canonical testing or CI
  documentation.

### Out of Scope

- A uniform or absolute coverage percentage target for all packages.
- Mutation-score requirements or mutation testing in CI.
- Replacing Codecov project/patch reporting or its existing upload workflow.
- Comparing reverse dependencies, every workspace package, or application/E2E behavior in this
  first increment.
- Making the report-only job a required branch-protection check without a separate maintainer
  decision informed by rollout evidence.
- Persisting manually maintained package coverage-baseline snapshots.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md`,
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: None expected. Create one only if implementation changes the established
  trust/security boundary between unprivileged pull-request workflows and the trusted
  `workflow_run` uploader.

## Design and Ownership Review

The gate executes pull-request code. It belongs only in the existing unprivileged
`pull_request` workflow, which has no secrets. The existing `upload_coverage_pr.yaml` is a trusted
`workflow_run` artifact uploader and must not execute fork-provided code or become the gate.

A repository-owned command under `contrib/dev-tools/checks/` is the initial candidate to own Cargo
metadata discovery, JSON report filtering, exact-ratio comparison, artifact retrieval, and
job-summary rendering. The workflow should provide immutable PR SHAs and orchestration only.

The preferred design avoids a base worktree and a second instrumented build: `coverage.yaml` emits
a compact base-commit package summary, while `generate_coverage_pr.yaml` derives the head summary
from its already-generated workspace report. The proof of concept must validate that this is
reliable for fork PRs, including artifact availability and retention. If it is not, document the
failure and evaluate the double-build fallback rather than silently weakening the comparison.

There is no network readiness, child-process protocol, or multi-step asynchronous-I/O deadline
beyond bounded GitHub Actions/Cargo commands; no reusable test fixture is expected. After the
first passing vertical slice, review whether the command remains straightforward orchestration. If
it grows into package-graph policy, special environment management, or complex coverage modeling,
replace it with a Rust tool rather than expanding shell complexity.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID  | Status | Task                                           | Notes / Expected Output                                                                                                                                                                                                                                                                                               |
| --- | ------ | ---------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Revalidate strategy and prototype alternatives | Recheck current Codecov component/status behavior, GitHub artifact APIs/retention, `cargo llvm-cov` JSON semantics, package layout, and security constraints. Prototype Codecov-component and one-build artifact-summary paths using a controlled package change; record runtime, availability, and failure behavior. |
| T2  | TODO   | Select evidence-backed implementation          | Compare prototype results against the direct base-worktree fallback. Obtain maintainer approval for the selected report-only design, source scope, baseline retrieval behavior, and proposed tolerance-evaluation period.                                                                                             |
| T3  | TODO   | Implement package summary and discovery        | Use Cargo metadata plus `git diff --name-status --find-renames "$base_sha" "$head_sha"` to identify directly changed workspace packages and preserve rename pairing. Carry old/new package paths into the baseline decision. Produce exact source-only per-package covered/instrumented-line summaries from existing workspace reports. Define root, new, removed, renamed, and workspace-config behavior. |
| T4  | TODO   | Publish and compare report-only summaries      | Publish base summaries on `develop`, retrieve the PR-base summary, compare it with the PR head summary, and write a readable non-blocking job summary. Make baseline unavailability explicit without fabricating a comparison.                                                                                        |
| T5  | TODO   | Add focused automated coverage                 | Test discovery, report filtering, artifact/base-summary selection, and ratio decisions: equal, increased, decreased, zero-total, new, removed, renamed, and unavailable baseline.                                                                                                                                     |
| T6  | TODO   | Review first vertical slice                    | Verify runtime, cleanup, summary quality, artifact retention, fork safety, and whether shell remains appropriate. Record observed deltas to recommend a tolerance and required-check decision.                                                                                                                        |
| T7  | TODO   | Reconcile completion records                   | Before final verification, verify issue frontmatter, task/checkpoint/acceptance tables, evidence links, workflow names, and the EPIC #1347 registration agree. Search for stale `TODO`/`IN_PROGRESS` states and provisional wording. |
| T8  | TODO   | Document rollout and verify                    | Update canonical documentation, run required checks, exercise manual pass/fail/exceptional scenarios, and review acceptance criteria. Do not enable a required regression gate without a separate maintainer approval.                                                                                                |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Candidate-mechanism research and controlled prototype evidence | Commit after evidence review. |
| T2 | Selected report-only design decision | Commit after maintainer approval. |
| T3 | Changed-package discovery and source-only summary command | Commit after focused command tests. |
| T4 | Base-summary publication, retrieval, and report-only comparison workflow | Commit after fork-safety and workflow validation. |
| T5 | Focused tests for selection, filtering, and ratio decisions | Commit after test-design review and focused tests. |
| T6 | First-slice runtime, cleanup, and safety review evidence | Documentation-only commit when it records a material decision. |
| T7 | Reconciled completion records | Documentation-only commit before final verification. |
| T8 | Final documentation, verification, and completion evidence | Commit after the final maintainer review. |

## Progress Tracking

### Workflow Checkpoints

- [x] CI workflow, Codecov configuration, and existing coverage commands analyzed before drafting.
- [x] PR #2148 prerequisite merged on 2026-09-06.
- [x] Draft reviewed and approved by user/maintainer.
- [x] GitHub subissue #2222 created under #1347.
- [x] Draft moved to `docs/issues/open/` with assigned issue number.
- [ ] Implementation completed.
- [ ] Automatic verification completed.
- [ ] Manual verification scenarios executed and recorded.
- [ ] Acceptance criteria reviewed after implementation.
- [ ] Implementation completion review recorded.

### Progress Log

- 2026-09-06 - GitHub Copilot - Analyzed current coverage CI before drafting. The PR coverage workflow generates one workspace Codecov report and has no package comparison. The first proposal was a direct changed-package base-vs-head double build in the unprivileged PR workflow.
- 2026-09-06 - User/maintainer - Required the implementation to revalidate the strategy in its current context before committing to it, including a proof of concept where appropriate.
- 2026-09-06 - GitHub Copilot - Revised the draft: begin with current-state revalidation and controlled prototypes. Prefer a report-only, one-build repository-owned summary comparison using the existing workspace report plus a `develop` baseline artifact; assess Codecov components in parallel. Retain the base-worktree double-build only as an evidence-backed fallback.
- 2026-09-15 - GitHub Copilot - Promoted the existing draft from ignored `.tmp/` storage to this canonical EPIC-prefixed folder-style draft path after confirming PR #2148 merged. Refreshed stale metadata and validation guidance; GitHub issue creation awaits maintainer review.
- 2026-09-15 - User/maintainer - Approved the refreshed folder-style draft.
- 2026-09-15 - GitHub Copilot - Created GitHub issue #2222 and moved the approved specification to this canonical open-issue path.

## Acceptance Criteria

- [ ] AC1: Current CI/tooling and candidate mechanisms are revalidated through recorded research
      and a controlled proof of concept before the implementation design is selected.
- [ ] AC2: A stable, always-present, report-only pull-request job compares every directly changed,
      comparable workspace package with the PR base revision.
- [ ] AC3: The comparison measures only each package's `src/` files using the same nightly Cargo
      coverage scope, features, toolchain, OS, and relevant environment at base and head.
- [ ] AC4: The comparison reports exact head/base covered and instrumented line counts and ratio
      deltas without parsing rounded percentages or applying an absolute package threshold.
- [ ] AC5: The job summary reports selected packages, base/head counts and rates, delta, and
      explicit baseline-unavailable or removed-package outcomes.
- [ ] AC6: New, deleted, and unmatched renamed packages do not cause a fabricated comparison;
      baseline unavailability is explicit and does not silently claim a passing comparison.
- [ ] AC7: The report-only check executes only in the unprivileged `pull_request` workflow; the trusted
      `workflow_run` uploader and Codecov artifact flow do not execute fork code or enforce this gate.
- [ ] AC8: Existing workspace-wide coverage reporting remains intact.
- [ ] AC9: Focused tests cover package selection and ratio decisions, including exceptional cases.
- [ ] AC10: The first implementation slice is reviewed for runtime, cleanup, artifact retention,
      maintainability, and fork-security behavior before rollout proceeds.
- [ ] AC11: Observed report-only results support a documented recommendation for tolerance and whether
      a required status is worthwhile; no required gate is enabled without later maintainer approval.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant automated tests pass.
- [ ] Manual verification scenarios are executed and documented.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated for the new CI behavior.

## Verification Plan

### Automatic Checks

- Focused tests for the coverage-regression command.
- `linter yaml`
- `linter all`
- `cargo +nightly fmt --all -- --check`
- `git diff --check`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
- Relevant workflow syntax/action-policy validation.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Command/Steps                                                                                               | Expected Result                                                                                           | Status | Evidence |
| --- | ------------------------------------- | ----------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | One-build comparison proof of concept | Run a controlled changed-package comparison using a `develop` summary artifact and the PR workspace report. | Summary has identical package/source scope, no base rebuild, and a correct delta.                         | TODO   | —        |
| M2  | Codecov components proof of concept   | Apply a disposable or non-blocking component configuration in a controlled branch/repository context.       | Confirm component path and status semantics, timing, and fork-upload compatibility.                       | TODO   | —        |
| M3  | Equal, improved, and reduced coverage | Exercise ratio calculation with controlled report fixtures or changes.                                      | Report-only summary correctly identifies each delta without blocking merge.                               | TODO   | —        |
| M4  | Non-package and unavailable baseline  | Exercise documentation/workspace-config-only changes and new, deleted, or unmatched renamed packages.       | No package is falsely selected; summary explicitly reports non-comparable outcomes.                       | TODO   | —        |
| M5  | Fork pull request                     | Inspect the generated workflow and a fork PR run.                                                           | The report-only check runs in `pull_request` without secrets; the trusted uploader remains artifact-only. | TODO   | —        |

## Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   | —        |
| AC2   | TODO                   | —        |
| AC3   | TODO                   | —        |
| AC4   | TODO                   | —        |
| AC5   | TODO                   | —        |
| AC6   | TODO                   | —        |
| AC7   | TODO                   | —        |
| AC8   | TODO                   | —        |
| AC9   | TODO                   | —        |
| AC10  | TODO                   | —        |
| AC11  | TODO                   | —        |

## Risks and Trade-offs

- Codecov component/status behavior or artifact APIs can change between drafting and implementation.
  Revalidate both with current documentation and a controlled proof of concept before selecting one.
- Base-summary artifacts can expire or be absent for a PR base SHA. The report-only job must report
  that explicitly; use the double-build fallback only if the evidence justifies its extra runtime.
- A strict zero-tolerance gate can reject a healthy refactor that removes covered code and changes
  the denominator. Collect report-only deltas before recommending a tolerance or required status.
- Package-local coverage commands can need controlled services or environment variables. The first
  design must use a documented, deterministic baseline environment and not inherit developer-only
  variables.
- A source-layout change, toolchain behavior change, or test-target selection change can alter a
  measured ratio without indicating a behavioral regression. Exact source filtering and transparent
  base/head counts limit ambiguity, but reviewers remain responsible for interpreting the result.
- This numerical guard can discourage legitimate refactors if treated as a proxy for test quality.
  Keep the #1347 qualitative, behavior-focused review policy authoritative.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions,
material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: Not yet assessed.
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.

## References

- Parent EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- Current PR coverage generator: `.github/workflows/generate_coverage_pr.yaml`
- Trusted coverage uploader: `.github/workflows/upload_coverage_pr.yaml`
- Main-branch coverage workflow: `.github/workflows/coverage.yaml`
- Current Codecov settings: `codecov.yaml`
- Related completed analysis: `docs/issues/closed/2140-1347-review-axum-http-server-integration-tests/coverage-evidence.md`
