#!/bin/bash

# Set CARGO_BUILD_JOBS to override the default cap on parallel Rust compiler jobs of the image build
# (see "Memory and CPU use" in docs/containers.md), for example: CARGO_BUILD_JOBS=8 ./docker-build.sh

echo "Building docker image ..."

build_args=()

if [ -n "${CARGO_BUILD_JOBS:-}" ]; then
    build_args+=(--build-arg "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS}")
fi

docker build --target release --tag torrust-tracker:release "${build_args[@]}" --file Containerfile .
