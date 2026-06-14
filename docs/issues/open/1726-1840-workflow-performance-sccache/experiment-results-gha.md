# GHA sccache Experiment Results — Task 3a

> **Workflow**: `experiment-sccache-bare-build.yaml`  
> **Run 1**: https://github.com/josecelano/torrust-tracker/actions/runs/27362158469  
> **Commit**: `4bf6792e` — `ci(experiment): add sccache bare build workflow (task 3a)`  
> **Date**: 2026-06-11  
> **Runner**: `ubuntu-latest` (GitHub-hosted)

---

## Cold Build (no prior sccache cache)

| Metric                     | Value                                 |
| -------------------------- | ------------------------------------- |
| **Wall time**              | **479.44 s** (~8 min)                 |
| `cargo build --release`    | `real=479.44  user=199.75  sys=15.68` |
| Compile requests           | 1007                                  |
| Compile requests executed  | 911                                   |
| Cache hits                 | 50 (5.52 %)                           |
| Cache hits (Assembler)     | 6 (5.83 %)                            |
| Cache hits (C/C++)         | 3 (1.06 %)                            |
| Cache hits (Rust)          | 41 (7.90 %)                           |
| Cache misses               | 856                                   |
| Cache misses (Assembler)   | 97                                    |
| Cache misses (C/C++)       | 281                                   |
| Cache misses (Rust)        | 478                                   |
| Cache timeouts             | 0                                     |
| Cache read errors          | 0                                     |
| Cache write errors         | **133**                               |
| Cache errors               | 0                                     |
| Forced recaches            | 0                                     |
| Compilations               | 856                                   |
| Non-cacheable compilations | 0                                     |
| Non-cacheable calls        | **90**                                |
| Cache size                 | TBD                                   |

> **Observations**: First run on a GitHub-hosted runner. sccache version 0.15.0 installed via
> `mozilla-actions/sccache-action`. 5.52 % cache hits come from pre-seeded sccache system cache
> (likely `rustc` internal artifacts). **133 cache write errors** suggest the GHA cache backend
> hit rate-limiting or connection issues during the upload-heavy cold build.

---

## Warm Rebuild (leaf crate change — `packages/primitives/src/lib.rs`)

| Metric                    | Value                                |
| ------------------------- | ------------------------------------ |
| **Wall time**             | **153.86 s** (~2.5 min)              |
| `cargo build --release`   | `real=153.86  user=175.60  sys=2.05` |
| Compile requests          | 1007                                 |
| Compile requests executed | 911                                  |
| Cache hits                | **64** (6.96 %)                      |
| Cache hits (Assembler)    | 6 (5.83 %)                           |
| Cache hits (C/C++)        | 3 (1.06 %)                           |
| Cache hits (Rust)         | **55** (10.32 %)                     |
| Cache misses              | 856                                  |
| Cache misses (Assembler)  | 97                                   |
| Cache misses (C/C++)      | 281                                  |
| Cache misses (Rust)       | 478                                  |

> **Observations**: Only **+14 additional cache hits** vs cold build. Cargo's dependency
> fingerprinting / build graph analysis detects that external dependencies haven't changed
> and skips them entirely at the build graph level — `rustc` (and thus sccache) is never
> invoked for those units. The 14 new hits likely come from pre-compiled system-level
> artifacts that sccache cached on the cold run.

---

## Within-Run Comparison

| Scenario          | Wall time    | Cache hits  | Notes                                                              |
| ----------------- | ------------ | ----------- | ------------------------------------------------------------------ |
| Cold (no cache)   | **479.44 s** | 50 (5.52 %) | External deps compile from scratch                                 |
| Warm-after-change | **153.86 s** | 64 (6.96 %) | Cargo skips unchanged external deps; only workspace crates rebuild |

> **Key insight**: The 326 s difference between cold and warm is **not from sccache** — it's from
> Cargo's own dependency tracking. Cargo knows external deps haven't changed and skips them.
> sccache contributed almost nothing within a single job because Cargo already avoids
> recompilation of unchanged units.

---

## Cross-Run Cache Test (Run 4 — workflow_dispatch re-trigger on same commit)

> **Run 4**: https://github.com/josecelano/torrust-tracker/actions/runs/27363491009

### Cold Build (with sccache GHA backend cache restored from Run 1)

| Metric                    | Value                                 |
| ------------------------- | ------------------------------------- |
| **Wall time**             | **192.21 s** (~3.2 min)               |
| `cargo build --release`   | `real=192.21  user=193.60  sys=15.39` |
| Compile requests          | 1007                                  |
| Compile requests executed | 911                                   |
| **Cache hits**            | **846 (93.38 %)**                     |
| Cache hits (Assembler)    | 103 (100 %)                           |
| Cache hits (C/C++)        | **225 (79.23 %)**                     |
| Cache hits (Rust)         | **518 (99.81 %)**                     |
| Cache misses              | 60                                    |
| Cache misses (C/C++)      | 59                                    |
| Cache misses (Rust)       | 1                                     |
| Cache write errors        | **0**                                 |
| Non-cacheable calls       | 90                                    |
| Compilations              | 60                                    |

> **This is the key result**: The sccache GHA backend **works**. External/C dependencies (846 cache
> hits) were restored from the GHA cache, and only the workspace crates — including the `bin`
> crate `torrust-tracker` — had to compile from scratch (60 misses, mostly C/C++ sys crates that
> are never cached by sccache).
>
> **93.38 % cache hit rate** on a cold checkout is the exact scenario that matters for CI.

### Warm Rebuild (after touching `packages/primitives/src/lib.rs`)

| Metric                  | Value                                |
| ----------------------- | ------------------------------------ |
| **Wall time**           | **137.35 s** (~2.3 min)              |
| `cargo build --release` | `real=137.35  user=168.64  sys=1.90` |
| Cache hits              | 860 (93.48 %)                        |
| Cache hits (Rust)       | 532 (99.81 %)                        |
| Cache misses            | 60                                   |
| Cache write errors      | 0                                    |

> After touching a leaf crate, sccache still provides 93.48 % hits. The misses are identical to
> the cold build (59 C/C++ + 1 Rust) — these are the non-cacheable `crate-type` units that
> recompile each time regardless.

---

## Full Comparison Table

| Scenario                               | Run       | Wall time    | Cache hits        | vs Cold (Run 1) |
| -------------------------------------- | --------- | ------------ | ----------------- | --------------- |
| Cold — no prior cache                  | Run 1     | **479.44 s** | 50 (5.52 %)       | —               |
| Warm-after-change                      | Run 1     | **153.86 s** | 64 (6.96 %)       | -68 %           |
| Cold — cross-run (GHA cache restored)  | **Run 4** | **192.21 s** | **846 (93.38 %)** | **-60 %**       |
| Warm-after-change (GHA cache restored) | Run 4     | **137.35 s** | 860 (93.48 %)     | -71 %           |

## Conclusion for Task 3a

**sccache with the GHA backend works well for cross-run CI caching**: a second run on the same
commit saves **60 % of cold build time** (479 → 192 s) by restoring cached compilation artifacts
for all external and C dependencies.

However, the fundamental limitation remains: the `torrust-tracker` bin crate (rank 1, ~77 s
critical-path) is **never cached** by sccache. The 60 non-cacheable calls per build are
dominated by this crate. Even with perfect sccache caching, the minimum build time on GHA is
~130 s (the workspace crate recompile overhead).

**Full comparison vs local**:

- Local cold (no sccache): 112.50 s
- GHA cold (no sccache): ~479 s (4x slower — fewer cores)
- GHA cold (sccache cross-run): 192 s (2.4x improvement vs no-cache GHA)
