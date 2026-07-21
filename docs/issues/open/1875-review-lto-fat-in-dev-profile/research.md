---
semantic-links:
  related-artifacts:
    - docs/issues/open/1875-review-lto-fat-in-dev-profile/ISSUE.md
    - Cargo.toml
    - Containerfile
    - commit 3c715fbb
---

# Research: Development-profile LTO

## Question

Should the tracker retain `lto = "fat"` in `[profile.dev]`?

## Decision

No. Remove the explicit development-profile LTO setting and use Cargo's default `lto = false` behavior. This follows the maintainer-approved policy:

1. Optimize development builds for compilation speed.
2. Optimize production builds for execution speed.

`[profile.release]` continues to use `lto = "fat"` with `opt-level = 3` because it produces the production artifact.

## Evidence

### Cargo and rustc documentation

Cargo documents the default development profile as `opt-level = 0`, `incremental = true`, `codegen-units = 256`, and `lto = false`. This profile is intended for normal development and debugging. The project overrides `opt-level` to `1`, but the default LTO setting remains appropriate for fast iteration.

Cargo documents `lto = "fat"` as whole-program LTO across the dependency graph, and `lto = "thin"` as a less expensive alternative. Both make linking slower in exchange for better optimized code. The rustc documentation likewise describes fat LTO as whole-program analysis at the cost of longer linking time.

Cargo further documents that `lto = false` permits thin local LTO across a crate's codegen units, while `lto = "off"` fully disables LTO. Removing the key restores Cargo's documented default rather than selecting a non-standard, project-specific optimization policy.

Sources:

- [Cargo profiles: LTO](https://doc.rust-lang.org/cargo/reference/profiles.html#lto)
- [Cargo profiles: default development profile](https://doc.rust-lang.org/cargo/reference/profiles.html#dev)
- [Rustc codegen options: LTO](https://doc.rust-lang.org/rustc/codegen-options/index.html#lto)

### Historic workaround analysis

Commit `3c715fbb` on 2024-06-17 changed `[profile.dev]` from `lto = "thin"` to `lto = "fat"`. Its commit message records a failure while running:

```text
docker build --target release --tag torrust-tracker:release --file Containerfile .
```

The failure occurred in a release invocation using Rust 1.79 stable in a container, while the host default was Rust 1.81 nightly. The error reported an invalid LLVM bitcode producer/reader value for Criterion.

However, `--target release` reaches the `release` Docker target, whose build stages pass `--release` to Cargo. Cargo's `--release` selects `[profile.release]`, not `[profile.dev]`. At that parent revision, `[profile.release]` already used `lto = "fat"`. Thus, the documented failing command cannot have been directly corrected by changing `[profile.dev]`; the causal connection is not supported by the retained evidence.

The current `Containerfile` retains separate debug and release pipelines. The debug pipeline has no `--release` flag and is the relevant regression check for `[profile.dev]`; the release pipeline continues to test the production setting independently.

### Current environment

Collected on 2026-07-21:

- Host rustc: `1.99.0-nightly`, LLVM `22.1.8`.
- Host cargo: `1.99.0-nightly`.
- Docker: `28.3.3`.
- Container base image: `docker.io/library/rust:slim-trixie`.

The repository MSRV is Rust 1.88. The production-container verification is authoritative because it uses the toolchain provided by the `Containerfile` base image.

## Verification implications

- Build `--target debug` after removing `[profile.dev].lto`; this is the meaningful Docker regression test for the changed setting.
- Build `--target release`; it does not validate `[profile.dev]`, but confirms the retained production fat-LTO configuration remains healthy.
- Do not add a per-package LTO override: Cargo does not allow profile overrides to set `lto`.

## Limitations

The original Rust 1.79/1.81-nightly container environment is not reproduced. Reproduction is unnecessary to make the current configuration correct because the recorded command used the release profile, whereas this change only removes an explicit development-profile setting.
