# syntax=docker/dockerfile:latest
#
# semantic-links:
#   skill-links:
#     - add-workspace-member
#   related-artifacts:
#     - Cargo.toml  # `[workspace] default-members` is the positive list of packages tested in the image
#     - .dockerignore  # allow-list of the build context; every workspace member must be admitted
#     - .hadolint.yaml  # hadolint global linting rules and ignore policies with rationale

# Torrust Tracker

## Builder Image
FROM docker.io/library/rust:slim-trixie AS chef
WORKDIR /tmp
RUN apt-get update \
 && apt-get install -y --no-install-recommends curl libssl-dev pkg-config \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm --locked torrust-cargo-chef@0.1.78 cargo-nextest@0.9.140
# Note: We use the `torrust-cargo-chef` fork (v0.1.78) while upstream PR
# https://github.com/LukeMathWalker/cargo-chef/pull/360 is pending. Once merged,
# switch back to upstream `cargo-chef` and remove this comment.

## Tester Image
FROM docker.io/library/rust:slim-trixie AS tester
WORKDIR /tmp

RUN apt-get update \
 && apt-get install -y --no-install-recommends curl sqlite3 time \
 && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
 && cargo binstall --no-confirm --locked cargo-nextest@0.9.140 \
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
# Canonical cargo-chef planner stage: copy the whole (allow-listed, see .dockerignore)
# build context and let `cargo chef prepare` enumerate packages and targets itself.
# Cargo is the only source of truth for workspace packages; nothing is listed here.
#
# This layer is rebuilt on every source change, but `cargo chef prepare` costs well
# under a second and the downstream `COPY --from=recipe` steps are keyed on the
# content checksum of the recipe files, so identical recipes keep the cook layers
# cached (measured in issue #2298, scenarios M5 and M6).
COPY . /build/src
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
# `cargo chef cook` keeps `--workspace` so the skeleton covers every member the
# recipe enumerates. `cargo nextest archive` deliberately omits `--workspace`: it
# then builds only the root `[workspace] default-members` (the positive list of
# packages tested inside the image), so no per-package exclusions are needed here.
RUN cargo chef cook --tests --workspace --all-features --recipe-path /build/recipe.json
# Pre-link warm-up: Create and discard a nextest archive to warm up the linker
# before final compilation. This improves incremental build cache efficiency
# by pre-faulting the linker phases, avoiding redundant linking work in later stages.
RUN cargo nextest archive --tests --all-features \
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
# See Cook (debug) above for why cook uses `--workspace` and archive does not.
RUN cargo chef cook --tests --workspace --all-features --recipe-path /build/recipe.json --release
# Pre-link warm-up: Create and discard a nextest archive to warm up the linker
# before final compilation. This improves incremental build cache efficiency
# by pre-faulting the linker phases, avoiding redundant linking work in later stages.
RUN cargo nextest archive --tests --all-features \
    --archive-file /build/temp.tar.zst --release && rm -f /build/temp.tar.zst


## Build Archive (debug)
FROM dependencies_debug AS build_debug
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --all-features \
    --archive-file /build/torrust-tracker-debug.tar.zst

## Build Archive (release)
FROM dependencies AS build
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --all-features \
    --archive-file /build/torrust-tracker.tar.zst --release


# Extract and Test (debug)
FROM tester AS test_debug
WORKDIR /test
COPY . /test/src/
COPY --from=build_debug \
  /build/torrust-tracker-debug.tar.zst \
  /test/torrust-tracker-debug.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-tracker-debug.tar.zst
RUN mkdir -p /test/src/storage/tracker/lib/database
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
RUN mkdir -p /test/src/storage/tracker/lib/database
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

RUN time mkdir -p /app/bin/ \
 && time cp -l /test/src/target/release/torrust-tracker /app/bin/torrust-tracker \
 && time cp -l /test/src/target/release/http_health_check /app/bin/http_health_check
RUN time mkdir -p /app/lib/ \
 && time cp -l $(realpath $(ldd /app/bin/torrust-tracker | grep "libz\.so\.1" | awk '{print $3}')) /app/lib/libz.so.1
RUN time chown -R root:root /app \
 && time chmod -R u=rw,go=r,a+X /app \
 && time chmod -R a+x /app/bin
RUN rm -rf /app/share/torrust/default/database


## Runtime
FROM gcr.io/distroless/cc-debian13:debug AS runtime
RUN ["/busybox/cp", "-sp", "/busybox/sh","/busybox/cat","/busybox/ls","/busybox/env", "/bin/"]
COPY --from=gcc --chmod=0555 /usr/local/bin/su-exec /bin/su-exec

ARG TORRUST_TRACKER_CONFIG_TOML_PATH="/etc/torrust/tracker/tracker.toml"
ARG USER_ID=1000
ARG UDP_PORT=6969
ARG HTTP_PORT=7070
ARG API_PORT=1212
ARG HEALTH_CHECK_API_PORT=1313

ENV TORRUST_TRACKER_CONFIG_TOML_PATH=${TORRUST_TRACKER_CONFIG_TOML_PATH}
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
