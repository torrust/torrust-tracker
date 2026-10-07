---
name: reopen-issue
description: Guide for reopening a closed GitHub issue or EPIC whose specification was archived in docs/issues/closed/. Covers deciding whether to reopen or file new work, reopening on GitHub, linking a new subissue, moving the spec back to docs/issues/open/, migrating legacy frontmatter, resetting closure checkpoints, and repairing live references. Use when new work belongs to a closed EPIC or a closed issue regressed or was incomplete. Triggers on "reopen issue", "reopen EPIC", "add subissue to closed EPIC", "unarchive issue spec", or "move spec back to open".
metadata:
  author: torrust
  version: "1.0"
---

# Reopening a Closed Issue or EPIC

This is the inverse of [cleanup-completed-issues](../cleanup-completed-issues/SKILL.md). It was
first used to reopen configuration EPIC #1978 for subissue #2466 (2026-10-07). The EPIC path is
exercised; the single-issue path is derived from it and should be refined on first use.

## Step 1: Decide Whether to Reopen

Reopen only with explicit maintainer approval; reopening changes shared GitHub state.

Reopen when the new work belongs to the original goal and its deliverable is not yet released (for
example, #1978's configuration schema v3.0.0 was active but not published on crates.io). Otherwise
create a new issue or EPIC and reference the closed one.

For a closed single issue, reopen when the closing PR did not meet its acceptance criteria or the
fix regressed before release. A regression after release is a new bug issue.

## Step 2: Prepare the Branch

Update `develop` first, check that it is not behind, and only then create the branch. Chaining
`git fetch`, the behind count, and `git checkout -b` with `&&` continues even when the count is
non-zero.

```bash
UPSTREAM_REMOTE="${UPSTREAM_REMOTE:-torrust}"
git checkout develop
git pull --ff-only "$UPSTREAM_REMOTE" develop
git rev-list --count HEAD.."$UPSTREAM_REMOTE"/develop   # must print 0
git checkout -b <branch>
```

When reopening an EPIC to add a subissue, reuse that subissue's specification branch
(`<issue>-<epic>-<description>-spec`), so the reopen, the subissue spec, and any ADR land in one PR.

## Step 3: Update GitHub

1. When adding a subissue, create it first (see [create-issue](../create-issue/SKILL.md)), so the
   reopen comment can cite it.
2. Reopen with the reason:

   ```bash
   gh issue reopen <number> --repo torrust/torrust-tracker --comment "Reopened to <reason>."
   ```

3. Link a new subissue natively; the REST endpoint needs the child's database `id`, not its number:

   ```bash
   CHILD_ID=$(gh api repos/torrust/torrust-tracker/issues/<child> --jq .id)
   gh api -X POST repos/torrust/torrust-tracker/issues/<epic>/sub_issues -F sub_issue_id="$CHILD_ID"
   ```

## Step 4: Move the Specification Back

```bash
git mv docs/issues/closed/<folder> docs/issues/open/
```

Then update the moved spec:

- **Frontmatter**: set `status` to `planned` or `in-progress`, `spec-path` to the `open/` path, and
  `last-updated-utc` to the current time. Specs archived before the v1 schema must be migrated to
  v1 in `open/` (`schema-version: 1`, `epic: null` where applicable, quoted `last-updated-utc`, no
  trailing slash in `related-artifacts`). Follow the migration checklist in
  [`contrib/dev-tools/checks/frontmatter-validator/README.md`](../../../../../contrib/dev-tools/checks/frontmatter-validator/README.md).
- **Archive-time edits**: revert body paths the archive rewrote to `closed/`, such as the
  "spec drafted in" checkpoint.
- **Checkpoints**: clear the "issue closed and spec moved" and "acceptance criteria reviewed" boxes.
- **Acceptance Verification**: set rows that depended on all work being done back to `TODO`, and
  correct stale counts (for example, the number of linked subissues).
- **Subissues table** (EPIC): add the new row with `TODO`. A `related-artifacts` entry must name a
  tracked file, so add the new child spec path in the commit that adds the spec.
- **Progress Log**: add an entry with the reason and the remaining work.
- **Parent EPIC** (single issue): set its row back to `IN_PROGRESS` and point it at `open/`.

The frontmatter validator checks **staged** content. Stage the edits, not only the rename, before
running it:

```bash
git add -A docs/issues/open/<folder>
cargo run --quiet --package frontmatter-validator --bin frontmatter-validator -- --staged
```

## Step 5: Repair Live References

```bash
rg '<folder>' --glob '!target/**' --glob '!storage/**'
```

- Update live navigational references outside `docs/issues/closed/` (for example, package docs) to
  the `open/` path.
- Leave links inside other closed specs and historical PR review records unchanged. They are
  archived records, the local link checker excludes `docs/issues/closed/`, and the spec returns
  there on closure.

When the reopened issue closes again, archive it with
[cleanup-completed-issues](../cleanup-completed-issues/SKILL.md); its reference search flips the
live references back.

## Step 6: Commit and Open the PR

Commit the reopen separately from the new subissue spec. When the reopened spec references a new
ADR or spec in `related-artifacts`, commit that artifact first, because the validator requires
tracked paths.

```bash
git commit -S -m "docs(issues): [#<number>] reopen <short title> to <reason>"
```

Use `Related to #<number>` in the PR body, never a closing keyword.
