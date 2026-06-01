---
doc-type: epic
status: planned
github-issue: 1840
spec-path: docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-06-01 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/README.md
    - docs/issues/drafts/README.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# EPIC #1840 - Improve PR Workflow Performance

## Goal

Reduce the execution time of the critical PR validation workflows, especially [`.github/workflows/container.yaml`](../../../../.github/workflows/container.yaml) and [`.github/workflows/testing.yaml`](../../../../.github/workflows/testing.yaml), so maintainers and contributors can get faster feedback without compromising verification quality.

## Why This Is Needed

These workflows are among the most important checks in the repository. They run automatically when a PR is opened and before code changes can be merged, so their runtime directly affects how quickly we can trust a change.

Recent runs on shared runners are slow enough to create a merge bottleneck:

- container workflow: 34m 57s
- testing workflow: 40m 44s

That delay encourages batching unrelated changes into larger PRs just to avoid repeated waiting. It also increases the cost of iterative review, especially now that AI agents are used to help produce changes and the project needs strong regression protection.

The problem is not only speed in the abstract. Slow checks reduce review throughput, make small follow-up fixes more painful, and weaken the feedback loop that keeps the project healthy.

## Scope

### In Scope

- Measure and explain the main runtime contributors in the two workflows.
- Keep a durable benchmark report around while the EPIC is active so each improvement can be compared against previous runs.
- Identify and prioritize improvements that shorten total wall-clock time or reduce idle waiting.
- Optimize for end-to-end PR wait time until all required checks complete, not just summed compute time across workflows.
- Preserve useful workflow concurrency unless data proves a sequencing change reduces end-user wait time.
- Keep the workflows trustworthy for PR validation and preserve the quality gates they enforce.
- Document any workflow changes that affect maintainers or contributors.
- Capture subissues as discrete, ordered improvements that can be delivered one at a time.

### Out of Scope

- Removing critical verification steps without an agreed replacement.
- Changing the overall PR validation policy without explicit maintainer approval.
- Optimizing unrelated workflows unless they directly affect these two critical paths.
- Prematurely changing multiple workflow areas at once before measuring impact.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Ordering policy:

- Subissue 1 (baseline analysis) is mandatory first.
- All later subissues are provisional and may be reordered based on baseline findings.

| Order | Issue                                                                                                    | Local Spec                                                                                     | Status | Notes                                                                                                                                                                                                                                                                                                                                  |
| ----- | -------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | #1841 - Baseline workflow profiling and bottleneck analysis                                              | `docs/issues/open/1841-1840-workflow-performance-baseline-analysis/ISSUE.md`                   | DONE   | Merged in PR #1848. Baseline report at `docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md`.                                                                                                                                                                                              |
| 2     | #1852 - Restrict recipe stage to manifest-only COPY                                                      | `docs/issues/open/1852-1840-workflow-performance-recipe-stage-manifest-only-copy/ISSUE.md`     | TODO   | Replace `COPY . /build/src` in the `recipe` stage with per-manifest COPY lines so the cook (dependency) layers are only invalidated when `Cargo.toml` or `Cargo.lock` changes, not on every `.rs` edit. High expected impact.                                                                                                          |
| 3     | #1851 - Audit `.dockerignore` to minimize Docker build context                                           | `docs/issues/open/1851-1840-workflow-performance-dockerignore-audit/ISSUE.md`                  | TODO   | Systematically exclude tracked repo paths not needed in any Containerfile stage to reduce context transfer size and reduce spurious cache invalidation of `build` and `test` stages.                                                                                                                                                   |
| 4     | #1853 - Narrow Containerfile build targets to tracker image needs                                        | `docs/issues/open/1853-1840-workflow-performance-containerfile-target-scope/ISSUE.md`          | TODO   | Execute only if baseline confirms significant time spent compiling or linking targets not required for the final tracker image.                                                                                                                                                                                                        |
| 5     | #1726 - Reduce Build Times with `sccache`                                                                | `docs/issues/open/1726-1840-workflow-performance-sccache/ISSUE.md`                             | TODO   | Existing GitHub issue; link it as a child issue after the EPIC is published. Order is provisional after baseline.                                                                                                                                                                                                                      |
| 6     | #1854 - Evaluate test execution policy in container image build                                          | `docs/issues/open/1854-1840-workflow-performance-container-test-gating/ISSUE.md`               | TODO   | Assess whether test execution inside container build is redundant, evaluate separating validation from packaging across multiple artifact types, and define safer gating plus optional debug-image paths for failing commits.                                                                                                          |
| 7     | #[To be assigned] - Improve dependency-layer cache reuse within each workflow                            | `docs/issues/drafts/1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md`           | TODO   | Ensure dependency layers are reused reliably inside each workflow when Cargo dependencies are unchanged. Defer optional cross-workflow cache-sharing and sequencing trade-offs to follow-up once this is working.                                                                                                                      |
| 8     | #[To be assigned] - Evaluate removing duplicate container build from container workflow                  | `docs/issues/drafts/1840-workflow-performance-container-workflow-build-deduplication/ISSUE.md` | TODO   | Assess whether PR-time container build in container workflow is redundant because testing workflow already builds an image for Docker E2E, and keep publish paths intact.                                                                                                                                                              |
| 9     | #[To be assigned] - Switch to a faster linker (mold or lld) to reduce link time                          | `docs/issues/drafts/1840-workflow-performance-alternative-linker/ISSUE.md`                     | TODO   | Baseline shows 35–117 s link time per binary (sections: null). Fair local relink: BFD = mold (54 s each) — compile dominates incremental builds. mold docs: 10–31× faster than BFD in cold builds (MySQL: 10.8 s → 0.46 s). 20+ binaries linked in container build.                                                                    |
| 10    | #[To be assigned] - Investigate splitting cook layer to isolate external dependency cache (p4, deferred) | `docs/issues/drafts/1840-workflow-performance-split-external-dep-cache-layer/ISSUE.md`         | TODO   | Low priority. C build scripts dominate cook time; workspace stub cost is near-zero. Revisit once other bottlenecks are resolved and workspace shrinks via EPIC #1669.                                                                                                                                                                  |
| 11    | #[To be assigned] - Publish stable base stages as pre-built Docker Hub images (p3, deferred)             | `docs/issues/drafts/1840-workflow-performance-prebuilt-base-images/ISSUE.md`                   | TODO   | Low priority. Base stages (`chef`, `tester`, `gcc`) are fast (3–7 min cold). Compile dominates (35+ min). Revisit if base stages grow or if CI runner cold-cache frequency increases.                                                                                                                                                  |
| 12    | #[To be assigned] - Pass Cargo registry/git caches into BuildKit cook stages                             | `docs/issues/drafts/1840-workflow-performance-buildkit-cargo-cache-mounts/ISSUE.md`            | TODO   | Adds `--mount=type=cache` for registry/git to cook stages. Local benefit: saves ~7 s download per cook rebuild (cold fetch 6.9 s → warm 0.16 s; registry 823 MB). CI benefit: none with ephemeral GitHub Actions runners (`type=gha` layer cache does not persist cache mount volumes). Evaluate target-dir cache mount variant as T5. |

## Delivery Strategy

This EPIC should proceed in small measurement-driven steps. The first objective is to understand where the time goes in the current workflows. After that, each subissue should target one bottleneck at a time so the impact of each change is observable and reversible if needed.

Performance decisions in this EPIC should prioritize user-facing wait time: the key metric is wall-clock time until all required PR checks complete. Reducing aggregate compute cost is welcome, but not at the expense of slower critical-path completion.

The baseline analysis is not a one-off report. Its benchmark artifact should remain in the subissue folder and be updated whenever a later optimization changes the performance profile, so the EPIC keeps a stable before/after comparison history.

One of the planned child issues is already tracked in GitHub as #1726. Once this EPIC is published, that issue should be linked as a subissue instead of being re-drafted here.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.

### Phase 1

- Outcome: establish a trustworthy baseline for the current workflows and identify the largest sources of delay.
- Exit criteria: the runtime contributors are documented well enough to choose the first optimization with confidence, and the baseline report contains both no-cache and warm-cache measurements.

### Phase 2

- Outcome: implement and validate the highest-value workflow improvement selected from baseline findings.
- Exit criteria: the change measurably improves one or both workflows without weakening verification coverage.

### Phase 3

- Outcome: continue with the next highest-value improvement based on measured results.
- Exit criteria: the workflows are faster, the change history is traceable, and any remaining bottlenecks are explicitly documented.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [ ] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-05-27 00:00 UTC - GitHub Copilot - Drafted the initial EPIC spec for PR workflow performance improvements - draft file created
- 2026-05-27 00:00 UTC - GitHub Copilot - Refined the EPIC to require a persistent baseline benchmark report and a measured first subissue - draft updated
- 2026-05-27 00:00 UTC - GitHub Copilot - Clarified that only baseline order is fixed and made later optimization order provisional - draft updated
- 2026-05-27 00:00 UTC - GitHub Copilot - Created GitHub EPIC issue #1840 and moved spec to `docs/issues/open/` - draft updated
- 2026-05-27 00:00 UTC - GitHub Copilot - Created baseline subissue #1841 and linked it as a GitHub child issue of #1840 - draft updated
- 2026-06-01 00:00 UTC - GitHub Copilot - Marked #1841 DONE (merged PR #1848); added sub-issues: recipe-stage-manifest-only-copy (p1), dockerignore-audit (p2), split-external-dep-cache-layer (p4 deferred); reordered table by expected impact
- 2026-06-01 00:00 UTC - GitHub Copilot - Added sub-issues: alternative-linker (p1, row 9), prebuilt-base-images (p3 deferred, row 11)
- 2026-06-01 00:00 UTC - GitHub Copilot - Added sub-issue: buildkit-cargo-cache-mounts (p2, row 12); local benchmark: cold fetch 6.9 s → warm 0.16 s; CI limitation documented
- 2026-06-01 00:00 UTC - GitHub Copilot - Promoted rows 2/3/4/6 from drafts to open: #1851 dockerignore-audit, #1852 recipe-manifest-only-copy, #1853 containerfile-target-scope, #1854 container-test-gating

## Acceptance Criteria

- [ ] The EPIC clearly explains why the two workflows are a project health priority.
- [ ] The EPIC identifies the current runtime pain points with concrete evidence.
- [ ] The EPIC requires a durable baseline benchmark report that can be reused for later comparisons.
- [ ] The EPIC keeps the optimization scope focused on measurable workflow improvements.
- [ ] The EPIC can be extended with prioritized subissues as new ideas are reviewed.
- [ ] Each completed subissue records automated verification evidence.
- [ ] Each completed subissue records manual verification evidence.
- [ ] Each completed subissue includes a post-implementation acceptance criteria review.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                            |
| ----- | ---------------------- | --------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | Draft references the two critical workflows and their current runtimes.                             |
| AC2   | DONE                   | Scope and delivery strategy are intentionally open-ended so subissues can be prioritized later.     |
| AC3   | DONE                   | The EPIC now requires a persistent baseline benchmark report that is updated as optimizations land. |
| AC4   | TODO                   | To be filled after the first profiling and optimization subissue is completed.                      |

## Risks and Trade-offs

- Risk: optimizing the wrong step first could save little time. Mitigation: begin with measured baseline profiling and one change at a time.
- Risk: shortening the workflows by skipping checks would reduce confidence. Mitigation: preserve validation intent and only replace steps with equivalent coverage when justified.
- Risk: workflow changes may affect contributor expectations. Mitigation: document behavior changes in the spec and in workflow docs when needed.

## References

- Related issues: #1726, #1841
- Related PRs: #TBD
- Related ADRs: #TBD
