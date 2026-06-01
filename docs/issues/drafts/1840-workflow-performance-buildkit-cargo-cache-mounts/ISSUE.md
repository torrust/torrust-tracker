---
doc-type: issue
issue-type: task
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-buildkit-cargo-cache-mounts/ISSUE.md
branch: "{issue-number}-buildkit-cargo-cache-mounts"
related-pr: null
last-updated-utc: 2026-06-01 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - .github/workflows/container.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
    - docs/issues/open/1726-1840-workflow-performance-sccache/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Pass Cargo registry/git caches into BuildKit to speed up cook stage rebuilds

## Goal

Add `--mount=type=cache` directives to the `cargo chef cook` RUN steps in the
Containerfile so that the Cargo registry and git caches survive across cook
layer invalidations on local developer machines. Evaluate whether the same
benefit can be extended to CI ephemeral runners.

## Background

### The cook stage bottleneck

The `dependencies` and `dependencies_debug` stages (cook stages) compile all
external Rust crates and are the most expensive part of the container build.
The cook layer is invalidated — and all external crates recompiled from scratch
— whenever `Cargo.lock` changes.

The cook RUN step has two sub-phases:

1. **Download**: fetch crate sources from `crates.io` into the Cargo registry
   (`/usr/local/cargo/registry` and `/usr/local/cargo/git`).
2. **Compile**: compile all external crates and place artifacts in
   `/build/src/target`.

### Proposed change

Add BuildKit cache mounts to the cook RUN steps:

```dockerfile
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --tests --benches --examples --workspace \
      --all-targets --all-features --recipe-path /build/recipe.json
```

This tells BuildKit to overlay the named cache volumes over the registry and
git paths during the RUN step. On a local machine with a long-lived Docker
daemon, the volumes persist between builds. When the cook layer is invalidated
(e.g. `Cargo.lock` changes), the crates are already in the registry cache and
do not need to be re-downloaded.

### Local benchmark: registry download time (2026-06-01)

To quantify the download-only saving, `cargo fetch` was run against a fresh
`CARGO_HOME` (simulating an empty registry cache) and then again against the
populated registry. Machine: Ryzen 9 7950X.

| State                     | Command                             | Time   |
| ------------------------- | ----------------------------------- | ------ |
| Cold (empty registry)     | `CARGO_HOME=/tmp/fresh cargo fetch` | 6.9 s  |
| Warm (registry populated) | `CARGO_HOME=/tmp/fresh cargo fetch` | 0.16 s |

Registry cache size after cold fetch: **823 MB**.

Interpretation: registry cache mounts save approximately **7 s** per cook layer
rebuild (the download phase). The compile phase (the dominant cost in the cook
stage) is **not affected** — compiled artifacts are not included in the registry
or git cache volumes.

### Critical limitation: ephemeral CI runners

`--mount=type=cache` volumes are managed by the local BuildKit daemon and are
stored in the daemon's cache directory (e.g. `/var/lib/docker/buildkit/`). They
are **not** included in the BuildKit layer cache exported via
`cache-from/cache-to: type=gha`.

The current CI workflow (`container.yaml`) uses:

```yaml
cache-from: type=gha,scope=container-<target>
cache-to: type=gha,scope=container-<target>,mode=max
```

`type=gha` exports and restores Docker image layer blobs. It does **not**
persist `--mount=type=cache` volumes. Each GitHub Actions job starts a fresh
ephemeral runner with a new Docker daemon, so the registry cache mount is always
empty.

Conclusion for CI:

- If the cook layer **is** in the GHA layer cache (no `Cargo.lock` change):
  the cook stage is skipped entirely; cache mounts have no effect.
- If the cook layer **is not** in the GHA layer cache (`Cargo.lock` changed):
  the cook stage runs on a fresh daemon; cache mounts are empty; downloads and
  compiles from scratch.

**Registry/git cache mounts provide zero benefit to CI with GitHub Actions
ephemeral runners in the current setup.**

The benefit is limited to local development builds where the Docker daemon is
long-lived (e.g. `docker build` run repeatedly on a developer machine).

### Paths to CI benefit

For the cache mounts to help in CI, one of the following would be required:

| Option                          | Complexity | Notes                                                                                 |
| ------------------------------- | ---------- | ------------------------------------------------------------------------------------- |
| Self-hosted runner              | Medium     | Persistent Docker daemon; cache mounts survive across jobs                            |
| Depot / Namespace / similar CI  | Low-Medium | Persistent BuildKit daemons as a service; cache mounts persist                        |
| `actions/cache` + volume export | High       | Manually tar/restore the BuildKit cache mount dir between runs; fragile, non-standard |

### Advanced variant: caching compiled artifacts

A more aggressive approach would add a cache mount for the target directory
(`/build/src/target`) in addition to the registry/git mounts:

```dockerfile
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/src/target \
    cargo chef cook ...
```

If the target cache mount is populated, cargo performs **incremental
compilation** — only changed or new crates are recompiled. For a minor
`Cargo.lock` change (one or two crates updated), this could reduce the cook
rebuild from 20+ minutes to a few minutes.

However, there is a structural incompatibility with cargo-chef's
cook-then-build layer split:

- The cook stage's target directory (compiled artifacts) is IN the Docker layer
  when no cache mount is used. Downstream stages (`FROM dependencies_debug AS
build_debug`) inherit these artifacts.
- When a `--mount=type=cache` is applied to the target path, the compiled
  artifacts live in the cache volume — they are **not** part of the resulting
  layer. Downstream stages see an empty target directory and must recompile
  everything.

Workarounds are possible but complex (e.g. copying artifacts out of the cache
mount before the RUN step ends, or restructuring the build to avoid the
cook/build layer split). These are tracked as a separate evaluation (see T5).

The same CI limitation applies: target cache mounts are also ephemeral on
GitHub Actions runners.

## Scope

### In scope

- Add `--mount=type=cache` for registry and git to both cook stages in the
  Containerfile.
- Verify the change does not break local builds or produce different artifacts.
- Document the CI limitation clearly in the implementation notes.
- Measure the actual improvement on local builds by timing a cook layer rebuild
  with and without cache mounts.
- Evaluate whether the target-dir cache mount variant is feasible (T5).

### Out of scope

- Switching to a self-hosted runner or a paid BuildKit service.
- Caching the target directory without a clear design that preserves the
  downstream stage compatibility.
- CI cache persistence via `actions/cache` volume export (too fragile).

## Implementation Plan

| Task ID | Description                                                                                                                                                                                  | Status |
| ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| T1      | Add `--mount=type=cache,target=/usr/local/cargo/registry` and `git` to both cook stages in the Containerfile                                                                                 | TODO   |
| T2      | Run a cook layer rebuild locally (trigger by modifying `Cargo.lock` or bumping a dep version) with and without cache mounts and record wall-clock time difference                            | TODO   |
| T3      | Verify that the resulting archives produce identical test results (`cargo nextest run` passes) with cache mounts enabled                                                                     | TODO   |
| T4      | Document the CI limitation (cache mounts are ephemeral on GitHub Actions) in a comment inside the Containerfile and in this spec                                                             | TODO   |
| T5      | Evaluate the target-dir cache mount variant: prototype a Containerfile that uses `--mount=type=cache,target=/build/src/target` and assess whether downstream stage compatibility is solvable | TODO   |
| T6      | Update the baseline benchmark report with new local timing numbers                                                                                                                           | TODO   |

## Risks and Trade-offs

| Risk                                                                | Likelihood | Mitigation                                                                                                       |
| ------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------- |
| Cache mount causes stale artifacts (wrong crate versions compiled)  | Low        | Cache is keyed by daemon lifetime; a fresh build always starts clean; `--no-cache` forces cold rebuild if needed |
| CI engineers expect CI improvement and are disappointed             | Medium     | Document CI limitation clearly before merging; set correct expectations in PR description                        |
| Target-dir cache mount breaks downstream stages                     | High       | Keep target-dir approach in T5 (prototype-only); do not merge until downstream compatibility is solved           |
| BuildKit syntax line (`# syntax=docker/dockerfile:latest`) required | Low        | Already present in the Containerfile; required for cache mount support                                           |

## Progress Tracking

### Checklist

- [x] T0 — proxy benchmark: cold `cargo fetch` 6.9 s, warm 0.16 s; registry 823 MB; CI limitation documented
- [ ] T1 — registry/git cache mounts added to both cook stages
- [ ] T2 — cook layer rebuild timed with and without cache mounts
- [ ] T3 — correctness: test suite passes with cache mounts enabled
- [ ] T4 — CI limitation documented in Containerfile comment
- [ ] T5 — target-dir cache mount variant evaluated
- [ ] T6 — baseline benchmark report updated

### Progress Log

Append one line per meaningful update.

| Date (UTC)       | Note                                                                                                                                                                                                                    |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-06-01 00:00 | Spec drafted. Proxy benchmark run locally: cold registry fetch 6.9 s, warm 0.16 s, registry 823 MB. CI limitation confirmed: `type=gha` layer cache does not persist `--mount=type=cache` volumes on ephemeral runners. |
