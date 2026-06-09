---
doc-type: issue
issue-type: task
status: open
priority: p1
github-issue: 1869
spec-path: docs/issues/open/1869-1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md
branch: "{issue-number}-dependency-layer-cache-reuse"
related-pr: null
last-updated-utc: 2026-06-09 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/closed/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
    - docs/issues/drafts/1840-workflow-performance-split-external-dep-cache-layer/ISSUE.md
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

## Update: `--external-only` flag for `cargo-chef` is now implemented

> **2026-06-09** — During investigation of this issue, a native `--external-only` flag was
> implemented for `cargo chef prepare` and published as a temporary fork
> [`torrust-cargo-chef`](https://crates.io/crates/torrust-cargo-chef) v0.1.78
> ([source](https://github.com/torrust/cargo-chef), tag `v0.1.78-torrust`).
> An upstream PR ([#360](https://github.com/LukeMathWalker/cargo-chef/pull/360)) has
> been opened; once merged, we will switch back to the official `cargo-chef`.
>
> The `--external-only` flag strips all `path = "..."` dependency entries from the
> recipe before serialisation, producing a stable third-party-only recipe that only
> changes when an actual external dependency is added, removed, or updated.
>
> This directly resolves T3's concern: with `--external-only`, the cook/build split
> **does** provide meaningful benefit because:
>
> 1. The third-party layer is immune to workspace-internal `Cargo.toml` changes.
> 2. Even if the full cook layer is invalidated by a `Cargo.toml` change, the third-party
>    artifacts survive in the layer below — only the stubs are rebuilt.
> 3. Cold build time does not regress (same number of crates compiled overall).
>
> The draft issue `docs/issues/drafts/1840-workflow-performance-split-external-dep-cache-layer/ISSUE.md`
> was originally investigating this same split as a separate concern. Since the
> `--external-only` approach supersedes that investigation, the draft is being closed
> as superseded (see the draft file for archived investigation notes).

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

| ID  | Status      | Task                                 | Notes / Expected Output                                                                                                                                                                                                                                                                                                                      |
| --- | ----------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE        | Reproduce current cache behavior     | Demonstrated via analysis in the draft issue `docs/issues/drafts/1840-workflow-performance-split-external-dep-cache-layer/ISSUE.md`: workspace `Cargo.toml` changes invalidate the entire cook layer. Confirmed by post-#1853 CI analysis showing workspace-coupling still compiled despite being excluded from final archive.               |
| T2  | DONE        | Identify invalidation inputs         | Root cause identified: `recipe.json` captures both external and workspace `path` dependencies; any workspace `Cargo.toml` change invalidates it. A `torrust-cargo-chef` fork with `--external-only` flag resolves this.                                                                                                                      |
| T3  | IN_PROGRESS | Implement in-workflow reuse strategy | Apply the three-layer cook pattern using `torrust-cargo-chef`'s `--external-only` flag: (1) third-party-only cook, (2) full cook on top, (3) build. Switch `cargo-chef` → `torrust-cargo-chef@0.1.78-torrust` in the Containerfile. Update both debug and release dependency stages. See GitHub comment for the proposed Dockerfile pattern. |
| T4  | TODO        | Validate impact on PR wait time      | Before/after evidence for dependency-stage reuse and effect on end-to-end check completion time.                                                                                                                                                                                                                                             |
| T5  | TODO        | Clean up superseded draft            | Remove `docs/issues/drafts/1840-workflow-performance-split-external-dep-cache-layer/` folder. Its contents are archived in the investigation notes within this spec (T1/T2) and the draft file itself carries a superseded banner linking to this issue.                                                                                     |
| T6  | TODO        | Draft follow-up scope                | Outline a separate follow-up issue for optional cross-workflow cache reuse, including race and sequencing trade-offs.                                                                                                                                                                                                                        |

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
- 2026-06-09 00:00 UTC - GitHub Copilot - Updated spec with `--external-only` cargo-chef flag implementation (published as `torrust-cargo-chef` fork); marked T1/T2 as DONE as they are resolved by the draft and fork; converted T3 from "Propose" to "Implement" with the three-layer cook pattern; closed the duplicate draft `docs/issues/drafts/1840-workflow-performance-split-external-dep-cache-layer/ISSUE.md` as superseded

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

| ID  | Scenario                              | Command/Steps                                                                                                                                                                                                                                                             | Expected Result                                                                                                                                                     | Status | Evidence            |
| --- | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ------------------- |
| M1  | Unchanged-dependency rerun            | Run `docker build` twice targeting `release` with unchanged Cargo dependency inputs and an app-code-only change (e.g., edit a workspace `.rs` file) between runs using the updated Containerfile with `torrust-cargo-chef`. Inspect per-layer build output.               | Third-party cook layer (`dependencies_thirdparty`) is cached and reused; only the full cook and build stages re-execute. Measurable reduction in second build time. | TODO   | {log/output/path}   |
| M2  | Invalidation trigger inspection       | Compare `recipe.json` vs `recipe-thirdparty.json` when only a workspace `Cargo.toml` changes (no `Cargo.lock` change). Confirm `recipe-thirdparty.json` is identical while `recipe.json` differs. Then trace which Docker layers are invalidated.                         | `recipe-thirdparty.json` is stable across workspace-only manifest changes. Docker cache keeps the `dependencies_thirdparty` layer.                                  | TODO   | {analysis link}     |
| M3  | Verify `torrust-cargo-chef` binary    | Build container image locally with the updated Containerfile (target `release`), then run E2E tests against it.                                                                                                                                                           | Image builds successfully with `torrust-cargo-chef`. `test` stage passes. Tracker binary runs and responds to announce requests.                                    | TODO   | {build log}         |
| M4  | Verify `torrust-cargo-chef` debug     | Build container image locally targeting `debug`, then run E2E tests against it.                                                                                                                                                                                           | Debug image builds and tests pass.                                                                                                                                  | TODO   | {build log}         |
| M5  | App-code-only performance improvement | 1. Build container targeting `release` (cold, full build). Record layer timestamps for `dependencies_thirdparty`, `dependencies`, and `build`. 2. Make a source-only change (e.g., add a comment to `src/lib.rs`). 3. Rebuild. Compare per-layer timestamps between runs. | Third-party layer is zero-cost on the rebuild. Total build time is reduced by the third-party compile time (tens of seconds depending on dependency count).         | TODO   | {timing comparison} |
| M6  | Critical-path impact check            | Compare before/after end-to-end wait time until all required checks finish.                                                                                                                                                                                               | Improvement is documented on user-facing wait time while keeping workflow concurrency.                                                                              | TODO   | {benchmark link}    |
| M7  | Follow-up definition                  | Capture candidate cross-workflow reuse options, including optional sequential orchestration, in a follow-up issue draft.                                                                                                                                                  | Follow-up scope is explicit and does not block this issue.                                                                                                          | TODO   | {draft link}        |

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
