---
semantic-links:
  skill-links:
    - release-new-version
    - write-markdown-docs
  related-artifacts:
    - docs/release_process.md
    - .github/workflows/deployment.yaml
    - Cargo.toml
---

# Torrust Tracker Release Process (v2.2.2)

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

### Container Image Tags

The `releases/v[semantic version]` branch name and Git tag retain the conventional `v` prefix,
but Docker image version tags do not. The release container workflow publishes these tags:

| Release type | Source version   | Docker image tags             |
| ------------ | ---------------- | ----------------------------- |
| Stable       | `v3.0.0`         | `3.0.0`, `3.0`, `3`, `latest` |
| Prerelease   | `v3.1.0-rc.1`    | `3.1.0-rc.1`                  |
| Development  | `develop` branch | `develop`                     |
| Development  | `main` branch    | `main`                        |

`latest` always identifies the newest stable release. The `3` and `3.0` tags are also mutable
and advance with later stable releases in their compatible version lines. Deployments that must
be repeatable should use a full version tag such as `3.0.0`, or preferably an immutable image
digest. Existing Docker Hub tags with a `v` prefix are retained as historical artifacts but are
not published for new releases.

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
