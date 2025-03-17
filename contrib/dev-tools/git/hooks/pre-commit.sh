#!/bin/bash

cargo +nightly fmt --check &&
    cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features &&
    cargo +nightly doc --no-deps --bins --examples --workspace --all-features &&
    cargo +nightly machete &&
    cargo +stable build &&
    CARGO_INCREMENTAL=0 cargo +stable clippy --no-deps --tests --benches --examples --workspace --all-targets --all-features -- -D clippy::correctness -D clippy::suspicious -D clippy::complexity -D clippy::perf -D clippy::style -D clippy::pedantic &&
    cargo +stable test --doc --workspace &&
    cargo +stable test --tests --benches --examples --workspace --all-targets --all-features
