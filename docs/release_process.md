#  Torrust Tracker Release Process (draft)

The purpose of this document is to describe the release process.

## Overview

Torrust Tracker is published in this order:

1. `develop` branch is ready for publishing.
2. create `release: version (semantic version)` commit.
3. push release commit to `main` branch.
4. check all status checks succeed for `main` branch.
5. push `main` branch to `releases\v(semantic version)` branch.
6. check all status checks success for `releases\v(semantic version)` branch.
7. create signed `v(semantic version)` tag.
8. create github release from `v(semantic version)` tag.
9. merge `main` branch into `develop` branch.

- At step `1.`, `develop` is automatically published to `dockerhub`.
- At step `3.`, `main` is automatically published to `dockerhub`.
- At step `5.`, `releases\v(semantic version)` is automatically published to `dockerhub` and `crate.io`.

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
