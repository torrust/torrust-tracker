# syntax=docker/dockerfile:latest
#
# semantic-links:
#   skill-links:
#     - add-workspace-member
#   related-artifacts:
#     - Cargo.toml  # `[workspace] default-members` is the positive list of packages tested in the image
#     - .dockerignore  # allow-list of the build context; every workspace member must be admitted
#     - .hadolint.yaml  # hadolint global linting rules and ignore policies with rationale

# Torrust Tracker — multi-platform image, cross-compiled with cargo-zigbuild (no QEMU for compiling)
#
#   docker buildx build --platform linux/amd64,linux/arm64 --target release .
#
# Everything that compiles or packages runs on $BUILDPLATFORM. The only step that
# ever runs on $TARGETPLATFORM is the `test*` stages (they execute the cross-compiled
# test binaries). For a foreign architecture that needs binfmt/QEMU, so it can be
# switched off without touching the rest of the build:
#
#   --build-arg TEST_GATE_RELEASE=test_skipped --build-arg TEST_GATE_DEBUG=test_skipped

ARG TORRUST_CARGO_CHEF_VERSION=0.1.78
ARG CARGO_NEXTEST_VERSION=0.9.140
# glibc floor for the produced binaries. Must be >= the glibc that the target-arch
# libssl/libcrypto (Debian trixie = 2.41) were built against, and <= the runtime image's glibc
# (distroless cc-debian13 = 2.41).
ARG GLIBC_VERSION=2.41

# Which stage must succeed before the release/debug artifacts are packaged.
# `test_*_passed` = run the tests (on $TARGETPLATFORM); `test_skipped` = don't.
ARG TEST_GATE_DEBUG=test_debug_passed
ARG TEST_GATE_RELEASE=test_release_passed

###############################################################################
## Builder toolchain (runs on the BUILD platform; shared by every target arch)
###############################################################################
FROM --platform=$BUILDPLATFORM ghcr.io/rust-cross/cargo-zigbuild:0.24.0 AS chef
WORKDIR /tmp

# Slowest-changing layers first. Each tool has its own ARG + RUN, so bumping one
# version only invalidates that tool's layer and the ones after it.
# libssl-dev (NATIVE arch) is for host-side artifacts only: proc-macros and build scripts
# (e.g. sqlx-macros -> native-tls -> openssl-sys) are always compiled for the build host.
# NOTE: zig, cargo-zigbuild, clang, libclang-dev are pre-installed in the image.
# cmake/ninja are NOT needed for aws-lc-sys (it uses the cc builder, not the
# cmake builder, because pre-generated bindings are available). They are included
# as a safety net for any transitive dependency that might invoke the cmake binary
# directly in its build.rs (rather than through the cmake Rust crate).
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates cmake curl libssl-dev ninja-build pkg-config python3 python3-venv \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

# zig is pre-installed at /usr/local/bin/zig (standalone binary from ziglang.org tarball).
# cargo-zigbuild is pre-installed at /usr/local/cargo/bin/cargo-zigbuild.
ENV ZIG_GLOBAL_CACHE_DIR=/cache/zig

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Note: We use our `torrust-cargo-chef` fork while upstream PR
# https://github.com/LukeMathWalker/cargo-chef/pull/360 is pending. Once merged,
# switch back to upstream `cargo-chef` and remove this comment.
# Fork package name: `torrust-cargo-chef` (binary: `cargo-chef`).
#
# We use `cargo install --git` instead of `cargo binstall --git` because the
# fork has no GitHub releases — `cargo binstall` can only download prebuilt
# binaries from releases and will fail with "Not found" when none exist.
# `cargo install --git` compiles the crate from source, which works without
# releases. The fork's default branch (`main`) carries both the `--external-only`
# and `--zigbuild` features used below, plus the libc-version-stripping fix.
RUN cargo install --locked --git https://github.com/oskolist/cargo-chef.git torrust-cargo-chef

ARG CARGO_NEXTEST_VERSION
RUN cargo binstall --no-confirm --locked "cargo-nextest@${CARGO_NEXTEST_VERSION}"

# cargo-zigbuild is pre-installed in the ghcr.io/rust-cross/cargo-zigbuild image.

# `cargo nextest archive` builds through `$CARGO [--color=...] test --no-run`.
# IMPORTANT: cargo-nextest's CargoCli::all_args() (nextest source: cargo_cli.rs)
# places the --color flag BEFORE the subcommand, so $1 is always --color=auto/
# always/never — NEVER "test". An earlier version of this shim checked
# `if [ "$1" = "test" ]`, which never matched, so it fell through to
# `exec "$REAL_CARGO" "$@"`, bypassing cargo-zigbuild entirely. That meant
# CC_*/CARGO_TARGET_*_LINKER were never set, and the cc crate fell back to the
# default cross-compiler (`aarch64-linux-gnu-gcc`) which does not exist on the
# aarch64 arm64 path.
#
# Fix: always delegate to `cargo-zigbuild` for every subcommand. cargo-zigbuild:
#  1. Normalizes global args (--color=...) to after the subcommand
#     (normalize_global_cargo_args in cargo-zigbuild src/bin/cargo-zigbuild.rs)
#  2. For test/build/zigtest/zigbuild subcommands: calls apply_command_env which
#     sets CC_*/CXX_*/CARGO_TARGET_*_LINKER to the zigcc wrapper script (which
#     strips --target=<rust-triple> via LinkerArgFilter, linker_args.rs:105 —
#     the cc crate v1.10.0 adds that flag, cc/src/lib.rs:3144, and `zig cc`
#     chokes on it because "unknown" is not a valid zig OS name)
#  3. For other subcommands (metadata, -vV, locate-project, ...): forwards to the
#     real cargo via the CARGO env var we export below
# The --target rewrite below converts the canonical rust triple (e.g.
# aarch64-unknown-linux-gnu) to the glibc-suffixed zig target (e.g.
# aarch64-unknown-linux-gnu.2.41), which cargo-zigbuild strips back internally.
# See https://github.com/nextest-rs/nextest/discussions/3185
# Use it ONLY as `CARGO=... cargo-nextest nextest ...` (never as a global ENV).
COPY --chmod=0755 <<"EOF" /usr/local/bin/cargo-nextest-zigbuild
#!/bin/sh
set -eu
REAL_CARGO=/usr/local/cargo/bin/cargo
export CARGO="$REAL_CARGO"   # avoid recursion: cargo-zigbuild calls the real cargo via CARGO env
# Rewrite ONLY the value of `--target` (both `--target <triple>` and
# `--target=<triple>`). Rewriting *every* argument equal to $RUST_TARGET is
# wrong: cargo-nextest runs `cargo metadata --format-version=1 --all-features
# --filter-platform $RUST_TARGET` through this shim (acquire_graph_data in
# nextest's dispatch/helpers.rs). A glibc-suffixed triple is not a rustc target,
# so the rewritten value made cargo fail with
# `could not find specification for target "aarch64-unknown-linux-gnu.2.41"`.
n=$#
target_value=
while [ "$n" -gt 0 ]; do
  a=$1; shift
  if [ -n "$target_value" ]; then
    if [ "$a" = "$RUST_TARGET" ]; then a="$ZIG_BUILD_TARGET"; fi
    target_value=
  else
    case "$a" in
      --target)                target_value=1 ;;
      "--target=$RUST_TARGET") a="--target=$ZIG_BUILD_TARGET" ;;
    esac
  fi
  set -- "$@" "$a"
  n=$((n - 1))
done
exec cargo-zigbuild "$@"
EOF

###############################################################################
## Per-architecture selection (BUILD platform, no emulation). Only ENV here.
##   RUST_TARGET : rustc target triple (also the `target/<dir>` name)
##   ZIG_TARGET  : zig triple == Debian multiarch triplet (used for /usr/lib/<triplet>)
##   DEB_ARCH    : Debian architecture name
###############################################################################
## The OPENSSL_* vars MUST be prefixed with the (uppercased) target triple, otherwise they also
## hit host-built proc-macros/build scripts and the host linker pulls in target-arch libs.
##
## The RUSTFLAGS below add the target-arch library directory to the linker search path.
## We use `-L` (not `-rpath-link`) because cargo-zigbuild's `zig cc` wrapper does not
## support the GNU ld-only `-rpath-link` argument and rejects it with
## "unsupported linker arg: -rpath-link" when linking test binaries.
FROM --platform=$BUILDPLATFORM chef AS chef_amd64
# We deliberately do NOT set CC_x86_64_unknown_linux_gnu or CXX_* here.
# When the ENV vars are absent, cargo-zigbuild's `add_env_if_missing`
# (see cargo-zigbuild src/zig/cargo_env.rs) installs its own `zigcc`
# wrapper script as CC_*/CXX_*/CARGO_TARGET_*_LINKER. That wrapper
# (src/zig/wrapper.rs) adds `-g`, `-fno-sanitize=all`, and
# `-target <glibc-suffixed-zig-triple>`, which are equivalent to the
# flags we previously hardcoded. Crucially, the wrapper's execute_compiler
# (src/zig/mod.rs) runs every arg through LinkerArgFilter, which strips
# `--target=<rust-triple>` (linker_args.rs:105) — the cc crate v1.10.0
# adds that flag (cc/src/lib.rs:3144) and `zig cc` chokes on it because
# "unknown" is not a valid zig OS name. If we pre-set CC_* to `zig cc`,
# add_env_if_missing won't override it (it checks env::var_os), so the
# wrapper never gets installed and the `--target` flag reaches `zig cc`
# raw, producing "unable to parse target query 'x86_64-unknown-linux-gnu'".
ARG GLIBC_VERSION
ENV RUST_TARGET=x86_64-unknown-linux-gnu \
    ZIG_TARGET=x86_64-linux-gnu \
    DEB_ARCH=amd64 \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-L/usr/lib/x86_64-linux-gnu" \
    X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu \
    X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR=/opt/openssl/include

FROM --platform=$BUILDPLATFORM chef AS chef_arm64
# See chef_amd64 for why CC_*/CXX_* are NOT set here — cargo-zigbuild's
# add_env_if_missing installs the zigcc wrapper (which strips --target=)
# only when those ENV vars are absent.
ARG GLIBC_VERSION
ENV RUST_TARGET=aarch64-unknown-linux-gnu \
    ZIG_TARGET=aarch64-linux-gnu \
    DEB_ARCH=arm64 \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-L/usr/lib/aarch64-linux-gnu" \
    AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu \
    AARCH64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR=/opt/openssl/include

## Target toolchain: rust std for the target + target-arch OpenSSL (dev files only used to LINK).
# TARGETARCH is provided by buildx (amd64 | arm64)
FROM --platform=$BUILDPLATFORM chef_${TARGETARCH} AS chef_target
ARG GLIBC_VERSION
ENV ZIG_BUILD_TARGET="${RUST_TARGET}.${GLIBC_VERSION}"

# Target-arch OpenSSL via Debian multiarch (nothing from it is ever executed here).
#  * zig always uses `-nostdinc`, so Debian's split header layout
#    (/usr/include/openssl + /usr/include/<triplet>/openssl/configuration.h)
#    is merged into ONE directory that is passed as OPENSSL_INCLUDE_DIR.
#    (We deliberately do NOT add /usr/include to zig's include path: host glibc headers
#    would clash with zig's own libc headers.)
#  * /opt/openssl/runtime holds the shared objects needed to *run* test binaries in `tester`.
RUN set -eux; \
    dpkg --add-architecture "${DEB_ARCH}"; \
    apt-get update; \
    apt-get install -y --no-install-recommends "libssl-dev:${DEB_ARCH}"; \
    mkdir -p /opt/openssl/include /opt/openssl/runtime; \
    cp -a /usr/include/openssl /opt/openssl/include/; \
    cp -a "/usr/include/${ZIG_TARGET}/openssl/." /opt/openssl/include/openssl/; \
    cp -L "/usr/lib/${ZIG_TARGET}/libssl.so.3" "/usr/lib/${ZIG_TARGET}/libcrypto.so.3" /opt/openssl/runtime/; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*

RUN rustup target add "${RUST_TARGET}"
WORKDIR /build/src

###############################################################################
## Chef Prepare (platform independent => shared by all target architectures)
###############################################################################
FROM --platform=$BUILDPLATFORM chef AS recipe
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

###############################################################################
## Helpers (all BUILD platform)
###############################################################################

## nextest binary for the TARGET arch (downloaded, never executed here) – used by the test stages
FROM --platform=$BUILDPLATFORM chef_${TARGETARCH} AS nextest_target
ARG CARGO_NEXTEST_VERSION
RUN cargo binstall --no-confirm --locked --disable-strategies compile \
      --targets "${RUST_TARGET}" --install-path /out/bin "cargo-nextest@${CARGO_NEXTEST_VERSION}"

## Su Exe Compile (cross-compiled with `zig cc`, replaces the former gcc stage)
FROM --platform=$BUILDPLATFORM chef_${TARGETARCH} AS su_exec
COPY ./contrib/dev-tools/su-exec/ /usr/local/src/su-exec/
RUN --mount=type=cache,target=/cache/zig \
    zig cc -target "${ZIG_TARGET}" -Wall -Werror -g \
      /usr/local/src/su-exec/su-exec.c -o /usr/local/bin/su-exec \
 && chmod +x /usr/local/bin/su-exec

## Database initialization: Tests at runtime require a pre-initialized SQLite3 database
# to test against a valid (not corrupted) schema. The VACUUM command optimizes the
# database file layout. The DB file is architecture independent, so it is created
# once on the build platform.
FROM --platform=$BUILDPLATFORM docker.io/library/debian:trixie-slim AS db_init
RUN apt-get update \
 && apt-get install -y --no-install-recommends sqlite3 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
COPY ./share/ /app/share/torrust
RUN mkdir -p /app/share/torrust/default/database/ \
 && sqlite3 /app/share/torrust/default/database/tracker.sqlite3.db "VACUUM;"

## Runtime skeleton: replaces the `RUN cp -s` / `RUN mkdir` of the runtime stage so that
# no process ever has to execute inside the (foreign-arch) runtime image.
FROM --platform=$BUILDPLATFORM docker.io/library/debian:trixie-slim AS skeleton
RUN mkdir -p /skel/bin /skel/var/lib/torrust/tracker /skel/var/log/torrust/tracker /skel/etc/torrust/tracker \
 && for t in sh cat ls env; do ln -s "/busybox/$t" "/skel/bin/$t"; done

## Packager: assembles /app on the build platform (chown/chmod/readelf work on any arch)
FROM --platform=$BUILDPLATFORM docker.io/library/debian:trixie-slim AS packager
RUN apt-get update \
 && apt-get install -y --no-install-recommends binutils \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

###############################################################################
## Cook (debug)
###############################################################################
FROM --platform=$BUILDPLATFORM chef_target AS dependencies_thirdparty_debug
WORKDIR /build/src
# Only third-party recipe: immune to workspace Cargo.toml changes.
COPY --from=recipe /build/recipe-thirdparty.json /build/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/cache/zig \
    cargo chef cook --zigbuild --target "${ZIG_BUILD_TARGET}" \
      --tests --workspace --all-features --recipe-path /build/recipe.json

FROM --platform=$BUILDPLATFORM dependencies_thirdparty_debug AS dependencies_debug
WORKDIR /build/src
# Full recipe on top — reuses third-party artifacts from parent layer.
COPY --from=recipe /build/recipe.json /build/recipe.json
# `cargo chef cook` keeps `--workspace` so the skeleton covers every member the
# recipe enumerates. `cargo nextest archive` deliberately omits `--workspace`: it
# then builds only the root `[workspace] default-members` (the positive list of
# packages tested inside the image), so no per-package exclusions are needed here.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/cache/zig \
    cargo chef cook --zigbuild --target "${ZIG_BUILD_TARGET}" \
      --tests --workspace --all-features --recipe-path /build/recipe.json
# Pre-link warm-up: Create and discard a nextest archive to warm up the linker
# before final compilation (see the shim in the chef stage for why CARGO is set).
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/cache/zig \
    CARGO=/usr/local/bin/cargo-nextest-zigbuild cargo-nextest nextest archive \
      --target "${RUST_TARGET}" --tests --all-features \
      --archive-file /build/temp.tar.zst \
 && rm -f /build/temp.tar.zst

###############################################################################
## Cook (release)
###############################################################################
FROM --platform=$BUILDPLATFORM chef_target AS dependencies_thirdparty
WORKDIR /build/src
# Only third-party recipe: immune to workspace Cargo.toml changes.
COPY --from=recipe /build/recipe-thirdparty.json /build/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/cache/zig \
    cargo chef cook --zigbuild --target "${ZIG_BUILD_TARGET}" \
      --tests --workspace --all-features --recipe-path /build/recipe.json --release

FROM --platform=$BUILDPLATFORM dependencies_thirdparty AS dependencies
WORKDIR /build/src
# Full recipe on top — reuses third-party artifacts from parent layer.
COPY --from=recipe /build/recipe.json /build/recipe.json
# See Cook (debug) above for why cook uses `--workspace` and archive does not.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/cache/zig \
    cargo chef cook --zigbuild --target "${ZIG_BUILD_TARGET}" \
      --tests --workspace --all-features --recipe-path /build/recipe.json --release
# Pre-link warm-up (see debug stage).
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/cache/zig \
    CARGO=/usr/local/bin/cargo-nextest-zigbuild cargo-nextest nextest archive \
      --target "${RUST_TARGET}" --tests --all-features \
      --archive-file /build/temp.tar.zst --release \
 && rm -f /build/temp.tar.zst

###############################################################################
## Cook Shipped
###############################################################################
## The binaries that ship are built by `cargo build` invocations of their own, never taken from the
## nextest archives above: `--tests` and `--all-features` enable dev-dependency features and every
## feature, which would ship every `testing` feature (ADR 20261021190318, rule A.5). The shipped
## builds use no test flag and name no feature, because the shipped packages need none today; name
## any feature they need explicitly here and in the "Shipped Features" job of
## `.github/workflows/testing.yaml`, which checks that no `testing` feature reaches the build.
##
## The cook stages stack on the third-party layers of the test cooks, so third-party crates whose
## features do not differ are compiled once; they only rebuild when the external dependencies change.
# Cook Third-party Shipped (debug)
FROM --platform=$BUILDPLATFORM dependencies_thirdparty_debug AS dependencies_shipped_thirdparty_debug
WORKDIR /build/src
RUN cargo chef cook --zigbuild --target "${ZIG_BUILD_TARGET}" --package torrust-tracker --bin torrust-tracker --recipe-path /build/recipe.json

# Cook Shipped (debug)
FROM --platform=$BUILDPLATFORM dependencies_shipped_thirdparty_debug AS dependencies_shipped_debug
WORKDIR /build/src
COPY --from=recipe /build/recipe.json /build/recipe.json
RUN cargo chef cook --zigbuild --target "${ZIG_BUILD_TARGET}" --package torrust-tracker --bin torrust-tracker --recipe-path /build/recipe.json

# Cook Third-party Shipped (release)
FROM --platform=$BUILDPLATFORM dependencies_thirdparty AS dependencies_shipped_thirdparty
WORKDIR /build/src
RUN cargo chef cook --zigbuild --target "${ZIG_BUILD_TARGET}" --release --package torrust-tracker --bins --recipe-path /build/recipe.json

# Cook Shipped (release)
FROM --platform=$BUILDPLATFORM dependencies_shipped_thirdparty AS dependencies_shipped
WORKDIR /build/src
COPY --from=recipe /build/recipe.json /build/recipe.json
RUN cargo chef cook --zigbuild --target "${ZIG_BUILD_TARGET}" --release --package torrust-tracker --bins --recipe-path /build/recipe.json

# Build Shipped (debug)
FROM --platform=$BUILDPLATFORM dependencies_shipped_debug AS build_shipped_debug
WORKDIR /build/src
COPY . /build/src
RUN cargo zigbuild --release --target "${ZIG_BUILD_TARGET}" --package torrust-tracker --bin torrust-tracker
RUN mkdir -p /out/bin && \
    mv "/build/src/target/${RUST_TARGET}/release/torrust-tracker" "/build/src/target/${RUST_TARGET}/release/http_health_check" /out/bin/

# Build Shipped (release)
FROM --platform=$BUILDPLATFORM dependencies_shipped AS build_shipped
WORKDIR /build/src
COPY . /build/src
RUN cargo zigbuild --release --target "${ZIG_BUILD_TARGET}" --package torrust-tracker --bins
RUN mkdir -p /out/bin && \
    mv "/build/src/target/${RUST_TARGET}/release/torrust-tracker" "/build/src/target/${RUST_TARGET}/release/http_health_check" /out/bin/

###############################################################################
## Build Archive (debug) — cross-compiled on the BUILD platform
###############################################################################
FROM --platform=$BUILDPLATFORM dependencies_debug AS build_debug
WORKDIR /build/src
COPY . /build/src
# the nextest archive (consumed by the test stage, on the target platform)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/cache/zig \
    CARGO=/usr/local/bin/cargo-nextest-zigbuild cargo-nextest nextest archive \
      --target "${RUST_TARGET}" --tests --all-features \
      --archive-file /build/torrust-tracker-debug.tar.zst

## Build Archive (release)
FROM --platform=$BUILDPLATFORM dependencies AS build
WORKDIR /build/src
COPY . /build/src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/cache/zig \
    CARGO=/usr/local/bin/cargo-nextest-zigbuild cargo-nextest nextest archive \
      --target "${RUST_TARGET}" --tests --all-features \
      --archive-file /build/torrust-tracker.tar.zst --release

###############################################################################
## Tester Image — the ONLY place that runs on $TARGETPLATFORM.
## No RUN here: nextest is a downloaded target-arch binary, the DB comes from db_init,
## and libssl/libcrypto (needed by the test binaries) are copied from `chef_target`.
###############################################################################
FROM docker.io/library/rust:slim-trixie AS tester
COPY --from=nextest_target /out/bin/cargo-nextest /usr/local/bin/cargo-nextest
COPY --from=chef_target /opt/openssl/runtime/ /usr/local/lib/openssl-runtime/
ENV LD_LIBRARY_PATH=/usr/local/lib/openssl-runtime
COPY --from=db_init /app/share/ /app/share/

# Extract and Test (debug)
FROM tester AS test_debug
WORKDIR /test
COPY . /test/src/
COPY --from=build_debug /build/torrust-tracker-debug.tar.zst /test/torrust-tracker-debug.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-tracker-debug.tar.zst
# WORKDIR creates the directory without needing a RUN (replaces `mkdir -p`)
WORKDIR /test/src/storage/tracker/lib/database
WORKDIR /test
# The native-tracker tests start the shipped binary, not the one built with the test features.
COPY --from=build_shipped_debug /out/bin/torrust-tracker /app/bin/torrust-tracker
RUN TORRUST_TRACKER_BINARY=/app/bin/torrust-tracker cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

# Extract and Test (release)
FROM tester AS test
WORKDIR /test
COPY . /test/src
COPY --from=build /build/torrust-tracker.tar.zst /test/torrust-tracker.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-tracker.tar.zst
WORKDIR /test/src/storage/tracker/lib/database
WORKDIR /test
# The native-tracker tests start the shipped binary, not the one built with the test features.
COPY --from=build_shipped \
  /out/bin/torrust-tracker \
  /out/bin/http_health_check \
  /app/bin/
RUN TORRUST_TRACKER_BINARY=/app/bin/torrust-tracker cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

###############################################################################
## Test gates: packaging depends on one of these. Selected via TEST_GATE_* build args.
###############################################################################
FROM scratch AS test_debug_passed
COPY --from=test_debug /test/src/Cargo.toml /gate

FROM scratch AS test_release_passed
COPY --from=test /test/src/Cargo.toml /gate

FROM --platform=$BUILDPLATFORM docker.io/library/debian:trixie-slim AS test_skipped
RUN echo skipped > /gate

FROM ${TEST_GATE_DEBUG} AS debug_test_gate
FROM ${TEST_GATE_RELEASE} AS release_test_gate

###############################################################################
## Assemble /app (BUILD platform; tested == shipped binaries)
## Runtime libs (libssl3t64, zlib1g, libzstd1, libc6) are already part of
## gcr.io/distroless/cc-debian13, so nothing is bundled here.
###############################################################################
FROM --platform=$BUILDPLATFORM packager AS app_debug
COPY --from=debug_test_gate /gate /tmp/test-gate
COPY --from=db_init /app/share/ /app/share/
COPY --from=build_shipped_debug /out/bin/ /app/bin/
RUN for b in /app/bin/*; do echo "== $b"; readelf -d "$b" | grep NEEDED || true; done \
 && chown -R root:root /app \
 && chmod -R u=rw,go=r,a+X /app \
 && chmod -R a+x /app/bin

FROM --platform=$BUILDPLATFORM packager AS app_release
COPY --from=release_test_gate /gate /tmp/test-gate
COPY --from=db_init /app/share/ /app/share/
COPY --from=build_shipped /out/bin/ /app/bin/
RUN for b in /app/bin/*; do echo "== $b"; readelf -d "$b" | grep NEEDED || true; done \
 && chown -R root:root /app \
 && chmod -R u=rw,go=r,a+X /app \
 && chmod -R a+x /app/bin \
 && rm -rf /app/share/torrust/default/database

###############################################################################
## Runtime (TARGET platform image, but with ZERO `RUN` instructions => no emulation)
###############################################################################
FROM gcr.io/distroless/cc-debian13:debug AS runtime
# busybox symlinks in /bin and the torrust directories come from the build-platform skeleton
COPY --from=skeleton /skel/bin/ /bin/
COPY --from=skeleton /skel/var/ /var/
COPY --from=skeleton /skel/etc/ /etc/
COPY --from=su_exec --chmod=0555 /usr/local/bin/su-exec /bin/su-exec

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

ENV ENV=/etc/profile
COPY --chmod=0555 ./share/container/entry_script_sh /usr/local/bin/entry.sh

VOLUME ["/var/lib/torrust/tracker","/var/log/torrust/tracker","/etc/torrust/tracker"]

ENV RUNTIME="runtime"
ENTRYPOINT ["/usr/local/bin/entry.sh"]


## Torrust-Tracker (debug)
FROM runtime AS debug
ENV RUNTIME="debug"
COPY --from=app_debug /app/ /usr/
CMD ["sh"]

## Torrust-Tracker (release) (default)
FROM runtime AS release
ENV RUNTIME="release"
COPY --from=app_release /app/ /usr/
HEALTHCHECK --interval=5s --timeout=5s --start-period=3s --retries=3 \
  CMD /usr/bin/http_health_check http://localhost:${HEALTH_CHECK_API_PORT}/health_check \
    || exit 1
CMD ["/usr/bin/torrust-tracker"]
