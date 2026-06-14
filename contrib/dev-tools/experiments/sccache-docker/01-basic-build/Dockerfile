# syntax=docker/dockerfile:latest

# Test 1: sccache with BuildKit cache mount
# Purpose: Verify sccache works inside Docker with RUN --mount=type=cache
# and that the cache persists across builds.

FROM docker.io/library/rust:trixie AS experiment

# Install sccache
RUN cargo install sccache --locked

# Config
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache
ENV CARGO_INCREMENTAL=0
ENV CARGO_TERM_COLOR=always

WORKDIR /app

# Copy source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Cold build: sccache cache should be empty
RUN --mount=type=cache,target=/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release 2>&1 && \
    echo "=== COLD BUILD COMPLETE ===" && \
    sccache --show-stats

# Warm build (same layer, same RUN): sccache should have cached external deps
RUN --mount=type=cache,target=/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release 2>&1 && \
    echo "=== WARM BUILD COMPLETE ===" && \
    sccache --show-stats