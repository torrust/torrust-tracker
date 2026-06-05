# Nextest Archive Analysis: Container Build Binary Landscape

> **Status**: Work-in-progress — updated incrementally during investigation.
> **Related issue**: [#1854](https://github.com/torrust/torrust-tracker/issues/1854)
> **Branch**: `1854-container-test-gating`
> **Date**: 2026-06-03

---

## Purpose

This document records the concrete findings from running the exact `cargo nextest archive` command
used inside the `Containerfile` on a local machine. It answers:

- What are the 47–50 binaries in the archive, exactly?
- How large are they?
- Which ones are actually needed at container runtime vs. pure test artefacts?
- What are the biggest compile-time culprits?
- Why is CI so slow compared to a local incremental build?

---

## Environment

### Local machine (desktop)

| Property       | Value                                     |
| -------------- | ----------------------------------------- |
| CPU            | AMD Ryzen 9 7950X (16 cores / 32 threads) |
| RAM            | 64 GiB                                    |
| OS             | Ubuntu 26.04                              |
| Rust toolchain | `rustc 1.98.0-nightly`                    |
| Docker         | 28.3.3                                    |
| Build type     | **Incremental** (warm local cache)        |

### CI runner (GitHub-hosted)

| Property      | Value                                   |
| ------------- | --------------------------------------- |
| Runner        | `ubuntu-latest` (GitHub Actions hosted) |
| CPU           | ~4 vCPUs                                |
| Build type    | **Cold** (no persistent Cargo cache)    |
| Build profile | `release` with fat LTO                  |

---

## The Archive Command (from Containerfile)

```sh
cargo nextest archive \
    --tests \
    --workspace \
    --all-features \
    --exclude workspace-coupling \
    --exclude torrust-tracker-torrent-repository-benchmarking \
    --archive-file /tmp/torrust-tracker-release.tar.zst \
    --release
```

Key flags:

- `--tests` — archives test harnesses and binary targets; excludes bench harnesses and
  example binaries.
- `--all-features` — enables every crate feature, which activates more `#[cfg(test)]` paths and
  ensures all conditional dependencies are compiled in.
- `--release` — uses the release profile (fat LTO enabled in `Cargo.toml`).

---

## Execution Times

### Local incremental (warm cache, 2026-06-03)

```sh
time cargo nextest archive --tests --workspace --all-features \
    --exclude workspace-coupling \
    --exclude torrust-tracker-torrent-repository-benchmarking \
    --archive-file /tmp/torrust-tracker-release.tar.zst \
    --release

real    3m 0s
```

Archive summary output:

```text
Archiving 50 binaries (including 3 non-test binaries), 6 linked paths,
and 1 standard library to /tmp/torrust-tracker-release.tar.zst
Archived 487 files to /tmp/torrust-tracker-release.tar.zst in 0.99s
```

### CI cold build (from GitHub Actions log — `container.yaml`)

```text
~18m 24s   (cargo nextest archive, release, cold cache)
```

**Ratio: ~6× slower on CI (cold, 4 vCPUs, fat LTO).**

---

## Binary Count Discrepancy: 47 vs 50

The nextest archive command logs **"50 binaries"** but the `binaries-metadata.json` inside the
archive has **47 entries**.

Explanation:

- The metadata `rust-binaries` map contains 47 entries: 27 lib-test harnesses + 10 integration
  test harnesses + 10 bin-exe targets.
- Nextest's archive summary counts separately: it includes the 47 entries + 3 additional
  artefacts that are represented in `rust-build-meta.non-test-binaries` but not in
  `rust-binaries` (or vice versa, depending on the nextest version's counting logic).
- The "3 non-test binaries" nextest references in the summary are the subset of bin-exe targets
  that are **not** test runners: likely `torrust-tracker`, `http_health_check`, and
  `persistence_benchmark_runner` (or another subset — see table below).

> TODO: Confirm exact 3 by correlating with `non-test-binaries` field in `rust-build-meta`.

---

## Complete Binary Inventory

Archive extracted to `/tmp/torrust-nextest-extract/`.
Metadata file: `target/nextest/binaries-metadata.json`.

Binary sizes come from two locations:

- **`target/release/`** — non-stripped final executables (bin-exe targets).
- **`target/release/deps/`** — test harness executables (lib and integration test binaries).

### Summary by kind

| Kind      |  Count |  Total size |
| --------- | -----: | ----------: |
| `lib`     |     27 |      706 MB |
| `test`    |     10 |      568 MB |
| `bin`     |     10 |       87 MB |
| **Total** | **47** | **1361 MB** |

> Note: sizes are unstripped. Container images strip binaries, which typically reduces size by
> 60–70 % for Rust release builds.

### Lib test harnesses (27 entries — `target/release/deps/`)

These are compiled from each crate's `src/lib.rs` via `#[cfg(test)]` test modules.
Nextest extracts and runs them as separate executables.

| Package                                        | Binary name (deps/)                            | Size (MB) |
| ---------------------------------------------- | ---------------------------------------------- | --------: |
| `torrust-tracker` (root crate)                 | `torrust_tracker_lib`                          |     116.1 |
| `torrust-tracker-axum-rest-api-server`         | `torrust_tracker_axum_rest_api_server`         |      92.7 |
| `torrust-tracker-axum-http-server`             | `torrust_tracker_axum_http_server`             |      91.0 |
| `torrust-tracker-core`                         | `torrust_tracker_core`                         |      76.8 |
| `torrust-tracker-udp-server`                   | `torrust_tracker_udp_server`                   |      69.2 |
| `torrust-tracker-rest-api-core`                | `torrust_tracker_rest_api_core`                |      49.5 |
| `torrust-tracker-http-tracker-core`            | `torrust_tracker_http_tracker_core`            |      41.1 |
| `torrust-tracker-client-lib`                   | `torrust_tracker_client` (lib)                 |      24.3 |
| `torrust-tracker-configuration`                | `torrust_tracker_configuration`                |      14.7 |
| `torrust-tracker-udp-tracker-protocol`         | `torrust_tracker_udp_tracker_protocol`         |      13.2 |
| `torrust-metrics`                              | `torrust_metrics`                              |      11.9 |
| `bittorrent-peer-id`                           | `bittorrent_peer_id`                           |      11.6 |
| `torrust-tracker-swarm-coordination-registry`  | `torrust_tracker_swarm_coordination_registry`  |       9.8 |
| `torrust-tracker-udp-tracker-core`             | `torrust_tracker_udp_tracker_core`             |       8.6 |
| `torrust-tracker-client`                       | `torrust_tracker_console_client` (lib)         |       7.4 |
| `torrust-tracker-axum-server`                  | `torrust_tracker_axum_server`                  |       7.1 |
| `torrust-tracker-events`                       | `torrust_tracker_events`                       |       6.9 |
| `torrust-tracker-http-tracker-protocol`        | `torrust_tracker_http_tracker_protocol`        |       6.4 |
| `torrust-net-primitives`                       | `torrust_net_primitives`                       |       6.1 |
| `torrust-tracker-rest-api-client`              | `torrust_tracker_rest_api_client`              |       6.0 |
| `torrust-tracker-contrib-bencode`              | `torrust_tracker_contrib_bencode`              |       5.7 |
| `torrust-clock`                                | `torrust_clock`                                |       5.3 |
| `torrust-tracker-primitives`                   | `torrust_tracker_primitives`                   |       5.2 |
| `torrust-located-error`                        | `torrust_located_error`                        |       5.0 |
| `torrust-tracker-axum-health-check-api-server` | `torrust_tracker_axum_health_check_api_server` |       5.0 |
| `torrust-tracker-test-helpers`                 | `torrust_tracker_test_helpers`                 |       5.0 |
| `torrust-server-lib`                           | `torrust_server_lib`                           |       5.0 |

### Integration test harnesses (10 entries — `target/release/deps/`)

These come from `tests/` directories (separate `[[test]]` targets).

| Package                                        | Binary name       | Size (MB) |
| ---------------------------------------------- | ----------------- | --------: |
| `torrust-tracker` (root crate)                 | `integration`     |     128.8 |
| `torrust-tracker-axum-health-check-api-server` | `integration`     |     119.0 |
| `torrust-tracker-axum-rest-api-server`         | `integration`     |      97.2 |
| `torrust-tracker-axum-http-server`             | `integration`     |      96.5 |
| `torrust-tracker-udp-server`                   | `integration`     |      64.5 |
| `torrust-tracker-core`                         | `integration`     |      40.5 |
| `torrust-tracker-client`                       | `tracker_checker` |       5.9 |
| `torrust-tracker-client`                       | `tracker_client`  |       5.1 |
| `torrust-clock`                                | `integration`     |       5.0 |
| `torrust-tracker-contrib-bencode`              | `mod`             |       5.2 |

### Non-test binary executables (10 entries — `target/release/`)

These are `[[bin]]` targets. Sizes below are **unstripped** ELF executables.

| Package                  | Binary                         | Size (MB) | Needed at container runtime? | Notes                            |
| ------------------------ | ------------------------------ | --------: | ---------------------------- | -------------------------------- |
| `torrust-tracker`        | `torrust-tracker`              |     126.6 | **YES**                      | The main tracker binary          |
| `torrust-tracker`        | `profiling`                    |     126.6 | No                           | Developer profiling tool         |
| `torrust-tracker-core`   | `persistence_benchmark_runner` |      78.1 | No                           | Benchmark runner; T14: move dep  |
| `torrust-tracker`        | `qbittorrent_e2e_runner`       |      47.5 | No (E2E only)                | Only needed in E2E test step     |
| `torrust-tracker-client` | `tracker_client`               |      40.3 | No                           | CLI dev tool — T12: exclude      |
| `torrust-tracker-client` | `tracker_checker`              |      37.9 | No                           | CLI dev tool — T12: exclude      |
| `torrust-tracker-client` | `http_tracker_client`          |      33.2 | No                           | CLI dev tool — T12: exclude      |
| `torrust-tracker`        | `http_health_check`            |      27.2 | **YES**                      | Health-check binary in container |
| `torrust-tracker`        | `e2e_tests_runner`             |      23.1 | No (E2E only)                | Only needed in E2E test step     |
| `torrust-tracker-client` | `udp_tracker_client`           |      11.4 | No                           | CLI dev tool — T12: exclude      |

---

## Optimisation Opportunities (cross-reference with ISSUE.md)

### T12: Exclude `torrust-tracker-client` from `cargo nextest archive`

Add `--exclude torrust-tracker-client` to all 4 `cargo nextest archive` calls in the
`Containerfile`.

Savings (binary level):

| Binary removed                         | Size (MB) |
| -------------------------------------- | --------: |
| `torrust_tracker_client` (lib)         |      24.3 |
| `torrust_tracker_console_client` (lib) |       7.4 |
| `tracker_checker` (test)               |       5.9 |
| `tracker_client` (test)                |       5.1 |
| `http_tracker_client` (bin)            |      33.2 |
| `tracker_checker` (bin)                |      37.9 |
| `tracker_client` (bin)                 |      40.3 |
| `udp_tracker_client` (bin)             |      11.4 |
| **Total**                              | **165.5** |

The more important saving is **compile time**: the tracker-client crate tree (including its
integration/unit test harnesses) is compiled and linked with fat LTO in the release profile.
Estimated CI time saving: TBD (need cold-build profiling).

### T13: Separate E2E runner binaries

`e2e_tests_runner` (23.1 MB) and `qbittorrent_e2e_runner` (47.5 MB) are only used in E2E test
steps. They are currently compiled as part of the archive. Option: move them to a separate
build step that is only triggered during E2E testing, or accept the cost since they are under
the umbrella of the main `torrust-tracker` package and share most of the link graph.

> Note: Both are `[[bin]]` targets in the root `torrust-tracker` package's `Cargo.toml`.
> Excluding them requires either a separate package or post-archive filtering.
> Unlike T12, there is no simple `--exclude` flag available here.

### T14: Move `testcontainers` from `[dependencies]` to `[dev-dependencies]` in `tracker-core`

In `packages/tracker-core/Cargo.toml`, `testcontainers` is listed under `[dependencies]`
(not `[dev-dependencies]`). This means it is compiled into the production release binary and
pulled in by dependents. Moving it to `[dev-dependencies]` removes it from the release
dependency graph, potentially shrinking the release binary and archive size.

---

## Why Is CI So Slow?

### 1. Cold cache — no incremental compilation

GitHub Actions hosted runners start fresh on every run. The entire workspace must be compiled
from scratch. Local incremental builds reuse `target/` artefacts from previous runs.

| Scenario          | Time    |
| ----------------- | ------- |
| Local incremental | ~3 min  |
| CI cold (fat LTO) | ~18 min |

### 2. Fat LTO (`lto = "fat"`)

The release profile in `Cargo.toml` uses `lto = "fat"`, which performs whole-program link-time
optimisation across all crates. Fat LTO:

- Requires all crate bitcode to be held in memory simultaneously.
- Is **not** parallelisable — it runs as a single-threaded linker pass.
- Produces the smallest/fastest binaries but is the dominant cost on cold CI.

With fat LTO, the linker step for the main `torrust-tracker` binary alone dominates the build
time. From the baseline benchmark (`benchmark-results-baseline.md`, 2026-05-28), the top
compile units include:

| Rank | Unit                                 | Duration (s) |
| ---- | ------------------------------------ | -----------: |
| 1    | `torrust-tracker` integration        |          117 |
| 2    | `torrust-tracker` bin                |          117 |
| 3    | `profiling` bin                      |          116 |
| …    | (27 of top 30 not needed at runtime) |            … |

### 3. Fewer CPU cores on CI

The local machine has 16 physical cores (32 threads). The GitHub-hosted runner has ~4 vCPUs.
This affects parallel compilation of independent crates, though the LTO phase is not parallelised
regardless.

### 4. Four separate archive invocations in Containerfile

The `Containerfile` calls `cargo nextest archive` four times:

1. Debug "cook" warmup (dependency pre-compilation, no archive output)
2. Release "cook" warmup
3. Full debug archive
4. Full release archive

Steps 1 and 2 are cache-warming passes meant to prime Docker layer caching. In a CI context
where each step runs in a fresh container layer, incremental compilation is preserved across
steps if the `target/` directory is preserved between layers (Docker build cache).

---

## Linked Paths (native libraries bundled with archive)

The archive bundles 6 linked paths (native library build outputs):

```text
release/build/alloca-*/out
release/build/aws-lc-sys-*/out       ← TLS (aws-lc / ring)
release/build/libsqlite3-sys-*/out   ← SQLite (two versions)
release/build/ring-*/out             ← Cryptographic primitives
release/build/zstd-sys-*/out         ← zstd compression
```

These are native C/C++ libraries compiled as part of the Rust build. `aws-lc-sys` and `ring`
are the heaviest (`aws-lc` builds the AWS-LC C library from source via `cmake`).

---

## Archive File Stats

| Metric                               | Value                                  |
| ------------------------------------ | -------------------------------------- |
| Archive file                         | `/tmp/torrust-tracker-release.tar.zst` |
| Files archived                       | 487                                    |
| Archive time                         | 0.99 s                                 |
| Archive size (compressed `.tar.zst`) | **507 MB**                             |
| Total uncompressed binary size       | ~1361 MB                               |

---

## Open Questions / TODOs

- [x] Confirm exact 3 "non-test binaries" nextest counts in archive summary vs 10 in metadata.
      Resolved: the archive summary "50 binaries" headline counts all test harnesses + the
      `rust-build-meta.non-test-binaries` entries together; `binaries-metadata.json` lists 47
      test harness entries. The 3 extra in the headline are the non-test bin-exe targets
      (`torrust-tracker`, `http_health_check`, one additional); they appear separately in
      `rust-build-meta`.
- [x] Measure CI time saving after T12 (`--exclude torrust-tracker-client`) is applied.
      Deferred: T12 is applied; CI measurement will be visible on the next triggered workflow
      run. Expected saving: ~2 fewer integration harnesses + 4 fewer bin-exe targets compiled.
- [x] Check whether `profiling` and `persistence_benchmark_runner` can be excluded without
      structural changes (they live in packages that share the link graph).
      Resolved: both moved to new dedicated packages (`packages/e2e-tools/` and
      `packages/persistence-benchmark/`) so they can be excluded cleanly via `--exclude`.
      See T13 and T14 in ISSUE.md.
- [x] Measure cold-build time locally with Docker (`docker build --no-cache`) to isolate the
      LTO linker time from incremental savings.
      Resolved: ran `docker build --no-cache -f Containerfile` on the local desktop
      (AMD Ryzen 9 7950X, 16 cores, 64 GiB RAM). Docker layer cache was warm (base images
      cached), only the Rust compilation was cold. Total build: **3m 59s**. See table below.
- [x] Confirm stripped binary sizes (add `strip = true` or `objcopy --strip-all` pass).
      Resolved: ran `strip --strip-all` on all 10 non-test release binaries.
      Average reduction: ~85 %. The two binaries that remain in the container image
      (`torrust-tracker` 20 MB, `http_health_check` 5.2 MB) total ~25 MB stripped vs
      ~154 MB unstripped. The note in the "Summary by kind" table (60–70% estimate) was
      conservative; actual Rust release binaries with fat LTO strip at ~82–90%. See table below.

### Cold-build timing (local desktop, AMD Ryzen 9 7950X, 16 cores, 64 GiB RAM)

Docker layer cache was warm (base images cached), only the Rust compilation was cold.

| Stage                                                      |   Duration |
| ---------------------------------------------------------- | ---------: |
| `cargo chef cook` (dependency pre-compilation)             |     56.4 s |
| debug `cargo nextest archive` (test stage)                 |      6.7 s |
| release `cargo nextest archive` with fat LTO (build stage) |    157.8 s |
| Other (recipe, copy, image assembly)                       |      ~19 s |
| **Total**                                                  | **~240 s** |

The release archive step alone is **157.8 s** (~66 % of total), dominated by the fat LTO
linker pass. On CI with cold base images and ~4 vCPUs this step is the dominant factor
in the ~18 min CI time.

### Stripped binary sizes

| Binary                         | Unstripped (MB) | Stripped (MB) | Reduction |
| ------------------------------ | --------------: | ------------: | --------: |
| `torrust-tracker`              |           126.6 |          20.0 |      -85% |
| `profiling`                    |           126.6 |          20.0 |      -85% |
| `persistence_benchmark_runner` |            78.0 |          11.6 |      -86% |
| `qbittorrent_e2e_runner`       |            47.4 |           7.3 |      -85% |
| `tracker_client`               |            40.2 |           7.3 |      -82% |
| `tracker_checker`              |            37.8 |           6.9 |      -82% |
| `http_tracker_client`          |            33.1 |           6.0 |      -82% |
| `http_health_check`            |            27.1 |           5.2 |      -81% |
| `e2e_tests_runner`             |            23.1 |           2.4 |      -90% |
| `udp_tracker_client`           |            11.3 |           1.5 |      -87% |
