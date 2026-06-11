# Experiment 3: sccache with GHA Backend vs Local Disk

**Date**: 2026-06-11
**Goal**: Test sccache behavior with `SCCACHE_GHA_ENABLED=true` inside Docker when GHA
credentials are not available (local/Docker context).

## Key Finding

**`SCCACHE_GHA_ENABLED=true` with missing credentials FAILS HARD — no graceful fallback.**

```text
sccache: error: Server startup failed: create gha cache failed:
ConfigInvalid (permanent) at  => cache url for ghac not found,
maybe not in github action environment?
```

sccache v0.15.0 does NOT fall back to local disk. The compile fails with exit code 101.

## Corrected Approach

After removing the hardcoded `SCCACHE_GHA_ENABLED=true`:

- sccache uses **local disk** cache by default (via `SCCACHE_DIR=/sccache`)
- Cold build: **12.73 s**, 0 hits, 102 misses — same as Experiment 1
- Warm build (second RUN layer): **1.05 s**, 0 compile requests — Cargo skipped everything
- **Cache write errors: 0** — local disk cache works correctly

## Implications for Task 3b

The correct integration strategy for the Containerfile is:

1. **Containerfile must NOT hardcode `SCCACHE_GHA_ENABLED=true`** — it would break local builds
2. On GHA runners, the `mozilla-actions/sccache-action` sets env vars on the host
3. Pass GHA env vars into Docker build via `docker/build-push-action` with `--secret-env`
4. Containerfile reads secrets and exports the env var conditionally:

   ```dockerfile
   RUN --mount=type=secret,id=SCCACHE_GHA_ENABLED \
       export SCCACHE_GHA_ENABLED=$(cat /run/secrets/SCCACHE_GHA_ENABLED) && \
       cargo build --release
   ```

5. For local builds: no secrets passed → `SCCACHE_GHA_ENABLED` unset → local disk cache used

## Decision: Strategy for Task 3b

**Recommended: GHA backend passed into Docker via `docker/build-push-action` with `secret-env`**

The `docker/build-push-action@v7` supports passing environment variables as build secrets:

```yaml
- name: Build Tracker Image
  uses: docker/build-push-action@v7
  with:
    secret-env: |
      "SCCACHE_GHA_ENABLED=${{ env.SCCACHE_GHA_ENABLED }}"
      "ACTIONS_RUNTIME_TOKEN=${{ env.ACTIONS_RUNTIME_TOKEN }}"
      "ACTIONS_CACHE_URL=${{ env.ACTIONS_CACHE_URL }}"
```

The Containerfile mounts them:

```dockerfile
RUN --mount=type=secret,id=SCCACHE_GHA_ENABLED \
    --mount=type=secret,id=ACTIONS_RUNTIME_TOKEN \
    --mount=type=secret,id=ACTIONS_CACHE_URL \
    export SCCACHE_GHA_ENABLED=true && \
    cargo build --release
```

This approach:

- Works on GHA (secrets are available)
- Falls back to local disk on local builds (secrets not passed)
- No infrastructure changes needed
- Uses the proven GHA backend (93.38 % hit rate from Task 3a)
