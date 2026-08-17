---
name: cleanup-completed-issues
description: Guide for archiving closed issue specification files from docs/issues/open/ to docs/issues/closed/. Covers verifying closure on GitHub, moving files, updating frontmatter, auditing and repairing affected documentation links, creating a branch, and opening a PR. Permanent deletion of closed specs is not automated — the user must explicitly request it. Use when cleaning up closed issue specs, archiving issue docs, or maintaining the docs/issues/ folder. Triggers on "cleanup issue", "archive issue", "move closed issue", "clean completed issues", or "maintain issue docs".
metadata:
  author: torrust
  version: "1.6"
---

# Cleaning Up Completed Issues

## Lifecycle

Closed issue specs follow this lifecycle:

1. **Archive** (automated by this skill): When an issue is closed, move its spec file from
   `docs/issues/open/` to `docs/issues/closed/`. The file stays in the closed buffer as a
   reference for ongoing and upcoming work.
2. **Permanent deletion** (user-driven): If the user wants specs permanently deleted, they
   will explicitly ask for it. This skill does not automate deletion.

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

Start from an up-to-date `develop`:

```bash
UPSTREAM_REMOTE="${UPSTREAM_REMOTE:-torrust}"
git checkout develop
git pull --ff-only "$UPSTREAM_REMOTE" develop
git checkout -b chore/cleanup-completed-issues
```

> **Edge case — branch already exists**: If a branch named `chore/cleanup-completed-issues`
> already exists (e.g., from a previous aborted run), first delete it, then recreate:
>
> ```bash
> git branch -D chore/cleanup-completed-issues
> git checkout develop
> git pull --ff-only "$UPSTREAM_REMOTE" develop
> git checkout -b chore/cleanup-completed-issues
> ```
>
> This ensures the branch is based on the latest `develop` and carries no stale commits
> from the prior attempt. If the branch has already been pushed to a remote, you may also
> need to delete it there:
>
> ```bash
> git push "$FORK_REMOTE" --delete chore/cleanup-completed-issues
> ```

### Step 0.5: Discover Archive Candidates in Both Open-Spec Formats (Mandatory)

Always scan both issue spec formats under `docs/issues/open/`:

1. **Directory specs** (multi-file issue folders)
2. **Single-file specs** (`*.md` files except `README.md` and `AGENTS.md`)

Do not proceed with archival if only one format was scanned.

```bash
echo "[open issue folders]"
find docs/issues/open -maxdepth 1 -mindepth 1 -type d -exec basename {} \; | sort

echo "[open single-file specs]"
find docs/issues/open -maxdepth 1 -type f -name '*.md' \
  ! -name 'README.md' ! -name 'AGENTS.md' -exec basename {} \; | sort
```

Optional unified number extraction for batch state verification:

```bash
{
  find docs/issues/open -maxdepth 1 -mindepth 1 -type d -exec basename {} \;
  find docs/issues/open -maxdepth 1 -type f -name '*.md' \
    ! -name 'README.md' ! -name 'AGENTS.md' -exec basename {} \;
} | sed -E 's/^([0-9]+).*/\1/' | sort -n | uniq
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

| Field              | Before                  | After                    |
| ------------------ | ----------------------- | ------------------------ |
| `status`           | `open`, `planned`, etc. | `done`                   |
| `spec-path`        | `docs/issues/open/...`  | `docs/issues/closed/...` |
| `last-updated-utc` | previous date           | current date             |

For directories with multiple files, update at minimum the main `ISSUE.md` plus any
supplementary files whose frontmatter references the `docs/issues/open/` path (e.g.,
`related-artifacts` links to the open spec). For supplementary docs without existing
frontmatter, add a minimal block with `spec-path`, `last-updated-utc`, and a
`semantic-links` section linking back to the parent issue spec.

Also check the spec's **Workflow Checkpoints** section and tick any checkboxes that
reflect completed work (manual verification, acceptance criteria review, etc.) based
on the actual content of the spec body. Add a progress log entry documenting the
archival action.

### Step 4: Audit and Repair Documentation References (Mandatory)

An archive move invalidates every live reference to the old `docs/issues/open/...` path.
After updating the moved documents' own frontmatter, search the repository for each old path
and update all **current** documentation links and references to the new `docs/issues/closed/...`
location. This includes:

- parent EPIC subissue tables and their frontmatter `semantic-links`;
- active issue specs that name the archived issue as a prerequisite, dependency, or related
  artifact;
- ADR frontmatter and body links; and
- frontmatter in moved supplementary artifacts (`evidence.md`, manual-verification records,
  and similar documents) that references the moved primary spec.

When modifying an affected document that has YAML frontmatter, keep its metadata current:

- preserve its existing `status` unless its actual lifecycle state changed;
- update any changed `spec-path` or `semantic-links.related-artifacts` value; and
- set `last-updated-utc` to the current date when that field exists.

Do not rewrite immutable historical records (for example, past PR review summaries) merely
because they accurately record the path that existed at the time. Update them only when they
function as a live navigational reference.

For each archived issue, search for the old path before finishing. For a single-file spec:

```bash
rg 'docs/issues/open/42-add-peer-expiry-grace-period\.md' \
  --glob '!target/**' --glob '!storage/**'
```

For a folder spec, search its folder prefix:

```bash
rg 'docs/issues/open/42-my-subissue-folder' \
  --glob '!target/**' --glob '!storage/**'
```

The remaining results must be either corrected or deliberately retained historical records.

### Step 5: Update Any Parent Epic Spec

If the closed issue was a subissue of an EPIC, update the epic's spec to reflect the
new `docs/issues/closed/` path and `DONE` status in its subissue table.

Example: if `docs/issues/open/EPIC.md` has a table row referencing a subissue at
`docs/issues/open/...` with `TODO` status, update both the path and status after archiving.

The parent EPIC is also an affected document under Step 4: update its frontmatter
`semantic-links` and `last-updated-utc` when applicable.

### Step 6: Validate and Commit

Before committing, confirm that every changed Markdown frontmatter block is valid YAML and that
each archived primary issue spec has `status: done`, a `spec-path` below `docs/issues/closed/`,
and a current `last-updated-utc`. Also run `git diff --check` after staging.

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

### Step 7: Push and Open a Pull Request

```bash
FORK_REMOTE="${FORK_REMOTE:-josecelano}"
git push "$FORK_REMOTE" chore/cleanup-completed-issues
```

Open a PR targeting `develop`:

```bash
gh pr create \
  --repo torrust/torrust-tracker \
  --base develop \
  --head "${FORK_REMOTE}:chore/cleanup-completed-issues" \
  --title "chore(issues): archive closed issue #${N} spec to docs/issues/closed" \
  --body "Archives the spec for issue #${N} (closed on GitHub) from \`docs/issues/open/\` to \`docs/issues/closed/\`.

- Verified issue #${N} is \`CLOSED\` on GitHub
- Updated frontmatter (\`status: done\`, \`spec-path\`, \`last-updated-utc\`)
- Updated workflow checkboxes where applicable
- Pre-commit hooks passed"
```
