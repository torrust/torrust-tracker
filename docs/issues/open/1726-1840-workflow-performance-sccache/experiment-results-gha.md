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

> **Observations**: Only **+14 additional cache hits** vs cold build. This is because Cargo's
> incremental compilation (even with `CARGO_INCREMENTAL=0`) detects that external dependencies
> haven't changed and skips them entirely — sccache doesn't even get invoked for those units.
> The 14 new hits likely come from pre-compiled system-level artifacts that sccache cached on
> the cold run.

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

## Cross-Run Test (Run 2 — to be measured)

After Run 1 finishes, the `mozilla-actions/sccache-action` post-job step uploads the sccache
cache to the GHA backend. **Re-trigger this workflow** (same commit) to test whether the GHA
backend restore on the next run provides any benefit.

| Expected                       | Value                                                                                       |
| ------------------------------ | ------------------------------------------------------------------------------------------- |
| Cold build after cache restore | TBD (should show cache hits for external deps)                                              |
| Expected improvement           | If GHA restore works well, external dep compiles should be cached → ~154 s instead of 479 s |

- [ ] Re-trigger workflow (same commit) via `workflow_dispatch`
- [ ] Record cold build wall time and sccache stats for Run 2
- [ ] Compare with Run 1 cold build

---

## Run 2 (Cross-Run Cache)

| Metric                  | Value   |
| ----------------------- | ------- |
| **Wall time**           | **TBD** |
| `cargo build --release` | TBD     |
| Cache hits              | TBD     |
| Cache misses            | TBD     |
