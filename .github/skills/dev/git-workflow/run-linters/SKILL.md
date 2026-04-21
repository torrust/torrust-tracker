---
name: run-linters
description: Run code quality checks and linters for the torrust-tracker project. Includes Rust clippy, rustfmt, markdown, YAML, TOML, spell checking, and shellcheck. Use when asked to lint code, check formatting, fix code quality issues, or prepare for commit. Triggers on "lint", "run linters", "check code quality", "fix formatting", "run clippy", "run rustfmt", or "pre-commit checks".
metadata:
  author: torrust
  version: "1.0"
---

# Run Linters

## Quick Reference

### Run All Linters

```bash
linter all
```

**Always run `linter all` before every commit. It must exit with code `0`.**

### Run a Single Linter

```bash
linter markdown     # Markdown (markdownlint)
linter yaml         # YAML (yamllint)
linter toml         # TOML (taplo)
linter cspell       # Spell checker (cspell)
linter clippy       # Rust code analysis (clippy)
linter rustfmt      # Rust formatting (rustfmt)
linter shellcheck   # Shell scripts (shellcheck)
```

## Common Workflows

### Before Any Commit

```bash
linter all          # Must pass with exit code 0
```

### Debug a Failing Full Run

```bash
# Identify which linter is failing
linter markdown
linter yaml
linter toml
linter cspell
linter clippy
linter rustfmt
linter shellcheck
```

### During Development (Rust only)

```bash
linter clippy       # Check logic and code quality
linter rustfmt      # Check formatting
```

## Fixing Common Issues

### Rust Formatting Errors (rustfmt)

```bash
cargo fmt           # Auto-fix all Rust source files
```

Formatting rules from `rustfmt.toml`:

- `max_width = 130`
- `group_imports = "StdExternalCrate"`
- `imports_granularity = "Module"`

### Rust Clippy Errors

Warnings are **errors** (configured as `-D warnings` in `.cargo/config.toml`).
Fix the underlying issue — do not `#[allow(...)]` unless truly unavoidable.

Example: unused variable → use `_var` prefix or actually use the value.

### Markdown Errors (markdownlint)

Common issues:

- Trailing whitespace
- Missing blank line before headings
- Incorrect heading levels
- Lines exceeding 120 characters

Configuration in `.markdownlint.json`.

### YAML Errors (yamllint)

Common issues:

- Trailing spaces
- Inconsistent indentation (2 spaces expected)
- Missing newline at end of file

Configuration in `.yamllint-ci.yml`.

### TOML Errors (taplo)

```bash
taplo fmt **/*.toml   # Auto-fix TOML formatting
```

### Spell Check Errors (cspell)

For legitimate technical terms not in dictionaries, add them to `project-words.txt`
(alphabetical order, one per line).

### Shell Script Errors (shellcheck)

Fix the reported issue in the shell script. Common: use `[[ ]]` instead of `[ ]`,
quote variables, avoid `eval`.

## Linter Details

See [references/linters.md](references/linters.md) for detailed documentation on each linter.
