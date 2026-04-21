---
name: cleanup-completed-issues
description: Guide for cleaning up completed and closed issues in the torrust-tracker project. Covers removing issue documentation files from docs/issues/ and committing the cleanup. Supports single issue cleanup or batch cleanup. Use when cleaning up closed issues, removing issue docs, or maintaining the docs/issues/ folder. Triggers on "cleanup issue", "remove issue", "clean completed issues", "delete closed issue", or "maintain issue docs".
metadata:
  author: torrust
  version: "1.0"
---

# Cleaning Up Completed Issues

## When to Clean Up

- **After PR merge**: Remove the issue file when its PR is merged
- **Batch cleanup**: Periodically clean up multiple closed issues during maintenance
- **Before releases**: Tidy documentation before major releases

## Cleanup Approaches

### Option 1: Single Issue Cleanup (Recommended)

1. Verify the issue is closed on GitHub
2. Remove the issue file from `docs/issues/`
3. Commit and push changes

### Option 2: Batch Cleanup

1. List all issue files in `docs/issues/`
2. Check status of each issue on GitHub
3. Remove all closed issue files
4. Commit and push with a descriptive message

## Step-by-Step Process

### Step 1: Verify Issue is Closed on GitHub

**Single issue:**

```bash
gh issue view {issue-number} --json state --jq .state
```

Expected: `CLOSED`

**Batch:**

```bash
for issue in 21 22 23 24; do
  state=$(gh issue view "$issue" --json state --jq .state 2>/dev/null || echo "NOT_FOUND")
  echo "$issue:$state"
done
```

### Step 2: Remove Issue Documentation File

```bash
# Single issue
git rm docs/issues/42-add-peer-expiry-grace-period.md

# Batch
git rm docs/issues/21-some-old-issue.md \
       docs/issues/22-another-old-issue.md
```

### Step 3: Commit and Push

```bash
# Single issue
git commit -S -m "chore(issues): remove closed issue #42 documentation"

# Batch
git commit -S -m "chore(issues): remove documentation for closed issues #21, #22, #23"

git push {your-fork-remote} {branch}
```

## Determining If an Issue File Should Stay

Keep issue files when:

- The issue is still open
- The PR is open (still being worked on)
- The specification is referenced from other active docs

Remove issue files when:

- The issue is **closed**
- The implementing PR is **merged**
- The file is no longer referenced by active work
