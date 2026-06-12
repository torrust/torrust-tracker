# Experiment 3b: sccache inside Docker — GHA Results

> **Workflow**: `experiment-sccache-docker.yaml`
> **Run 3 (cold)**: https://github.com/josecelano/torrust-tracker/actions/runs/27401589341
> **Commit**: `be0627f9` — `fix(ci): map ACTIONS_RESULTS_URL to ACTIONS_CACHE_URL for sccache ghac`
> **Runner**: `ubuntu-latest` (GitHub-hosted)

---

## Run 3 — Cold Docker Build (first push, no prior cache)

**Total workflow duration**: **29 min 28 s** (07:32:37 → 08:02:05 UTC)

### Docker build stage breakdown

| Stage                                                                 | GHA time        | Local time (Ryzen 9) | Slowdown                      |
| --------------------------------------------------------------------- | --------------- | -------------------- | ----------------------------- |
| Chef (sccache install from source)                                    | ~45 s           | ~127 s               | Faster on GHA! (newer runner) |
| `dependencies_thirdparty` (external deps `cargo chef cook --release`) | **3 min 52 s**  | 52.75 s              | ~4.4x                         |
| `dependencies` (workspace cook `cargo chef cook --release`)           | **2 min 40 s**  | 31.19 s              | ~5.1x                         |
| Dependencies pre-link warmup (`cargo nextest archive`)                | ~37 s           | 4.80 s               | ~7.7x                         |
| **Build** (`cargo nextest archive --release` with real source)        | **14 min 24 s** | ~162 s               | ~5.3x                         |
| Unit tests inside container (`cargo nextest run`)                     | ~6 s            | ~2 s                 | ~3x                           |
| **Total Docker build**                                                | **~22 min**     | ~5 min               | ~4.4x                         |

### GHA credential passing

- `SCCACHE_GHA_ENABLED=true` passed → works ✅
- `ACTIONS_RUNTIME_TOKEN` passed (redacted in logs) → works ✅
- `ACTIONS_CACHE_URL` mapped from `${{ env.ACTIONS_RESULTS_URL }}` → works ✅
- sccache daemon inside Docker started successfully (no "ghac not found" error) ✅

### sccache stats

- **Host daemon**: 0 hits, 0 misses (expected — no compilation happened on the host)
- **Inside Docker stats**: Not captured in logs (sccache --show-stats not called inside Containerfile)

The `RUSTC_WRAPPER=sccache` was active during all `cargo chef cook` and `cargo nextest archive`
steps inside Docker. On a cold run, all 856+ units are cache misses (same as Task 3a cold).

### Key finding

The **third-party dependencies layer** (`dependencies_thirdparty`) took **3 min 52 s** on GHA.
This is the layer most likely to benefit from sccache caching, because:

- It changes only when `Cargo.lock` changes
- It's the layer that would need recompilation even with BuildKit layer cache invalidated

The **Build stage** (14 min 24 s) is dominated by the `torrust-tracker` bin crate which sccache
can never cache — same limitation as all previous experiments.

---

## Run 4 (warm re-trigger) — to be measured

When re-triggered (same commit), the BuildKit `cache-from: type=gha` should hit for all layers
where `Cargo.lock` hasn't changed. On a warm run, the `dependencies_thirdparty` step should
show `CACHED` in the build output (0 s rebuild).

If `Cargo.lock` were to change, the BuildKit layer cache would miss, and sccache would be tested:

- External deps: ~232 Rust units sccache could cache → would save ~3 min 52 s on a cross-run
- Workspace crates: ~160 Rust units → mostly bin (never cached) + tight coupling = minimal benefit

**Verdict**: sccache inside Docker adds value only in the narrow scenario where `Cargo.lock`
changes but individual crate sources haven't (saving ~4 min of third-party dep compilation on GHA).
The BuildKit layer cache already handles the common case (`Cargo.lock` unchanged) perfectly.
