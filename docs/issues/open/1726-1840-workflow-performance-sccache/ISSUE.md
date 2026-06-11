---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1726
spec-path: docs/issues/open/1726-1840-workflow-performance-sccache/ISSUE.md
branch: 1726-reduce-build-times-sccache
related-pr: null
last-updated-utc: 2026-06-11 15:53
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

### Task 3: CI Research (A/B)

Benchmark CI behavior on GitHub-hosted runners before deciding on replacement.

- [ ] Run and record baseline CI timings with current setup (`Swatinem/rust-cache`) for
      at least two comparable pushes (cold-ish and repeat).

- [ ] Create an experiment branch/workflow variant using `sccache` (GHA backend):
  - Add the following two steps **before** any `cargo` step in jobs that compile Rust
    (`format`, `check`, `build`, `unit`, `database-compatibility`, `e2e`):

    ```yaml
    - name: Install sccache
      uses: mozilla-actions/sccache-action@v0.0.10

    - name: Enable sccache
      run: |
        echo "RUSTC_WRAPPER=sccache" >> "$GITHUB_ENV"
        echo "SCCACHE_GHA_ENABLED=true" >> "$GITHUB_ENV"
        echo "CARGO_INCREMENTAL=0" >> "$GITHUB_ENV"
    ```

  To purge the remote cache (e.g. after a toolchain or `Cargo.lock` bump), increment
  `SCCACHE_GHA_VERSION` in the workflow env:

  ```yaml
  env:
    SCCACHE_GHA_VERSION: 1 # bump to bust the cache
  ```

- [ ] Verify that the `linter` install step (`cargo install --locked --git ...`) still works
      correctly with the chosen env setup.
- [ ] Push the experiment branch and check that the CI workflow passes end-to-end.
- [ ] Compare CI timing before and after by inspecting workflow run durations on GitHub.
      Record per-job times, especially `unit`, for first and repeat runs.
- [ ] Optional: if results are mixed, test a hybrid strategy (retain small Cargo dependency
      cache, avoid full `target` cache, and keep `sccache` for compilation units).

- Checkpoint: recommendation documented: keep current cache, switch to `sccache`, or use hybrid.

Commit message: `ci(testing): benchmark sccache against current cache strategy`

---

## Acceptance Criteria

- [x] Local benchmark report exists with baseline vs `sccache` (cold, warm, warm-after-change).
- [ ] CI benchmark report exists with current strategy vs `sccache` strategy (first and repeat runs).
- [x] Recommendation is documented with evidence: **reject sccache for local development**.
- [ ] If adoption is recommended, implementation changes are applied and verified (`linter all`, tests, CI).
- [x] If adoption is not recommended, issue documents why and proposes next optimization steps.
  - Conclusion: `torrust-tracker` bin crate (77 s critical-path) is never cached; workspace is too
    tightly coupled for sccache to provide meaningful benefit locally. CI may still be worth
    exploring (Task 3).
