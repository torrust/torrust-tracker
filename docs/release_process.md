---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/index.md
    - .github/workflows/deployment.yaml
    - .github/workflows/deployment-packages.yaml
    - Cargo.toml
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
---

# Torrust Tracker Release Process (v2.2.2)

> **Per-package versioning policy**: as of ADR [20260629000000](adrs/20260629000000_adopt_independent_package_versioning.md),
> all publishable workspace packages version **and are published** independently.
> The tracker application release process below publishes only
> `torrust-tracker` (the root binary crate). All dependency crates are published
> via `deployment-packages.yaml` as they evolve throughout the development cycle.
> For details, see [Publishing a Workspace Package](#publishing-a-workspace-package).

## Version

> **The `[semantic version]` is bumped according to releases, new features, and breaking changes.**
>
> _The `develop` branch uses the (semantic version) suffix `-develop`._

## Process

**Note**: this guide assumes that the your git `torrust` remote is like this:

```sh
git remote show torrust
```

```s
* remote torrust
  Fetch URL: git@github.com:torrust/torrust-tracker.git
  Push  URL: git@github.com:torrust/torrust-tracker.git
...
```

### 1. The `develop` branch is ready for a release

The `develop` branch should have the version `[semantic version]-develop` that is ready to be released.

### 2. Stage `develop` HEAD for merging into the `main` branch

```sh
git fetch --all
git push --force torrust develop:staging/main
```

### 3. Create Release Commit

```sh
git stash
git switch staging/main
git reset --hard torrust/staging/main
# change `[semantic version]-develop` to `[semantic version]`.
git add -A
git commit -m "release: version [semantic version]"
git push torrust
```

### 4. Create and Merge Pull Request from `staging/main` into `main` branch

Pull request title format: "Release Version `[semantic version]`".

This pull request merges the new version into the `main` branch.

### 5. Push new version from `main` HEAD to `releases/v[semantic version]` branch

```sh
git fetch --all
git push torrust main:releases/v[semantic version]
```

> **Check that the deployment is successful!**

### 6. Create Release Tag

```sh
git switch releases/v[semantic version]
git tag --sign v[semantic version]
git push --tags torrust
```

Make sure the [deployment](https://github.com/torrust/torrust-tracker/actions/workflows/deployment.yaml) workflow was successfully executed and the new version for the `torrust-tracker` binary crate was published on [crates.io](https://crates.io/crates/torrust-tracker).

All dependency crates are published independently via
`deployment-packages.yaml` as they evolve throughout the release cycle —
they should already be on crates.io by this point.

### 7. Create Release on Github from Tag

This is for those who wish to download the source code.

### 8. Stage `main` HEAD for merging into the `develop` branch

Merge release back into the develop branch.

```sh
git fetch --all
git push --force torrust main:staging/develop
```

### 9. Create Comment that bumps next development version

```sh
git stash
git switch staging/develop
git reset --hard torrust/staging/develop
# change `[semantic version]` to `(next)[semantic version]-develop`.
git add -A
git commit -m "develop: bump to version (next)[semantic version]-develop"
git push torrust
```

### 10. Create and Merge Pull Request from `staging/develop` into `develop` branch

Pull request title format: "Version `[semantic version]` was Released".

This pull request merges the new release into the `develop` branch and bumps the version number.

## Publishing a Workspace Package

With independent package versioning, any workspace crate can be published at its own cadence
without waiting for a full tracker release.

> **Important**: all workspace packages are published **independently** via
> `deployment-packages.yaml` as they evolve throughout the development cycle.
> By the time a tracker release happens, all dependency crates are already on
> crates.io — the tracker release workflow only publishes `torrust-tracker`
> itself. See [Real-World Example](#real-world-example-a-full-release-cycle) below.

### Branch and Tag Conventions

| Concept        | Convention                            | Example                                            |
| -------------- | ------------------------------------- | -------------------------------------------------- |
| Release branch | `releases/pkg/<crate-name>/v<semver>` | `releases/pkg/torrust-tracker-udp-protocol/v0.2.0` |
| Release tag    | `pkg/<crate-name>/v<semver>` (signed) | `pkg/torrust-tracker-udp-protocol/v0.2.0`          |

Pushing a branch matching `releases/pkg/**` triggers the CI workflow
`deployment-packages.yaml`, which publishes the package to crates.io.

### When to Publish Independently

Whenever a workspace crate's version changes. Examples:

- You fixed a bug in `torrust-tracker-core` and bumped it from v0.3.0 to v0.3.1.
- You added a new endpoint in `torrust-tracker-rest-api-protocol` and bumped it to v0.4.0.
- You need to publish a crate for the first time (initial release).
- You need to publish a crate for extraction to a standalone repository.

### Automated Workflow (primary path)

1. Ensure the package has its own explicit `version` field (not `version.workspace = true`).
2. Verify the package builds and passes tests:

   ```sh
   cargo test -p <crate-name>
   ```

3. Create the release branch from `develop`:

   ```sh
   git fetch --all
   git push torrust develop:releases/pkg/<crate-name>/v<semver>
   ```

4. CI (`deployment-packages.yaml`) runs tests and publishes to crates.io automatically.
5. Once successful, create the signed tag:

   ```sh
   git fetch --all
   git push torrust torrust/main:pkg/<crate-name>/v<semver>  # fast-forward tag branch
   git tag --sign pkg/<crate-name>/v<semver>                 # or tag from any reachable commit
   git push --tags torrust
   ```

6. Update the `version` field in the workspace root `Cargo.toml` dependency entry for
   the published crate (e.g., from `3.0.0-develop` to `0.1.0`). Do **not** remove the
   `path = "..."` — it ensures workspace builds always use the local copy regardless
   of the published version.

### Manual Fallback

If CI is unavailable or you need to publish without creating a Git reference:

1. Ensure the package has its own explicit `version` field.
2. Verify the package builds and passes tests:

   ```sh
   cargo test -p <crate-name>
   ```

3. Perform a dry-run publish:

   ```sh
   cargo publish -p <crate-name> --dry-run
   ```

4. Publish:

   ```sh
   cargo publish -p <crate-name>
   ```

> **Note on dependency order**: if the package has workspace-internal dependencies that are
> not yet published, publish them first. The workspace root `Cargo.toml` documents the
> dependency graph.

### Real-World Example: A Full Release Cycle

This example shows how independent package publishing works in practice over a typical
release cycle, from development through tracker release.

#### Starting Point

Workspace has three packages:

- `torrust-tracker-primitives` v0.1.0 (published)
- `torrust-tracker-core` v0.2.0 (published, depends on `primitives`)
- `torrust-tracker` v3.0.0-develop (unpublished, depends on both)

The tracker binary `v3.0.0-develop` references `primitives 0.1.0` and `core 0.2.0`
via `path = "..."` in the workspace.

#### Week 1 — Bugfix in `primitives`

A bug is discovered in `torrust-tracker-primitives`. Fix is merged to `develop`,
version bumped to `0.1.1`.

```sh
# Publish independently — no need to wait for tracker release
git push torrust develop:releases/pkg/torrust-tracker-primitives/v0.1.1
# CI publishes v0.1.1 to crates.io
git tag --sign pkg/torrust-tracker-primitives/v0.1.1 && git push --tags torrust
```

External consumers can now use `primitives 0.1.1`. The tracker still uses the
`path` dependency, so it gets the fix automatically.

#### Week 3 — New feature in `core`

A new API is added to `torrust-tracker-core`. Version bumped to `0.3.0`.

```sh
git push torrust develop:releases/pkg/torrust-tracker-core/v0.3.0
# CI publishes v0.3.0 to crates.io
git tag --sign pkg/torrust-tracker-core/v0.3.0 && git push --tags torrust
```

External consumers of `core` can now use the new feature. The tracker workspace
still uses the local `path` dependency.

#### Week 5 — Tracker release

The release commit bumps the tracker version from `3.0.0-develop` to `3.0.0`.

```sh
# Create release branch — only publishes torrust-tracker itself
git push torrust main:releases/v3.0.0
# CI publishes only torrust-tracker v3.0.0
# primitives 0.1.1 and core 0.3.0 are already on crates.io
```

**Key observation**: the tracker release did NOT need to publish `primitives` or `core`.
They were already on crates.io from weeks 1 and 3. The tracker release only published
one crate: `torrust-tracker` itself.

#### Why This Matters

- Each crate's version history reflects its own changes (accurate SemVer signals).
- No unnecessary version bumps on unrelated crates.
- External consumers get fixes and features immediately, not whenever the next
  tracker release happens.
- The tracker release is a lightweight final step, not a batch bottleneck.
