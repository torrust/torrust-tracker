---
name: commit-changes
description: Guide for committing changes in the torrust-tracker project. Covers conventional commit format, pre-commit verification checklist, GPG signing, and commit quality guidelines. Use when committing code, running pre-commit checks, or following project commit standards. Triggers on "commit", "commit changes", "how to commit", "pre-commit", "commit message", "commit format", or "conventional commits".
metadata:
  author: torrust
  version: "1.0"
---

# Committing Changes

This skill guides you through the complete commit process for the Torrust Tracker project.

## Quick Reference

```bash
# One-time setup: install the pre-commit Git hook
./contrib/dev-tools/git/install-git-hooks.sh

# Stage changes
git add <files>

# Commit with conventional format and GPG signature (MANDATORY)
# The pre-commit hook runs ./contrib/dev-tools/git/hooks/pre-commit.sh automatically
git commit -S -m "<type>[(<scope>)]: <description>"
```

## Conventional Commit Format

We follow [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Commit Message Structure

```text
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Scope should reflect the affected package or area (e.g., `tracker-core`, `udp-protocol`, `ci`, `docs`).

### Commit Types

| Type       | Description                           | Example                                                      |
| ---------- | ------------------------------------- | ------------------------------------------------------------ |
| `feat`     | New feature or enhancement            | `feat(tracker-core): add peer expiry grace period`           |
| `fix`      | Bug fix                               | `fix(udp-protocol): resolve endianness in announce response` |
| `docs`     | Documentation changes                 | `docs(agents): add root AGENTS.md`                           |
| `style`    | Code style changes (formatting, etc.) | `style: apply rustfmt to all source files`                   |
| `refactor` | Code refactoring                      | `refactor(tracker-core): extract peer list to own module`    |
| `test`     | Adding or updating tests              | `test(http-tracker-core): add announce response tests`       |
| `chore`    | Maintenance tasks                     | `chore: update dependencies`                                 |
| `ci`       | CI/CD related changes                 | `ci: add workflow for container publishing`                  |
| `perf`     | Performance improvements              | `perf(torrent-repository): switch to dashmap`                |

## GPG Commit Signing (MANDATORY)

**All commits must be GPG signed.** Use the `-S` flag:

```bash
git commit -S -m "your commit message"
```

### GPG Timeout Handling

If the GPG passphrase prompt times out (`gpg: signing failed: Timeout`), the agent **must**:

1. **Stop immediately.** Do not retry, do not use `--no-gpg-sign`, do not skip signing.
2. **Notify the user.** Tell them the GPG passphrase prompt timed out and they need to retry
   the commit manually by running the same `git commit -S` command themselves.
3. **Wait for the user** to confirm the commit was completed before proceeding.

This rule is absolute. Never bypass GPG signing for any reason.

## Pre-commit Verification (MANDATORY)

### Git Hook

The repository ships a `pre-commit` Git hook that runs `./contrib/dev-tools/git/hooks/pre-commit.sh`
automatically on every `git commit`. Install it once after cloning:

```bash
./contrib/dev-tools/git/install-git-hooks.sh
```

Once installed, the hook fires on every commit and you do not need to run the script manually.

### Automated Checks

If the hook is not installed, run the script explicitly before committing.
**It must exit with code `0`.**

> **⏱️ Expected runtime: ~1 minute** on a modern developer machine with warm caches.
> AI agents should set a command timeout of **at least 3 minutes** before invoking this script.
> AI agents should also set `TORRUST_GIT_HOOKS_LOG_DIR=.tmp` so per-step log files
> are written inside the workspace (git-ignored) instead of `/tmp`.

```bash
TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh
```

The script runs:

1. `cargo machete` — unused dependency check
2. `linter all` — all linters (markdown, YAML, TOML, clippy, rustfmt, shellcheck, cspell)
3. `cargo test --doc --workspace` — documentation tests

For AI execution, prefer structured output first:

```bash
./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
```

If it fails and deeper diagnostics are needed, retry with:

```bash
./contrib/dev-tools/git/hooks/pre-commit.sh --format=text --verbosity=verbose
```

### Manual Checks (Cannot Be Automated)

Verify these by hand before committing:

- **Self-review the diff**: read through `git diff --staged` and check for obvious mistakes,
  debug artifacts, or unintended changes
- **Documentation updated**: if public API or behaviour changed, doc comments and any relevant
  `docs/` pages reflect the change
- **`AGENTS.md` updated**: if architecture, package structure, or key workflows changed, the
  relevant `AGENTS.md` file is updated
- **New technical terms added to `project-words.txt`**: any new jargon or identifiers that
  cspell does not know about are added alphabetically

### Debugging a Failing Run

```bash
linter markdown    # Markdown
linter yaml        # YAML
linter toml        # TOML
linter clippy      # Rust code analysis
linter rustfmt     # Rust formatting
linter shellcheck  # Shell scripts
linter cspell      # Spell checking
```

Fix Rust formatting automatically:

```bash
cargo fmt
```

## Hashtag Usage Warning

**Only use `#` when intentionally referencing a GitHub issue.**

GitHub auto-links `#NUMBER` to issues. Avoid accidental references:

- ✅ `feat(tracker-core): add feature (see #42)` — intentional reference
- ❌ `fix: make feature #1 priority` — accidentally links to issue #1

Use ordered Markdown lists or plain numbers instead of `#N` step labels.

## Commit Quality Guidelines

### Good Commits (✅)

- **Atomic**: Each commit represents one logical change
- **Descriptive**: Clear, concise description of what changed
- **Tested**: All tests pass
- **Linted**: All linters pass
- **Conventional**: Follows conventional commit format
- **Signed**: GPG signature present

### Commits to Avoid (❌)

- Too large: multiple unrelated changes in one commit
- Vague messages like "fix stuff" or "WIP"
- Missing scope when a package is clearly affected
- Unsigned commits
