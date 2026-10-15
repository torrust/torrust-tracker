---
name: cleanup-completed-issues
description: Guide for archiving closed issue specification files from docs/issues/open/ to docs/issues/closed/. Covers verifying closure on GitHub, moving files, updating frontmatter, auditing and repairing affected documentation links, creating a branch, and opening a PR. Permanent deletion of closed specs is not automated — the user must explicitly request it. Also covers a read-only periodic audit that checks docs/issues/open/ holds only open issues. Use when cleaning up closed issue specs, archiving issue docs, auditing docs/issues/, or maintaining the docs/issues/ folder. Triggers on "cleanup issue", "archive issue", "move closed issue", "clean completed issues", "audit issue specs", or "maintain issue docs".
metadata:
  author: torrust
  version: "1.12"
---

# Cleaning Up Completed Issues

## Lifecycle

Closed issue specs follow this lifecycle:

1. **Archive** (automated by this skill): When an issue is closed, move its spec file from
   `docs/issues/open/` to `docs/issues/closed/`. The file stays in the closed buffer as a
   reference for ongoing and upcoming work.
2. **Permanent deletion** (user-driven): If the user wants specs permanently deleted, they
   will explicitly ask for it. This skill does not automate deletion.

See [`docs/issues/closed/README.md`](../../../../../docs/issues/closed/README.md) for the purpose
of the closed buffer folder.

Related lifecycle docs:

- Open issue specs: [`docs/issues/open/README.md`](../../../../../docs/issues/open/README.md)
- Closed issue buffer: [`docs/issues/closed/README.md`](../../../../../docs/issues/closed/README.md)
- Reopening an archived issue or EPIC: [reopen-issue](../reopen-issue/SKILL.md)
  <!-- skill-link: reopen-issue -->

## When to Archive

- **After PR merge**: Move the issue file when its PR is merged and the issue is closed on GitHub.
- **Batch archive**: Periodically move multiple closed issue files during maintenance.
- **Before releases**: Tidy `docs/issues/` before major releases.
- **Periodically**: Run the [Periodic Audit](#periodic-audit) to catch specs whose closing
  pull request was forgotten.

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

### Step 0.5: Discover Archive Candidates (Mandatory)

All new issue specifications use directories. Scan directory specs under
`docs/issues/open/` and include legacy single-file specs while they remain:

```bash
echo "[open issue folders]"
find docs/issues/open -maxdepth 1 -mindepth 1 -type d -exec basename {} \; | sort

echo "[legacy open single-file specs]"
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

### Step 0.6: Reconcile Drafts That Already Have a GitHub Issue (Mandatory)

During issue-document maintenance, inspect `docs/issues/drafts/` for specifications whose
frontmatter has a non-null `github-issue`. A draft must not remain in `drafts/` once it is linked
to a GitHub issue:

- if the GitHub issue is open, move the spec to `docs/issues/open/` using the assigned issue number;
- if the GitHub issue is closed, include the spec in this archive operation and move it directly to
  `docs/issues/closed/`; and
- update `status`, `spec-path`, `related-pr`, and workflow checkpoints to match the recorded state.

Use the GitHub state check in Step 1 before choosing either destination. Do not assume a draft is
unopened merely because it is still filed below `docs/issues/drafts/`.

### Step 0.7: Block EPIC Closure While Draft Subissues Remain (Mandatory)

Do not close or archive an EPIC merely because all currently created GitHub child issues are
closed. Before treating an EPIC as a closure candidate, run this check after reconciling linked
drafts in Step 0.6. It detects both the EPIC-number-prefixed draft-directory convention and
explicit parent declarations or links:

```bash
EPIC_NUMBER=1669
{
  find docs/issues/drafts -maxdepth 1 -mindepth 1 -type d -printf '%p\n' | rg "/${EPIC_NUMBER}(-|$)"
  rg -l -i "(^epic: ${EPIC_NUMBER}$|epic.*\\[#${EPIC_NUMBER}\\]|\\[#${EPIC_NUMBER}\\].*epic)" \
    docs/issues/drafts --glob '*.md'
} | sort -u
```

If it finds any draft specifications, the EPIC remains open. Do not close its GitHub issue or move
its specification to `docs/issues/closed/` until every listed draft has been resolved: create or
link its GitHub issue and move the draft according to Step 0.6, or explicitly remove its parent
EPIC relationship because it no longer belongs there. Record that resolution in the affected
documents before rerunning this check.

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

### Step 2: Move Issue Specification to `docs/issues/closed/`

**Directory specification:**

```bash
git mv docs/issues/open/42-my-subissue-folder/ docs/issues/closed/
```

**Legacy single-file specification:**

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

For directories with multiple files, update at minimum the main `ISSUE.md` or
`EPIC.md` plus any supplementary files whose frontmatter references the
`docs/issues/open/` path (e.g., `related-artifacts` links to the open spec). For
supplementary docs without existing
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

For a folder spec, search its folder name alone, not the full `docs/issues/open/...` path, so
relative links such as `../../open/42-my-subissue-folder/evidence.md` are found too:

```bash
rg '42-my-subissue-folder' \
  --glob '!target/**' --glob '!storage/**'
```

The local link checker excludes `docs/issues/closed/`, so a stale link in an already archived spec
is not reported by `linter all`; this search is the only check for it.

The remaining results must be either corrected or deliberately retained historical records.

### Step 5: Update Any Parent Epic Spec

If the closed issue was a subissue of an EPIC, update the epic's spec to reflect the
new `docs/issues/closed/` path and `DONE` status in its subissue table.

Example: if a parent `docs/issues/open/<epic-slug>/EPIC.md` has a table row
referencing a subissue at `docs/issues/open/...` with `TODO` status, update both
the path and status after archiving.

The parent EPIC is also an affected document under Step 4: update its frontmatter
`semantic-links` and `last-updated-utc` when applicable.

### Step 6: Validate and Commit

Before committing, confirm that every changed Markdown frontmatter block is valid YAML and that
each archived primary issue spec has `status: done`, a `spec-path` below `docs/issues/closed/`,
and a current `last-updated-utc`. Also run `git diff --cached --check` after staging.

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

## Periodic Audit

The closing pull request is easy to forget. Run this read-only audit periodically, and after a
batch of merges, to confirm that `docs/issues/open/` holds only specs whose GitHub issue is open.
It changes nothing: it only reports. Run it from the repository root on an up-to-date `develop`.

> Automating this workflow in a script is tracked by
> [#1774](../../../../../docs/issues/open/1774-automate-cleanup-completed-issues-skill-script/ISSUE.md).
> Until that lands, use the commands below.

The issue number is the leading number of the spec's folder name (for example `2414` in
`2414-2410-si-22-2-...`), not the EPIC number that follows it.

```bash
REPO=torrust/torrust-tracker
OPEN_ISSUES=$(gh issue list --repo "$REPO" --state open --limit 5000 --json number --jq '.[].number' \
  | LC_ALL=C sort -u)

spec_numbers() {
  find "$1" -maxdepth 1 -mindepth 1 \( -type d -o -name '*.md' \) \
    ! -name README.md ! -name AGENTS.md -exec basename {} \; \
    | sed -E 's/^([0-9]+).*/\1/' | grep -E '^[0-9]+$' | LC_ALL=C sort -u
}

echo "[1] open/ specs whose issue is not open"
comm -23 <(spec_numbers docs/issues/open) <(echo "$OPEN_ISSUES")

echo "[2] closed/ specs whose issue is open"
comm -12 <(spec_numbers docs/issues/closed) <(echo "$OPEN_ISSUES")

echo "[3] drafts already linked to a GitHub issue"
grep -rlE '^github-issue: [0-9]+' docs/issues/drafts --include=ISSUE.md --include=EPIC.md

echo "[4] open/ specs with a terminal status"
grep -lE '^status: (done|closed|completed|resolved)' \
  docs/issues/open/*/ISSUE.md docs/issues/open/*/EPIC.md
```

Empty output under every heading means the folders are consistent. Otherwise:

| Finding                                       | Meaning                                                    | Action                                                                                                               |
| --------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| [1] a number                                  | The issue is closed, or no such GitHub issue exists        | Confirm with [Step 1](#step-1-verify-issue-is-closed-on-github). If `CLOSED`, archive it with Steps 0-7.            |
| [2] a number                                  | The issue is open again, or the spec was filed in `closed/` early | Follow [reopen-issue](../reopen-issue/SKILL.md) if it regressed; otherwise move the spec back to `open/`.     |
| [3] a path                                    | A draft must not stay in `drafts/` once it has an issue   | Reconcile it with [Step 0.6](#step-06-reconcile-drafts-that-already-have-a-github-issue-mandatory).                  |
| [4] a path                                    | The frontmatter and the folder disagree                    | Check the issue state. If it is closed, archive it; if it is open, correct the `status` field.                       |

Rules for reading the result:

- **A closed EPIC child list is not a closed EPIC.** An EPIC can have draft subissues that have no
  GitHub issue yet, so finding no open child issue does not mean the EPIC is complete. An EPIC
  appears under [1] only when its own GitHub issue is closed. Before closing or archiving one,
  apply [Step 0.7](#step-07-block-epic-closure-while-draft-subissues-remain-mandatory).
- **`status` in `closed/` is not checked.** Legacy specs there use inconsistent values (`closed`,
  `completed`, `resolved`, and others), so the GitHub issue state in [2] is the authority.
- **A number in [1] is not proof of closure.** A spec can be misnumbered. Always run Step 1 before
  moving anything.
