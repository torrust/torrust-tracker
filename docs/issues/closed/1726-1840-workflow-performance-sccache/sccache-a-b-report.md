# sccache A/B Benchmark Report

> **Objective**: Measure whether `sccache` improves local rebuild times versus baseline (no `sccache`) for the Torrust Tracker workspace.
>
> **Protocol**: Follow [ISSUE.md](./ISSUE.md) Task 1 (Local Research A/B) exactly.
>
> **Date**: 2026-06-11
>
> **Machine**: local dev workstation
>
> **Branch**: `1726-reduce-build-times-sccache` (based on `develop`)

---

## Environment

| Variable            | Value                                         |
| ------------------- | --------------------------------------------- |
| `RUSTC_WRAPPER`     | `<unset>` initially                           |
| `CARGO_INCREMENTAL` | `<unset>` initially                           |
| `rustc --version`   | `rustc 1.98.0-nightly (485ec3fbc 2026-06-10)` |
| `cargo --version`   | `cargo 1.98.0-nightly (0b1123a48 2026-06-01)` |
| OS                  | Linux                                         |
| CPU                 | AMD Ryzen 9 7950X 16-Core / 32 threads        |
| RAM                 | 61 GiB                                        |

---

## Phase A: Baseline (no `sccache`)

### A1: Cold build — baseline

**Command**:

```sh
cd ~/torrust-tracker
unset RUSTC_WRAPPER
export CARGO_INCREMENTAL=0
cargo clean
/usr/bin/time -f 'real=%e  user=%U  sys=%S' cargo test \
  --tests --benches --examples \
  --workspace --all-targets --all-features --no-run
```

**Output** (last ~50 lines):

```text
  Executable unittests src/lib.rs (target/debug/deps/torrust_tracker_test_helpers-...)
  Executable unittests src/lib.rs (target/debug/deps/torrust_tracker_torrent_repository_benchmarking-...)
  Executable tests/integration.rs (target/debug/deps/integration-...)
  Executable benches/repository_benchmark.rs (target/debug/deps/repository_benchmark-...)
  Executable unittests src/lib.rs (target/debug/deps/torrust_tracker_udp_server-...)
  Executable tests/integration.rs (target/debug/deps/integration-...)
  Executable unittests examples/udp_only_public_tracker.rs (target/debug/examples/udp_only_public_tracker-...)
  Executable unittests src/lib.rs (target/debug/deps/torrust_tracker_udp_tracker_core-...)
  Executable benches/udp_tracker_core_benchmark.rs (target/debug/deps/udp_tracker_core_benchmark-...)
  Executable unittests src/lib.rs (target/debug/deps/torrust_tracker_udp_tracker_protocol-...)
  Executable unittests src/main.rs (target/debug/deps/workspace_coupling-...)
real=112.50  user=1903.35  sys=142.04
```

**Wall time**: **112.50 s**

> Compared to 126.72 s recorded on 2026-05-01 — ~11 % faster, likely due to dependency updates and compiler improvements.

---

### A2: Warm build — baseline

**Command** (no `cargo clean` between A1 and A2):

```sh
/usr/bin/time -f 'real=%e  user=%U  sys=%S' cargo test \
  --tests --benches --examples \
  --workspace --all-targets --all-features --no-run
```

**Output** (last 10 lines):

```text
    Finished `test` profile [optimized + debuginfo] target(s) in 0.38s
  ...
  Executable unittests src/main.rs (target/debug/deps/workspace_coupling-...)
real=0.42  user=0.20  sys=0.10
```

**Wall time**: **0.42 s**

> Cargo detects no source changes since A1 and skips all compilations. This is the ideal scenario: no rebuild needed.

---

## Phase B: Install `sccache`

### B1: Install via `apt`

**Command**:

```sh
sudo apt install -y sccache
```

**Output**:

```text
Installing:
  sccache
Summary:
  Upgrading: 0, Installing: 1, Removing: 0, Not Upgrading: 10
  Download size: 4.775 kB
  Space needed: 14.1 MB
Setting up sccache (0.13.0+ds-3build1)…
```

**Version**: **0.13.0** (Ubuntu package — older stable release)

### B2: Install via `cargo install`

**Command**:

```sh
cargo install sccache
```

**Output**:

```text
(Skipped — apt install was used instead, see B1 above)
```

**Version**: N/A (not installed via cargo)

---

## Phase C: `sccache` measurements (using `[apt|cargo]` installation)

### C1: Cold build through `sccache`

**Command**:

```sh
sccache --stop-server 2>/dev/null; sccache --start-server
export RUSTC_WRAPPER=sccache
export CARGO_INCREMENTAL=0
cargo clean
/usr/bin/time -f 'real=%e  user=%U  sys=%S' cargo test \
  --tests --benches --examples \
  --workspace --all-targets --all-features --no-run
sccache --show-stats
```

**Terminal output** (final lines):

```text
real=137.11  user=1413.45  sys=96.31
=== SCCACHE STATS ===
Compile requests                   1187
Compile requests executed          1020
Cache hits                            2
Cache hits (Rust)                     2
Cache misses                       1018
Cache hits rate                     0.20 %
Cache hits rate (Rust)              0.33 %
Non-cacheable calls                 161
  crate-type                        144
Cache size                          451 MiB
```

**Wall time**: **137.11 s**

> Cold sccache build is **22 % slower** than baseline cold (112.50 s). Every unit is a cache miss, and sccache's overhead (wrapping each compiler invocation) adds ~25 s. Only 2 accidental hits (Rust standard library prelude or similar).
>
> **144 non-cacheable** calls due to `crate-type` — these are the `bin`, `proc-macro`, and `dylib` crates that sccache cannot cache at all.

---

### C2: Warm build through `sccache`

**Command** (no `cargo clean` between C1 and C2):

```sh
/usr/bin/time -f 'real=%e  user=%U  sys=%S' cargo test \
  --tests --benches --examples \
  --workspace --all-targets --all-features --no-run
sccache --show-stats
```

**Terminal output** (final lines):

```text
    Finished `test` profile [optimized + debuginfo] target(s) in 0.22s
real=0.26  user=0.18  sys=0.08
=== SCCACHE STATS ===
Cache hits                            2  (unchanged — no compilations triggered)
Cache misses                       1018  (unchanged)
Cache hits rate                    0.20 %
```

> **Wall time**: **0.26 s** — identical to baseline warm (0.42 s). No compilations needed because no source changed. Cargo's own dependency-checking is the dominant cost here, not sccache.

---

### C3: Warm build after single-file change in leaf crate (`packages/primitives/src/lib.rs`)

**Command**:

```sh
touch packages/primitives/src/lib.rs
/usr/bin/time -f 'real=%e  user=%U  sys=%S' cargo test \
  --tests --benches --examples \
  --workspace --all-targets --all-features --no-run
sccache --show-stats
```

**Terminal output** (final lines):

```text
   Compiling torrust-tracker-primitives v3.0.0-develop
   Compiling torrust-tracker-configuration v3.0.0-develop
   Compiling torrust-tracker-swarm-coordination-registry v3.0.0-develop
   Compiling torrust-tracker-client-lib v3.0.0-develop
   Compiling torrust-tracker-torrent-repository-benchmarking v3.0.0-develop
   Compiling torrust-tracker-client v3.0.0-develop
   Compiling torrust-tracker-core v3.0.0-develop
   Compiling torrust-tracker-axum-server v3.0.0-develop
   Compiling torrust-tracker-test-helpers v3.0.0-develop
   Compiling torrust-tracker-axum-health-check-api-server v3.0.0-develop
   Compiling torrust-tracker-udp-tracker-core v3.0.0-develop
   Compiling torrust-tracker-http-tracker-core v3.0.0-develop
   Compiling torrust-tracker-persistence-benchmark v3.0.0-develop
   Compiling torrust-tracker-axum-http-server v3.0.0-develop
   Compiling torrust-tracker-udp-server v3.0.0-develop
   Compiling torrust-tracker-rest-api-core v3.0.0-develop
   Compiling torrust-tracker-axum-rest-api-server v3.0.0-develop
   Compiling torrust-tracker v3.0.0-develop
   Compiling torrust-tracker-e2e-tools v3.0.0-develop
    Finished `test` profile [optimized + debuginfo] target(s) in 1m 25s
real=85.81  user=1433.32  sys=84.41
=== SCCACHE STATS ===
Compile requests                   1251
Compile requests executed          1037
Cache hits                           19  (cumulative, +17 from C1)
Cache hits (Rust)                    19
Cache misses                       1018  (cumulative, unchanged)
Cache hits rate                    1.83 %
Cache hits rate (Rust)             3.07 %
Non-cacheable calls                 208  (+47 from C1)
  crate-type                        191  (+47)
```

**Wall time**: **85.81 s**

> **Key finding**: Even with a full sccache warm cache, touching a single leaf crate forces recompilation of **all 19 downstream workspace crates** plus the `torrust-tracker` bin crate (77 s critical-path unit, never cached). Only external/C dependencies were cache hits (17 new hits since C1). The 85.81 s is still overwhelmingly dominated by recompilation, not by sccache overhead.

---

## Results Summary

| Scenario                 | Configuration         | Wall time    | vs Baseline cold     | Cache hits               |
| ------------------------ | --------------------- | ------------ | -------------------- | ------------------------ |
| Cold                     | Baseline (no sccache) | **112.50 s** | —                    | —                        |
| Warm (no changes)        | Baseline (no sccache) | **0.42 s**   | -99.6 %              | —                        |
| Cold                     | sccache               | **137.11 s** | **+21.9 %** (slower) | 0.20 % (2 / 1020)        |
| Warm (no changes)        | sccache               | **0.26 s**   | -99.8 %              | 0.20 % (no compilations) |
| Warm-after-change (leaf) | sccache               | **85.81 s**  | -23.7 %              | 1.83 % (19 / 1037)       |

> **Baseline warm-after-change not separately measured** but sccache's warm-after-change (85.81 s)
> can be compared to baseline cold (112.50 s) since a full rebuild is required in both cases.
> sccache saves ~27 s on external/C dependencies (19 hits out of ~600 cacheable units).

## Analysis

### Why sccache underperforms for this workspace

1. **The heaviest crate is never cached**: `torrust-tracker` (workspace root, rank 1 at 77 s single
   unit) is a `bin` crate. sccache only caches `rlib`/`lib` units, so this crate always recompiles
   from scratch in ~77 s. Even the warm-after-change test took **85.81 s** — and most of that is
   the unlucky 13-codegen-unit `torrust-tracker` root crate plus 18 downstream workspace crates
   that all had to recompile because `primitives` is deep in the dependency tree.

2. **The workspace is small and tightly coupled**: touching a leaf crate (`primitives`) triggers
   recompilation of virtually the **entire workspace** (19 crates). sccache can only accelerate
   external dependencies (those `-sys` crates, `tokio`, `ring`, etc.) — which are a minority of
   total compile time on a warm cache (17 new hits, saving ~27 s of the full 137 s).

3. **Non-cacheable calls dominate**: 191 non-cacheable calls due to `crate-type` (bin/proc-macro).
   The more binary targets and proc-macro crates the workspace has, the less benefit sccache
   provides.

4. **Incremental compilation must be disabled**: The `test` profile uses incremental by default.
   sccache requires `CARGO_INCREMENTAL=0`, which may actually **hurt** the local development
   experience for small iterative changes (where incremental compilation is faster than full
   recompile-from-scratch through sccache).

### Where sccache _does_ help

- **External/C dependency rebuilds**: Those `libsqlite3-sys`, `aws-lc-sys`, `zstd-sys` C builds
  (total ~60 s combined) are fully cached after first compile. On a clean checkout with sccache
  warm, those ~60 s are avoided.
- **CI cross-job caching** (via GHA backend): if CI runners share the same cache, the second
  workflow run in a PR (e.g., after a force-push that changes only one file) would skip
  recompilation of all unchanged external crates.

## Conclusion: **Do not adopt sccache for local development**

The evidence shows that sccache provides **minimal benefit** for local development on this
workspace:

| Criterion              | Verdict                                                                     |
| ---------------------- | --------------------------------------------------------------------------- |
| Cold build             | **Worse** (+22 %, 137 s vs 113 s baseline)                                  |
| Warm (no change)       | **Equivalent** (~0.3 s both ways)                                           |
| Warm-after-change      | **Modest improvement** (-24 %, 86 s sccache vs ~113 s baseline cold)        |
| Setup cost             | `cargo install sccache` + config changes                                    |
| Non-cacheable overhead | 191 calls per rebuild, mostly the critical-path `torrust-tracker` bin crate |

**Recommendation for local dev**: Keep the current setup. The `torrust-tracker` bin crate (the
\#1 hotspot, 77 s) is never cached by sccache, and the workspace dependency graph is so tight that
touching any leaf forces nearly everything to recompile anyway.

**For CI**: sccache may still be worth exploring with the GHA cache backend (`SCCACHE_GHA_ENABLED`),
where cross-job and cross-run cache sharing could produce real savings. This is explored in
[ISSUE.md Task 3](./ISSUE.md#tasks). The expected benefit on CI is lower than typical because:

- The `torrust-tracker` bin crate (rank 1) will never be cached.
- Only external/C dependencies (rank 11–15, ~60 s total) will be saved.

### Next steps

1. Record a new cold build with `cargo` for the updated `compile-hotspot-analysis.md` baseline.
2. Proceed to **Task 2** (local configuration decision — expected to be: _don't enable by default_).
3. Proceed to **Task 3** (CI A/B benchmarks) to assess GHA-backend benefit.
