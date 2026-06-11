---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1726
spec-path: docs/issues/open/1726-1840-workflow-performance-sccache/ISSUE.md
branch: 1726-reduce-build-times-sccache
related-pr: 1905
last-updated-utc: 2026-06-11 16:51
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - docs/issues/closed/1742-ci-change-aware-workflows-epic.md
    - .github/workflows/
---

# Reduce Build Times with `sccache`

## Goal

Research whether `sccache` is effective for this workspace in local development and GitHub-hosted
CI runners, and decide if it should be adopted fully, partially, or not at all.

This issue is intentionally evidence-driven. No workflow replacement is assumed until benchmarks
confirm a measurable benefit.

Further build-time improvements (crate splitting, linker changes, C-dependency reduction) are left
for follow-up issues.

## Background

A benchmark run on 2026-05-01 measured the following for a clean workspace:

| Command                                                                            | Wall time    |
| ---------------------------------------------------------------------------------- | ------------ |
| `cargo clean`                                                                      | 1.28 s       |
| `cargo fetch`                                                                      | 0.20 s       |
| `cargo test --tests --benches --examples --workspace --all-targets --all-features` | **142.47 s** |

**89 % of the 142 s is compilation; only 10 % is test execution.**

The `unit` job in `.github/workflows/testing.yaml` runs the same full-workspace test command
after a clean checkout. `Swatinem/rust-cache` is already present in every CI job and appears to
have limited benefit for this workspace based on size and transfer estimates:

- The `target/` directory after a build is ~9 GB.
- GitHub Actions cache restore/upload at 30–70 MB/s costs 130–300 s — more than a cold build.
- Cache is keyed per-job and per-toolchain; no cross-job sharing occurs.
- Any `Cargo.lock` change invalidates the entire cache.

`sccache` may help because it caches individual codegen units keyed by source content hash, so a
miss on one changed crate does not invalidate unrelated crates. The GHA cache backend
(`SCCACHE_GHA_ENABLED=true`) uses GitHub's own cache storage with no extra infrastructure.

However, there are known limitations that may reduce the effective benefit:

- **Non-sticky runners**: on GitHub-hosted runners, every job starts with an empty local disk;
  compiled objects must be fetched from the GHA cache backend on every run. First-run cache
  misses are expected.
- **`bin`, `dylib`, `cdylib`, and `proc-macro` crates are never cached** by sccache — it only
  caches `rlib`/`lib` units. The heaviest crate in this workspace,
  `torrust-tracker` (rank 1, 77 s single unit), is a `bin` crate and will **always** recompile.
- **Incremental compilation must be disabled**: Cargo enables incremental compilation by default
  in the `dev` profile for workspace members. sccache cannot cache incrementally compiled units;
  `CARGO_INCREMENTAL=0` (or `incremental = false` in the profile) is required.
- **Rate-limiting**: if the GHA cache service is rate-limited, sccache silently skips storing
  objects; builds continue but cache population may be incomplete.

Therefore, the decision to adopt `sccache` must be based on measured repeat-run behavior, not
assumptions.

Full benchmark data and compile-hotspot analysis are in
[`compile-hotspot-analysis.md`](./compile-hotspot-analysis.md). The live sccache A/B experiment report with all commands,
timestamps, and measured output is in
[`sccache-a-b-report.md`](./sccache-a-b-report.md).

## References

- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1726
- `sccache` repository: https://github.com/mozilla/sccache
- `mozilla-actions/sccache-action`: https://github.com/mozilla-actions/sccache-action
- Compile hotspot analysis: [`docs/issues/1726-1840-workflow-performance-sccache/compile-hotspot-analysis.md`](./compile-hotspot-analysis.md)
- CI workflow: [`.github/workflows/testing.yaml`](../../../.github/workflows/testing.yaml)

---

## Tasks

### Task 0: Create a local branch

- Branch name: `1726-reduce-build-times-sccache`
- Commands:

  ```sh
  git fetch --all --prune
  git checkout develop
  git pull --ff-only
  git checkout -b 1726-reduce-build-times-sccache
  ```

- Checkpoint: `git branch --show-current` outputs `1726-reduce-build-times-sccache`.

---

### Task 1: Local Research (A/B)

Measure whether `sccache` improves local rebuild times versus baseline.

- [x] Baseline (no `sccache`) measurement:

  ```sh
  unset RUSTC_WRAPPER
  export CARGO_INCREMENTAL=0
  cargo clean
  /usr/bin/time -f 'real=%e' cargo test --tests --benches --examples \
    --workspace --all-targets --all-features --no-run
  /usr/bin/time -f 'real=%e' cargo test --tests --benches --examples \
    --workspace --all-targets --all-features --no-run
  ```

  Baseline cold: **112.50 s** / Warm: **0.42 s** (see [`sccache-a-b-report.md`](./sccache-a-b-report.md#a1-cold-build--baseline)).

- [x] Install `sccache`:

  ```sh
  sudo apt install -y sccache    # version 0.13.0
  ```

- [x] Run a cold build through `sccache`:

  ```sh
  sccache --stop-server 2>/dev/null; sccache --start-server
  export RUSTC_WRAPPER=sccache
  export CARGO_INCREMENTAL=0
  cargo clean
  /usr/bin/time -f 'real=%e' cargo test --tests --benches --examples \
    --workspace --all-targets --all-features --no-run
  sccache --show-stats
  ```

  Cold via sccache: **137.11 s** (0.20 % cache hits — expected first-run misses).

- [x] Run a warm build (no `cargo clean`) through `sccache` to confirm cache hits:

  ```sh
  /usr/bin/time -f 'real=%e' cargo test --tests --benches --examples \
    --workspace --all-targets --all-features --no-run
  sccache --show-stats
  ```

  Warm via sccache: **0.26 s** (nothing changed, no compilations triggered).

- [x] Run a warm build after a single-file change in a leaf crate
      (e.g., touch a file in `packages/primitives/`) to confirm only the affected
      downstream units miss the cache.

  Warm-after-change: **85.81 s** (1.83 % cache hits — only external/C deps saved).

- [x] Compare baseline vs `sccache` results in a table (cold, warm, warm-after-change).

  See [Results Summary](./sccache-a-b-report.md#results-summary) in the sccache A/B report.

- Checkpoint: ✅ **TASK 1 COMPLETE** — Data shows that sccache **does not materially improve local rebuilds**.
  See [Analysis](./sccache-a-b-report.md#analysis) for detailed reasoning.
  - Cold: +22 % (worse)
  - Warm-after-change: -24 % (modest, only external deps saved)
  - Root cause: `torrust-tracker` bin crate (77 s critical path) is never cached by sccache

Commit message: `docs(build): record local sccache benchmark results`

---

### Task 2: Local Configuration Decision

Decide whether to enable `sccache` in `.cargo/config.toml` for developers.

- [-] ~If local research is positive~ — **not applicable (research was negative)**.
- [-] ~If enabled, update `AGENTS.md` and/or `README.md`~ — **not applicable (rejected)**.
- [x] Verify `linter all` still exits `0` — **confirmed: all linters pass** (run on 2026-06-11).

- Checkpoint: ✅ **TASK 2 COMPLETE** — explicit decision: **do not enable sccache for local
  development**. The benchmark evidence (cold: +22 % slower, warm-after-change: -24 % modest)
  does not justify the overhead. See [Analysis](./sccache-a-b-report.md#analysis) for full
  reasoning. The root `torrust-tracker` bin crate (77 s critical-path) is never cached by sccache,
  and the workspace dependency graph is too tight for meaningful benefit.

Commit message: `docs(build): document local sccache decision (reject)`

---

### Task 3: CI Research — Docker workflow (A/B)

**Context**: The primary target is the `container.yaml` workflow, which builds inside Docker
using `cargo-chef` for layer caching. The E2E tests run inside the container image. sccache
must work _inside_ the Docker build to be useful here — adding it only to the GHA runner
outside Docker would not accelerate the `docker build` step.

The approach is **progressive**: start with a simple bare build on the runner, integrate
sccache into Docker, then run the full E2E suite.

**Self-sufficiency**: Each experiment workflow is designed to run its own A/B comparison in
a single push. When possible, the workflow runs two builds back-to-back (cold then warm) and
outputs both results. When the GHA cache is only persisted via post-job actions (e.g. `sccache`
writes to GHA cache on job completion), the second run requires a manual re-trigger — the
instructions for each step make this explicit.

**Measuring results**: Cold builds are timed from the workflow run output (look for
`real=` from `/usr/bin/time`). Warm builds are measured similarly after a
re-trigger. The comparison is documented in the issue spec.

---

#### Task 3a: Bare cargo build with sccache on GHA runner

Build the `release` profile directly on the GHA runner (no Docker) to isolate sccache's
effectiveness from Docker-specific overhead.

```yaml
# In the experiment workflow, before any cargo step:
- name: Install sccache
  uses: mozilla-actions/sccache-action@v0.0.10

- name: Enable sccache
  run: |
    echo "RUSTC_WRAPPER=sccache" >> "$GITHUB_ENV"
    echo "SCCACHE_GHA_ENABLED=true" >> "$GITHUB_ENV"
    echo "CARGO_INCREMENTAL=0" >> "$GITHUB_ENV"
```

- [x] Create `experiment-sccache-bare-build.yaml` workflow (based on a simplified
      `container.yaml` but using bare `cargo build --release` instead of `docker build`).
      The workflow runs cold `cargo build --release` first, then a warm rebuild
      (no `cargo clean`) to measure cache effectiveness. Both results are output as
      workflow annotations.
- [x] Push the experiment branch to the `josecelano` fork and verify the workflow passes.
      **Cold run**: first push — no sccache cache on GHA yet.
      Results: **479.44 s** (5.52 % cache hits, 133 cache write errors).
      See [`experiment-results-gha.md`](./experiment-results-gha.md).
- [x] Re-trigger workflow via `workflow_dispatch` (same commit) to test cross-run sccache
      GHA backend caching.
      **Cross-run cold**: **192.21 s** (93.38 % cache hits, 0 write errors).
      **Cross-run warm-after-change**: **137.35 s** (93.48 % cache hits).
- [x] Record cold vs warm timing and sccache stats from the GHA run output. Capture the
      `cargo build --release` wall time and the `sccache --show-stats` output from each run.
      Warm-after-change: **153.86 s** (6.96 % cache hits — Cargo avoids external deps naturally).

- Checkpoint: ✅ **TASK 3a COMPLETE** — sccache GHA backend provides **93.38 % cache hit rate**
  on cross-run builds, reducing cold build time from **479 s to 192 s** (60 % reduction).
  See [`experiment-results-gha.md`](./experiment-results-gha.md) for full data.

Commit message: `ci(experiment): benchmark sccache cross-run GHA caching`

---

#### Task 3b: sccache inside Docker build

Create an experiment workflow that builds the Docker image with sccache enabled _inside_
the Containerfile build.

##### Strategy selection

We adopt **Strategy B1 (mount sccache cache dir into Docker)** as the primary approach.
Strategy B2 (GHA backend inside Docker) is documented as a discarded alternative below.

**Why B1 (mount) is preferred over B2 (GHA backend inside Docker)**:

| Criterion                | B1 — Mount host sccache                                                                                 | B2 — GHA backend inside Docker                                                                                                                     |
| ------------------------ | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Changes to Containerfile | Minimal — add `RUSTC_WRAPPER=sccache` and `SCCACHE_DIR` env to compiler stages                          | Requires installing `sccache` binary inside each Docker stage, configuring GHA credentials, and handling network access from within `docker build` |
| Authentication           | None needed — uses local disk cache on the runner                                                       | Needs GitHub token (`ACTIONS_RUNTIME_TOKEN`, `ACTIONS_CACHE_URL`) passed via `--secret` into Docker build context                                  |
| Network dependency       | None — local disk only                                                                                  | Each `docker build` layer that compiles Rust must fetch cache objects from GHA cache API over HTTPS                                                |
| Layer cache interaction  | Complements BuildKit layer cache — sccache caches at codegen-unit level, BuildKit at Docker layer level | Same as B1 but with added HTTP overhead per compiler invocation                                                                                    |
| Complexity               | Low — `docker build` with `--volume` mount and env passthrough                                          | High — custom Containerfile stages, secret mounts, GHA auth passthrough                                                                            |
| Runner portability       | Works on any runner with Docker (local, GHA, self-hosted)                                               | Tied to GHA-specific environment variables                                                                                                         |

**Why not B2**: The GHA cache backend inside Docker requires passing secrets (`ACTIONS_RUNTIME_TOKEN`,
`ACTIONS_CACHE_URL`) securely into the build context. While `docker/build-push-action` supports
`--secret`, the sccache client inside Docker would need these mounted at the right paths and
the `SCCACHE_GHA_ENABLED=true` env var set. This adds complexity without clear benefit over B1 —
the local disk on the GHA runner is ephemeral, but the sccache daemon on the host runner writes to
the GHA cache backend automatically (via `mozilla-actions/sccache-action`), and the Docker build
reads from the host's local sccache cache which is prepopulated by the daemon. The host daemon
handles the GHA backend sync; the Docker build just needs the local cache dir.

**Strategy B1 in detail**:

- The GHA runner has sccache installed and running (from `mozilla-actions/sccache-action`).
  The action starts an sccache daemon that uses the GHA cache backend (`SCCACHE_GHA_ENABLED=true`).
- Docker `build` mounts the sccache cache directory into the build container via
  `--volume /home/runner/.cache/sccache:/home/runner/.cache/sccache`.
- The Containerfile compiler stages set `RUSTC_WRAPPER=sccache` and `SCCACHE_DIR=/home/runner/.cache/sccache`.
- sccache reads cached objects from the local disk (which is synced to GHA cache by the host
  daemon). Writes from inside Docker go to the same directory, and the host daemon syncs them
  to the GHA cache backend after the job.

- [ ] Create `experiment-sccache-docker.yaml` workflow that builds the full Docker image
      (same `target: release`) with: - `mozilla-actions/sccache-action` before Docker build - `docker/build-push-action` with extra `--volume` mount for sccache cache dir - A modified `Containerfile.sccache-experiment` that sets `RUSTC_WRAPPER=sccache`
      and `SCCACHE_DIR` in the compiler stages
- [ ] Push and verify the workflow passes end-to-end.
- [ ] Record first-run timing for the `docker build` step (cold — no sccache cache yet).
- [ ] Re-trigger the same workflow (same commit) to measure warm-run timing with
      sccache cache populated by the first run.
- [ ] Compare with baseline (current `container.yaml` timing from a recent run on `develop`).

---

#### Task 3c: Full E2E with sccache-warmed Docker build

Run the complete E2E test suite (qBittorrent SQLite3, MySQL, PostgreSQL) with a
sccache-warmed Docker build to measure end-to-end workflow improvement.

- [ ] Add the E2E steps to the experiment workflow (same as `container.yaml`:
      `e2e_tests_runner`, `qbittorrent_e2e_runner` for all 3 database drivers).
- [ ] Push and run the workflow (cold), then re-trigger (warm). Record total workflow
      duration for both runs.
- [ ] Compare with recent `container.yaml` run durations on `develop`.

---

#### Task 3d: Decision and cleanup

- [ ] Document recommendation: adopt sccache for Docker builds, adopt hybrid, or reject.
- [ ] If adopted, modify the real `container.yaml` workflow and `Containerfile` with
      the proven sccache integration.
- [ ] If rejected, document why for the Docker context (e.g., "GitHub-hosted runner
      non-sticky disk + sccache GHA backend fetch time outweighs benefit for this
      workspace's build pattern; B1 mount strategy adds Docker complexity without
      proportional gain").
- [ ] Remove experiment workflow files.
- [ ] Verify `linter all` still exits `0`.

- Checkpoint: final decision with evidence for CI/Docker context.

Commit message: `ci(container): validate sccache for docker build workflow`

---

## Acceptance Criteria

- [x] Local benchmark report exists with baseline vs `sccache` (cold, warm, warm-after-change).
- [ ] ~~CI benchmark report exists~~ — **replaced by progressive sub-tasks** (3a → 3d below).
- [x] Recommendation is documented with evidence: **reject sccache for local development**.
- [x] **Task 3a: Bare cargo build with sccache on GHA runner (cold vs warm timing).**
  - Cold: **479.44 s** → Cross-run with sccache: **192.21 s** ✅ (60 % reduction, 93.38 % hit rate)
- [ ] Task 3b: sccache inside Docker build (Strategy B1 — mount host cache).
- [ ] Task 3c: Full E2E with sccache-warmed Docker build.
- [ ] Task 3d: Final decision and cleanup (adopt, reject, or hybrid for Docker context).
- [x] If adoption is not recommended, issue documents why and proposes next optimization steps.
  - Conclusion: `torrust-tracker` bin crate (77 s critical-path) is never cached; workspace is too
    tightly coupled for sccache to provide meaningful benefit locally. CI/Docker is still under
    evaluation (Task 3a–3d).
