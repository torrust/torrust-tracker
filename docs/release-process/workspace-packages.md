---
semantic-links:
  skill-links:
    - release-workspace-package
    - write-markdown-docs
  related-artifacts:
    - docs/release_process.md
    - .github/workflows/deployment-packages.yaml
    - Cargo.toml
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
---

# Releasing Workspace Packages

## Publishing a Workspace Package

With independent package versioning, any workspace crate can be published at its own cadence
without waiting for a full tracker release.

> **Important**: all workspace packages are published **independently** via
> `deployment-packages.yaml` as they evolve throughout the development cycle.
> By the time a tracker release happens, all dependency crates are already on
> crates.io — the tracker release workflow only publishes `torrust-tracker`
> itself. See [Real-World Example](#real-world-example-a-full-release-cycle) below.

### Branch and Tag Conventions

| Concept        | Workspace package                     | Standalone-repository crate |
| -------------- | ------------------------------------- | --------------------------- |
| Release branch | `releases/pkg/<crate-name>/v<semver>` | none (published manually)   |
| Release tag    | `pkg/<crate-name>/v<semver>` (signed) | `v<semver>` (signed)        |
| Example tag    | `pkg/torrust-tracker-udp-protocol/v0.2.0` | `v0.6.0`                |

Pushing a branch matching `releases/pkg/**` triggers the CI workflow
`deployment-packages.yaml`, which publishes the package to crates.io.

The `pkg/<crate-name>/` prefix only tells workspace crates apart. A crate in its own
repository has a single package, so its tag is `v<semver>`.

**No GitHub release is created for a package.** GitHub Releases are used only for tracker
application releases (see the ADR above). A package is documented by its crate README and
`Cargo.toml` metadata, and tracked by crates.io and the signed tag.

### Who Does What

| Step                                               | Agent                                                       | Maintainer |
| -------------------------------------------------- | ----------------------------------------------------------- | ---------- |
| Version bump, changelog entry, tests, PR           | yes                                                         |            |
| Review and merge the PR                            |                                                             | yes        |
| `cargo publish --dry-run`                          | yes                                                         |            |
| Push the release branch (workspace package)        |                                                             | yes        |
| `cargo publish` (standalone crate, manual)         | only if cargo is authenticated and the maintainer approved  | or yes     |
| Create and push the signed tag                     |                                                             | yes        |

Agents never push to the upstream repository, so the release branch and the tag are always
pushed by the maintainer.

### When to Publish Independently

Whenever a workspace crate's version changes. Examples:

- You fixed a bug in `torrust-tracker-core` and bumped it from v0.3.0 to v0.3.1.
- You added a new endpoint in `torrust-tracker-rest-api-protocol` and bumped it to v0.4.0.
- You need to publish a crate for the first time (initial release).
- You need to publish a crate for extraction to a standalone repository.

### Preconditions (both variants)

1. The package has its own explicit `version` field (not `version.workspace = true`) with the
   new version.
2. The version bump and the changelog entry (when the package keeps one) are merged. **The
   commit that gets published is the merged commit that carries both.**
3. Tests pass:

   ```sh
   cargo test -p <crate-name>
   ```

4. A dry run succeeds:

   ```sh
   cargo publish -p <crate-name> --dry-run
   ```

5. If the package has workspace-internal dependencies that are not yet published, publish them
   first. The workspace root `Cargo.toml` documents the dependency graph.

### Workspace Package (primary path)

1. Make the preconditions true on `develop` (merged PR).
2. The maintainer pushes the merged `develop` commit to the release branch:

   ```sh
   git fetch --all
   git push torrust <merged-commit-sha>:refs/heads/releases/pkg/<crate-name>/v<semver>
   ```

   The branch name is a publishing trigger, not a tag. Push exactly the commit from
   precondition 2, not a later `develop` HEAD.

3. CI (`deployment-packages.yaml`) runs tests and publishes to crates.io automatically. Check
   that the workflow succeeded and that the version is on crates.io.
4. The maintainer creates the signed tag on **the published commit** and pushes it. The tag
   and the release branch have different names, so they never collide:

   ```sh
   git tag --sign pkg/<crate-name>/v<semver> <merged-commit-sha>
   git push torrust refs/tags/pkg/<crate-name>/v<semver>
   ```

5. Update the `version` field in the workspace root `Cargo.toml` dependency entry for
   the published crate (e.g., from `3.0.0-develop` to `0.1.0`) in a follow-up PR. Do **not**
   remove the `path = "..."` — it ensures workspace builds always use the local copy regardless
   of the published version.

### Standalone-Repository Crate

A crate that lives in its own repository has no `releases/pkg/**` workflow, and may have no
`develop` branch. The differences from a workspace package:

1. Merge the preconditions on the default branch (`main` when there is no `develop`).
2. Publish manually from that merged commit, after the dry run:

   ```sh
   git switch main && git pull --ff-only
   git rev-parse HEAD   # the published commit; record it
   cargo publish
   ```

3. The maintainer creates the signed tag `v<semver>` on **that commit** and pushes it:

   ```sh
   git tag --sign v<semver> <published-commit-sha>
   git push <upstream-remote> refs/tags/v<semver>
   ```

4. Create no GitHub release and no release branch.

### Manual Fallback (workspace package)

If CI is unavailable, publish from the merged commit instead of pushing the release branch:

```sh
git switch --detach <merged-commit-sha>
cargo publish -p <crate-name> --dry-run
cargo publish -p <crate-name>
```

Then tag that commit as in step 4 of the workspace path. Only someone with a crates.io token
for the crate can do this; agents publish only if cargo is authenticated and the maintainer
approved.

### Failure Handling

- **Already published**: crates.io versions are immutable. Do not republish; bump the version.
- **CI failed before publish**: fix on `develop` through a PR, then push a new release branch
  commit for the same version if the version was not published.
- **Tag points to the wrong commit**: do not move a pushed tag; ask the maintainer.

### Real-World Example: A Full Release Cycle

This example shows how independent package publishing works in practice over a typical
release cycle, from development through tracker release.

> The commands below are abbreviated. The steps above are authoritative: push the merged
> commit that carries the version bump, and tag that same commit.

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

## Pointing to This Process from Package Documentation

This document is the single source of truth. Package documentation carries only a pointer and
the package's own differences, never a copy of the steps.

- **Workspace packages**: the package README does not describe releasing and is not changed.
  A workspace README that already describes releasing links to this document instead.
- **Standalone repositories**: the repository's release document (or README) has a short
  "Releasing" section with this text, then lists only its own differences (default branch, no
  workflow, tag name):

  ```markdown
  ## Releasing

  Releases follow the process defined for workspace packages in the Torrust Tracker, which is the
  source of truth:
  [Releasing Workspace Packages](https://github.com/torrust/torrust-tracker/blob/develop/docs/release-process/workspace-packages.md).
  Do not copy its steps here.

  Differences in this repository: <default branch>; <no `releases/pkg/**` workflow>; tags are
  `v<semver>`.
  ```

The link targets `develop` on purpose: readers always see the current process, and a permalink
would go stale.
