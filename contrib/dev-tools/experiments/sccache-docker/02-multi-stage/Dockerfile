# Multi-stage Dockerfile to test sccache across build stages
#
# Simulates the real Containerfile structure:
#   recipe (generate recipe.json) → cook (compile deps) → build (compile project)
# Key question: are sccache cache mounts shared across stages?

FROM docker.io/library/rust:trixie AS base

RUN cargo install sccache --locked

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache
ENV CARGO_INCREMENTAL=0
ENV CARGO_TERM_COLOR=always
ENV SCCACHE_IDLE_TIMEOUT=0

WORKDIR /app

FROM base AS recipe
# Only manifests needed for recipe generation
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && touch src/lib.rs
# cargo chef prepare needs the binary
RUN cargo install cargo-chef --locked
RUN cargo chef prepare --recipe-path /app/recipe.json

FROM base AS cook
# Pre-build dependencies using the recipe
COPY --from=recipe /app/recipe.json /app/recipe.json
RUN --mount=type=cache,target=/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path /app/recipe.json 2>&1 && \
    echo "=== COOK COMPLETE ===" && \
    sccache --show-stats

FROM base AS build
# Full build with real source
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN --mount=type=cache,target=/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release 2>&1 && \
    echo "=== BUILD COMPLETE ===" && \
    sccache --show-stats