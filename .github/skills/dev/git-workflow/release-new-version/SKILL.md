---
name: release-new-version
description: Guide for releasing a new version of the Torrust Tracker using the standard staging branch, tag, and crate publication workflow. Covers version bump, release commit, staging branch promotion, PR to main, release branch/tag creation, crate publication, and merge-back to develop. Use when asked to "release", "cut a version", "publish a new version", or "create release vX.Y.Z".
metadata:
  author: torrust
  version: "1.0"
---

# Release New Version

Primary reference: [`docs/release_process.md`](../../../../../docs/release_process.md)

## Release Steps (Mandatory Order)

1. Stage `develop` → `staging/main`
2. Create release commit (bump version)
3. PR `staging/main` → `main`
4. Push `main` → `releases/vX.Y.Z`
5. Create signed tag `vX.Y.Z` on that branch
6. Verify deployment workflow + crate publication
7. Create GitHub release
8. Stage `main` → `staging/develop` (merge-back)
9. Bump next dev version, PR `staging/develop` → `develop`

Do not reorder these steps.

## Version Naming Rules

- Version in code: `X.Y.Z` (release) or `X.Y.Z-develop` (development)
- Git tag: `vX.Y.Z`
- Release branch: `releases/vX.Y.Z`
- Staging branches: `staging/main`, `staging/develop`

## Pre-Flight Checklist

Before starting:

- [ ] Clean working tree (`git status`)
- [ ] `develop` branch is up to date with `torrust/develop`
- [ ] All CI checks pass on `develop`
- [ ] Working version in manifests is `X.Y.Z-develop`

## Commands

### 1) Stage develop → staging/main

```bash
git fetch --all
git push --force torrust develop:staging/main
```

### 2) Create Release Commit

```bash
git stash
git switch staging/main
git reset --hard torrust/staging/main
# Edit version in all Cargo.toml files:
#   change X.Y.Z-develop → X.Y.Z
git add -A
git commit -S -m "release: version X.Y.Z"
git push torrust
```

Edit `version` in:

- `Cargo.toml` (workspace)
- All packages under `packages/` that publish crates
- `console/tracker-client/Cargo.toml`
- `contrib/bencode/Cargo.toml`

Also update any internal path dependency `version` constraints.

### 3) PR staging/main → main

Create PR: "Release Version X.Y.Z" (title format)  
Base: `torrust/torrust-tracker:main`  
Head: `staging/main`  
Merge after CI passes.

### 4) Push releases/vX.Y.Z branch

```bash
git fetch --all
git push torrust main:releases/vX.Y.Z
```

### 5) Create Signed Tag

```bash
git switch releases/vX.Y.Z
git reset --hard torrust/releases/vX.Y.Z
git tag --sign vX.Y.Z
git push --tags torrust
```

### 6) Verify Deployment Workflow

Check the
[deployment workflow](https://github.com/torrust/torrust-tracker/actions/workflows/deployment.yaml)
ran successfully and the following crates were published:

- `torrust-tracker-contrib-bencode`
- `torrust-located-error`
- `torrust-tracker-primitives`
- `torrust-clock`
- `torrust-tracker-configuration`
- `torrust-tracker-torrent-repository`
- `torrust-tracker-test-helpers`
- `torrust-tracker`

Crates must be published in dependency order. Each must be indexed on crates.io before the next
publishes.

### 7) Create GitHub Release

Create a release from tag `vX.Y.Z` after the deployment workflow passes.

### 8) Merge-back: Stage main → staging/develop

```bash
git fetch --all
git push --force torrust main:staging/develop
```

### 9) Bump Next Dev Version

```bash
git stash
git switch staging/develop
git reset --hard torrust/staging/develop
# Edit version in all Cargo.toml files:
#   change X.Y.Z → (next)X.Y.Z-develop (e.g. 3.0.0 → 3.0.1-develop)
git add -A
git commit -S -m "develop: bump to version (next)X.Y.Z-develop"
git push torrust
```

Create PR: "Version X.Y.Z was Released"  
Base: `torrust/torrust-tracker:develop`  
Head: `staging/develop`

## Failure Handling

- **Deployment workflow failed**: fix and rerun on same release branch
- **Crate already published**: do not republish; cut a patch release
- **Partial state (tag exists but branch doesn't)**: investigate before proceeding
