---
semantic-links:
  related-artifacts:
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/ISSUE.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - contrib/dev-tools/workflow-benchmarks/run-container-baseline.sh
    - contrib/dev-tools/workflow-benchmarks/run-testing-baseline.sh
---

# Baseline Workflow Benchmark Results

Recorded on: 2026-05-28

This file is the living benchmark artifact for the workflow-performance EPIC.
Update it whenever a later optimization changes the performance profile so future
runs can be compared against the same baseline.

## Measurement Environment

| Property    | Value                                                                |
| ----------- | -------------------------------------------------------------------- |
| **Date**    | 2026-05-28                                                           |
| **Host OS** | Ubuntu 26.04 LTS "Resolute Raccoon" — kernel 7.0.0-15-generic        |
| **CPU**     | AMD Ryzen 9 7950X — 16 cores / 32 threads @ up to 5883 MHz           |
| **RAM**     | 64 GiB total (62 GiB available at measurement time)                  |
| **Disk**    | 1.8 TiB root volume (`/dev/mapper/ubuntu--vg-ubuntu--lv`), 76 % used |
| **Docker**  | 28.3.3                                                               |
| **Rust**    | rustc 1.98.0-nightly (57d06900f 2026-05-27) / cargo 1.98.0-nightly   |
| **Linker**  | system default (`cc` / BFD linker; no `mold` or `lld`)               |

> These are **local developer-machine numbers**, not CI times. GitHub-hosted
> runners use a different CPU/RAM profile, so absolute durations will differ.
> Use the ratios and bottleneck rankings — not the raw seconds — when
> reasoning about what to optimize first.

## How to Reproduce

```bash
# Cold run (clears Docker builder cache and isolated Cargo dirs)
./contrib/dev-tools/workflow-benchmarks/run-container-baseline.sh --cold
./contrib/dev-tools/workflow-benchmarks/run-testing-baseline.sh --cold

# Warm run (immediately after, no cache reset)
./contrib/dev-tools/workflow-benchmarks/run-container-baseline.sh
./contrib/dev-tools/workflow-benchmarks/run-testing-baseline.sh

# Linker-heavy target profiling (release, all targets)
cargo build --timings --all-targets --release --workspace --all-features
# HTML report written to: target/cargo-timings/cargo-timing.html
```

Evidence logs are stored under
`docs/issues/open/1841-1840-workflow-performance-baseline-analysis/evidence/`.

## Cache Reset Procedure (Cold Run)

The following was performed before the cold run to approximate shared-runner
first-run conditions:

```bash
docker builder prune -af          # clear all Docker BuildKit cache
docker image rm -f torrust-tracker:local torrust-tracker:e2e-local  # drop local images
# testing script additionally isolates CARGO_HOME and CARGO_TARGET_DIR
# under .tmp/workflow-benchmarks/ and removes them before the cold run
```

The local Cargo registry (`~/.cargo/registry`) was **not** cleared because
GitHub-hosted runners also receive a pre-warmed package registry via
`Swatinem/rust-cache`. Clearing it would produce times that are
artificially slower than the real CI cold run.

## Measurement Table

CI runs `container` debug and release targets in parallel (matrix strategy) and
`testing` unit(nightly) + unit(stable) + docker-e2e in parallel.
CI wall time therefore approximates **max(parallel jobs)**, whereas the scripts
run jobs sequentially. Sequential totals are noted and CI-equivalent wall time is
estimated in the Notes column.

| Workflow  | Run Type        | Sequential Total | CI-equivalent Wall Time | Main Bottleneck          | Notes                                                              |
| --------- | --------------- | ---------------- | ----------------------- | ------------------------ | ------------------------------------------------------------------ |
| container | cold / no-cache | ~499 s (~8.3 m)  | ~260 s (~4.3 m)         | release compile+link     | debug=239 s, release=260 s run in parallel on CI                   |
| container | warm / cached   | ~2 s             | ~2 s                    | none (all layers cached) | Both targets hit Docker layer cache fully                          |
| testing   | cold / no-cache | ~767 s (~12.8 m) | ~510 s (~8.5 m)         | docker-e2e Docker build  | unit≈257 s, docker-e2e≈510 s; lint exited 1 (see §Notes)           |
| testing   | warm / cached   | ~393 s (~6.6 m)  | ~331 s (~5.5 m)         | docker-e2e Docker build  | unit≈62 s, docker-e2e≈331 s; docker build not fully cached locally |

## Internal Phase Breakdown

### Container Workflow

Phases mirror `.github/workflows/container.yaml` → job `test` (matrix: debug, release).

| Phase             | Cold Run | Warm Run | Notes                                                                                          |
| ----------------- | -------- | -------- | ---------------------------------------------------------------------------------------------- |
| build (debug)     | 239 s    | 2 s      | Bottleneck on cold: `dependencies_debug` cook (~47 s) + `build_debug` nextest archive (~131 s) |
| inspect (debug)   | 0 s      | 0 s      | Negligible                                                                                     |
| build (release)   | 260 s    | 0 s      | Bottleneck on cold: `dependencies` cook (~64 s) + `build` nextest archive (~157 s)             |
| inspect (release) | 0 s      | 0 s      | Negligible                                                                                     |

### Testing Workflow

Phases mirror `.github/workflows/testing.yaml` → jobs `unit` + `docker-e2e`.

#### Unit job

| Phase             | Cold Run  | Warm Run | Notes                                             |
| ----------------- | --------- | -------- | ------------------------------------------------- |
| fetch             | 7 s       | 0 s      | Warm: all crates already in registry              |
| install_linter    | 5 s       | 0 s      | Warm: binary already in `~/.cargo/bin`            |
| format            | 0 s       | 1 s      | Negligible                                        |
| lint              | 48 s      | 16 s     | Exits 1 on both runs; see Notes below             |
| test_docs         | 58 s      | 29 s     | Warm benefits from incremental compilation        |
| test_unit         | 139 s     | 16 s     | Warm: incremental; cold dominated by compile+link |
| **unit subtotal** | **257 s** | **62 s** |                                                   |

#### Docker E2E job

| Phase                      | Cold Run  | Warm Run  | Notes                                                                                          |
| -------------------------- | --------- | --------- | ---------------------------------------------------------------------------------------------- |
| docker_build_e2e           | 312 s     | 234 s     | Dominant phase; warm still slow — local Docker cache does not cover `dependencies` cook layers |
| e2e_tracker                | 79 s      | 16 s      | Warm: image already built                                                                      |
| e2e_qbittorrent_sqlite     | 61 s      | 24 s      | Container startup + torrent seeding                                                            |
| e2e_qbittorrent_mysql      | 29 s      | 29 s      | Consistent; DB startup dominates                                                               |
| e2e_qbittorrent_postgresql | 29 s      | 28 s      | Consistent; DB startup dominates                                                               |
| **e2e subtotal**           | **510 s** | **331 s** |                                                                                                |

Notes:

- `lint` exited with code 1 on both cold and warm runs. This indicates existing
  lint issues in the working tree at the time of measurement and does not affect
  the timing validity; the step still ran to completion and consumed the measured time.
- Local Docker layer cache only partially covers `docker_build_e2e` on the warm
  run because the `COPY . /build/src` and `COPY . /test/src` layers are
  invalidated by any file change. The 234 s warm time reflects cache hits for
  base images and dependency layers but a fresh `build` stage.

## Linker-Heavy Target Analysis (Container Build Path)

Source: `cargo build --timings --all-targets --release --workspace --all-features`
run on 2026-05-28. Full HTML report:
`docs/issues/open/1841-1840-workflow-performance-baseline-analysis/evidence/cargo-timing-release-20260528T074109Z.html`

Total `cargo build` wall time reported by `--timings`: **188 s** (warm incremental,
local machine).

Top 30 compile units by duration:

| Rank | Duration | Crate / Package                                 | Target                                           | Runtime image? | Notes                                               |
| ---- | -------- | ----------------------------------------------- | ------------------------------------------------ | -------------- | --------------------------------------------------- |
| 1    | 117 s    | torrust-tracker                                 | integration (test)                               | **no**         | Integration test binary; not shipped in image       |
| 2    | 117 s    | torrust-tracker                                 | torrust-tracker (bin)                            | **yes**        | Main tracker binary — required                      |
| 3    | 116 s    | torrust-tracker                                 | profiling (bin)                                  | **no**         | Profiling helper binary; not in runtime image       |
| 4    | 109 s    | torrust-tracker                                 | torrust_tracker_lib (lib, test)                  | **no**         | Test variant of the lib; not shipped                |
| 5    | 109 s    | torrust-tracker-axum-health-check-api-server    | integration (test)                               | **no**         | Integration test binary; not shipped                |
| 6    | 104 s    | torrust-tracker-core                            | persistence_benchmark_runner (bin)               | **no**         | Benchmark binary; not shipped                       |
| 7    | 103 s    | torrust-tracker-core                            | torrust_tracker_core (lib, test)                 | **no**         | Test variant of the lib; not shipped                |
| 8    | 94 s     | torrust-tracker-axum-http-server                | integration (test)                               | **no**         | Integration test binary; not shipped                |
| 9    | 93 s     | torrust-tracker-axum-rest-api-server            | integration (test)                               | **no**         | Integration test binary; not shipped                |
| 10   | 92 s     | torrust-tracker-axum-rest-api-server            | torrust_tracker_axum_rest_api_server (lib, test) | **no**         | Test variant of the lib; not shipped                |
| 11   | 89 s     | torrust-tracker-axum-http-server                | torrust_tracker_axum_http_server (lib, test)     | **no**         | Test variant of the lib; not shipped                |
| 12   | 78 s     | torrust-tracker-udp-server                      | torrust_tracker_udp_server (lib, test)           | **no**         | Test variant of the lib; not shipped                |
| 13   | 71 s     | torrust-tracker-udp-server                      | integration (test)                               | **no**         | Integration test binary; not shipped                |
| 14   | 60 s     | torrust-tracker                                 | qbittorrent_e2e_runner (bin)                     | **no**         | E2E test runner binary; not shipped                 |
| 15   | 56 s     | torrust-tracker-rest-api-core                   | torrust_tracker_rest_api_core (lib, test)        | **no**         | Test variant of the lib; not shipped                |
| 16   | 52 s     | torrust-tracker-http-tracker-core               | torrust_tracker_http_tracker_core (lib, test)    | **no**         | Test variant of the lib; not shipped                |
| 17   | 51 s     | torrust-tracker-client                          | tracker_client (bin)                             | **no**         | CLI client binary; not in runtime image             |
| 18   | 50 s     | torrust-tracker-core                            | integration (test)                               | **no**         | Integration test binary; not shipped                |
| 19   | 48 s     | torrust-tracker-http-tracker-core               | http_tracker_core_benchmark (bench, test)        | **no**         | Benchmark; not shipped                              |
| 20   | 47 s     | torrust-tracker-client                          | tracker_checker (bin)                            | **no**         | CLI checker binary; not in runtime image            |
| 21   | 46 s     | torrust-tracker-udp-tracker-core                | udp_tracker_core_benchmark (bench, test)         | **no**         | Benchmark; not shipped                              |
| 22   | 46 s     | libsqlite3-sys                                  | build-script (run)                               | **yes**        | SQLite3 C library compilation — required by runtime |
| 23   | 45 s     | torrust-tracker-core                            | persistence_benchmark_runner (bin, test)         | **no**         | Benchmark test variant; not shipped                 |
| 24   | 44 s     | torrust-tracker                                 | e2e_tests_runner (bin)                           | **no**         | E2E test runner binary; not shipped                 |
| 25   | 41 s     | torrust-tracker-client                          | http_tracker_client (bin)                        | **no**         | CLI client binary; not in runtime image             |
| 26   | 39 s     | torrust-tracker-torrent-repository-benchmarking | repository_benchmark (bench, test)               | **no**         | Benchmark; not shipped                              |
| 27   | 35 s     | torrust-tracker                                 | qbittorrent_e2e_runner (bin, test)               | **no**         | E2E runner test variant; not shipped                |
| 28   | 35 s     | torrust-tracker                                 | profiling (bin, test)                            | **no**         | Profiling test variant; not shipped                 |
| 29   | 35 s     | torrust-tracker                                 | e2e_tests_runner (bin, test)                     | **no**         | E2E runner test variant; not shipped                |
| 30   | 35 s     | torrust-tracker                                 | http_health_check (bin)                          | **yes**        | Health-check binary — required by runtime image     |

**Of the top 30 compile units, only 3 are required by the tracker runtime image**
(`torrust-tracker` bin, `libsqlite3-sys` build script, `http_health_check` bin).
The remaining 27 units are test binaries, benchmarks, or utility binaries that
are compiled by the `--tests --benches --examples --all-targets` flags in the
Containerfile `cargo nextest archive` commands but are never included in the
final runtime image.

## Docker Layer Breakdown (Cold Run)

> **Note — per-layer capture requires `--progress plain`.** The initial cold run
> (`container-baseline-20260527T210123Z.log`) was captured before `--progress plain`
> was added to the script; Docker's BuildKit wrote per-step output to **stderr** only,
> so it was not saved in the evidence log. The `run-container-baseline.sh` script was
> updated on 2026-05-28 to pass `--progress plain`, which routes step output through
> stdout so it is captured alongside the phase-timing lines. Re-run the script with
> `--cold` to populate a new evidence log with per-layer durations.
>
> **Sub-command timing inside RUN steps**: BuildKit reports one wall-clock time per
> `RUN` instruction. When a `RUN` instruction chains multiple commands with `&&` or
> `;`, the individual command times are invisible at the step level. `time` wrappers
> were added on 2026-05-28 to every multi-command `RUN` block in the `Containerfile`
> (e.g. `apt-get update`, `cc` compile, `cp`/`chown`/`chmod` post-processing steps).
> With `--progress plain` these `time` outputs appear inline in the step's stdout/stderr
> stream and are captured in the evidence log.

The layer structure and approximate timings listed below were observed in the
terminal output during the initial cold run and are provided as a structural
reference until a new evidence log is available.

### Debug target (`--target debug`)

| Layer (Dockerfile stage → step)                         | Approx. Cold Duration | Description                                   |
| ------------------------------------------------------- | --------------------- | --------------------------------------------- |
| `chef` — install cargo-chef                             | ~7 s                  | Download and compile cargo-chef               |
| `recipe` — `cargo chef prepare`                         | ~0.1 s                | Generate `recipe.json` dependency manifest    |
| `dependencies_debug` — `cargo chef cook` (cook)         | ~47 s                 | Pre-compile dependency crates (debug profile) |
| `dependencies_debug` — `cargo nextest archive` (warmup) | ~8 s                  | Warm nextest archive with dep-only crates     |
| `build_debug` — `cargo nextest archive` (full)          | ~131 s                | Compile + link all targets (debug profile)    |
| `test_debug` — `cargo nextest run` (×2)                 | ~23 s total           | Execute tests inside container                |

**Total observed (debug)**: ~216 s (cf. `build_debug_seconds=239` in the log;
the discrepancy reflects Docker overhead and steps with sub-second durations
not listed above).

### Release target (`--target release`)

| Layer (Dockerfile stage → step)                   | Approx. Cold Duration | Description                                     |
| ------------------------------------------------- | --------------------- | ----------------------------------------------- |
| `recipe` — `cargo chef prepare`                   | (cached from debug)   | Shared with debug target; no additional cost    |
| `dependencies` — `cargo chef cook` (cook)         | ~64 s                 | Pre-compile dependency crates (release profile) |
| `dependencies` — `cargo nextest archive` (warmup) | ~14 s                 | Warm nextest archive with dep-only crates       |
| `build` — `cargo nextest archive` (full)          | ~157 s                | Compile + link all targets (release profile)    |
| `test` — `cargo nextest run` (×2)                 | ~23 s total           | Execute tests inside container                  |

**Total observed (release)**: ~258 s (cf. `build_release_seconds=260` in the
log).

### Key observations

- The `build_*` stages dominate: 131 s (debug) and 157 s (release), reflecting
  the cost of linking all non-runtime binaries and test targets.
- `dependencies_*` stages (~47–64 s) benefit from Docker layer caching on warm
  runs; re-running after a `Cargo.lock` change invalidates these layers.
- The `recipe` stage is effectively free (<1 s) and is shared between debug and
  release via Docker layer cache.

### Finding: `.tmp/` missing from `.dockerignore` inflated COPY steps by ~30 s

During the initial cold run, the `COPY . /build/src` step in the `recipe` and
`build_*` stages took approximately **30 s** — a cost that should be
near-instant. Investigation revealed that the `.tmp/` directory (used by the
`run-testing-baseline.sh` cold-run benchmark to isolate `CARGO_HOME` and
`CARGO_TARGET_DIR`) was not listed in `.dockerignore`.

`.tmp/` is the workspace-local temp directory used by AI agent tools (e.g.
`TORRUST_GIT_HOOKS_LOG_DIR=.tmp` routes pre-commit/pre-push logs there). The
benchmark script `run-testing-baseline.sh` also writes its isolated
`CARGO_HOME` and `CARGO_TARGET_DIR` under `.tmp/workflow-benchmarks/`. After
a cold run, that sub-directory can reach several gigabytes of cargo registry
and build artifacts, causing Docker to include it in the build context and copy
it into intermediate stages.

**Fix applied (2026-05-28)**: `/.tmp/` was added to `.dockerignore`. Re-running
the cold benchmark after this fix should reduce all `COPY . /…` steps to under
1 s.

**Lesson**: Any directory that is git-ignored but resides in the project root
must also be explicitly excluded from the Docker build context via `.dockerignore`.
These two ignore mechanisms are independent — git does not feed into Docker.
The per-step timing captured by `--progress plain` makes this category of
problem immediately visible; without it, the slow `COPY` would have been hidden
inside the aggregate stage time.

## Cargo Build Phase Analysis (Frontend vs Codegen vs Linker)

Source: `cargo build --timings --all-targets --release --workspace --all-features`
(same run as the Linker-Heavy Target Analysis above; total wall time 188 s, warm
incremental).

### How `cargo --timings` tracks phases

`cargo --timings` records two **sections** per compilation unit:

| Section name | Covers                                                                 |
| ------------ | ---------------------------------------------------------------------- |
| `frontend`   | Parsing, macro expansion, type-checking, borrow-checking, MIR lowering |
| `codegen`    | LLVM IR generation and object-file emission (`rustc` internal)         |

The **linker** is an external process invoked by `rustc` after codegen. It is
not tracked as a named section; its wall time appears as the gap between the end
of `codegen` and the end of the compilation unit's overall `duration`, or — for
units where `rustc` hands off immediately to the linker — as a `null` sections
field in the timing data.

### Units with section tracking (compilation-dominated, top 15)

These are external dependency crates compiled incrementally. Each unit is at
most ~8 s because individual crate compilation is parallelised.

| Rank | Total (s) | Frontend (s) | Codegen (s) | Crate                                      |
| ---- | --------- | ------------ | ----------- | ------------------------------------------ |
| 1    | 8.3       | 3.1          | 5.1         | torrust-tracker (lib)                      |
| 2    | 7.7       | 1.9          | 5.8         | torrust-tracker-axum-rest-api-server (lib) |
| 3    | 7.6       | 4.4          | 3.1         | tokio                                      |
| 4    | 7.5       | 7.2          | 0.2         | bollard-stubs                              |
| 5    | 7.4       | 3.8          | 3.6         | sqlx-postgres                              |
| 6    | 7.1       | 2.4          | 4.7         | criterion                                  |
| 7    | 6.5       | 2.5          | 4.0         | criterion (test variant)                   |
| 8    | 6.0       | 4.2          | 1.9         | h2                                         |
| 9    | 5.9       | 2.3          | 3.7         | regex-automata                             |
| 10   | 5.7       | 1.0          | 4.7         | torrust-tracker-configuration              |
| 11   | 5.6       | 2.0          | 3.6         | clap_builder                               |
| 12   | 5.4       | 3.7          | 1.8         | sqlx-postgres (test variant)               |
| 13   | 5.2       | 2.1          | 3.1         | sqlx-mysql                                 |
| 14   | 5.1       | 1.6          | 3.5         | toml_edit                                  |
| 15   | 4.6       | 2.3          | 2.2         | sqlx-core                                  |

**No single crate compilation takes more than ~8 s.** Frontend and codegen time
per crate are roughly balanced for most units.

### Units without section tracking (linker/C-build dominated, top 20)

These units report `sections: null` in the timing data, meaning `cargo` did not
capture frontend/codegen section boundaries. For final binary and test targets
this is the signature of a **linker invocation** — `rustc` hands all `.rlib`
object files to the external linker and waits; no `rustc`-internal phase tracking
occurs. For C build scripts (`build-script (run)`) the time is C compiler
invocation.

| Rank | Total (s) | Crate                                        | Target                                   |
| ---- | --------- | -------------------------------------------- | ---------------------------------------- |
| 1    | 117       | torrust-tracker                              | integration (test)                       |
| 2    | 117       | torrust-tracker                              | torrust-tracker (bin)                    |
| 3    | 116       | torrust-tracker                              | profiling (bin)                          |
| 4    | 109       | torrust-tracker                              | torrust_tracker_lib (lib,test)           |
| 5    | 109       | torrust-tracker-axum-health-check-api-server | integration (test)                       |
| 6    | 104       | torrust-tracker-core                         | persistence_benchmark_runner (bin)       |
| 7    | 103       | torrust-tracker-core                         | torrust_tracker_core (lib,test)          |
| 8    | 94        | torrust-tracker-axum-http-server             | integration (test)                       |
| 9    | 93        | torrust-tracker-axum-rest-api-server         | integration (test)                       |
| 10   | 92        | torrust-tracker-axum-rest-api-server         | lib (test)                               |
| 11   | 89        | torrust-tracker-axum-http-server             | lib (test)                               |
| 12   | 78        | torrust-tracker-udp-server                   | lib (test)                               |
| 13   | 71        | torrust-tracker-udp-server                   | integration (test)                       |
| 14   | 60        | torrust-tracker                              | qbittorrent_e2e_runner (bin)             |
| 15   | 56        | torrust-tracker-rest-api-core                | lib (test)                               |
| 16   | 52        | torrust-tracker-http-tracker-core            | lib (test)                               |
| 17   | 51        | torrust-tracker-client                       | tracker_client (bin)                     |
| 18   | 50        | torrust-tracker-core                         | integration (test)                       |
| 19   | 48        | torrust-tracker-http-tracker-core            | http_tracker_core_benchmark (bench,test) |
| 20   | 47        | torrust-tracker-client                       | tracker_checker (bin)                    |

**Build scripts (C compiler)**:

| Duration (s) | Crate          | Notes                                 |
| ------------ | -------------- | ------------------------------------- |
| 46           | libsqlite3-sys | SQLite3 C source compilation          |
| 33           | aws-lc-sys     | AWS-LC (BoringSSL fork) C compilation |
| 26           | zstd-sys       | zstd C source compilation             |

### Conclusion: the build is linker-dominated

- **Individual crate compilation** (frontend + codegen): ≤ 8 s per crate.
- **Binary/test target linking**: 35–117 s per binary — an order of magnitude more than any single crate compilation.
- **Root cause**: the workspace compiles ~20+ binary and test targets (`--all-targets`), each of which requires a full linker invocation over the entire transitive closure of `.rlib` objects.

Switching to a faster linker (e.g. `mold` or `lld`) or removing non-runtime binary targets from the build (subissue #2) are the two highest-leverage optimisations.

## Comparison Notes

### What dominated the cold run?

- **Container workflow**: The `build` and `dependencies` Dockerfile stages, which
  run `cargo nextest archive --tests --benches --examples --all-targets` for both
  debug (131 s archive + 47 s cook) and release (157 s archive + 64 s cook) profiles.
  The linking step for all non-runtime targets is the dominant cost.

- **Testing workflow**: The Docker E2E job (`docker_build_e2e` = 312 s) dominates
  because it re-executes the same full `cargo nextest archive` build inside the
  container. On CI, the `unit` job (139 s compile) and `docker-e2e` job run in
  parallel, so CI wall time is approximately 510 s.

### Which phases benefited from the warm cache?

- `test_unit`: 139 s → 16 s (incremental Rust compilation).
- `test_docs`: 58 s → 29 s (incremental).
- `fetch` and `install_linter`: 12 s → 0 s (registry and binary caches).
- `e2e_tracker`: 79 s → 16 s (image already in daemon cache).
- Container `build (debug)` and `build (release)`: essentially 0 s (all Docker
  layers cached).

### Which phases are not helped much by caching?

- `docker_build_e2e` warm: still 234 s because the `COPY . /build/src` layer
  invalidates on any file change, forcing the `build` stage to rerun.
- qBittorrent E2E phases: 29 s each regardless; dominated by container startup
  and DB initialisation, not by build time.

### Which linker-heavy targets appear unrelated to the final runtime image?

All test binaries, benches, and utility binaries in the top 30 list (27 out of
30 units). The most significant by time:

1. `torrust-tracker` integration tests — 117 s
2. `torrust-tracker` profiling bin — 116 s
3. All package-level integration test and lib-test variants — typically 50–110 s each

These are compiled because the Containerfile uses `--tests --benches --examples
--all-targets`. Narrowing the Containerfile build flags to only the targets
required for the runtime image is the most impactful next optimization (see
subissue #2 in the EPIC).

### Which measurements should be repeated after the next optimization?

After subissue #2 (narrow Containerfile targets):

- Re-run `run-container-baseline.sh --cold` and warm.
- Re-run `run-testing-baseline.sh --cold` and warm (the `docker_build_e2e` phase).
- Re-run `cargo build --timings --all-targets --release` to compare the new top-30.

## Follow-up

Append a new dated note after each later optimization.

- **2026-05-28** — Initial baseline captured. Container cold≈499 s sequential
  (CI≈260 s parallel). Testing cold≈767 s sequential (CI≈510 s parallel, dominated
  by docker-e2e). 27 of the top 30 compile units are not required by the runtime
  image; narrowing Containerfile build flags is the recommended first optimization.
- **2026-05-28** — `/.tmp/` added to `.dockerignore`; `time` wrappers added to all
  multi-command `RUN` blocks in the `Containerfile`; `--progress plain` added to
  `run-container-baseline.sh`. Re-run `--cold` to capture a new baseline log with
  accurate per-step and per-command durations.
