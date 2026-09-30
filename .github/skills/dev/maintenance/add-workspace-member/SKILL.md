---
name: add-workspace-member
description: Add or remove an explicit Cargo workspace member in Torrust Tracker. Use when editing the root workspace members list, adding a developer-tool crate, registering a new standalone workspace package, or deciding whether a package is tested inside the container image.
metadata:
  author: torrust
  version: "2.0"
semantic-links:
  related-artifacts:
    - Cargo.toml
    - Containerfile
    - .dockerignore
    - .github/skills/dev/maintenance/add-rust-dependency/SKILL.md
    - docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md
---

# Add a Cargo Workspace Member

Use this workflow when changing the root `Cargo.toml` `[workspace]` section. Every in-repo crate
is listed explicitly in `members`, including crates reachable as path dependencies, because the
cargo-chef `--external-only` skeleton strips path dependencies and would otherwise lose them.

The container build derives packages and targets from Cargo (see the
[positive-list container build ADR](../../../../../docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md)).
Do not edit the `Containerfile` for a package change; it contains no per-package lists.

## Required Review

1. Add or remove the crate path in root `Cargo.toml` `[workspace].members`.
2. Decide whether the crate is tested inside the container image and update
   `[workspace].default-members` accordingly. This is the only positive list for container test
   archives; `cargo nextest archive` runs without `--workspace` in the `Containerfile`.
   - Product crates (root crate and `packages/*` that are part of the tracker or its tests) are
     default members.
   - Tracker verification tools (CLI clients, benchmarks, host-only E2E runners) and AI-harness
     tools are members but **not** default members.
3. Make the crate reachable in the Docker build context. `.dockerignore` is a default-deny
   allow-list; `cargo chef prepare` runs `cargo metadata`, which aborts if any member's manifest or
   declared target file is missing.
   - Crates under `packages/`, `console/`, `src/`, or `tests/` are already admitted.
   - A crate anywhere else (for example `contrib/dev-tools/…`) needs its own `!/path/to/crate/`
     line in the interim AI-harness block. Remove the line when the crate leaves the workspace.
   - Never add a broad negation; admit the crate directory only.
4. Removing a member: delete it from `members`, from `default-members` if present, and its
   `.dockerignore` line if it had one.

## Verification

Run the narrow validation appropriate to the change before the normal repository gate:

```bash
cargo metadata --no-deps --format-version 1 > /dev/null
docker build --target recipe --file Containerfile .
```

For a changed `default-members` entry, also run:

```bash
docker build --target test_debug --file Containerfile .
```

Then run `linter all`, `cargo test --doc --workspace`, and the mandatory pre-commit workflow.

## Related Skills

- [`add-rust-dependency`](../add-rust-dependency/SKILL.md) — add an external dependency.
