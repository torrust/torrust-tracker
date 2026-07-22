# Linter Documentation

This document provides detailed documentation for each linter used in the Torrust Tracker project.

## Overview

The project uses the `linter` binary from
[torrust/torrust-linting](https://github.com/torrust/torrust-linting) as a unified wrapper around
all linters.

Install: `cargo install --locked --git https://github.com/torrust/torrust-linting --bin linter`

## Rust Linters

### clippy

**Tool**: Rust's official linter.  
**Config**: `.cargo/config.toml` (global `rustflags`)  
**Run**: `linter clippy`

Warnings are treated as errors via `-D warnings` in `.cargo/config.toml`.
Do not suppress warnings with `#[allow(...)]` unless absolutely necessary.

**Critical flags** (from `.cargo/config.toml`):

- `-D warnings` — all warnings are errors
- `-D unused` — unused items are errors
- `-D rust-2018-idioms` — enforces Rust 2018 idioms
- `-D future-incompatible`

### rustfmt

**Tool**: Rust code formatter.  
**Config**: `rustfmt.toml`  
**Run**: `linter rustfmt`  
**Auto-fix**: `cargo fmt`

Key formatting settings:

- `max_width = 130`
- `group_imports = "StdExternalCrate"`
- `imports_granularity = "Module"`

## Documentation Linters

### markdownlint

**Tool**: markdownlint  
**Config**: `.markdownlint.json`  
**Run**: `linter markdown`

### cspell (Spell Checker)

**Tool**: cspell  
**Config**: `cspell.json`  
**Dictionary**: `project-words.txt`  
**Run**: `linter cspell`

Add technical terms to `project-words.txt` (one per line), then run
`./contrib/dev-tools/git/format-project-words.sh`. The formatter uses `LC_ALL=C sort -u`;
the pre-commit hook runs it automatically and requests restaging if it changes the dictionary.

## Configuration Linters

### yamllint

**Tool**: yamllint  
**Config**: `.yamllint-ci.yml`  
**Run**: `linter yaml`

Expected: 2-space indentation, no trailing whitespace, newline at EOF.

### taplo

**Tool**: taplo  
**Config**: `.taplo.toml`  
**Run**: `linter toml`  
**Auto-fix**: `taplo fmt **/*.toml`

## Script Linters

### shellcheck

**Tool**: shellcheck  
**Run**: `linter shellcheck`

Checks all shell scripts. Use `[[ ]]` over `[ ]`, quote variables (`"$var"`), and avoid `eval`.
