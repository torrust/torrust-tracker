# syntax=docker/dockerfile:latest

# Torrust Tracker

## Builder Image
FROM docker.io/library/rust:slim-trixie AS chef
WORKDIR /tmp
RUN apt-get update \
 && apt-get install -y --no-install-recommends curl libssl-dev pkg-config \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm --locked torrust-cargo-chef@0.1.78 cargo-nextest
# Note: We use the `torrust-cargo-chef` fork (v0.1.78) while upstream PR
# https://github.com/LukeMathWalker/cargo-chef/pull/360 is pending. Once merged,
# switch back to upstream `cargo-chef` and remove this comment.

## Tester Image
FROM docker.io/library/rust:slim-trixie AS tester
WORKDIR /tmp

RUN apt-get update \
 && apt-get install -y --no-install-recommends curl sqlite3 time \
 && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
 && cargo binstall --no-confirm --locked cargo-nextest \
 && apt-get purge -y --auto-remove curl \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
# Database initialization: Tests at runtime require a pre-initialized SQLite3 database
# to test against a valid (not corrupted) schema. The VACUUM command optimizes the
# database file layout. This image layer is inherited by test_debug and test stages.

COPY ./share/ /app/share/torrust
RUN time mkdir -p /app/share/torrust/default/database/ \
 && time sqlite3 /app/share/torrust/default/database/tracker.sqlite3.db "VACUUM;"

## Su Exe Compile
FROM docker.io/library/debian:trixie-slim AS gcc
RUN apt-get update \
 && apt-get install -y --no-install-recommends gcc libc6-dev \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
COPY ./contrib/dev-tools/su-exec/ /usr/local/src/su-exec/
RUN cc -Wall -Werror -g /usr/local/src/su-exec/su-exec.c -o /usr/local/bin/su-exec \
 && chmod +x /usr/local/bin/su-exec


## Chef Prepare (look at project and see wat we need)
FROM chef AS recipe
WORKDIR /build/src
# Manifest-only copy: `cargo chef prepare` only needs Cargo.toml manifests and Cargo.lock
# to build recipe.json — it does not read any .rs source files.
# Copying the full source tree here would cause Docker to invalidate this layer (and
# therefore the expensive `cargo chef cook` dependency layers) on every source-code change.
# By copying only manifests, the cook layers stay cached for source-only edits.
#
# MAINTENANCE: Keep this list in sync with all in-repo path crates (packages/, console/,
# contrib/). This includes the root crate itself plus every crate reachable as a path
# dependency from the root — i.e. all packages discovered by `cargo metadata --no-deps`
# whose manifest path is inside this repository. Note: the `[workspace].members` key in
# the root Cargo.toml only lists packages not auto-discovered via path dependencies; it
# is a much smaller set and should NOT be used as the authoritative list here.
# Every new in-repo path crate must have a corresponding COPY line added; every removed
# or moved crate must have its line updated or removed accordingly.
COPY Cargo.toml Cargo.lock ./
COPY console/tracker-client/Cargo.toml console/tracker-client/
# The following packages are excluded from cargo nextest archive (see Cook and
# Build stages below) because they are not part of the production tracker service
# and do not need to be tested inside the container image:
#   - workspace-coupling (analysis/coupling tool, no production value)
#   - torrust-tracker-torrent-repository-benchmarking (benchmarking only)
#   - torrust-tracker-client (CLI dev tools: tracker_client, tracker_checker, etc.)
#   - torrust-tracker-e2e-tools (E2E runners + profiling tool, GHA host-only)
#   - torrust-tracker-persistence-benchmark (persistence layer dev benchmarking tool)
# Their Cargo.toml manifests and stub source files must still be present here
# because `cargo chef prepare` uses `cargo metadata` internally to enumerate
# all workspace members, and `cargo metadata` aborts if any member's manifest
# or declared target file is missing. `cargo chef prepare` has no `--exclude`
# flag (only `--bin`), so these stubs cannot be omitted from the recipe stage.
COPY contrib/dev-tools/analysis/workspace-coupling/Cargo.toml contrib/dev-tools/analysis/workspace-coupling/
COPY packages/e2e-tools/Cargo.toml packages/e2e-tools/
COPY packages/persistence-benchmark/Cargo.toml packages/persistence-benchmark/
COPY packages/axum-health-check-api-server/Cargo.toml packages/axum-health-check-api-server/
COPY packages/axum-http-server/Cargo.toml packages/axum-http-server/
COPY packages/axum-rest-api-server/Cargo.toml packages/axum-rest-api-server/
COPY packages/axum-server/Cargo.toml packages/axum-server/
COPY packages/configuration/Cargo.toml packages/configuration/
COPY packages/events/Cargo.toml packages/events/
COPY packages/http-protocol/Cargo.toml packages/http-protocol/
COPY packages/http-core/Cargo.toml packages/http-core/
COPY packages/primitives/Cargo.toml packages/primitives/
COPY packages/rest-api-client/Cargo.toml packages/rest-api-client/
COPY packages/rest-api-application/Cargo.toml packages/rest-api-application/
COPY packages/rest-api-protocol/Cargo.toml packages/rest-api-protocol/
COPY packages/rest-api-runtime-adapter/Cargo.toml packages/rest-api-runtime-adapter/
COPY packages/swarm-coordination-registry/Cargo.toml packages/swarm-coordination-registry/
COPY packages/test-helpers/Cargo.toml packages/test-helpers/
COPY packages/torrent-repository-benchmarking/Cargo.toml packages/torrent-repository-benchmarking/
COPY packages/tracker-client/Cargo.toml packages/tracker-client/
COPY packages/tracker-core/Cargo.toml packages/tracker-core/
COPY packages/udp-protocol/Cargo.toml packages/udp-protocol/
COPY packages/udp-server/Cargo.toml packages/udp-server/
COPY packages/udp-core/Cargo.toml packages/udp-core/
# Create stub source files for every in-repo target.
# `cargo chef prepare` runs `cargo metadata` internally, which requires every
# package to have at least one resolvable target file on disk — whether the
# target is explicitly declared in Cargo.toml (e.g. [lib], [[bin]], [[bench]])
# or auto-detected by Cargo (e.g. src/lib.rs, src/main.rs, src/bin/*.rs).
# Packages with no source files at all cause `cargo metadata` to abort with
# "no targets specified in the manifest". Examples and tests also need stubs
# when auto-detected, because Cargo validates them during manifest loading.
#
# The canonical list below was derived from:
#   cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[].src_path'
# filtered to paths inside this repository. Re-run that command whenever a
# new package, binary, example, or bench target is added to the workspace and
# add the corresponding mkdir / touch lines here.
#
# MAINTENANCE: When adding a new in-repo crate or target, add the corresponding
# stub lines below AND the Cargo.toml COPY line in the manifest-only block above.
RUN mkdir -p \
      src/bin \
      packages/e2e-tools/src/bin \
      packages/persistence-benchmark/src/bin \
      contrib/dev-tools/analysis/workspace-coupling/src \
      console/tracker-client/src/bin \
      packages/axum-health-check-api-server/src \
      packages/axum-http-server/src \
      packages/axum-http-server/examples \
      packages/axum-rest-api-server/src \
      packages/axum-server/src \
      packages/configuration/src \
      packages/events/src \
      packages/http-protocol/src \
      packages/http-core/src \
      packages/http-core/benches \
      packages/primitives/src \
      packages/rest-api-client/src \
      packages/rest-api-application/src \
      packages/rest-api-protocol/src \
      packages/rest-api-runtime-adapter/src \
      packages/swarm-coordination-registry/src \
      packages/test-helpers/src \
      packages/torrent-repository-benchmarking/src \
      packages/torrent-repository-benchmarking/benches \
      packages/tracker-client/src \
      packages/tracker-core/src \
      packages/udp-protocol/src \
      packages/udp-server/src \
      packages/udp-server/examples \
      packages/udp-core/src \
      packages/udp-core/benches \
 && touch \
      src/lib.rs \
      src/main.rs \
      src/bin/http_health_check.rs \
      packages/e2e-tools/src/bin/e2e_tests_runner.rs \
      packages/e2e-tools/src/bin/profiling.rs \
      packages/e2e-tools/src/bin/qbittorrent_e2e_runner.rs \
      packages/persistence-benchmark/src/bin/persistence_benchmark_runner.rs \
      contrib/dev-tools/analysis/workspace-coupling/src/main.rs \
      console/tracker-client/src/lib.rs \
      console/tracker-client/src/bin/http_tracker_client.rs \
      console/tracker-client/src/bin/tracker_checker.rs \
      console/tracker-client/src/bin/tracker_client.rs \
      console/tracker-client/src/bin/udp_tracker_client.rs \
      packages/axum-health-check-api-server/src/lib.rs \
      packages/axum-http-server/src/lib.rs \
      packages/axum-http-server/examples/http_only_public_tracker.rs \
      packages/axum-rest-api-server/src/lib.rs \
      packages/axum-server/src/lib.rs \
      packages/configuration/src/lib.rs \
      packages/events/src/lib.rs \
      packages/http-protocol/src/lib.rs \
      packages/http-core/src/lib.rs \
      packages/http-core/benches/http_tracker_core_benchmark.rs \
      packages/primitives/src/lib.rs \
      packages/rest-api-client/src/lib.rs \
      packages/rest-api-application/src/lib.rs \
      packages/rest-api-protocol/src/lib.rs \
      packages/rest-api-runtime-adapter/src/lib.rs \

      packages/swarm-coordination-registry/src/lib.rs \
      packages/test-helpers/src/lib.rs \
      packages/torrent-repository-benchmarking/src/lib.rs \
      packages/torrent-repository-benchmarking/benches/repository_benchmark.rs \
      packages/tracker-client/src/lib.rs \
      packages/tracker-core/src/lib.rs \
      packages/udp-protocol/src/lib.rs \
      packages/udp-server/src/lib.rs \
      packages/udp-server/examples/udp_only_public_tracker.rs \
      packages/udp-core/src/lib.rs \
      packages/udp-core/benches/udp_tracker_core_benchmark.rs
RUN cargo chef prepare --recipe-path /build/recipe.json
# Generate an external-only recipe for the third-party dependency layer.
# The `--external-only` flag strips all `path = "..."` dependency entries,
# producing a stable recipe that is immune to workspace-internal Cargo.toml
# changes (e.g., reorganising workspace members, renaming packages). The recipe
# still changes when external dependency metadata changes — for example, adding
# or removing a crate, updating a version, or toggling feature flags on external
# dependencies — regardless of whether Cargo.lock is modified.
# This is from the `torrust-cargo-chef` fork (see chef stage above).
RUN cargo chef prepare --external-only --recipe-path /build/recipe-thirdparty.json


## Cook Third-party (debug)
FROM chef AS dependencies_thirdparty_debug
WORKDIR /build/src
# Only third-party recipe: immune to workspace Cargo.toml changes.
COPY --from=recipe /build/recipe-thirdparty.json /build/recipe.json
RUN cargo chef cook --tests --workspace --all-features --recipe-path /build/recipe.json

## Cook (debug)
FROM dependencies_thirdparty_debug AS dependencies_debug
WORKDIR /build/src
# Full recipe on top — reuses third-party artifacts from parent layer.
COPY --from=recipe /build/recipe.json /build/recipe.json
# Note: `cargo chef cook` does not support `--exclude` (the cargo-chef CLI only
# exposes `--workspace` and `--package`, not `--exclude`). The excluded workspace
# members (workspace-coupling, torrust-tracker-torrent-repository-benchmarking,
# torrust-tracker-client, torrust-tracker-contrib-bencode,
# torrust-tracker-e2e-tools, torrust-tracker-persistence-benchmark) are therefore
# still compiled as part of the cook skeleton (their Cargo.toml manifests are in
# the recipe, so cargo-chef cooks them). The build-time savings come from the
# archive/build stages: `cargo nextest archive` below is passed `--exclude` so
# those packages are not compiled from real source in the final archive. See Cook
# (release) and Build stages.
RUN cargo chef cook --tests --workspace --all-features --recipe-path /build/recipe.json
# Pre-link warm-up: Create and discard a nextest archive to warm up the linker
# before final compilation. This improves incremental build cache efficiency
# by pre-faulting the linker phases, avoiding redundant linking work in later stages.
RUN cargo nextest archive --tests --workspace --all-features \
    --exclude workspace-coupling \
    --exclude torrust-tracker-torrent-repository-benchmarking \
    --exclude torrust-tracker-client \
    --exclude torrust-tracker-contrib-bencode \
    --exclude torrust-tracker-e2e-tools \
    --exclude torrust-tracker-persistence-benchmark \
    --archive-file /build/temp.tar.zst && rm -f /build/temp.tar.zst

## Cook Third-party (release)
FROM chef AS dependencies_thirdparty
WORKDIR /build/src
# Only third-party recipe: immune to workspace Cargo.toml changes.
COPY --from=recipe /build/recipe-thirdparty.json /build/recipe.json
RUN cargo chef cook --tests --workspace --all-features --recipe-path /build/recipe.json --release

## Cook (release)
FROM dependencies_thirdparty AS dependencies
WORKDIR /build/src
# Full recipe on top — reuses third-party artifacts from parent layer.
COPY --from=recipe /build/recipe.json /build/recipe.json
# Note: `cargo chef cook` does not support `--exclude` — see Cook (debug) above.
RUN cargo chef cook --tests --workspace --all-features --recipe-path /build/recipe.json --release
# Pre-link warm-up: Create and discard a nextest archive to warm up the linker
# before final compilation. This improves incremental build cache efficiency
# by pre-faulting the linker phases, avoiding redundant linking work in later stages.
RUN cargo nextest archive --tests --workspace --all-features \
    --exclude workspace-coupling \
    --exclude torrust-tracker-torrent-repository-benchmarking \
    --exclude torrust-tracker-client \
    --exclude torrust-tracker-contrib-bencode \
    --exclude torrust-tracker-e2e-tools \
    --exclude torrust-tracker-persistence-benchmark \
    --archive-file /build/temp.tar.zst --release && rm -f /build/temp.tar.zst


## Build Archive (debug)
FROM dependencies_debug AS build_debug
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --workspace --all-features \
    --exclude workspace-coupling \
    --exclude torrust-tracker-torrent-repository-benchmarking \
    --exclude torrust-tracker-client \
    --exclude torrust-tracker-contrib-bencode \
    --exclude torrust-tracker-e2e-tools \
    --exclude torrust-tracker-persistence-benchmark \
    --archive-file /build/torrust-tracker-debug.tar.zst

## Build Archive (release)
FROM dependencies AS build
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --workspace --all-features \
    --exclude workspace-coupling \
    --exclude torrust-tracker-torrent-repository-benchmarking \
    --exclude torrust-tracker-client \
    --exclude torrust-tracker-contrib-bencode \
    --exclude torrust-tracker-e2e-tools \
    --exclude torrust-tracker-persistence-benchmark \
    --archive-file /build/torrust-tracker.tar.zst --release


# Extract and Test (debug)
FROM tester AS test_debug
WORKDIR /test
COPY . /test/src/
COPY --from=build_debug \
  /build/torrust-tracker-debug.tar.zst \
  /test/torrust-tracker-debug.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-tracker-debug.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

RUN time mkdir -p /app/bin/ \
 && time cp -l /test/src/target/debug/torrust-tracker /app/bin/torrust-tracker
RUN time mkdir /app/lib/ \
 && time cp -l $(realpath $(ldd /app/bin/torrust-tracker | grep "libz\.so\.1" | awk '{print $3}')) /app/lib/libz.so.1
RUN time chown -R root:root /app \
 && time chmod -R u=rw,go=r,a+X /app \
 && time chmod -R a+x /app/bin

# Extract and Test (release)
FROM tester AS test
WORKDIR /test
COPY . /test/src
COPY --from=build \
  /build/torrust-tracker.tar.zst \
  /test/torrust-tracker.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-tracker.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

RUN time mkdir -p /app/bin/ \
 && time cp -l /test/src/target/release/torrust-tracker /app/bin/torrust-tracker \
 && time cp -l /test/src/target/release/http_health_check /app/bin/http_health_check
RUN time mkdir -p /app/lib/ \
 && time cp -l $(realpath $(ldd /app/bin/torrust-tracker | grep "libz\.so\.1" | awk '{print $3}')) /app/lib/libz.so.1
RUN time chown -R root:root /app \
 && time chmod -R u=rw,go=r,a+X /app \
 && time chmod -R a+x /app/bin


## Runtime
FROM gcr.io/distroless/cc-debian13:debug AS runtime
RUN ["/busybox/cp", "-sp", "/busybox/sh","/busybox/cat","/busybox/ls","/busybox/env", "/bin/"]
COPY --from=gcc --chmod=0555 /usr/local/bin/su-exec /bin/su-exec

ARG TORRUST_TRACKER_CONFIG_TOML_PATH="/etc/torrust/tracker/tracker.toml"
ARG TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER="sqlite3"
ARG USER_ID=1000
ARG UDP_PORT=6969
ARG HTTP_PORT=7070
ARG API_PORT=1212
ARG HEALTH_CHECK_API_PORT=1313

ENV TORRUST_TRACKER_CONFIG_TOML_PATH=${TORRUST_TRACKER_CONFIG_TOML_PATH}
ENV TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER=${TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER}
ENV USER_ID=${USER_ID}
ENV UDP_PORT=${UDP_PORT}
ENV HTTP_PORT=${HTTP_PORT}
ENV API_PORT=${API_PORT}
ENV HEALTH_CHECK_API_PORT=${HEALTH_CHECK_API_PORT}
ENV TZ=Etc/UTC

EXPOSE ${UDP_PORT}/udp
EXPOSE ${HTTP_PORT}/tcp
EXPOSE ${API_PORT}/tcp
EXPOSE ${HEALTH_CHECK_API_PORT}/tcp

RUN mkdir -p /var/lib/torrust/tracker /var/log/torrust/tracker /etc/torrust/tracker

ENV ENV=/etc/profile
COPY --chmod=0555 ./share/container/entry_script_sh /usr/local/bin/entry.sh

VOLUME ["/var/lib/torrust/tracker","/var/log/torrust/tracker","/etc/torrust/tracker"]

ENV RUNTIME="runtime"
ENTRYPOINT ["/usr/local/bin/entry.sh"]


## Torrust-Tracker (debug)
FROM runtime AS debug
ENV RUNTIME="debug"
COPY --from=test_debug /app/ /usr/
RUN env
CMD ["sh"]

## Torrust-Tracker (release) (default)
FROM runtime AS release
ENV RUNTIME="release"
COPY --from=test /app/ /usr/
HEALTHCHECK --interval=5s --timeout=5s --start-period=3s --retries=3 \  
  CMD /usr/bin/http_health_check http://localhost:${HEALTH_CHECK_API_PORT}/health_check \
    || exit 1
CMD ["/usr/bin/torrust-tracker"]
