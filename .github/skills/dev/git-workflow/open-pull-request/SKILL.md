---
name: open-pull-request
description: Open a pull request from a feature branch using GitHub CLI (preferred) or GitHub MCP tools. Covers pre-flight checks, correct base/head configuration for fork workflows, title/body conventions, and post-creation validation. Use when asked to "open PR", "create pull request", or "submit branch for review".
metadata:
  author: torrust
  version: "1.0"
---

# Open a Pull Request

## CLI vs MCP Decision Rule

- **Inner loop (fast local branch work):** prefer GitHub CLI (`gh pr create`).
- **Outer loop (cross-system coordination):** use MCP tools for structured/authenticated access.

## Pre-flight Checks

Before opening a PR:

- [ ] Working tree is clean (`git status`)
- [ ] Upstream target repository confirmed from workspace metadata (`Cargo.toml` → `repository`)
- [ ] Branch is rebased on the latest `develop` from upstream (`<upstream-remote>/develop`); verify with `git log --oneline <upstream-remote>/develop..HEAD` and rebase if behind
- [ ] Branch is pushed to your fork remote
- [ ] Commits are GPG signed (`git log --show-signature -n 1`)
- [ ] All pre-commit checks passed (`linter all`, `cargo machete`, tests)
- [ ] PR body claims are aligned with the actual commit range (`<upstream-remote>/develop..HEAD`)
- [ ] If manual verification used temporary local-only patches, PR body explicitly says they are not included

### Keeping the branch up to date

Always rebase your branch on the latest upstream `develop` before pushing — both when opening
a PR for the first time and when pushing updates to an existing PR:

```bash
# Identify the upstream remote (commonly "torrust"; verify with git remote -v)
UPSTREAM_REMOTE=torrust  # replace if your remote has a different name

git fetch $UPSTREAM_REMOTE
git rebase $UPSTREAM_REMOTE/develop

# Then push (use --force-with-lease when rewriting history)
git push --force-with-lease <fork-remote> <branch-name>
```

> In general, every PR targeting `develop` should sit on top of the latest commit in
> `<upstream-remote>/develop`. Check this whenever you push or re-push.

<!-- markdownlint-disable-next-line MD028 -->

> Important:
>
> - Never push directly to `develop` or `main`.
> - Always open the PR in the **upstream repository**, not in your fork.
> - For merges into `develop` or `main`, the PR head must be a fork branch (`<fork-owner>:<branch>`), not an upstream branch.
> - Remote names vary by contributor (`josecelano`, `origin`, `torrust`, `upstream`, etc.); resolve remotes dynamically.
>
> Resolve upstream from `Cargo.toml` (`repository = "https://github.com/torrust/torrust-tracker"`) and use that value for `gh pr create --repo ...`.

## Title and Description Convention

PR title: use Conventional Commit style, include issue reference.

Examples:

- `feat(tracker-core): [#42] add peer expiry grace period`
- `docs(agents): set up basic AI agent configuration (#1697)`

PR body must include:

- Summary of changes
- Files/packages touched
- Validation performed
- Issue link (see rules below)

PR body must not include:

- Claims about code changes that are not present in the branch diff
- Ambiguous wording that mixes temporary local verification patches with committed implementation

## Issue Linking Rules

GitHub auto-closes an issue when a merged PR body contains `Closes #N`, `Fixes #N`, or `Resolves #N`.
Choose the correct keyword based on what the PR contains:

| PR type                                                                                 | Keyword to use  | Example            |
| --------------------------------------------------------------------------------------- | --------------- | ------------------ |
| **Spec-only** — PR contains only the issue spec document, no implementation             | `Related to #N` | `Related to #1780` |
| **Implementation** — PR implements the issue (whether or not it also includes the spec) | `Closes #N`     | `Closes #1780`     |

> **Rule:** only use `Closes`/`Fixes`/`Resolves` when the PR fully resolves the issue.
> A spec-only PR does **not** resolve the issue — use `Related to #N` to avoid auto-closing it.

### Identifying the PR type

Before writing the PR body, check the diff:

```bash
git diff <upstream-remote>/develop...HEAD --name-only
```

- Diff touches only `docs/issues/` → spec-only → use `Related to #N`
- Diff touches source code, tests, or other non-spec files → implementation → use `Closes #N`
- Diff touches both spec and implementation → combined → use `Closes #N`

## Option A (Preferred): GitHub CLI

```bash
gh pr create \
  --repo <upstream-owner>/<upstream-repo> \
  --base develop \
  --head <fork-owner>:<branch-name> \
  --title "<title>" \
  --body "<body>"
```

Example upstream resolution from `Cargo.toml`:

```bash
UPSTREAM_REPO=$(grep '^repository\s*=\s*"https://github.com/' Cargo.toml | sed -E 's#.*github.com/([^\"]+).*#\1#')
gh pr create --repo "$UPSTREAM_REPO" --base develop --head <fork-owner>:<branch-name> --title "<title>" --body "<body>"
```

If successful, `gh` prints the PR URL.

## Option B: GitHub MCP Tools

When MCP pull request management tools are available, create the PR with:

- `base`: `develop`
- `head`: `<fork-owner>:<branch-name>`
- Capture and share the resulting PR URL.

## Post-creation Validation

- [ ] PR targets `torrust/torrust-tracker:develop`
- [ ] Head branch is correct
- [ ] CI workflows started
- [ ] Issue linked with the correct keyword (`Related to` for spec-only, `Closes` for implementation)
- [ ] PR body still matches branch diff and commit history after final rebases/edits

Quick body-accuracy verification:

```bash
gh pr view <pr-number> --repo <upstream-owner>/<upstream-repo> --json body
git diff --name-only <upstream-remote>/develop...HEAD
git log --oneline <upstream-remote>/develop..HEAD
```

## Troubleshooting

- `fatal: ... does not appear to be a git repository`: push to correct remote (`git remote -v`)
- `A pull request already exists`: open existing PR URL instead of creating new
- Permission errors on upstream: use `owner:branch` fork syntax
