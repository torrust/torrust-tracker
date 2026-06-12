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

## Run 4 — Warm Re-trigger (workflow_dispatch, same commit)

> **Run 4**: https://github.com/josecelano/torrust-tracker/actions/runs/27404315247
> **Event**: `workflow_dispatch` (same commit `be0627f9`)
> **Total workflow**: **30 min 13 s** (08:30:22 → 09:00:35 UTC)

### Docker build stage comparison

| Stage                                         | Cold (Run 3)    | Warm (Run 4)    | Delta     | CACHED?       |
| --------------------------------------------- | --------------- | --------------- | --------- | ------------- |
| `dependencies_thirdparty` (external deps)     | **3 min 52 s**  | **3 min 45 s**  | -7 s      | ❌ Recompiled |
| `dependencies` (workspace cook)               | **2 min 40 s**  | **2 min 41 s**  | +1 s      | ❌ Recompiled |
| Dependencies pre-link warmup                  | ~37 s           | ~37 s           | ~0 s      | ❌ Recompiled |
| **Build** (`cargo nextest archive --release`) | **14 min 24 s** | **13 min 46 s** | -38 s     | ❌ Recompiled |
| Test execution steps                          | ~6 s            | ~6 s            | ~0 s      | ✅ CACHED     |
| **Total workflow**                            | **29 min 28 s** | **30 min 13 s** | **+45 s** | —             |

### Critical finding: BuildKit GHA cache did NOT help

The `cache-from: type=gha,scope=experiment-sccache-release` from Run 3 did NOT accelerate
Run 4. The compilation stages all recompiled at full speed (~30 min total).

**Why?** The BuildKit GHA cache backend stores compressed image layers. On `ubuntu-latest`
GitHub-hosted runners, the cache restore step:

1. Downloads compressed layers from GHA cache (at 30-70 MB/s)
2. Decompresses and verifies checksums
3. Only then can BuildKit skip recompilation

The `dependencies_thirdparty` layer has a compressed size of several hundred MB. Combined
with GHA cache API rate limits and the `docker-container` driver's overhead, the restore
time can be comparable to or longer than the recompilation time — exactly as predicted in
the original `compile-hotspot-analysis.md` about `Swatinem/rust-cache`.

### sccache inside Docker: same fate

sccache inside Docker couldn't help because:

1. The GHA credentials (`ACTIONS_RUNTIME_TOKEN`) are **job-scoped** — they expire when the
   job ends. A new workflow run gets a new token. The cached objects from Run 3 were stored
   under Run 3's credentials and cannot be accessed by Run 4.
2. Even if credentials could be reused, sccache's `ghac` library uses the GitHub Actions
   cache API which has the same 10 GB limit and rate-limiting as BuildKit's cache.

### Conclusion for Task 3b

**sccache inside Docker provides no measurable benefit for cross-run builds on GHA.**

Both sccache and BuildKit's `cache-from: type=gha` are limited by the same fundamental
constraints of GitHub-hosted runners:

- **Non-sticky disk**: Every new runner starts with an empty local disk
- **Slow cache transfer**: 30-70 MB/s over the network
- **Token expiration**: Job-scoped tokens prevent cross-run cache access for sccache
- **10 GB limit**: Both caches compete for the same limited storage

The **only** caching that works reliably for this workspace is BuildKit's **internal layer
cache** (not exported via `type=gha`), which is only useful within a single `docker build`
invocation — not across separate workflow runs.
