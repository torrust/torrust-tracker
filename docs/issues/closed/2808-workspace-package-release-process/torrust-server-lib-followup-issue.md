---
spec-path: docs/issues/closed/2808-workspace-package-release-process/torrust-server-lib-followup-issue.md
last-updated-utc: "2026-11-11 17:26"
semantic-links:
  related-artifacts:
    - docs/issues/closed/2808-workspace-package-release-process/ISSUE.md
---

# Follow-up issue text for torrust/torrust-server-lib

Created as [torrust/torrust-server-lib#9](https://github.com/torrust/torrust-server-lib/issues/9) (T6 of torrust/torrust-tracker#2808). The text below is the issue body as created, with the published commits verified from the `.cargo_vcs_info.json` of each crate on crates.io. The candidate commits first listed here (`86e233a`, `ea333ec`, `2df974f`) were wrong or ambiguous for 0.1.0, 0.3.0 and 0.5.0.

## Title

Backfill release tags for 0.1.0 to 0.5.0 and replace the duplicated release checklist with a pointer

## Body

## Background

`torrust-server-lib` 0.6.0 was published manually and tagged `v0.6.0` on the published commit. Releases 0.1.0 to 0.5.0 have no tags. Separately, `docs/release_process.md` still ends with a three-step checklist (version-bump PR, publish, signed tag) that repeats the steps of the source of truth, now [Releasing Workspace Packages](https://github.com/torrust/torrust-tracker/blob/develop/docs/release-process/workspace-packages.md) in the Torrust Tracker (torrust/torrust-tracker#2808). The old anchor `docs/release_process.md#publishing-a-workspace-package` keeps working there, because the index document redirects to the new document.

## 1. Backfill signed tags `v<semver>` (the maintainer pushes the tags)

Each published crate records the commit it was built from in `.cargo_vcs_info.json`. The table below comes from downloading each `.crate` from `static.crates.io` and reading that file; each commit exists on `main`.

| Version | Published commit (`.cargo_vcs_info.json`) | Commit date | Subject                                                                    |
| ------- | ----------------------------------------- | ----------- | -------------------------------------------------------------------------- |
| 0.1.0   | `b87f6fa927ff1d7bb23d906a8011a2da451fcb41` | 2026-06-19  | chore: fix CI - add yamllint config, fix TOML formatting, add CI badge     |
| 0.2.0   | `d17fdb1989cfe8dda989c63950920ce767b55191` | 2026-07-31  | feat(registry)!: add runtime service metadata                              |
| 0.3.0   | `2b3f8ec6e20a81fcd0c25dcc1c4e3a29fe4fea17` | 2026-09-16  | Merge pull request #1 from torrust/2234-token-aware-shutdown-wait          |
| 0.4.0   | `f3690ed2cac700870c4825fdf63a102d9029fc51` | 2026-10-26  | feat(registry): add Registar::deregister                                   |
| 0.5.0   | `0ad8ca094d2a32d111c7c21c57bfb8d3b15ebfa5` | 2026-11-06  | Merge torrust/torrust-server-lib#4: feat(signals): deprecate legacy shutdown API |

`v0.6.0` already points to `2dd04f494722b126b1dba8a65006cb4dfa335384`, which matches the commit recorded in the 0.6.0 crate.

Note: the earlier guess from commit history (the commit that bumped `version`) was wrong for 0.3.0 and 0.5.0, where the published commit is the merge commit, and ambiguous for 0.1.0 (`86e233a` or `b87f6fa`). Use the table above.

Tag command, per version (signed, on the published commit):

```sh
git tag --sign v<semver> <published-commit>
git push <upstream-remote> refs/tags/v<semver>
```

## 2. Replace the duplicated checklist

In `docs/release_process.md`, delete the "Checklist" section and keep the pointer plus the differences (no `develop` branch, no workflow, tag `v<semver>`), as the "Pointing to This Process from Package Documentation" section of [Releasing Workspace Packages](https://github.com/torrust/torrust-tracker/blob/develop/docs/release-process/workspace-packages.md) prescribes.

## Acceptance criteria

- [ ] Signed tags `v0.1.0` to `v0.5.0` exist, each on the published commit in the table.
- [ ] `docs/release_process.md` has no copy of the release steps.
