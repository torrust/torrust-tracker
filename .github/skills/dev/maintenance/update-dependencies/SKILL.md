---
name: update-dependencies
description: Guide for updating project dependencies in the torrust-tracker project. Covers the manual cargo update workflow including branch creation, running checks, committing, and pushing. Distinguishes trivial updates (Cargo.lock only) from breaking-change updates (code rework needed). Use when updating dependencies, running cargo update, or bumping deps. Triggers on "update dependencies", "cargo update", "update deps", or "bump dependencies".
metadata:
  author: torrust
  version: "1.2"
semantic-links:
  skill-links:
    - update-github-workflow-actions
  related-artifacts:
    - .github/dependabot.yaml
    - .github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md
    - docs/templates/CARGO-DEPENDENCY-UPDATE-PR.md
---

# Updating Dependencies

This skill guides you through updating project dependencies for the Torrust Tracker project.

Use `.github/skills/dev/maintenance/add-rust-dependency/SKILL.md` when introducing a new crate.
This skill is for updating already-declared dependencies.
Use `.github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md` for GitHub Actions
workflow dependency updates and organization action-allowlist synchronization.
When updating crates and workflow actions together, complete the crate update first and use the
same dedicated branch for the subsequent workflow-action update.

Delivery policy:

- Never push directly to `develop` or `main`.
- Merges into `develop` or `main` must go through a PR opened in `torrust/torrust-tracker` from a fork branch (`<fork-owner>:<branch>`).
- Remote names are contributor-specific (`josecelano`, `origin`, `torrust`, etc.); use your configured fork remote.

## Update Categories

Before starting, decide which category the update falls into:

| Category     | Description                                  | Branch / Issue                                                 |
| ------------ | -------------------------------------------- | -------------------------------------------------------------- |
| **Trivial**  | `cargo update` only — no code changes needed | Timestamped branch, no issue required                          |
| **Breaking** | Dependency change requires code rework       | If small: same branch. If large: open a separate issue per dep |

Use `cargo update --dry-run` or read the dependency changelog to classify before starting.

## Quick Reference

```bash
# Get one high-resolution timestamp (YYYYMMDD-HHMMSS) for this invocation.
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
UPDATE_OUTPUT=".tmp/${TIMESTAMP}-cargo-update.txt"
UPDATE_BRANCH="${TIMESTAMP}-update-dependencies"

# Create branch
git checkout develop && git pull --ff-only
git checkout -b "$UPDATE_BRANCH"

# Ensure the workspace-local ignored log directory exists.
mkdir -p .tmp

# Update dependencies
cargo update 2>&1 | tee "$UPDATE_OUTPUT"

# If Cargo.lock has no changes, nothing to do — stop here.

# Verify
./contrib/dev-tools/git/hooks/pre-commit.sh --format=json

# Commit and push (using the captured `cargo update` output as the commit body)
git add Cargo.lock
git commit -S -m "chore: update dependencies" -m "$(cat "$UPDATE_OUTPUT")"
git push {your-fork-remote} "$UPDATE_BRANCH"

# Open a PR targeting torrust/torrust-tracker:develop. Use
# docs/templates/CARGO-DEPENDENCY-UPDATE-PR.md and replace its Cargo-output
# placeholder with the complete "$UPDATE_OUTPUT" contents verbatim.
```

## Complete Workflow

### Step 1: Create a Branch

Generate one high-resolution timestamp for both the branch and captured output filename. This
avoids ordinary same-day/same-second collisions and prevents the output capture of one invocation
from overwriting another's:

```bash
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
UPDATE_OUTPUT=".tmp/${TIMESTAMP}-cargo-update.txt"
UPDATE_BRANCH="${TIMESTAMP}-update-dependencies"
git checkout develop
git pull --ff-only
git checkout -b "$UPDATE_BRANCH"

mkdir -p .tmp
```

Confirm the branch is cut from an up-to-date `develop` before running anything. A branch cut from
`main` (or any older branch) lacks the current hooks and tooling, so every commit fails or checks
against stale rules.

For breaking-change updates that require a tracked issue:

```bash
UPDATE_BRANCH="${TIMESTAMP}-{issue-number}-update-dependencies"
git checkout -b "$UPDATE_BRANCH"
```

### Step 2: Run Cargo Update

```bash
mkdir -p .tmp
cargo update 2>&1 | tee "$UPDATE_OUTPUT"
```

If `Cargo.lock` has no changes, there is nothing to update — exit early.

Review `"$UPDATE_OUTPUT"` to identify any major version bumps that may be breaking.

This unique filename prevents captured-output collisions only. Concurrent update workflows must
not share a Git working tree because Git's branch checkout and index remain shared.

### Step 3: Handle Breaking Changes

If any updated dependency introduced a breaking API change:

- **Small rework** (a few lines, no design decisions): fix it in this branch and continue.
- **Large rework** (architectural impact or significant effort): revert that specific dependency
  in `Cargo.toml`, keep the other trivial updates, and open a new issue for the breaking
  dependency separately.

```bash
# Revert a single crate to its current locked version to defer it
cargo update --precise {old-version} {crate-name}
```

`cargo update --precise` is only recorded in `Cargo.lock`; the next plain `cargo update` moves the
crate again whenever the manifest requirement is loose (for example `"0"` or `"1"`). To defer a
breaking release durably, also pin the manifest requirement to the current series (for example
`ringbuf = "0.5"`), commit that pin, and open a follow-up issue or PR that raises the pin together
with the required code changes. Loosely-ranged `0.x` crates are the usual source of breaking
minors, so after any `cargo update` run `cargo check --workspace --all-targets` to find them, and
separate lints introduced by a newer toolchain from real dependency breakage by comparing with the
base branch.

A manifest requirement bump (for example `"0.27.0"` to `"0.28.0"`) is outside the compatible range
of the old requirement. It is not a lockfile-only update: say so in the commit and the PR
description.

### Step 4: Verify

```bash
cargo machete
./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
```

If the run fails and deeper diagnostics are needed, retry with:

```bash
./contrib/dev-tools/git/hooks/pre-commit.sh --format=text --verbosity=verbose
```

Fix any failures before proceeding.

### Step 5: Commit and Push

Use the complete output captured from `cargo update` as the commit body. This preserves the
authoritative package-by-package update, addition, and removal list in Git history instead of
maintaining a manually abbreviated summary. Do not edit or summarize this output for the commit
body unless it contains information that must not be committed.

```bash
git add Cargo.lock
git commit -S -m "chore: update dependencies" -m "$(cat "$UPDATE_OUTPUT")"
git push {your-fork-remote} "$UPDATE_BRANCH"
```

### Step 6: Open PR

Target: `torrust/torrust-tracker:develop`  
Title: `chore: update dependencies`

Use [the Cargo dependency-update PR template](../../../../../docs/templates/CARGO-DEPENDENCY-UPDATE-PR.md)
for the PR-body structure. Include the complete `"$UPDATE_OUTPUT"` contents in the PR description
as well as the commit body, replacing the template's placeholder verbatim. Do not replace it with
a manually abbreviated package list unless the output contains information that must not be
published.

## Optional: One Commit per Dependency

When reviewers want each update traceable, update one package at a time instead of one big
`cargo update`:

1. Run `cargo update --dry-run` and list the packages that would change.
2. For each package, run `cargo update -p <crate>` (use `<crate>@<version>` when several versions
   exist), capture its complete output, and commit `Cargo.lock` with that output as the body.
3. A per-crate update can leave related packages behind (for example a sibling crate of the same
   project). Finish with a plain `cargo update` for the remainder and confirm that
   `cargo update --dry-run` reports nothing left.
4. A command that changes no package version only re-resolves dependency edges in `Cargo.lock`.
   Say that in its commit message instead of claiming an update, and mention newly added
   transitive packages (`Adding ...` lines) in the commit that introduces them.
5. Prove each commit: re-run its command from the parent commit and check that the resulting
   `Cargo.lock` equals the committed one.

There is no single `cargo update` run in this mode, so the PR description holds each step's
output in its own block, as it appears in the commit bodies. List the verification actually run
(for example the workspace test suite on the final tree) and nothing that was not run.

## CI Failures Unrelated to the Change

If CI jobs fail before compiling the project (for example a shared build cache such as `sccache`
returning a storage quota or availability error), the change is not the cause. Read the first
failing step of each failed job to confirm, re-run the failed jobs once the quota or outage clears
instead of changing code, and record the cause in the PR conversation.

## Decision Guide

| Scenario                                       | Action                                                     |
| ---------------------------------------------- | ---------------------------------------------------------- |
| `cargo update` with no code changes            | Trivial — timestamped branch, no issue                     |
| Breaking change, small rework (< 1 hour)       | Fix in the same branch, note in PR description             |
| Breaking change, large rework (> 1 hour)       | Defer: revert that dep, open a separate issue, separate PR |
| Multiple breaking deps, independent migrations | One issue + PR per dependency to keep diffs reviewable     |
