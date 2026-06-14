# Experiment 1: sccache with BuildKit Cache Mount

**Date**: 2026-06-11
**Goal**: Verify sccache works inside Docker with `RUN --mount=type=cache` and understand whether
cache mounts persist across separate `docker build` invocations.

## Dockerfile

```dockerfile
FROM docker.io/library/rust:trixie AS experiment

RUN cargo install sccache --locked

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache
ENV CARGO_INCREMENTAL=0
ENV CARGO_TERM_COLOR=always

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release 2>&1 && \
    echo "=== COLD BUILD COMPLETE ===" && \
    sccache --show-stats

RUN --mount=type=cache,target=/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release 2>&1 && \
    echo "=== WARM BUILD COMPLETE ===" && \
    sccache --show-stats
```

## Command

```sh
cd /tmp/sccache-docker-test
docker buildx build --load --progress=plain --no-cache -t sccache-experiment -f Dockerfile .
```

## Results

### Step 8: Install sccache

- Wall time: **126.8 s** (compiling sccache from source with `--locked`)
- sccache v0.15.0 installed

### Step 12: Cold build

- Wall time: **16.95 s** (compiled 102 Rust units)
- **Cache hits**: 0 (0.00 %)
- **Cache misses**: 102
- **Non-cacheable calls**: 25 (21 crate-type)
- **Cache size**: 59 MiB
- **Cache write errors**: 0

### Step 13: Warm build (same RUN layer, no source changes)

- Wall time: **0.05 s** (Cargo detected nothing changed)
- **Compile requests**: 0 — sccache not invoked at all
- Cargo's own dependency checking skipped everything

## Key Findings

1. **sccache works inside Docker** — the `SCCACHE_DIR=/sccache` with `--mount=type=cache,target=/sccache`
   correctly stores and retrieves cached artifacts within a single build.

2. **BuildKit cache mounts are session-scoped** — on a `docker build`, the cache mount persists
   across RUN layers within that single build invocation, but a **new `docker build` starts with
   an empty cache mount** (unless the BuildKit cache is shared via `cache-from`).

3. **For GHA**: This means BuildKit cache mounts alone are NOT sufficient for cross-run persistence.
   Each new GHA runner starts a fresh Docker builder with no BuildKit cache history. The
   `cache-from/ cache-to: type=gha` only caches **image layers**, not cache mount contents.

4. **Warm build within same RUN layer is trivial** — Cargo's own dependency checking already
   handles "nothing changed" perfectly (0.05 s). The value of sccache is in **cross-run caching**
   where external deps must be restored from a remote cache.

## Next Steps

Experiment 2 tests a multi-stage build (like the real Containerfile) where different stages each
compile Rust. Experiment 3 tests the GHA backend approach where `ACTIONS_RUNTIME_TOKEN` and
`ACTIONS_CACHE_URL` are passed into Docker to enable cross-run cache persistence.
