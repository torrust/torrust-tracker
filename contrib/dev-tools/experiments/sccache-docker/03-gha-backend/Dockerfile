# Test: sccache with GHA backend config inside Docker
#
# CRITICAL FINDING: When SCCACHE_GHA_ENABLED=true but GHA credentials are
# not available (ACTIONS_RUNTIME_TOKEN, ACTIONS_CACHE_URL), sccache FAILS HARD
# with: "cache url for ghac not found, maybe not in github action environment?"
#
# This means:
# 1. We CANNOT hardcode SCCACHE_GHA_ENABLED=true in the Containerfile
# 2. On GHA runners, the env vars are automatically set by the runner itself
# 3. We must pass them into Docker via --secret (docker/build-push-action)
# 4. For local builds, sccache should use local disk (no GHA_ENABLED set)
#
# This experiment verifies the correct approach: only set SCCACHE_GHA_ENABLED=true
# when the GHA credentials are available via --secret mounts. Otherwise use default
# local disk cache.

FROM docker.io/library/rust:trixie AS experiment

RUN cargo install sccache --locked

# NOTE: SCCACHE_GHA_ENABLED is NOT set here. It will be set via --secret on GHA.
# When unset, sccache uses local disk cache by default (works everywhere).
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache
ENV CARGO_INCREMENTAL=0
ENV CARGO_TERM_COLOR=always

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Cold build: sccache uses local disk (no GHA creds available)
RUN --mount=type=cache,target=/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release 2>&1 && \
    echo "=== COLD BUILD COMPLETE ===" && \
    sccache --show-stats

# Warm build: verify sccache reused local disk cache
RUN --mount=type=secret,id=ACTIONS_RUNTIME_TOKEN \
    --mount=type=secret,id=ACTIONS_CACHE_URL \
    cargo build --release 2>&1 && \
    echo "=== WARM BUILD COMPLETE ===" && \
    sccache --show-stats