#  Torrust Tracker Release Process (v2.2.1)

## Version:
> **The `[semantic version]` is bumped according to releases, new features, and breaking changes.**
>
> *The `develop` branch uses the (semantic version) suffix `-develop`.*

## Process:

### 1. The `develop` branch is ready for a release.
The `develop` branch should have the version `[semantic version]-develop` that is ready to be released.

### 2. Stage `develop` HEAD for merging into the `main` branch:

```sh
git fetch --all
git push --force torrust develop:staging/main
```

### 3. Create Release Commit:

```sh
git stash
git switch staging/main
# change `[semantic version]-develop` to `[semantic version]`.
git add -A
git commit -m "release: version [semantic version]"
git push torrust
```

### 4. Create and Merge Pull Request from `staging/main` into `main` branch.

Pull request title format: "Release Version `[semantic version]`".

This pull request merges the new version into the `main` branch.

### 5. Push new version from `main` HEAD to `releases/v[semantic version]` branch:

```sh
git fetch --all
git push torrust main:releases/v[semantic version]
```

> **Check that the deployment is successful!**

### 6. Create Release Tag:

```sh
git switch releases/v[semantic version]
git tag --sign v[semantic version]
git push --tags torrust
```

### 7. Create Release on Github from Tag.
This is for those who wish to download the source code.

### 8. Stage `main` HEAD for merging into the `develop` branch:
Merge release back into the develop branch.

```sh
git fetch --all
git push --force torrust main:staging/develop
```
### 9. Create Comment that bumps next development version:

```sh
git stash
git switch staging/main
# change `[semantic version]` to `(next)[semantic version]-develop`.
git add -A
git commit -m "develop: bump to version (next)[semantic version]-develop"
git push torrust
```

### 10. Create and Merge Pull Request from `staging/develop` into `develop` branch.

Pull request title format: "Version `[semantic version]` was Released".

This pull request merges the new release into the `develop` branch and bumps the version number.
