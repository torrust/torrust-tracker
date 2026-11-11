---
name: release-workspace-package
description: Guide for releasing a single workspace package, or a crate that lives in its own standalone Torrust repository, to crates.io. Drives the process documented in docs/release-process/workspace-packages.md. Use when asked to "release a package", "publish a crate", "publish torrust-<crate> to crates.io", "tag a package release", or "release torrust-server-lib".
metadata:
  author: torrust
  version: "1.0"
---

# Release a Workspace Package

The steps live in one place, the source of truth:
[`docs/release-process/workspace-packages.md`](../../../../../docs/release-process/workspace-packages.md).
This skill does not copy them. Read that document first and follow it in order. To release the
tracker binary instead, use [`release-new-version`](../release-new-version/SKILL.md).

## Decide the Variant

| The crate lives in                      | Variant                                      | Tag                      |
| --------------------------------------- | -------------------------------------------- | ------------------------ |
| the `torrust-tracker` workspace         | "Workspace Package" (`releases/pkg/**` flow) | `pkg/<crate-name>/v<semver>` |
| its own repository (for example `torrust-server-lib`) | "Standalone-Repository Crate"  | `v<semver>`              |

## Agent Checklist

- [ ] Identify the crate, the variant and the new version; confirm the package has its own
      explicit `version`.
- [ ] Prepare the version-bump PR (version, changelog entry) and the preconditions from the
      document, including the dry run.
- [ ] Do the agent steps of "Who Does What" in the document only. Publish only if cargo is
      authenticated and the maintainer approved.
- [ ] Hand the maintainer the exact commit SHA and the commands for the branch push and the
      signed tag, as the document lists them. Agents never push to upstream.
- [ ] Create no GitHub release for a package.

## Rules

- Publish only from the merged commit; tag that exact commit.
- Never move or delete a pushed tag; never republish a published version.
- If the document and this skill disagree, the document wins; fix this skill.

## Skill Links

Review this skill when changing:

- `docs/release-process/workspace-packages.md` — the process this skill drives.
- `.github/workflows/deployment-packages.yaml` — the workflow that publishes a workspace package.
