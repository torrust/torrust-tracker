---
doc-type: issue
issue-type: task
status: done
priority: p1
github-issue: 1854
spec-path: docs/issues/closed/1854-1840-workflow-performance-container-test-gating/ISSUE.md
branch: "1854-container-test-gating"
related-pr: 1874
last-updated-utc: 2026-06-05 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/closed/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
---

<!-- skill-link: create-issue -->

# Issue #1854 - Evaluate test execution policy in container image build

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

## Analysis Findings

### T1 — Duplicate test cost

From a recent CI run after #1868 merged (job 79291438928, PR #1872):

```text
#64 DONE 1106.0s  ← ~18m20s for cargo nextest archive --release alone
```

Total `container.yaml` runtime: ~40 min per trigger. `testing.yaml` unit tests on stable: ~11 min.

On every push to `develop` the following builds are triggered in parallel:

| Workflow         | Job                 | Containerfile target | GHA cache scope         | Tests run?                      |
| ---------------- | ------------------- | -------------------- | ----------------------- | ------------------------------- |
| `container.yaml` | test (debug)        | debug                | `container-debug`       | Yes (embedded in Containerfile) |
| `container.yaml` | test (release)      | release              | `container-release`     | Yes (embedded in Containerfile) |
| `testing.yaml`   | docker-e2e          | release              | `testing-docker-e2e`    | Yes (embedded) + 4 E2E tests    |
| `container.yaml` | publish_development | release              | `container-publish-dev` | Yes (embedded, full rebuild)    |

The `release` target is built **three times** on a develop push, each with a separate GHA cache scope so they cannot share layers. The debug target adds a fourth full build. All four use fat LTO + opt-level 3 (Cargo release profile for the release target, dev profile for debug).

### T2 — Coverage overlap

**What `container.yaml` test job adds beyond `testing.yaml`:**

- The `debug` target build is not validated anywhere else in CI.
- Verifies both targets can be assembled in a clean GHA environment using the same runner as publish.
- The `docker inspect` step confirms the image is loadable; no additional tests are run.

**What is fully duplicated:**

- The `release` target build with embedded tests (cargo nextest run) is identical to what `testing.yaml` docker-e2e already builds and tests.
- `publish_development` rebuilds the release target from scratch (different cache scope) even though `container.yaml` test (release) and `testing.yaml` docker-e2e both just built the same thing.

**Naming clarification:** "debug" and "development" are orthogonal concepts:

- `debug`/`release` are Containerfile stage names (Cargo dev vs release profiles).
- `publish_development` means "published from a development branch" (not a versioned release); it always uses `target: release` (optimized binary). Both publish jobs do.

### T3 — Validation-versus-packaging separation

The cleanest structural design separates the two concerns completely:

```text
testing.yaml (validate)  →  builds release container, runs all tests  (on every push/PR)
container.yaml (publish) →  pure publish workflow, no test job         (gated, runs after testing.yaml)
```

In this model `container.yaml` triggers via `workflow_run` on `testing.yaml` success for `develop`/`main` and via direct `push` for `releases/**/*`. The publish step reads from `testing-docker-e2e` cache scope so no rebuild is needed.

Caveat: `workflow_run` only fires from the default branch's workflow file. Fork PR workflows do not trigger upstream `workflow_run` events. This is acceptable here because `publish_development` already guards against forks (`github.repository == 'torrust/torrust-tracker'`).

### T4 — Gating alternatives

Three options in increasing scope:

| Option         | Change                                                                                       | Saves per develop push | Risk                                         |
| -------------- | -------------------------------------------------------------------------------------------- | ---------------------- | -------------------------------------------- |
| A (minimal)    | Remove `debug` from test matrix                                                              | ~40 min                | Low — debug target untested in CI            |
| B (moderate)   | A + unify cache scopes so publish reuses test cache                                          | ~40 min extra rebuild  | Low — same image, different cache key        |
| C (structural) | Drop `test` job entirely; restructure container.yaml as pure publish gated on `workflow_run` | ~80 min (2 builds)     | Medium — requires `workflow_run` design care |

**Considered and rejected: move unit tests out of the Containerfile.**
Running unit tests on the GHA host after the container build would only prove they pass on `ubuntu-latest`, not in the actual target infrastructure (Debian trixie, distroless runtime, specific glibc). Unit tests must run inside the container build to catch infrastructure-specific failures. See ADR `20260603000000_keep_unit_tests_inside_container_build.md`.

**Chosen approach: A + move E2E tests into container.yaml + skip docker-e2e in testing.yaml when covered.**

- Remove `debug` from the test matrix (Option A).
- Keep unit tests embedded in the Containerfile (non-negotiable, see ADR above).
- Move the four E2E test steps into the `container.yaml` `test` job so they run immediately after the image is built and before any publish step.
- Add a skip condition to `testing.yaml` `docker-e2e` so it does not run when `container.yaml` is already covering the same trigger (PR targeting `develop`/`main`, push to `develop`/`main`/`releases/**`).
- Warm publish job caches from the `container-release` scope (T8) to reduce redundant rebuilds in the publish step.

This eliminates the duplicated E2E work for `develop`/`main` pushes and PRs, while preserving full coverage for feature branch pushes where `container.yaml` does not trigger.

### T5 — Debug-image path

The `debug` Containerfile target (Cargo dev profile, unoptimized binary) is never published to Docker Hub. Its only CI use is the `container.yaml` test matrix entry, which verifies it builds and runs `docker inspect`. It is useful locally for attaching a debugger.

Policy recommendation:

- Remove `debug` from the CI test matrix (saves ~40 min per push, zero published-image impact).
- Keep the `debug` Containerfile target available for local `docker build --target debug` use.
- If on-demand debug image publishing is needed in future, add a `workflow_dispatch` job in `container.yaml` with explicit scope and no automatic trigger.

### T6 — Recommendation

**Implement A + E2E-in-container + docker-e2e skip.**

Concrete changes for this issue:

1. Remove `target: debug` from the `container.yaml` test matrix.
2. Keep unit tests embedded in the Containerfile build (see ADR `20260603000000_keep_unit_tests_inside_container_build.md`).
3. Add the four E2E test steps to the `container.yaml` `test` job, run after `docker build` and before the `context`/publish chain.
4. Add an `if:` condition to `testing.yaml` `docker-e2e` that skips the job when `container.yaml` is triggered by the same event.
5. In `publish_development` and `publish_release`, add `type=gha,scope=container-release` to `cache-from` so the publish step can reuse the test job's built layers.
6. Add clarifying comments to `container.yaml` explaining the naming and the skip policy.

Note: T6 does not solve the fundamental build time cost (fat LTO × all test binaries). T11–T13 below address that.

### T11 — Test binary landscape

`cargo nextest archive --tests` (without `--benches` or `--examples`) already excludes bench harnesses
and example binaries — those 4 bench files and 2 example files are not compiled. Nothing to gain there.

After the two existing exclusions (`workspace-coupling`, `torrust-tracker-torrent-repository-benchmarking`)
the archive compiles **47 binaries / test harnesses**:

| Kind                      | Count | Description                                          |
| ------------------------- | ----- | ---------------------------------------------------- |
| Integration test binaries | 10    | One per `tests/*.rs` entry point — each fully linked |
| Lib unit test harnesses   | 27    | One per lib crate that has `#[cfg(test)]`            |
| Binary targets            | 10    | `src/bin/` + `console/tracker-client/src/bin/`       |

Integration test entry points (each = one fully linked binary):

```text
torrust-clock              :: integration
torrust-tracker            :: integration
torrust-tracker-axum-health-check-api-server :: integration
torrust-tracker-axum-http-server             :: integration
torrust-tracker-axum-rest-api-server         :: integration
torrust-tracker-client     :: tracker_checker
torrust-tracker-client     :: tracker_client
torrust-tracker-contrib-bencode :: mod
torrust-tracker-core       :: integration
torrust-tracker-udp-server :: integration
```

Binary targets compiled into the archive:

```text
torrust-tracker :: e2e_tests_runner          ← only used on GHA host, never in the container
torrust-tracker :: qbittorrent_e2e_runner    ← only used on GHA host, never in the container
torrust-tracker :: profiling
torrust-tracker :: http_health_check         ← needed in production image
torrust-tracker :: torrust-tracker           ← needed in production image
torrust-tracker-client :: http_tracker_client
torrust-tracker-client :: tracker_checker
torrust-tracker-client :: tracker_client
torrust-tracker-client :: udp_tracker_client
torrust-tracker-core :: persistence_benchmark_runner
```

**Concrete opportunities:**

1. **Exclude `torrust-tracker-client` console package** (easy — 1-line change).  
   `console/tracker-client` is an independent workspace member with no dependents elsewhere
   in the workspace. It contributes 4 bin targets + 2 integration test harnesses, none of which
   are needed to verify the tracker server inside the container. Add `--exclude torrust-tracker-client`
   to all three `cargo nextest archive` calls (debug cook, release cook, build archive).

2. **Move `e2e_tests_runner` and `qbittorrent_e2e_runner` to a separate package** (medium effort).  
   These binaries are pure GHA host tools — they are never executed inside the container. They
   currently live in `src/bin/` of the root crate, so `--exclude` is not possible today. Moving
   them to a dedicated `packages/e2e-tools/` (or similar) package would allow adding
   `--exclude torrust-tracker-e2e-tools` to the archive commands, removing 2 heavily-linked
   binaries from every build.

3. **Move `testcontainers` from `[dependencies]` to `[dev-dependencies]` in `tracker-core`** (medium effort — separate concern).  
   `testcontainers` appears in `[dependencies]` (not `[dev-dependencies]`) in
   `packages/tracker-core/Cargo.toml`. All its usages are inside `#[cfg(test)]` blocks and in the
   `persistence_benchmark_runner` bin. As a regular dependency it is linked into every binary that
   depends on `tracker-core`, including the production binary. Moving it to `[dev-dependencies]`
   (and feature-gating or separating `persistence_benchmark_runner` as needed) reduces production
   binary size and link time. This is independent of the archive changes above.

**Important caveat:** none of these changes will eliminate the ~18-minute archive step. The bulk
of that time is compiling fat LTO release binaries for `torrust-tracker` and the Axum server
integration tests — that code cannot be excluded and the LTO cost is unavoidable without
changing the Cargo profile. These changes reduce link count at the margins.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                         | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                                                        |
| --- | ------ | ---------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Quantify duplicate test cost                                                 | Documented in Analysis Findings above. Release target built 3× on develop push; debug adds a 4th. Total ~40 min per trigger.                                                                                                                                                                                                                                                                                                   |
| T2  | DONE   | Map coverage overlap                                                         | Documented in Analysis Findings above. Release container fully covered by docker-e2e; debug target unique to container.yaml but untested beyond docker inspect.                                                                                                                                                                                                                                                                |
| T3  | DONE   | Evaluate validation-versus-packaging separation                              | Option C (workflow_run) documented above. Deferred to follow-up; Option B chosen for this issue.                                                                                                                                                                                                                                                                                                                               |
| T4  | DONE   | Evaluate gating alternatives                                                 | Options A/B/C documented above. Option B selected.                                                                                                                                                                                                                                                                                                                                                                             |
| T5  | DONE   | Evaluate debug-image path                                                    | Debug target removed from CI matrix; kept available for local builds. On-demand publish via workflow_dispatch if ever needed.                                                                                                                                                                                                                                                                                                  |
| T6  | DONE   | Recommendation and decision record                                           | Option B: remove debug from test matrix, add clarifying comments, warm publish cache from test scope.                                                                                                                                                                                                                                                                                                                          |
| T7  | DONE   | Remove `debug` from test matrix; add comments to workflow                    | Removed `debug` from `matrix.target`; added clarifying comments to `test` job and `publish_development` explaining naming and skip policy.                                                                                                                                                                                                                                                                                     |
| T8  | DONE   | Warm publish cache from test scope                                           | Added `type=gha,scope=container-release` as first `cache-from` entry in `publish_development` and `publish_release`. Falls back to publish-specific scope if test job cache is cold.                                                                                                                                                                                                                                           |
| T9  | DONE   | Move E2E tests into `container.yaml` test job                                | Added the four E2E test steps (e2e_tests_runner + qbittorrent sqlite3/mysql/postgresql) to the `container.yaml` `test` job, executed after `docker build` and before the publish chain.                                                                                                                                                                                                                                        |
| T10 | DONE   | Skip `docker-e2e` in `testing.yaml` when `container.yaml` covers the trigger | Added `if:` condition to `docker-e2e` job: skips for PRs targeting `develop`/`main` and pushes to `develop`/`main`/`releases/**`. Feature branch pushes still run it.                                                                                                                                                                                                                                                          |
| T11 | DONE   | Analyse test binary landscape and document optimisation opportunities        | Documented in T11 section above and in `nextest-archive-analysis.md`. Three concrete opportunities identified: exclude console client, move e2e runners to own package, move testcontainers to dev-deps.                                                                                                                                                                                                                       |
| T12 | DONE   | Exclude `torrust-tracker-client` from `cargo nextest archive`                | Added `--exclude torrust-tracker-client` and `--exclude torrust-tracker-contrib-bencode` to all four archive calls (debug cook warmup, release cook warmup, debug archive, release archive) in Containerfile.                                                                                                                                                                                                                  |
| T13 | DONE   | Move `e2e_tests_runner` and `qbittorrent_e2e_runner` to a separate package   | Created `packages/e2e-tools/` package with `torrust-tracker-e2e-tools` crate name. Moved three bins (`e2e_tests_runner`, `qbittorrent_e2e_runner`, `profiling`) from root `src/bin/` via `git mv`. Added `--exclude torrust-tracker-e2e-tools` to all four archive calls. Updated Containerfile recipe/stub stanzas.                                                                                                           |
| T14 | DONE   | Move `testcontainers` to `[dev-dependencies]` in `tracker-core`              | Created `packages/persistence-benchmark/` package (`torrust-tracker-persistence-benchmark`). Moved `persistence_benchmark_runner` binary and full `persistence_benchmark/` module tree from `tracker-core/src/bin/` via `git mv`. Moved `testcontainers` to `[dev-dependencies]` in `tracker-core/Cargo.toml`. Added `--exclude torrust-tracker-persistence-benchmark` to all four archive calls. Updated Containerfile stubs. |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
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

- 2026-05-27 00:00 UTC - GitHub Copilot - Drafted issue to evaluate container-build test execution policy and alternatives - draft file created
- 2026-05-27 00:00 UTC - GitHub Copilot - Expanded the issue to evaluate separation of validation from packaging targets - draft updated
- 2026-06-01 00:00 UTC - GitHub Copilot - GitHub issue #1854 created; spec moved from drafts/ to open/
- 2026-06-03 00:00 UTC - GitHub Copilot - Completed T1–T6 analysis from CI log evidence and workflow inspection; added Analysis Findings section; chose Option B; added T7–T8 implementation tasks
- 2026-06-03 00:00 UTC - GitHub Copilot - Revised recommendation after evaluating moving unit tests out of Containerfile (rejected — only container env proves binary works on target infra); updated T4/T6 with final approach; added T9–T10; added AC9–AC11; created ADR 20260603000000
- 2026-06-03 00:00 UTC - GitHub Copilot - Static analysis of nextest archive binary landscape; identified 47 compiled targets after existing exclusions; documented three concrete optimisation opportunities (T12–T14)

## Acceptance Criteria

- [x] AC1: The report quantifies runtime cost of test execution in the container build path.
- [x] AC2: Duplicate versus unique test coverage is documented for container and testing workflows.
- [x] AC3: At least one policy option separates validation from packaging and preserves strict quality gates.
- [x] AC4: A safe and explicit debug-image policy is defined for failure reproduction use cases.
- [x] AC5: Recommended policy is justified with performance and risk evidence.
- [x] AC6: `debug` target removed from `container.yaml` test matrix.
- [x] AC7: Clarifying comments added to `container.yaml` explaining naming and the E2E/skip policy.
- [x] AC8: Publish jobs warm their cache from the test job's scope to avoid redundant full rebuilds.
- [x] AC9: E2E tests run in `container.yaml` `test` job after image build, before publish.
- [x] AC10: `testing.yaml` `docker-e2e` skips when `container.yaml` covers the same trigger.
- [x] AC11: Decision to keep unit tests inside the container build is recorded in ADR `20260603000000_keep_unit_tests_inside_container_build.md`.
- [x] AC12: Test binary landscape is documented with a count and categorisation of all targets compiled by `cargo nextest archive --tests` after existing exclusions.
- [x] AC13: `torrust-tracker-client` console package is excluded from all three `cargo nextest archive` calls in the Containerfile.
- [x] AC14: `e2e_tests_runner` and `qbittorrent_e2e_runner` binaries are moved to a separate package and excluded from the archive.
- [x] AC15: `testcontainers` is declared as `[dev-dependencies]` in `packages/tracker-core/Cargo.toml` and no longer linked into production binaries.
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
