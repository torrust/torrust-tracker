---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/index.md
    - docs/release-process/tracker.md
    - docs/release-process/workspace-packages.md
    - .github/workflows/deployment.yaml
    - .github/workflows/deployment-packages.yaml
    - Cargo.toml
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
---

# Torrust Tracker Release Process (v2.2.2)

> **Per-package versioning policy**: as of ADR [20260629000000](adrs/20260629000000_adopt_independent_package_versioning.md),
> all publishable workspace packages version **and are published** independently.
> The tracker application release process publishes only
> `torrust-tracker` (the root binary crate). All dependency crates are published
> via `deployment-packages.yaml` as they evolve throughout the development cycle.
> For details, see [Publishing a Workspace Package](#publishing-a-workspace-package).
>
> Use the table below to choose the process.

## Version

> **The `[semantic version]` is bumped according to releases, new features, and breaking changes.**
>
> _The `develop` branch uses the (semantic version) suffix `-develop`._

## Release Processes

| Process                              | Document                                                           |
| ------------------------------------ | ------------------------------------------------------------------ |
| Tracker application (binary) release | [release-process/tracker.md](release-process/tracker.md)           |
| Workspace package release            | [release-process/workspace-packages.md](release-process/workspace-packages.md) |

## Publishing a Workspace Package

This section moved to [release-process/workspace-packages.md](release-process/workspace-packages.md).
