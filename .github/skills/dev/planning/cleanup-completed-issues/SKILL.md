---
name: cleanup-completed-issues
description: Guide for archiving closed issue specification files from docs/issues/open/ to docs/issues/closed/. Covers verifying closure on GitHub, moving files, updating frontmatter, creating a branch, and opening a PR. Does NOT cover permanent deletion of closed specs — that is a manual user-driven action. Use when cleaning up closed issue specs, archiving issue docs, or maintaining the docs/issues/ folder. Triggers on "cleanup issue", "archive issue", "move closed issue", "clean completed issues", or "maintain issue docs".
metadata:
  author: torrust
  version: "1.3"
---

# Cleaning Up Completed Issues

## Lifecycle

Closed issue specs follow a single automated stage:

1. **Archive**: When an issue is closed, move its spec file from `docs/issues/open/` to
   `docs/issues/closed/`. The file stays in the closed buffer as a reference.
2. **Permanent deletion**: Not part of this skill. If the user wants specs permanently
   deleted, they will explicitly ask for it.

See [`docs/issues/closed/README.md`](../../../../docs/issues/closed/README.md) for the purpose
of the closed buffer folder.

Related lifecycle docs:

- Open issue specs: [`docs/issues/open/README.md`](../../../../docs/issues/open/README.md)
- Closed issue buffer: [`docs/issues/closed/README.md`](../../../../docs/issues/closed/README.md)

## When to Archive

- **After PR merge**: Move the issue file when its PR is merged and the issue is closed on GitHub.
- **Batch archive**: Periodically move multiple closed issue files during maintenance.
- **Before releases**: Tidy `docs/issues/` before major releases.

## Prerequisites

- GitHub CLI (`gh`) must be authenticated and have access to the `torrust/torrust-tracker` repository.

## Step-by-Step Process

### Step 0: Create a Working Branch (Mandatory)

Always create a new branch for this work. Never commit directly to `develop`.

```bash
git checkout develop
git pull torrust develop
git checkout -b chore/cleanup-completed-issues
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

**Directory (multi-file subissue spec):**

```bash
git mv docs/issues/open/42-my-subissue-folder/ docs/issues/closed/
```

**Single file:**

```bash
git mv docs/issues/open/42-add-peer-expiry-grace-period.md docs/issues/closed/
```

**Batch files:**

```bash
git mv docs/issues/open/21-some-old-issue.md \
  docs/issues/open/22-another-old-issue.md \
  docs/issues/closed/
```

Note: `git mv` on a directory moves all files inside it atomically.

### Step 3: Update Frontmatter of Moved Files

After moving, update the spec's YAML frontmatter to reflect the closed state:

| Field                 | Before                   | After                     |
| --------------------- | ------------------------ | ------------------------- |
| `status`              | `open`, `planned`, etc.  | `done`                    |
| `spec-path`           | `docs/issues/open/...`   | `docs/issues/closed/...`  |
| `last-updated-utc`    | previous date            | current date              |

For directories with multiple files, update at minimum the main `ISSUE.md` plus any
supplementary files whose frontmatter references the `docs/issues/open/` path (e.g.,
`related-artifacts` links to the open spec).

Also check the spec's **Workflow Checkpoints** section and tick any checkboxes that
reflect completed work (manual verification, acceptance criteria review, etc.) based
on the actual content of the spec body. Add a progress log entry documenting the
archival action.

### Step 4: Update Any Parent Epic Spec

If the closed issue was a subissue of an EPIC, update the epic's spec to reflect the
new `docs/issues/closed/` path and `DONE` status in its subissue table.

Example: if `docs/issues/open/EPIC.md` has a table row referencing a subissue at
`docs/issues/open/...` with `TODO` status, update both the path and status after archiving.

### Step 5: Commit

```bash
# Single issue
git commit -S -m "chore(issues): archive closed issue #42 spec to docs/issues/closed"

# Batch
git commit -S -m "chore(issues): archive closed issue specs #21, #22, #23 to docs/issues/closed"
```

Run the pre-commit hooks before finishing:

```bash
./contrib/dev-tools/git/hooks/pre-commit.sh
```

### Step 6: Push and Open a Pull Request

```bash
git push {your-fork-remote} chore/cleanup-completed-issues
```

Open a PR targeting `develop`:

```bash
gh pr create \
  --repo torrust/torrust-tracker \
  --base develop \
  --head {your-fork-remote}:chore/cleanup-completed-issues \
  --title "chore(issues): archive closed issue #{N} spec to docs/issues/closed" \
  --body "Archives the spec for issue #${N} (closed on GitHub) from \`docs/issues/open/\` to \`docs/issues/closed/\`.

- Verified issue #${N} is \`CLOSED\` on GitHub
- Updated frontmatter (\`status: done\`, \`spec-path\`, \`last-updated-utc\`)
- Updated workflow checkboxes where applicable
- Pre-commit hooks passed"
