---
name: create-feature-branch
description: Guide for creating feature branches following the torrust-tracker branching conventions. Covers branch naming format, lifecycle, and common patterns. Use when creating branches for issues, starting work on tasks, or setting up development branches. Triggers on "create branch", "new branch", "checkout branch", "branch for issue", or "start working on issue".
metadata:
  author: torrust
  version: "1.0"
---

# Creating Feature Branches

This skill guides you through creating feature branches following the Torrust Tracker branching
conventions.

## Branch Naming Convention

**Format**: `{issue-number}-{short-description}` (preferred)

Alternative formats (no tracked issue):

- `feat/{short-description}`
- `fix/{short-description}`
- `chore/{short-description}`

**Rules**:

- Always start with the GitHub issue number when one exists
- Use lowercase letters only
- Separate words with hyphens (not underscores)
- Keep description concise but descriptive

## Creating a Branch

### Standard Workflow

```bash
# Ensure you're on latest develop
git checkout develop
git pull --ff-only

# Create and checkout branch for issue #42
git checkout -b 42-add-peer-expiry-grace-period
```

### With MCP GitHub Tools

1. Get the issue number and title
2. Format the branch name: `{number}-{kebab-case-description}`
3. Create the branch from `develop`
4. Checkout locally: `git fetch && git checkout {branch-name}`

## Branch Naming Examples

✅ **Good branch names**:

- `42-add-peer-expiry-grace-period`
- `156-refactor-udp-server-socket-binding`
- `203-add-e2e-mysql-tests`
- `1697-ai-agent-configuration`

❌ **Avoid**:

- `my-feature` — no issue number
- `FEATURE-123` — all caps
- `fix_bug` — underscores instead of hyphens
- `42_add_support` — underscores

## Complete Branch Lifecycle

### 1. Create Branch from `develop`

```bash
git checkout develop
git pull --ff-only
git checkout -b 42-add-peer-expiry-grace-period
```

### 2. Develop

Make commits following [commit conventions](../commit-changes/SKILL.md).

### 3. Pre-commit Checks

```bash
cargo machete
linter all
cargo test --doc --workspace
cargo test --tests --benches --examples --workspace --all-targets --all-features
```

### 4. Push to Your Fork

```bash
git push {your-fork-remote} 42-add-peer-expiry-grace-period
```

### 5. Create Pull Request

Target branch: `torrust/torrust-tracker:develop`

### 6. Cleanup After Merge

```bash
git checkout develop
git pull --ff-only
git branch -d 42-add-peer-expiry-grace-period
```

## Converting Issue Title to Branch Name

1. Get issue number (e.g., #42)
2. Take issue title (e.g., "Add Peer Expiry Grace Period")
3. Convert to lowercase kebab-case: `add-peer-expiry-grace-period`
4. Prefix with issue number: `42-add-peer-expiry-grace-period`
