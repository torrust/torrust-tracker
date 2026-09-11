---
semantic-links:
  skill-links:
    - update-dependencies
  related-artifacts:
    - .github/skills/dev/maintenance/update-dependencies/SKILL.md
---

<!-- skill-link: update-dependencies -->

# Cargo Dependency Update Pull Request

> Use this template only for the GitHub PR body of a Cargo dependency update. Keep the update, validation, commit, and push process in the [`update-dependencies` skill](../../.github/skills/dev/maintenance/update-dependencies/SKILL.md). Do not commit a populated copy of this template.

## Summary

{State that the workspace lockfile was updated to the latest releases compatible with the supported Rust version. Note any deferred breaking dependency separately.}

## Files/packages touched

- `Cargo.lock`
- {Other changed manifest, source, test, or documentation path, if a small breaking-change rework was required.}

## Validation

- `cargo machete`
- `./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- {Additional focused checks performed for a breaking-change rework, if any.}

## cargo update output

```text
{Replace this placeholder with the complete, unedited contents of .tmp/<timestamp>-cargo-update.txt.}
```
