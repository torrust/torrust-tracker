---
name: run-pre-commit-checks
description: Run all mandatory pre-commit verification steps for the torrust-tracker project. Covers the pre-commit script (automated checks), manual review steps, and individual linter commands for debugging. Use before any commit or PR to ensure all quality gates pass. Triggers on "pre-commit checks", "run all checks", "verify before commit", or "check everything".
metadata:
  author: torrust
  version: "1.0"
---

# Run Pre-commit Checks

## Git Hook (Recommended Setup)

The repository ships a `pre-commit` Git hook that runs `./contrib/dev-tools/git/hooks/pre-commit.sh`
automatically on every `git commit`. Install it once after cloning:

```bash
./contrib/dev-tools/git/install-git-hooks.sh
```

After installation the hook fires automatically; you do not need to invoke the script
manually before each commit.

> **For AI agents**: before invoking the script manually, check whether the hook is installed:
>
> ```bash
> [[ -x "$(git rev-parse --git-path hooks)/pre-commit" ]] && echo "installed" || echo "not installed"
> ```
>
> If installed, skip the manual run — `git commit` will trigger it automatically.
> Running both would execute every check twice.

## Automated Checks

> **⏱️ Expected runtime: ~1 minute** on a modern developer machine with warm caches.
> AI agents should set a command timeout of **at least 3 minutes** before invoking
> `./contrib/dev-tools/git/hooks/pre-commit.sh`.

Run the pre-commit script. **It must exit with code `0` before every commit.**

```bash
./contrib/dev-tools/git/hooks/pre-commit.sh
```

The script runs these steps in order:

1. `cargo machete` - unused dependency check
2. `linter all` - all linters (markdown, YAML, TOML, clippy, rustfmt, shellcheck, cspell)
3. `cargo test --doc --workspace` - documentation tests

## Output Modes

The pre-commit script supports concise human output, verbose human output, and JSON output for
automation.

```bash
# Default: text + concise
./contrib/dev-tools/git/hooks/pre-commit.sh

# Explicit text + concise
./contrib/dev-tools/git/hooks/pre-commit.sh --format=text --verbosity=concise

# Text + verbose streaming command output
./contrib/dev-tools/git/hooks/pre-commit.sh --format=text --verbosity=verbose

# Compatibility alias
./contrib/dev-tools/git/hooks/pre-commit.sh --format=text --verbose

# Structured output (single JSON document to stdout)
./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
```

Flag behavior:

- `--format=<text|json>` defaults to `text`
- `--verbosity=<concise|verbose>` defaults to `concise`
- `--verbose` is an alias for `--verbosity=verbose`
- Duplicate `--format`/`--verbosity` flags: last value wins
- Invalid values or unknown flags exit with code `2` and print usage guidance to stderr
- In `--format=json`, structured output remains JSON regardless of verbosity value
- Per-step logs are written to `TORRUST_GIT_HOOKS_LOG_DIR` (default: `/tmp`)

For restricted agent environments that cannot write outside the workspace, run with:

```bash
TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh
```

The `.tmp/` directory is git-ignored.
Because `.tmp/` is workspace-local, clean stale `pre-commit-*.log` files periodically.

## Check Tier Ownership

Check ownership is intentionally split by gate:

- Pre-commit: fast local gate (`cargo machete`, `linter all`, `cargo test --doc --workspace`)
- Pre-push: nightly toolchain checks + full stable test suite (no duplicates of pre-commit; no E2E)
- CI: merge authority with full validation and E2E matrix jobs

E2E tests are intentionally excluded from both pre-commit and pre-push. They run only in CI.

> **MySQL tests**: MySQL-specific tests require a running instance and a feature flag:
>
> ```bash
> TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test --package bittorrent-tracker-core
> ```
>
> These are not run by the pre-commit script.

## Manual Checks (Cannot Be Automated)

Verify these by hand before committing:

- **Self-review the diff**: read through `git diff --staged` for debug artifacts or unintended changes
- **Documentation updated**: if public API or behaviour changed, doc comments and `docs/` pages reflect it
- **`AGENTS.md` updated**: if architecture or key workflows changed, the relevant `AGENTS.md` is updated
- **New technical terms in `project-words.txt`**: new jargon added alphabetically

## Before Opening a PR (Recommended)

```bash
cargo +nightly doc --no-deps --bins --examples --workspace --all-features
```

## Troubleshooting Output Modes

- Concise mode shows high-signal per-step summaries only. On failure, it prints the log path and
  a short failure tail.
- Verbose mode streams full command output to the terminal. Use this for deep local debugging.
- JSON mode emits one structured document to stdout; diagnostics and usage errors go to stderr.
- If concise output is too short for debugging, re-run the same command with
  `--format=text --verbosity=verbose`.

## Debugging Individual Linters

Run individual linters to isolate a failure:

```bash
linter markdown    # Markdown
linter yaml        # YAML
linter toml        # TOML
linter clippy      # Rust code analysis
linter rustfmt     # Rust formatting
linter shellcheck  # Shell scripts
linter cspell      # Spell checking
```

| Failure             | Fix                                     |
| ------------------- | --------------------------------------- |
| Unused dependency   | Remove from `Cargo.toml`                |
| Clippy warning      | Fix the underlying issue                |
| rustfmt error       | Run `cargo fmt`                         |
| Markdown lint error | Fix formatting per `.markdownlint.json` |
| Spell check error   | Add term to `project-words.txt`         |
| Test failure        | Fix the failing test or code            |
| Doc build error     | Fix Rust doc comment                    |
