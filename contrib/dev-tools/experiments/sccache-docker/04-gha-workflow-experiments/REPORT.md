# Experiment 4: GHA Workflow Experiments (Task 3a & 3b)

**Date**: 2026-06-11 to 2026-06-12
**Goal**: Run sccache A/B benchmarks on GitHub Actions runners.

## File Locations at Experiment Time

These files were originally at the repository root or `.github/workflows/` during the
experiment. They are archived here after the experiment concluded.

| File                                 | Original location                                      | Purpose                                                                            |
| ------------------------------------ | ------------------------------------------------------ | ---------------------------------------------------------------------------------- |
| `Containerfile.sccache-experiment`   | Repository root (`/Containerfile.sccache-experiment`)  | Modified Containerfile with sccache installed in `chef` stage, GHA credential ARGs |
| `experiment-sccache-bare-build.yaml` | `.github/workflows/experiment-sccache-bare-build.yaml` | Task 3a: bare `cargo build --release` with sccache on GHA runner                   |
| `experiment-sccache-docker.yaml`     | `.github/workflows/experiment-sccache-docker.yaml`     | Task 3b: full Docker build with sccache inside Containerfile + E2E tests           |

## Experiment 3a: Bare Build Results

- **Cold run**: 479.44 s (5.52 % cache hits)
- **Cross-run cold** (re-trigger, GHA cache restored): **192.21 s** (93.38 % cache hits)
- **Cross-run warm-after-change**: **137.35 s** (93.48 % cache hits)
- **Verdict**: sccache with GHA backend provides **60 % reduction** on cross-run bare builds.
  **Adopted for non-Docker CI jobs.**

## Experiment 3b: Docker Build Results

- **Cold run**: 29 min 28 s (full Docker build with sccache inside)
- **Warm re-trigger**: 30 min 13 s (no improvement — all stages recompiled)
- **Key issue**: GHA `ACTIONS_RUNTIME_TOKEN` is job-scoped — expires between runs.
  BuildKit `cache-from: type=gha` also didn't accelerate (restore > recompile).
- **Verdict**: **Rejected for Docker builds.** No measurable benefit.

## GHA Credential Fixes Applied

1. `SCCACHE_GHA_ENABLED` must be hardcoded `true` — cannot use `${{ env.SCCACHE_GHA_ENABLED }}`
   because `mozilla-actions/sccache-action` sets it at runtime, after workflow parse time.
2. Modern GHA runners use V2 cache API (`ACTIONS_RESULTS_URL`). sccache's `ghac` library looks
   for `ACTIONS_CACHE_URL` — must be mapped from `ACTIONS_RESULTS_URL`.

## Links

- Task 3a full results: [`docs/issues/open/1726-1840-workflow-performance-sccache/experiment-results-gha.md`](../../../../docs/issues/open/1726-1840-workflow-performance-sccache/experiment-results-gha.md)
- Task 3b full results: [`docs/issues/open/1726-1840-workflow-performance-sccache/experiment-docker-gha-results.md`](../../../../docs/issues/open/1726-1840-workflow-performance-sccache/experiment-docker-gha-results.md)
- Issue spec: [`docs/issues/open/1726-1840-workflow-performance-sccache/ISSUE.md`](../../../../docs/issues/open/1726-1840-workflow-performance-sccache/ISSUE.md)
