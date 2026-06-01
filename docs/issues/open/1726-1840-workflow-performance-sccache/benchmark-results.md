---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1726-1840-workflow-performance-sccache/ISSUE.md
---

# Cargo Build & Test Benchmark Results

Recorded on: 2026-05-01  
Machine: local dev (clean workspace)

---

## Command Timings

| #   | Command                                                                            | Wall time    | User CPU     | Sys CPU |
| --- | ---------------------------------------------------------------------------------- | ------------ | ------------ | ------- |
| 1   | `cargo clean`                                                                      | **1.28 s**   | 0.04 s       | 1.21 s  |
| 2   | `cargo fetch`                                                                      | **0.20 s**   | 0.11 s       | 0.07 s  |
| 3   | `cargo test --tests --benches --examples --workspace --all-targets --all-features` | **142.47 s** | 2171 s (CPU) | 151 s   |

---

## Breakdown of Command 3 (142.47 s total)

| Phase                                              | Duration | Share |
| -------------------------------------------------- | -------- | ----- |
| Compilation (`test` profile, from clean)           | ~127 s   | ~89 % |
| Test execution (sum of all `finished in Xs` lines) | ~13.6 s  | ~10 % |
| Process startup / harness overhead                 | ~1.9 s   | ~1 %  |

### Evidence

- `cargo test ... --no-run` (build-only, from clean): **126.72 s wall / 2m06s reported by Cargo**
- Warm rerun of full command (artifacts already built): **15.26 s wall / 0.63 s Cargo build phase**

> **Conclusion: the bottleneck is compilation, not test execution.**

---

## Slowest Test Binaries (execution time only)

| Rank | Execution time | Binary / suite                                                                    |
| ---- | -------------- | --------------------------------------------------------------------------------- |
| 1    | **5.04 s**     | `tests/integration.rs` — `torrust_tracker_udp_server` (6 tests)                   |
| 2    | **3.21 s**     | `unittests src/lib.rs` — `torrust_tracker_swarm_coordination_registry` (95 tests) |
| 3    | **2.08 s**     | `unittests src/lib.rs` — `torrust_tracker_udp_server` (122 tests)                 |
| 4    | **2.05 s**     | `tests/integration.rs` — `torrust_tracker_axum_health_check_api_server` (7 tests) |
| 5    | **0.36 s**     | `tests/integration.rs` — `torrust_tracker_axum_rest_api_server` (53 tests)        |
| 6    | **0.23 s**     | `tests/integration.rs` — `bittorrent_tracker_core` (5 tests)                      |
| 7    | **0.21 s**     | `tests/integration.rs` — `torrust_tracker_axum_http_server` (52 tests)            |
| …    | ≤ 0.10 s       | all remaining binaries                                                            |

Top 4 binaries account for **12.38 s** out of **13.60 s** total execution time (~91 %).

The slow integration tests in ranks 1, 3, and 4 are expected: they spin up real server instances and use OS-level socket connections. Rank 2 (`swarm_coordination_registry`) runs 95 async tests against an in-memory registry with `tokio::time` sleep calls inside test cases, which adds up.

---

## Compile Hotspot Analysis

Run from a clean build with `cargo test ... --no-run --timings`.  
Total wall time: **126 s** (matches the `--no-run` measurement above).  
Total CPU-time across all parallel jobs: **2088 s** (summed across all units).

### Top 20 — longest single compilation unit (critical path)

These are the crates that directly control the minimum possible build time because nothing
can be parallelised past them.

| Rank | Max single unit | Sum (all units) | # units | Crate                                             |
| ---- | --------------- | --------------- | ------- | ------------------------------------------------- |
| 1    | 77.19 s         | 606.43 s        | 13      | `torrust-tracker` (workspace root)                |
| 2    | 67.46 s         | 83.09 s         | 3       | `torrust-tracker-axum-health-check-api-server`    |
| 3    | 62.94 s         | 182.15 s        | 5       | `bittorrent-tracker-core`                         |
| 4    | 60.87 s         | 96.73 s         | 4       | `torrust-tracker-torrent-repository-benchmarking` |
| 5    | 59.04 s         | 116.97 s        | 3       | `torrust-tracker-axum-rest-api-server`            |
| 6    | 56.97 s         | 116.96 s        | 3       | `torrust-tracker-axum-http-server`                |
| 7    | 50.02 s         | 99.74 s         | 3       | `torrust-tracker-udp-server`                      |
| 8    | 33.82 s         | 34.21 s         | 2       | `torrust-tracker-rest-api-core`                   |
| 9    | 31.01 s         | 60.37 s         | 3       | `bittorrent-http-tracker-core`                    |
| 10   | 28.50 s         | 48.40 s         | 3       | `bittorrent-udp-tracker-core`                     |
| 11   | 21.01 s         | 22.01 s         | 3       | `aws-lc-sys` (external C build)                   |
| 12   | 18.94 s         | 19.36 s         | 2       | `bittorrent-http-tracker-protocol`                |
| 13   | 18.86 s         | 24.76 s         | 5       | `libsqlite3-sys` (external C build)               |
| 14   | 14.48 s         | 24.06 s         | 4       | `torrust-tracker-contrib-bencode`                 |
| 15   | 13.28 s         | 13.58 s         | 3       | `zstd-sys` (external C build)                     |
| 16   | 12.76 s         | 15.60 s         | 2       | `torrust-tracker-configuration`                   |
| 17   | 12.71 s         | 14.19 s         | 2       | `torrust-tracker-swarm-coordination-registry`     |
| 18   | 12.27 s         | 46.54 s         | 5       | `torrust-tracker-client`                          |
| 19   | 12.08 s         | 13.23 s         | 2       | `torrust-tracker-metrics`                         |
| 20   | 9.85 s          | 10.18 s         | 2       | `torrust-tracker-axum-server`                     |

### Heaviest external/C dependencies

| Sum     | Max unit | Crate            |
| ------- | -------- | ---------------- |
| 24.76 s | 18.86 s  | `libsqlite3-sys` |
| 22.01 s | 21.01 s  | `aws-lc-sys`     |
| 13.58 s | 13.28 s  | `zstd-sys`       |
| 9.71 s  | 5.58 s   | `tokio`          |
| 7.89 s  | 5.23 s   | `ring`           |
| 7.71 s  | 5.00 s   | `regex-automata` |
| 6.96 s  | 3.36 s   | `zerocopy`       |
| 6.62 s  | 3.55 s   | `openssl`        |
| 5.12 s  | 5.12 s   | `bollard-stubs`  |

---

## Recommendations

### Ranked optimization plan (compile — biggest gains first)

**1 — `sccache` (easiest, zero code changes, works on CI and locally)**

Caches compiled artifacts keyed by source hash. After the first cold build, every
subsequent clean build skips already-cached units. For the 126 s cold build here, a
warm `sccache` run would be roughly 5–10 s (only changed crates recompile).

```sh
cargo install sccache
export RUSTC_WRAPPER=sccache
cargo test --tests --benches --examples --workspace --all-targets --all-features
```

Add `RUSTC_WRAPPER=sccache` to `.cargo/config.toml` or CI env to make it permanent.

#### 2 — CI caching: the current setup doesn't help and here is why

`Swatinem/rust-cache` is already present in the `unit`, `check`, `database-compatibility`,
and `e2e` jobs, but it provides little to no benefit for this workspace. The reasons:

- **Cache size vs transfer speed tradeoff.** A cold `target/` for this workspace is ~9 GB.
  GitHub Actions cache upload/download runs at roughly 30–70 MB/s on `ubuntu-latest`.
  Restoring a 9 GB cache therefore costs 130–300 s — which is _more_ than the 127 s
  cold build. The cache pays off only if restore is faster than compile, which it isn't
  here.
- **No cross-job cache sharing.** Each job (format, check, unit, e2e) has its own cache
  key (`${{ runner.os }}-${{ matrix.toolchain }}-...`). They never share a build from a
  previous job in the same run. The `unit` job always rebuilds from scratch.
- **Cache is invalidated too often.** `Swatinem/rust-cache` keys on `Cargo.lock` hash
  plus toolchain. Any dependency bump or toolchain update flushes the entire cache.

The options that actually work at this scale:

| Option                                             | Mechanism                                                                                | Expected gain                                |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------- | -------------------------------------------- |
| **`sccache` with S3/GCS backend**                  | Caches individual codegen units by content hash; misses are granular, not all-or-nothing | ~80–90 % compile time saved on repeat pushes |
| **`sccache` with GitHub Actions cache backend**    | Same as above but uses GH cache storage instead of S3; free, but limited to 10 GB total  | ~60–80 % saved on repeat pushes              |
| **Shared `sccache` server** (self-hosted runner)   | Single cache server shared across all jobs and runs                                      | ~90 % saved; best ROI for a busy repo        |
| **Reduce what is compiled** (see points 3–8 below) | Smaller total work means smaller cache and faster misses                                 | Permanent gain, works in CI and locally      |

The most pragmatic immediate action is `sccache` with the GitHub Actions cache backend —
it requires no infrastructure, is free within the 10 GB limit, and unlike `Swatinem/rust-cache`
it caches at the _crate unit_ level so a single changed crate doesn't force a full rebuild.

```yaml
# In every job that compiles Rust, add before the cargo step:
- name: Install sccache
  uses: mozilla-actions/sccache-action@v0.0.6

- name: Enable sccache
  run: |
    echo "RUSTC_WRAPPER=sccache" >> "$GITHUB_ENV"
    echo "SCCACHE_GHA_ENABLED=true" >> "$GITHUB_ENV"
```

Remove the `Swatinem/rust-cache` step from those same jobs — the two caches conflict
and the `sccache` GHA backend handles registry caching as well.

**3 — Reduce monomorphisation in `torrust-tracker` (rank 1, 77 s single unit, 606 s total)**

The root crate compiles 13 separate codegen units (one per binary + test variants).
Each pays the full monomorphisation cost. Strategies:

- Move heavy generic code behind a `#[inline(never)]` boundary or into a shared
  internal crate so it is compiled once and linked.
- Extract large `impl` blocks into a `tracker-impl` crate that binaries depend on,
  rather than living in the root crate.

**4 — Split `bittorrent-tracker-core` (rank 3, 63 s single unit, 182 s CPU)**

This is the most-depended-upon workspace crate. Its size directly multiplies the cost
of every downstream crate that imports it. Consider splitting it along its subdomain
boundaries (e.g., separate announce logic, scrape logic, auth) so that a change in
one subdomain only forces recompilation of a smaller unit.

**5 — Reduce `--all-features` feature flag explosion**

The `--all-features` flag enables every combination of features across the workspace.
Many crates compile multiple times under different feature sets. Profile which feature
combinations are exercised in practice; disable unused combinations in CI by running
per-crate with only the features that combination actually exercises.

**6 — Link-time: switch to `lld` or `mold` linker**

Linking is not the dominant cost here (compile is), but switching the linker reduces
the final 10–20 % of cold build time at no code-change cost.

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

**7 — C build scripts: `aws-lc-sys`, `libsqlite3-sys`, `zstd-sys` (combined 60 s)**

These C libraries are compiled from source each clean build. Options:

- `SQLITE_USE_SYSTEM` / `SQLX_SQLITE_USE_SYSTEM` env vars make `libsqlite3-sys` use
  the system-installed SQLite, skipping the C compile entirely.
- `aws-lc-sys` can be replaced by `ring` for TLS if the feature set allows it, saving
  ~21 s. Check whether `aws-lc` is pulled in by `rustls` and whether the `ring`
  backend can be selected instead.

**8 — `torrust-tracker-contrib-bencode` (rank 14, 14 s single unit)**

The `bencode` crate in `contrib/` takes ~14 s per unit despite being a small
domain-specific library. Investigate whether it carries unexpectedly heavy trait
bounds or large constant arrays that inflate codegen time. Adding
`codegen-units = 16` to its dev profile would parallelise it.

---

### To speed up test execution (minor gain, ~10 % of total time)

- The slow integration tests (UDP server 5.04 s, health-check 2.05 s) spin up real OS
  sockets; they cannot be sped up without test-design changes.
- `swarm_coordination_registry` (3.21 s, 95 tests) likely contains real `sleep` calls.
  Replacing them with the project's `clock` mock would cut this to near zero.
- `cargo nextest` runs test binaries in parallel and reports per-test timing; it would
  reduce the 15.26 s warm execution to roughly 6–8 s on a multi-core machine.

  ```sh
  cargo install cargo-nextest
  cargo nextest run --workspace --all-features
  ```
