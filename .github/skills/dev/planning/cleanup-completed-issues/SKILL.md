---
name: cleanup-completed-issues
description: Guide for cleaning up completed and closed issues in the torrust-tracker project. Covers moving closed issue documentation files from docs/issues/open/ to docs/issues/closed/ and eventually deleting them. Supports single issue cleanup or batch cleanup. Use when cleaning up closed issues, archiving issue docs, or maintaining the docs/issues/ folder. Triggers on "cleanup issue", "archive issue", "move closed issue", "clean completed issues", "delete closed issue", or "maintain issue docs".
metadata:
  author: torrust
  version: "1.2"
---

# Cleaning Up Completed Issues

## Two-Stage Lifecycle

Closed issue specs are **not deleted immediately**. They go through a two-stage lifecycle:

1. **Stage 1 — Archive**: When an issue is closed, move its spec file from `docs/issues/open/` to
   `docs/issues/closed/`. The file stays here as a reference buffer while adjacent issues are
   still in progress.
2. **Stage 2 — Delete**: Once the spec is no longer referenced by active work (typically after
   the next one or two related issues are also closed), delete it permanently.

See [`docs/issues/closed/README.md`](../../../../docs/issues/closed/README.md) for the purpose
of the buffer folder.

Related lifecycle docs:

- Open issue specs: [`docs/issues/open/README.md`](../../../../docs/issues/open/README.md)
- Closed issue buffer: [`docs/issues/closed/README.md`](../../../../docs/issues/closed/README.md)

## When to Archive (Stage 1)

- **After PR merge**: Move the issue file when its PR is merged and the issue is closed.
- **Batch archive**: Periodically move multiple closed issue files during maintenance.
- **Before releases**: Tidy `docs/issues/` before major releases.

## When to Delete (Stage 2)

- The spec is no longer referenced by any open issue or active work.
- The related issue series has progressed far enough that the context is no longer needed.

## Step-by-Step Process

### Step 0: Create a Working Branch

This cleanup task may not have a dedicated GitHub issue. Use a descriptive branch name
without an issue prefix:

```bash
git checkout -b chore/cleanup-completed-issues-from-open
```

If there is a linked issue (e.g., automating this process), prefix the branch accordingly:

```bash
# When automating this process uses a tracking issue
git checkout -b 1774-automate-cleanup-completed-issues
```

### Step 1: Verify Issue is Closed on GitHub

**Single issue:**

```bash
gh issue view {issue-number} --repo torrust/torrust-tracker --json state --jq .state
```

Expected: `CLOSED`

**Batch:**

```bash
for issue in 21 22 23 24; do
  state=$(gh issue view "$issue" --repo torrust/torrust-tracker --json state --jq .state 2>/dev/null || echo "NOT_FOUND")
  echo "$issue: $state"
done
```

### Step 2: Move Issue File to `docs/issues/closed/`

**Single file:**

```bash
git mv docs/issues/open/42-add-peer-expiry-grace-period.md docs/issues/closed/
```

**Directory (multi-file subissue spec):**

```bash
git mv docs/issues/open/42-my-subissue-folder/ docs/issues/closed/
```

**Batch files:**

```bash
git mv docs/issues/open/21-some-old-issue.md \
  docs/issues/open/22-another-old-issue.md \
       docs/issues/closed/
```

Note: `git mv` on a directory moves all files inside it atomically.

### Step 3: Update Frontmatter of Moved Files

After moving, the spec's YAML frontmatter fields must reflect the closed state:

| Field                 | Before                   | After                     |
| --------------------- | ------------------------ | ------------------------- |
| `status`              | `open`, `planned`, etc.  | `done`                    |
| `spec-path`           | `docs/issues/open/...`   | `docs/issues/closed/...`  |
| `last-updated-utc`    | previous date            | current date              |

For directories with multiple files, update at minimum the main `ISSUE.md` plus any
supplementary files whose frontmatter references the `docs/issues/open/` path (e.g.,
`related-artifacts` links to the open spec).

### Step 4: Update Any Parent Epic Spec

If the closed issue was a subissue of an EPIC, update the epic's spec to reflect the
new `docs/issues/closed/` path and `DONE` status in its subissue table.

Example: if `docs/issues/open/EPIC.md` has a table row referencing a subissue at
`docs/issues/open/...` with `TODO` status, update both the path and status after archiving.

### Step 5: Commit and Push

```bash
# Single issue
git commit -S -m "chore(issues): archive closed issue #42 spec to docs/issues/closed"

# Batch
git commit -S -m "chore(issues): archive closed issue specs #21, #22, #23 to docs/issues/closed"

git push {your-fork-remote} {branch}
```

### Step 6 (Stage 2): Delete When No Longer Needed

```bash
git rm docs/issues/closed/42-add-peer-expiry-grace-period.md
git commit -S -m "chore(issues): remove closed issue #42 spec (no longer referenced)"
```

## Determining File Placement

| Condition                               | Action                        |
| --------------------------------------- | ----------------------------- |
| Issue still open                        | Keep in `docs/issues/open/`   |
| Issue closed, related work still active | Move to `docs/issues/closed/` |
| Issue closed, no longer referenced      | Delete permanently            |
