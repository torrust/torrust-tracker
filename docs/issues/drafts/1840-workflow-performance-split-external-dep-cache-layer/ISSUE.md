---
doc-type: issue
issue-type: task
status: draft
priority: p4
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-split-external-dep-cache-layer/ISSUE.md
branch: "{issue-number}-split-external-dep-cache-layer"
related-pr: null
last-updated-utc: 2026-06-01 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - Cargo.toml
    - Cargo.lock
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Investigate splitting cook layer to isolate external dependency cache

## Goal

Determine whether the `cargo-chef` cook stage can be split into two independent
Docker layers — one for external (third-party) Cargo dependencies and one for
workspace package stubs — so that external dependency compilation is cached
independently of workspace package structure changes.

## Background

The current [`Containerfile`](../../../../Containerfile) uses `cargo-chef` to
pre-compile all Cargo dependencies before copying real source code. The process
has two steps:

1. `cargo chef prepare` scans every `Cargo.toml` in the workspace and produces
   a `recipe.json` that captures the full dependency graph (both external crates
   and workspace-internal packages) while stripping source code, replacing each
   workspace member's implementation with an empty stub.
2. `cargo chef cook` compiles all external crates using those stubs. The
   resulting compiled artifacts are cached as a Docker layer.

The cook layer is invalidated whenever `recipe.json` changes. `recipe.json`
changes whenever **any** `Cargo.toml` in the workspace changes — including when:

- A workspace package adds, removes, or upgrades an external dependency.
- A new workspace package is added or removed.
- A workspace package's feature flags or other manifest metadata are changed.

Because third-party crate information and workspace package metadata are
entangled in a single recipe, even a pure internal change — for example,
restructuring a workspace package's Cargo.toml without adding any external
dependency — invalidates the entire cook layer. This forces a full
re-compilation of every external crate, even though the external dep versions
have not changed.

This project has 26 workspace packages under `packages/`, plus the root crate.
These packages change frequently; they are tightly coupled to the main binary
and most application logic lives inside them. By contrast, external dependency
versions change only when a developer explicitly updates `Cargo.lock`.

If workspace Cargo.toml changes are significantly more frequent than Cargo.lock
changes, the cook layer may be invalidated far more often than necessary,
undermining the intended caching benefit of `cargo-chef`.

### Preliminary timing analysis

A `cargo timings` run on the full workspace (June 2026) shows that the largest
single contributors to compilation time are C-library build scripts:

| Crate                          | Cook time |
| ------------------------------ | --------- |
| `libsqlite3-sys` build scripts | ~21s      |
| `aws-lc-sys` build script      | ~14s      |
| `zstd-sys` build script        | ~11s      |
| `ring` build script            | ~5s       |

By contrast, workspace package stubs (the empty `src/lib.rs`/`src/main.rs`
shells that `cargo-chef` compiles during cook) are near-zero each — their
full-source compilation times (e.g. `torrust-tracker-core` at 2.4s,
`torrust-tracker-configuration` at 2.1s) are incurred in the `build` stage
**after** the source copy, not in the cook stage.

This finding reduces the expected benefit of a split cook layer: even if the
external-dep layer is perfectly cached, the total cook time saved on a
workspace-`Cargo.toml`-only change is only the sum of workspace **stub**
compilations (likely a few seconds total), not the C build scripts (~51s+).
The C build scripts are external crates and would still execute in the inner
cook layer.

The optimization remains worth investigating only after other higher-impact
changes (target scope narrowing, `.dockerignore` audit, cache reuse policy)
have been applied and workspace-package compilation time becomes a material
fraction of the remaining cook time. See EPIC #1669: if most workspace packages
are extracted as external crates, this issue becomes moot.

### Relationship to EPIC #1669

EPIC #1669 aims to extract several generic workspace packages into standalone
repositories. Once extracted, those packages will be consumed as external crates
and their version bumps will appear in `Cargo.lock` rather than as workspace
`Cargo.toml` edits. This will naturally shift the invalidation trigger toward a
more stable baseline over time. This issue is more valuable in the short term
while the workspace is still large.

### Distinction from existing issues

- `1840-workflow-performance-dependency-layer-cache-reuse`: that issue covers
  the CI-level cache backend (GHA cache keys, BuildKit cache mounts) and whether
  cache entries are being reused across jobs and workflow runs. This issue is
  about the Containerfile layer structure itself — what `cargo chef` stages are
  defined and what invalidates them.

## Scope

### In Scope

- Measure the frequency of cook layer invalidation in recent git history: how
  often do workspace `Cargo.toml` files change without also changing `Cargo.lock`?
- Investigate whether `cargo-chef` supports generating a recipe scoped to
  external dependencies only (excluding workspace members).
- Investigate alternative approaches to separating external dep compilation from
  workspace stub compilation (see Known Candidate Approaches below).
- If a viable approach is found: prototype it and measure the before/after effect
  on warm build times when only a workspace `Cargo.toml` is modified (no new
  external deps).
- Validate that cold build time does not regress.
- If no viable approach is found: document the investigation findings and close
  the issue.

### Out of Scope

- Changes to build targets (covered by the containerfile-target-scope issue).
- CI-level cache backend configuration (covered by dependency-layer-cache-reuse).
- Changes to `Cargo.toml` dependency versions or workspace package structure
  beyond what is needed to validate the prototype.

## Known Candidate Approaches

| ID  | Approach                    | Description                                                                                                               | Feasibility Notes                                                                                        |
| --- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| A1  | `cargo-chef` filter flag    | Use a `cargo chef prepare` option to generate an external-only recipe                                                     | Needs investigation — not documented in `cargo-chef` README as of 2026-06                                |
| A2  | Post-process `recipe.json`  | Strip workspace member entries from `recipe.json` after `cargo chef prepare`                                              | Potentially feasible but fragile; `recipe.json` format is an internal detail of `cargo-chef`             |
| A3  | `cargo fetch` pre-stage     | Copy only `Cargo.toml`/`Cargo.lock`; run `cargo fetch --locked`; cook on top                                              | Pre-fetches source archives but does not compile; may not preserve compiled artifact cache across layers |
| A4  | Minimal synthetic workspace | Construct a synthetic top-level `Cargo.toml` that declares only external deps; cook it first; cook the full recipe on top | Fully separates external vs internal invalidation but adds manifest maintenance overhead                 |

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                     | Notes / Expected Output                                                                                                                    |
| --- | ------ | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Measure cook layer invalidation frequency in git log     | Count commits in the last 6 months that changed a workspace `Cargo.toml` without also changing `Cargo.lock`. Record the ratio.             |
| T2  | TODO   | Investigate `cargo-chef` filter capabilities             | Read `cargo-chef` source and docs; test `cargo chef prepare` options; determine if workspace-member exclusion is natively supported.       |
| T3  | TODO   | Evaluate candidate approaches A1–A4                      | Score each approach for feasibility, complexity, and maintenance cost. Select the most promising for prototyping or conclude not feasible. |
| T4  | TODO   | Prototype the chosen approach (if feasible)              | Build a proof-of-concept Containerfile with a split cook stage; confirm it builds correctly locally.                                       |
| T5  | TODO   | Measure warm build time improvement                      | Run the warm baseline with a workspace `Cargo.toml` change (no new external dep); compare cook stage rebuild time before and after split.  |
| T6  | TODO   | Validate cold build time is unchanged                    | Run the cold baseline; confirm total build time is within measurement noise of the original baseline.                                      |
| T7  | TODO   | Document findings and update Containerfile if beneficial | If split is beneficial: update the Containerfile. If not: write a findings note and close as declined.                                     |

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

- 2026-06-01 00:00 UTC - GitHub Copilot - Drafted cook layer split investigation issue from EPIC #1840 discussion - draft file created
- 2026-06-01 12:00 UTC - GitHub Copilot - Downgraded priority to p4 after cargo timings analysis: C build scripts dominate cook time; workspace stub cost is near-zero; split benefit is marginal until other bottlenecks are resolved first

## Acceptance Criteria

- [ ] AC1: Cook layer invalidation frequency is measured and documented (ratio of workspace-Cargo.toml-only changes vs Cargo.lock changes over the last 6 months).
- [ ] AC2: Feasibility of each candidate approach (A1–A4) is evaluated and a recommendation is documented.
- [ ] AC3: If feasible: a split cook layer is prototyped, builds correctly, and warm build time with a workspace `Cargo.toml`-only change is measured before and after.
- [ ] AC4: If feasible: cold build time does not regress compared to the baseline analysis (`#1841`).
- [ ] AC5: If not feasible or not beneficial: findings are documented and the issue is explicitly closed as declined with a rationale.
- [ ] `linter all` exits with code `0`
- [ ] All CI checks pass for any changes to `Containerfile`
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- CI checks pass for any changes to `Containerfile`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                               | Command/Steps                                                                                                                               | Expected Result                                                                                     | Status | Evidence         |
| --- | ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- | ------ | ---------------- |
| M1  | Measure cook invalidation frequency                    | `git log --oneline --follow --diff-filter=M -- '**/Cargo.toml' Cargo.lock` and classify each change by type                                 | Ratio of workspace-Cargo.toml-only changes vs Cargo.lock changes recorded.                          | TODO   | {analysis link}  |
| M2  | Warm build with workspace `Cargo.toml` change (before) | Modify a workspace package `Cargo.toml` (add a comment or feature flag; no new dep); warm baseline run; record cook stage rebuild duration. | Cook layer fully rebuilt (baseline measurement).                                                    | TODO   | {benchmark link} |
| M3  | Warm build with workspace `Cargo.toml` change (after)  | Same change after implementing the split cook; warm baseline run; record cook stage rebuild duration.                                       | External dep layer preserved; only workspace stubs layer rebuilt. Total cook time noticeably lower. | TODO   | {benchmark link} |
| M4  | Cold build time unchanged                              | Full cold run via `./contrib/dev-tools/workflow-benchmarks/run-container-baseline.sh`                                                       | Total cold build time within measurement noise of baseline from `#1841`.                            | TODO   | {benchmark link} |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence         |
| ----- | ---------------------- | ---------------- |
| AC1   | TODO                   | {analysis link}  |
| AC2   | TODO                   | {analysis link}  |
| AC3   | TODO                   | {benchmark link} |
| AC4   | TODO                   | {benchmark link} |
| AC5   | TODO                   | {findings link}  |
