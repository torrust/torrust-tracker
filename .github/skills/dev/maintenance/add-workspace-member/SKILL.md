---
name: add-workspace-member
description: Add or remove an explicit Cargo workspace member in Torrust Tracker. Use when editing the root workspace members list, adding a developer-tool crate, or registering a new standalone workspace package.
metadata:
  author: torrust
  version: "1.0"
semantic-links:
  related-artifacts:
    - Cargo.toml
    - Containerfile
    - .dockerignore
    - .github/skills/dev/maintenance/add-rust-dependency/SKILL.md
---

# Add a Cargo Workspace Member

Use this workflow when changing the root `Cargo.toml` `[workspace].members` list. It applies to
explicit members only; path dependencies can be auto-discovered separately by Cargo.

## Required Review

1. Add or remove the member in root `Cargo.toml`.
2. Read the semantic link above that list and review `Containerfile` at the cargo-chef recipe
   stage. Add or remove its manifest copy and all target stubs required by `cargo metadata`.
3. Decide whether the member has value in container test archives.
   - Production-relevant members remain included.
   - Developer-only analysis, checks, benchmarks, clients, and host-only E2E tools are normally
     excluded from every `cargo nextest archive` invocation.
   - Keep the explanation and all four archive exclusion lists synchronized.
4. Review `.dockerignore`; a manifest copied in the recipe stage must not be excluded from the
   build context.

## Verification

Run the narrow validation appropriate to the change before the normal repository gate:

```bash
docker build --target recipe --file Containerfile .
```

For a changed archive inclusion/exclusion, also run:

```bash
docker build --target test_debug --file Containerfile .
```

Then run `linter all`, `cargo test --doc --workspace`, and the mandatory pre-commit workflow.

## Related Skills

- [`add-rust-dependency`](../add-rust-dependency/SKILL.md) — add an external dependency.
