# Follow-up issue text for torrust/torrust-server-lib

Ready for the maintainer to create (T6 of torrust/torrust-tracker#2808). Nothing has been created.

## Title

Backfill release tags for versions <= 0.5.0 and replace the duplicated release checklist with a pointer

## Body

### Background

`torrust-server-lib` 0.6.0 was published manually and tagged `v0.6.0`. Releases 0.1.0 to 0.5.0 have
no tags. Separately, `docs/release_process.md` still ends with a three-step checklist (version-bump
PR, publish, signed tag) that repeats the steps of the source of truth, which is now
[Releasing Workspace Packages](https://github.com/torrust/torrust-tracker/blob/develop/docs/release-process/workspace-packages.md)
in the Torrust Tracker. The old anchor `docs/release_process.md#publishing-a-workspace-package` keeps
working there, because the index document redirects to the new document.

### 1. Backfill signed tags `v<semver>` (maintainer pushes the tags)

Candidate commits, from the commit that bumped `version` in `Cargo.toml`. Verify each against the
crates.io publish date before tagging (the dates below come from `crates.io/api/v1/crates/torrust-server-lib/versions`
and `git log`):

| Version | Candidate commit | Commit date | crates.io `created_at` | Note                                                                |
| ------- | ---------------- | ----------- | ---------------------- | ------------------------------------------------------------------- |
| 0.1.0   | `86e233a`        | 2026-06-19  | 2026-06-19             | Also `b87f6fa` (same version, same day): confirm which was published |
| 0.2.0   | `d17fdb1`        | 2026-07-31  | 2026-07-31             |                                                                     |
| 0.3.0   | `ea333ec`        | 2026-09-16  | 2026-09-16             |                                                                     |
| 0.4.0   | `f3690ed`        | 2026-10-26  | 2026-10-26             |                                                                     |
| 0.5.0   | `2df974f`        | 2026-11-06  | 2026-11-06             |                                                                     |

Tag only the versions whose published commit is verified (for example by comparing
`cargo package` contents or the crates.io `.crate` checksum with the commit). Leave unverified
versions untagged and say so here.

### 2. Replace the duplicated checklist

In `docs/release_process.md`, delete the "Checklist" section and keep the pointer plus the
differences (no `develop` branch, no workflow, tag `v<semver>`), as the "Standalone repositories"
text in the Tracker document prescribes.

### Acceptance criteria

- [ ] Signed tags `v0.1.0` to `v0.5.0` exist for the verified versions only, each on the published commit.
- [ ] `docs/release_process.md` has no copy of the release steps.
