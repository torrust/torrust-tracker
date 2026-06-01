---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-recipe-stage-manifest-only-copy/ISSUE.md
branch: "{issue-number}-recipe-stage-manifest-only-copy"
related-pr: null
last-updated-utc: 2026-06-01 00:00
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
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Restrict recipe stage to manifest-only COPY to prevent spurious cook cache invalidation

## Goal

Prevent the `cargo chef cook` (dependency) layers from being invalidated on
every source code change by replacing the full-tree `COPY . /build/src` in the
`recipe` stage with a manifest-only copy of `Cargo.toml` and `Cargo.lock` files.

## Background

The current [`Containerfile`](../../../../Containerfile) `recipe` stage does:

```dockerfile
FROM chef AS recipe
WORKDIR /build/src
COPY . /build/src          # copies the entire source tree
RUN cargo chef prepare --recipe-path /build/recipe.json
```

The `cargo chef prepare` command only reads `Cargo.toml` manifests and
`Cargo.lock` to build `recipe.json`. It does not read any `.rs` source files.
This is explicitly stated in `cargo-chef`'s own CLI description:

> "Analyze the current project to determine the **minimum subset of files
> (Cargo.lock and Cargo.toml manifests)** required to build it and cache
> dependencies"

However, because `COPY . /build/src` copies all source files into the recipe
stage, Docker invalidates that layer's cache whenever **any tracked file
changes** — including `.rs` files, documentation, shell scripts, and anything
else in the build context. Since the recipe stage is upstream of both
`dependencies` and `dependencies_debug` cook stages, this cascades:

```text
COPY . /build/src  ← cache miss on any file change
  → cargo chef prepare  → recipe.json changes (or not — Docker can't tell)
    → COPY --from=recipe recipe.json  ← invalidated regardless
      → cargo chef cook               ← full external dep recompile
```

The cook stage recompiles everything: C build scripts (`libsqlite3-sys` ~21s,
`aws-lc-sys` ~14s, `zstd-sys` ~11s, `ring` ~5s) and hundreds of Rust crates.
On a warm run where only application code changed, this cost is paid
unnecessarily every time.

### The fix

Replace the full-tree copy with a manifest-only copy in the recipe stage:

```dockerfile
FROM chef AS recipe
WORKDIR /build/src
COPY Cargo.toml Cargo.lock ./
COPY packages/axum-health-check-api-server/Cargo.toml packages/axum-health-check-api-server/
COPY packages/axum-http-server/Cargo.toml packages/axum-http-server/
COPY packages/axum-rest-api-server/Cargo.toml packages/axum-rest-api-server/
COPY packages/axum-server/Cargo.toml packages/axum-server/
COPY packages/clock/Cargo.toml packages/clock/
COPY packages/configuration/Cargo.toml packages/configuration/
COPY packages/events/Cargo.toml packages/events/
COPY packages/http-protocol/Cargo.toml packages/http-protocol/
COPY packages/http-tracker-core/Cargo.toml packages/http-tracker-core/
COPY packages/located-error/Cargo.toml packages/located-error/
COPY packages/metrics/Cargo.toml packages/metrics/
COPY packages/net-primitives/Cargo.toml packages/net-primitives/
COPY packages/peer-id/Cargo.toml packages/peer-id/
COPY packages/primitives/Cargo.toml packages/primitives/
COPY packages/rest-api-client/Cargo.toml packages/rest-api-client/
COPY packages/rest-api-core/Cargo.toml packages/rest-api-core/
COPY packages/server-lib/Cargo.toml packages/server-lib/
COPY packages/swarm-coordination-registry/Cargo.toml packages/swarm-coordination-registry/
COPY packages/test-helpers/Cargo.toml packages/test-helpers/
COPY packages/torrent-repository-benchmarking/Cargo.toml packages/torrent-repository-benchmarking/
COPY packages/tracker-client/Cargo.toml packages/tracker-client/
COPY packages/tracker-core/Cargo.toml packages/tracker-core/
COPY packages/udp-protocol/Cargo.toml packages/udp-protocol/
COPY packages/udp-server/Cargo.toml packages/udp-server/
COPY packages/udp-tracker-core/Cargo.toml packages/udp-tracker-core/
COPY console/tracker-client/Cargo.toml console/tracker-client/
COPY contrib/bencode/Cargo.toml contrib/bencode/
COPY contrib/dev-tools/analysis/workspace-coupling/Cargo.toml contrib/dev-tools/analysis/workspace-coupling/
RUN cargo chef prepare --recipe-path /build/recipe.json
```

After this change, the recipe stage cache (and therefore the cook layers) is
only invalidated when `Cargo.toml` or `Cargo.lock` actually changes — not on
every `.rs` edit. For a typical PR that modifies only source code, the cook
layers remain fully cached.

### Maintenance cost

The manifest-only COPY list must be kept in sync with the workspace member list
in the root `Cargo.toml`. Every time a new workspace package is added or an
existing one is moved or removed, the Containerfile must be updated. The
`cargo-chef` documentation acknowledges this trade-off; it uses `COPY . .` in
its canonical example purely for simplicity and portability. This project's
workspace is relatively stable (packages are being extracted to separate repos
under EPIC #1669, reducing the list over time), so the maintenance overhead is
low and proportional to how often the workspace structure changes.

A CI check that validates all workspace member directories have a corresponding
`COPY` line in the Containerfile can catch drift automatically.

### Distinction from existing issues

- `1840-workflow-performance-dockerignore-audit`: that issue reduces the build
  context size (bytes transferred to the BuildKit daemon) and reduces spurious
  invalidation of the `build` and `test` stages. This issue prevents spurious
  invalidation of the `recipe` and `cook` stages, which is a separate and
  higher-value fix: the cook stages contain the entire external dependency
  compilation cost (~200–400s).
- `1840-workflow-performance-dependency-layer-cache-reuse`: that issue covers
  the CI-level cache backend (GHA cache keys, BuildKit cache mounts). This
  issue is about the Containerfile layer structure itself.

## Scope

### In Scope

- Replace `COPY . /build/src` in the `recipe` stage with individual
  `COPY <manifest> <dest>/` lines for every workspace member.
- Verify that `cargo chef prepare` produces an equivalent `recipe.json` with
  the manifest-only copy.
- Verify that the full build pipeline (all Containerfile targets) still works
  end-to-end after the change.
- Measure warm build time before and after with a source-only change (no
  `Cargo.toml` or `Cargo.lock` modification) to confirm cook layers are cached.
- Document the maintenance requirement (keeping manifest list in sync).
- Optionally: add a CI check or script to verify that every workspace member in
  `Cargo.toml` has a corresponding `COPY` line in the Containerfile.

### Out of Scope

- Changing the `build` or `test` stage `COPY . /build/src` instructions (those
  require the full source tree and cannot be restricted without a larger
  redesign).
- Changes to `.dockerignore` (covered by the `dockerignore-audit` issue).
- Cross-workflow cache backend configuration.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                           | Notes / Expected Output                                                                                                           |
| --- | ------ | -------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Replace full-tree COPY with manifest-only COPY in recipe stage | One `COPY <manifest> <dest>/` line per workspace member, plus root `Cargo.toml` and `Cargo.lock`.                                 |
| T2  | TODO   | Verify recipe.json equivalence                                 | Build locally; diff `recipe.json` output before and after to confirm it is identical.                                             |
| T3  | TODO   | Verify full build pipeline                                     | Run `docker build --target release .` locally; confirm all stages succeed.                                                        |
| T4  | TODO   | Measure warm build time improvement                            | Run warm baseline (`run-container-baseline.sh`) with a source-only change; confirm cook layers show cache hit; record time saved. |
| T5  | TODO   | Document maintenance requirement in Containerfile              | Add inline comment above the manifest COPY block explaining the sync requirement.                                                 |
| T6  | TODO   | Optionally add CI drift check                                  | Script or CI step that compares workspace members in `Cargo.toml` against `COPY` lines in `Containerfile` and fails on mismatch.  |

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

- 2026-06-01 00:00 UTC - GitHub Copilot - Drafted recipe stage manifest-only copy issue from EPIC #1840 discussion - draft file created

## Acceptance Criteria

- [ ] AC1: The `recipe` stage uses manifest-only COPY (no full-tree copy); every workspace member `Cargo.toml` and root `Cargo.lock` is explicitly listed.
- [ ] AC2: `recipe.json` produced by the new stage is identical to the one produced by the old full-tree copy stage (verified by diff).
- [ ] AC3: Full build pipeline (`docker build --target release .`) completes successfully with no regressions.
- [ ] AC4: Warm baseline run with a source-only change shows cook layers hitting cache; time saved is recorded.
- [ ] AC5: Containerfile contains an inline comment documenting the manifest list maintenance requirement.
- [ ] `linter all` exits with code `0`
- [ ] All CI checks pass for the changed `Containerfile`
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- All CI checks pass for the changed `Containerfile`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                    | Command/Steps                                                                                                             | Expected Result                                                                                    | Status | Evidence         |
| --- | ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | ------ | ---------------- |
| M1  | Diff recipe.json before and after           | Build with old Containerfile; save `recipe.json`; build with new; diff both files.                                        | Files are identical.                                                                               | TODO   | {diff output}    |
| M2  | Full cold build succeeds                    | `docker build --target release --no-cache .`                                                                              | All stages complete; release image produced.                                                       | TODO   | {log path}       |
| M3  | Warm build with source-only change          | Edit a `.rs` file (no manifest change); run `./contrib/dev-tools/workflow-benchmarks/run-container-baseline.sh` warm run. | Cook stages show `CACHED` in BuildKit output; total warm build time significantly lower than cold. | TODO   | {benchmark link} |
| M4  | Cook layer invalidated on Cargo.toml change | Edit a workspace `Cargo.toml` (add/remove a feature flag); warm run.                                                      | Cook stages are rebuilt (expected); confirm the invalidation is correct and deliberate.            | TODO   | {benchmark link} |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence         |
| ----- | ---------------------- | ---------------- |
| AC1   | TODO                   | {diff link}      |
| AC2   | TODO                   | {diff link}      |
| AC3   | TODO                   | {CI run link}    |
| AC4   | TODO                   | {benchmark link} |
| AC5   | TODO                   | {diff link}      |
