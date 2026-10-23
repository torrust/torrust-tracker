---
name: open-pull-request
description: Open a pull request from a feature branch using GitHub CLI (preferred) or GitHub MCP tools. Covers pre-flight checks, correct base/head configuration for fork workflows, title/body conventions, and post-creation validation. Use when asked to "open PR", "create pull request", or "submit branch for review".
metadata:
  author: torrust
  version: "1.1"
---

# Open a Pull Request

## CLI vs MCP Decision Rule

- **Inner loop (fast local branch work):** prefer GitHub CLI (`gh pr create`).
- **Outer loop (cross-system coordination):** use MCP tools for structured/authenticated access.

## Pre-flight Checks

Before opening a PR:

- [ ] Working tree is clean: `git status --short` has no output. **Do not open a PR while any tracked or untracked change is present.** Commit the intended changes, or deliberately discard/stash unrelated changes first.
- [ ] Upstream target repository confirmed from workspace metadata (`Cargo.toml` → `repository`)
- [ ] Branch is based on a recent upstream `develop` (`<upstream-remote>/develop`): run `git fetch <upstream-remote>`, then check `git log --oneline HEAD..<upstream-remote>/develop`. Rebasing before opening the PR is recommended and is required when the branch conflicts with `develop` (see [Keeping the branch up to date](#keeping-the-branch-up-to-date)).
- [ ] Branch is pushed to your fork remote
- [ ] Commits are GPG signed (`git log --show-signature -n 1`)
- [ ] All pre-commit checks passed (`linter all`, `cargo machete`, tests)
- [ ] PR body claims are aligned with the actual commit range (`<upstream-remote>/develop..HEAD`)
- [ ] If manual verification used temporary local-only patches, PR body explicitly says they are not included
- [ ] PR body paragraphs are written as single continuous lines (no hard line wrapping)

### Required pre-flight order

Run these checks immediately before `gh pr create` or an equivalent MCP creation request:

```bash
git status --short
git fetch <upstream-remote>
git log --oneline HEAD..<upstream-remote>/develop
```

Stop if the first command prints any path: restore a clean working tree first, so no uncommitted changes are accidentally omitted from, or confused with, the proposed commit range. If the final command prints commits, rebase on the fetched upstream `develop` before creating the PR unless that would be a bottleneck; the rebase is required when the branch conflicts with `develop`. A clean working tree is required both before rebasing and before opening the PR.

### Keeping the branch up to date

Rebasing onto the latest upstream `develop` is **recommended, not required**, as recorded in the
[ADR to stop requiring pull-request rebases](../../../../../docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md).
A rebase detects integration problems before merge; the requirement was dropped only to reduce CI
CPU consumption and runner costs. Rebase:

- when opening a PR;
- when you know `develop` has changed since your last push; and
- in general, whenever you can and it is not a bottleneck or a problem.

Skip a recommended rebase that would be a bottleneck or a problem, for example when a queue of open
PRs would rerun expensive CI after every merge, or when an in-progress review round cites the
current commit ids. A rebase or other update is **required** only when:

1. the branch conflicts with `develop`;
2. it needs a change that landed on `develop` later, or a reviewer asks for the combination to be
   tested before merge; or
3. the maintainer merge tool refuses it because GitHub's recorded base is stale and its own
   refresh is unavailable (see the `merge-pull-request` skill).

```bash
# Identify the upstream remote (commonly "torrust"; verify with git remote -v)
UPSTREAM_REMOTE=torrust  # replace if your remote has a different name

git fetch $UPSTREAM_REMOTE
git rebase $UPSTREAM_REMOTE/develop

# Then push (use --force-with-lease when rewriting history)
git push --force-with-lease <fork-remote> <branch-name>
```

Because a rebase rewrites the branch's commit ids and rebases still happen, no durable reference may
cite a PR-branch commit by id. Durable references are repository artifacts (issue specs, evidence
files, review audits), the PR body, which the maintainer merge tool copies into the merge commit,
and review replies, which stay on GitHub after the branch is rewritten.
Cite the Conventional Commit subject instead; ids are durable only once on `develop`. The rule holds
whether or not the branch is ever rebased: an id cited during a normal review round becomes wrong at
the next rebase. Run this check from the repository root before every push that changes an issue
spec, evidence file, or review audit, and before pushing after a rebase:

```bash
UPSTREAM_REMOTE=torrust  # replace if your remote has a different name

git diff -z --name-only --diff-filter=d "$UPSTREAM_REMOTE/develop"...HEAD |
  xargs -0 -r grep -IohE '\b[0-9a-f]{7,40}\b' | sort -u |
  while read -r token; do
    git rev-parse -q --verify "$token^{commit}" >/dev/null || continue
    git merge-base --is-ancestor "$token" "$UPSTREAM_REMOTE/develop" ||
      echo "PR-branch commit id: $token"
  done
```

Empty output means no file the branch changes cites a commit that is not on `develop`. The check
scans every changed text file, not only the issue folder, so it also covers review audits and
evidence files. It keeps all-digit tokens, because an abbreviated commit id can be all digits. It
reports a token only when it resolves to a local commit outside `develop`, so `develop` commits,
info-hashes, checksums, GitHub review or comment IDs, and decimal values pass without a manual
read. It skips ids it cannot resolve: commits that exist only on another fork, and ambiguous
abbreviations. It covers only the worktree; the PR body and review replies are a manual read.

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

### Body Formatting for GitHub

Before opening the PR, review and reformat the body text following the `write-markdown-docs`
checklist for GitHub surfaces:

- Write each paragraph as a **single continuous line** — do not hard-wrap at any fixed column width
- Use GitHub Flavored Markdown (GFM) conventions
- Check for accidental `#NUMBER` autolinks (only use `#NUMBER` for intentional issue/PR references)

### Title

PR title: use Conventional Commit style, include issue reference.

Examples:

- `feat(tracker-core): [#42] add peer expiry grace period`
- `docs(agents): [#1697] set up basic AI agent configuration`

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

The keyword closes the issue wherever it appears in the body, including a sentence that only
mentions it (for example "the link will change to `Closes #N` later"): the merge tool copies the
whole body into the merge commit. The branch commit messages land on `develop` too, and GitHub acts
on a keyword in any of them. A negated mention closes the issue too: the body of PR #2613 said "Does
not close #2606" and closed #2606 on merge. Do not write a closing keyword followed by an issue
reference anywhere in the body or in a commit message of a PR that must not close that issue; write
"Related to #N" and describe the remaining work without the keyword. Before creating or editing the
body, and before pushing commits, scan them. The pattern covers the reference forms GitHub accepts
after a keyword (`#N`, `owner/repo#N`, and an issue URL), with or without a colon:

```bash
grep -niE '\b(close[sd]?|fix(e[sd])?|resolve[sd]?):?[[:space:]]+([A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+#[0-9]+|#[0-9]+|https://github\.com/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/issues/[0-9]+)' <body-file>
```

Commit messages are wrapped, so a line-based `grep` misses a keyword whose reference wraps to the
next line. Fold the newlines before scanning the messages, and print the matches with `-o`, because
the folded input is a single line:

```bash
git log --format=%B <upstream-remote>/develop..HEAD | tr '\n' ' ' | grep -oiE '\b(close[sd]?|fix(e[sd])?|resolve[sd]?):?[[:space:]]+([A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+#[0-9]+|#[0-9]+|https://github\.com/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/issues/[0-9]+)'
```

A commit message that matches cannot be fixed after the merge. Reword it before the merge, using the
history-rewrite rules in the repository instructions.

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
- [ ] Body and branch commit messages scanned with the closing-keyword commands from [Issue Linking Rules](#issue-linking-rules); every match is an issue this PR closes
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
