---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - issue #1726
    - .github/actions/enable-sccache/action.yml
    - "issue #3019"
    - .github/workflows/testing.yaml
    - .github/workflows/os-compatibility.yaml
    - .github/workflows/db-compatibility.yaml
    - .github/workflows/coverage.yaml
    - .github/workflows/db-benchmarking.yaml
    - .github/workflows/container.yaml
    - contrib/dev-tools/experiments/sccache-docker/
---

# Adopt `sccache` for non-Docker CI builds only

## Description

This ADR records the decision to adopt `sccache` for GitHub Actions workflow jobs that compile
Rust directly on the runner (outside Docker containers), and to **reject** it for local
development and Docker-based workflow jobs.

The decision is evidence-driven, based on controlled benchmarks in three contexts: local dev,
GHA bare builds, and GHA Docker builds.

## Context

The Torrust Tracker workspace has a cold full-workspace compile time of ~127 s on a high-end
local machine (~480 s on GHA 2-core runners). The `unit` job in `.github/workflows/testing.yaml`
runs `cargo test --tests --benches --examples --workspace --all-targets --all-features` after
every clean checkout.

The existing `Swatinem/rust-cache` setup in CI provides negligible benefit because the
`target/` directory is ~9 GB and GitHub's cache throughput (30–70 MB/s) means restore time
exceeds recompile time. Cache is also invalidated on any `Cargo.lock` change.

`sccache` was evaluated as an alternative because it caches at the codegen-unit level
(individual `rlib` compilations keyed by source hash), which is more granular than
Docker-layer caching and should survive single-file changes without invalidating unrelated units.

The heaviest compilation unit is `torrust-tracker` (the workspace root `bin` crate, ~77 s)
— this can **never** be cached by sccache because sccache only caches `rlib`/`lib` units.

## Decision

### Adopt: Bare CI builds on GHA runners

Add `sccache` to all non-Docker CI jobs that compile Rust, specifically:

- `testing.yaml` → `unit` job (nightly and stable toolchains)
- `testing.yaml` → `docker-e2e` job (cargo steps before Docker build)
- `os-compatibility.yaml` → all compilation jobs
- `db-compatibility.yaml` → all compilation jobs
- `coverage.yaml` → compilation jobs

**Evidence**: Task 3a experiment on `experiment-sccache-bare-build.yaml`:

| Scenario                             | Wall time    | Cache hits  | vs No-Cache Baseline |
| ------------------------------------ | ------------ | ----------- | -------------------- |
| Cold build (no prior cache)          | 479.44 s     | 5.52 %      | —                    |
| Cross-run cold (GHA backend restore) | **192.21 s** | **93.38 %** | **-60 %**            |
| Cross-run warm-after-change          | 137.35 s     | 93.48 %     | -71 %                |

**Implementation**: Add two steps to each job before the first `cargo` command:

```yaml
- name: Install sccache
  uses: mozilla-actions/sccache-action@v0.0.10

- name: Enable sccache
  run: |
    echo "RUSTC_WRAPPER=sccache" >> "$GITHUB_ENV"
    echo "SCCACHE_GHA_ENABLED=true" >> "$GITHUB_ENV"
    echo "CARGO_INCREMENTAL=0" >> "$GITHUB_ENV"
```

**Risk**: The `CARGO_INCREMENTAL=0` env var disables incremental compilation, which may slow
down iterative local development. Within CI, this is acceptable because every job starts from
scratch anyway.

### Amendment (2026-11-24): Build Without `sccache` When Its Server Does Not Start

On 2026-11-23, four bare CI jobs failed before compiling anything, with
`sccache: error: Timed out waiting for server startup` (issue #3019). The `sccache` client
waits at most 10 seconds for its server, and with the GitHub Actions cache backend the server
talks to the cache service while it starts. When that service is slow, the first `cargo`
command fails, and so does the job. A cache is an optimization: when it is unavailable, the
job must build slower, not fail.

So the `Enable sccache` step above is replaced by the local composite action
`.github/actions/enable-sccache`, used after `mozilla-actions/sccache-action`:

- It runs `sccache --start-server` with `SCCACHE_GHA_ENABLED=true` and
  `SCCACHE_IDLE_TIMEOUT=0` in the step environment, because the server selects its cache
  backend when it starts.
- When the server starts, it exports `RUSTC_WRAPPER=sccache`, `SCCACHE_GHA_ENABLED=true` and
  `SCCACHE_IDLE_TIMEOUT=0`. With no idle timeout the server lives for the whole job, so no later
  compile starts a new server without this check.
- When the server does not start, it stops any server that may still come up late, exports
  nothing for `sccache`, and shows a `sccache disabled` warning annotation. The job builds
  without the cache.
- It exports `CARGO_INCREMENTAL=0` in both cases.

`SCCACHE_IGNORE_SERVER_IO_ERROR` was not used: in sccache v0.20.0 it only applies when the
client fails to read a compile response from a server that is already running. The startup
timeout returns from `connect_or_start_server` before that, so the variable does not prevent
the failure. A local reproduction confirmed it (issue #3019, `manual-verification-evidence.md`).
Retrying the startup was also rejected: each attempt adds up to 10 seconds against a service
that is already failing, while a build without the cache has a known cost.

### Reject: Local development

**Evidence** (Task 1):

| Scenario                       | Baseline | With sccache | Delta              |
| ------------------------------ | -------- | ------------ | ------------------ |
| Cold build                     | 112.50 s | 137.11 s     | **+22 %** (slower) |
| Warm (no changes)              | 0.42 s   | 0.26 s       | Equivalent         |
| Warm-after-change (leaf touch) | ~113 s   | 85.81 s      | -24 %              |

The cold build overhead (+22 %) and non-cacheable bin crate (77 s) make sccache a net loss
for local development.

### Reject: Docker builds (container.yaml)

**Evidence** (Task 3b — experiment workflow with sccache inside Containerfile):

| Scenario                      | Wall time   | Notes                                  |
| ----------------------------- | ----------- | -------------------------------------- |
| Cold Docker build             | 29 min 28 s | Full compile with sccache inside       |
| Warm re-trigger (same commit) | 30 min 13 s | All stages recompiled — no improvement |

The warm run showed identical compilation times because:

1. **GHA token expiration**: `ACTIONS_RUNTIME_TOKEN` expires when the job ends.
2. **BuildKit GHA cache same fate**: `cache-from: type=gha` also failed to accelerate.
3. **Non-sticky runners**: Every GHA runner starts empty; cache must be fetched over network.

## Consequences

### Positive

1. **Non-Docker CI builds will be ~60 % faster** on cross-run cache hits (479 s → 192 s on
   the `unit` job).
2. Zero-code-change adoption — only workflow YAML additions needed.
3. The GHA cache backend requires no infrastructure (free within 10 GB limit).

### Negative

1. Extra CI job time on first run: ~45 s to compile `sccache` from source (via
   `mozilla-actions/sccache-action`, already precompiled binary — marginal cost).
2. `CARGO_INCREMENTAL=0` in CI — no impact (CI always starts fresh).
3. Cache eviction within the 10 GB shared limit (same pool as
   `Swatinem/rust-cache`). **`Swatinem/rust-cache` was removed** from sccache-enabled jobs
   because the repo cache was at 13.03 GB / 10 GB (over limit) and sccache proved superior
   (93.38 % hit rate vs ~0 % for Swatinem). See Q&A in the issue spec for the full analysis.

### Neutral

1. The `torrust-tracker` bin crate (~77 s) must still compile from scratch every time.
2. sccache hits only external/C dependencies and `lib` workspace crates — `bin`, `dylib`,
   `cdylib`, and `proc-macro` crates are never cached.

## Alternatives Considered

| Alternative                           | Rejection Reason                                                                                         |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| sccache for local dev                 | Cold build +22 % slower (Task 1).                                                                        |
| sccache inside Docker builds          | No measurable benefit on GHA (Task 3b). Token expiration prevents cross-run access.                      |
| Remove `Swatinem/rust-cache` entirely | **Done** — removed from sccache-enabled jobs. Cache pool at 130 % capacity made keeping both impossible. |
| Depot Cache / S3 backend for sccache  | Adds infrastructure cost. GHA backend is free and proven (93.38 %).                                      |
