# Experiment 2: Multi-Stage Build with sccache

**Date**: 2026-06-11
**Goal**: Test whether sccache with `--mount=type=cache` shares artifacts across Docker
multi-stage build stages — mirroring the real Containerfile structure.

## Dockerfile Structure

```text
base (install sccache)
  ├── recipe (generate recipe.json from manifests)
  ├── cook (cargo chef cook --release using recipe)
  └── build (cargo build --release with real source)
```text

## Command

```sh
cd 02-multi-stage
docker buildx build --load --progress=plain --no-cache -t sccache-multistage -f Dockerfile .
```text

## Results

Only the `build` stage executed (default final stage in Docker multi-stage builds).
Key observations:

### Step 6 (base stage): Install sccache

- Wall time: **130.7 s** (same as Experiment 1)

### Step 10 (build stage): Compile and run

- Wall time: **14.28 s**
- **Cache hits**: 0 (0.00 %)
- **Cache misses**: 102
- **Cache size**: 59 MiB
- **Non-cacheable calls**: 25 (21 crate-type)
- **Cache write errors**: 0

## Key Findings

1. **BuildKit cache mounts are stage-scoped**: Each `FROM` stage gets its own cache mount
   namespace. The `/sccache` mount in the `cook` stage is NOT visible to the `build` stage.
   This means `cargo chef cook` (third-party deps) and `cargo build` (workspace crates) cannot
   share sccache artifacts across stages using cache mounts alone.

2. **The real Containerfile has a different inheritance structure**: The production Containerfile
   uses `FROM dependencies_thirdparty AS dependencies` — stages inherit compiled artifacts
   via filesystem inheritance (Docker layers), NOT via cache mounts. `cargo chef` handles
   the dependency caching at the filesystem level. sccache would be complementary.

3. **For the real Containerfile**: sccache would need to work alongside `cargo-chef`, not replace
   it. The `cargo chef cook --release` stages would benefit from sccache for cross-run external
   dependency caching (on GHA), while `cargo-chef` handles within-build layer caching.

## Conclusion for Task 3b

The cache-mount-only approach will not work across multi-stage builds. Two alternatives remain:

1. **GHA backend inside Docker (B2)**: Pass `ACTIONS_RUNTIME_TOKEN` and `ACTIONS_CACHE_URL`
   into Docker via `--secret`. sccache directly reads/writes to the GHA cache API — no mount
   sharing needed. This is tested in Experiment 3.

2. **Install sccache in every stage that compiles Rust**: Each stage runs its own sccache
   daemon pointing to the GHA backend. This works but requires modifying each compiler stage
   in the Containerfile.
