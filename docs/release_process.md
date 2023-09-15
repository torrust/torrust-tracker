#  Torrust Tracker Release Process (draft 2)

The purpose of this document is to describe the release process.

## Overview

Torrust Tracker is published according to this protocol:

0. After release create new pull request into `develop` branch:

- The `develop` branch has the (semantic version) suffix `-develop`.
- The version is bumped according to releases, new features, and breaking changes.

- [ ] `develop` is ready for branching for a release.
- [ ] force-push develop to `staging` branch.
- [ ] commit `release: version (semantic version)`, that removes the `-develop` suffix.
- [ ] create pull request to merge `staging` into `main` branch.
- [ ] check all status checks succeed for `main` branch.
- [ ] push `main` branch to `releases/(semantic version)` branch.
- [ ] check all status checks success for `releases/(semantic version)` branch.
- [ ] create signed `v(semantic version)` tag from `releases/(semantic version) HEAD`.
- [ ] create github release from `v(semantic version)` tag.
- [ ] merge the `main` branch back into `develop` branch, assuring that the (semantic version) has the suffix `-develop`.

- At step `1.`, `develop` is automatically published to `dockerhub`.
- At step `3.`, `main` is automatically published to `dockerhub`.
- At step `6.`, `releases\v(semantic version)` is automatically published to `dockerhub` and `crate.io`.

## Development Branch

The `develop` branch, the default branch for the repository is automatically published to dockerhub with the `develop` label. This process happens automatically when a pull request is merged in, and the `container.yaml` workflow is triggered.

## Main Branch

The `main` branch is the staging branch for releases.

A release commit needs to be made that prepares the repository for the release, this commit should include:

- Changing the semantic version.
- Finalizing the release notes and changelog.

The title of the commit should be: `release: version (semantic version)`.

This commit should be committed upon the head of the development branch, and pushed to the `main` branch.

Once the release has succeeded, the `main` branch should be merged back into the `develop` branch.

## Releases Branch

According to the patten `releases/v(semantic version)`, the `main` branch head is published to here to trigger the deployment workflows.

The repository deployment environment for crates.io is only available for the `releases/**/*` patten of branches.

Once the publishing workflows have succeeded; we can make the git-tag.

## Release Tag

Create a Signed Tag with a short message in the form `v(semantic version)` and push it to the repository.

## Github Release

From the newly published tag, create a Github Release using the web-interface.


## Merge back into development branch

After this is all successful, the `main` branch should be merged into the `develop` branch.
