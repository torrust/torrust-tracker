---
name: cleanup-completed-issues
description: Guide for cleaning up completed and closed issues in the torrust-tracker project. Covers moving closed issue documentation files from docs/issues/open/ to docs/issues/closed/ and eventually deleting them. Supports single issue cleanup or batch cleanup. Use when cleaning up closed issues, archiving issue docs, or maintaining the docs/issues/ folder. Triggers on "cleanup issue", "archive issue", "move closed issue", "clean completed issues", "delete closed issue", or "maintain issue docs".
metadata:
  author: torrust
  version: "1.1"
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

```bash
# Single issue
git mv docs/issues/open/42-add-peer-expiry-grace-period.md docs/issues/closed/

# Batch
git mv docs/issues/open/21-some-old-issue.md \
  docs/issues/open/22-another-old-issue.md \
       docs/issues/closed/
```

### Step 3: Commit and Push

```bash
# Single issue
git commit -S -m "chore(issues): archive closed issue #42 spec to docs/issues/closed"

# Batch
git commit -S -m "chore(issues): archive closed issue specs #21, #22, #23 to docs/issues/closed"

git push {your-fork-remote} {branch}
```

### Step 4 (Stage 2): Delete When No Longer Needed

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
