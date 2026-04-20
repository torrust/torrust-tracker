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
- [ ] Branch is pushed to your fork remote
- [ ] Commits are GPG signed (`git log --show-signature -n 1`)
- [ ] All pre-commit checks passed (`linter all`, `cargo machete`, tests)

## Title and Description Convention

PR title: use Conventional Commit style, include issue reference.

Examples:

- `feat(tracker-core): [#42] add peer expiry grace period`
- `docs(agents): set up basic AI agent configuration (#1697)`

PR body must include:

- Summary of changes
- Files/packages touched
- Validation performed
- Issue link (`Closes #<issue-number>`)

## Option A (Preferred): GitHub CLI

```bash
gh pr create \
  --repo torrust/torrust-tracker \
  --base develop \
  --head <fork-owner>:<branch-name> \
  --title "<title>" \
  --body "<body>"
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
- [ ] Issue linked in description

## Troubleshooting

- `fatal: ... does not appear to be a git repository`: push to correct remote (`git remote -v`)
- `A pull request already exists`: open existing PR URL instead of creating new
- Permission errors on upstream: use `owner:branch` fork syntax
