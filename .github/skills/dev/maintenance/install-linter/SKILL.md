---
name: install-linter
description: Install the torrust-linting `linter` binary and its external tool dependencies. Use when setting up a new development environment, after a fresh clone, or when the `linter` binary is missing. Triggers on "install linter", "setup linter", "linter not found", "install torrust-linting", "missing linter binary", or "set up development environment".
metadata:
  author: torrust
  version: "1.0"
---

# Install the Linter

The project uses a unified `linter` binary from
[torrust/torrust-linting](https://github.com/torrust/torrust-linting) to run all quality checks.

## Install the `linter` Binary

```bash
cargo install --locked --git https://github.com/torrust/torrust-linting --bin linter
```

Verify the installation:

```bash
linter --version
```

## Install External Tool Dependencies

The `linter` binary delegates to external tools. Install them if they are not already present:

| Linter      | Tool             | Install command                       |
| ----------- | ---------------- | ------------------------------------- |
| Markdown    | markdownlint-cli | `npm install -g markdownlint-cli`     |
| YAML        | yamllint         | `pip3 install yamllint`               |
| TOML        | taplo            | `cargo install taplo-cli --locked`    |
| Spell check | cspell           | `npm install -g cspell`               |
| Shell       | shellcheck       | `apt install shellcheck`              |
| Rust        | clippy / rustfmt | bundled with `rustup` (no extra step) |

> The `linter` binary will attempt to install missing npm-based tools automatically on first run.
> System-packaged tools (`yamllint`, `shellcheck`) must be installed manually.

## Configuration Files

The `linter` binary has **no configuration file of its own**. It delegates to each
external tool, which reads its own config file from the project root. These files are
already present in the repository — no manual setup is needed:

| File                 | Used by      |
| -------------------- | ------------ |
| `.markdownlint.json` | markdownlint |
| `.yamllint-ci.yml`   | yamllint     |
| `.taplo.toml`        | taplo        |
| `cspell.json`        | cspell       |
| `rustfmt.toml`       | rustfmt      |

> **Note**: Files listed in `.gitignore` are **not** automatically excluded from linting.
> Each tool has its own ignore mechanism (e.g. `.markdownlintignore` for markdownlint).
> Add `.gitignore` paths to the appropriate per-linter ignore file when needed.

## Verify Full Setup

After installing the binary and its dependencies, run all linters to confirm everything works:

```bash
linter all
```

It must exit with code `0`. See the `run-linters` skill for day-to-day usage.
