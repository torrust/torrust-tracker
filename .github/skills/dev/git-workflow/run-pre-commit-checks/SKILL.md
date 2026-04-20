---
name: run-pre-commit-checks
description: Run all mandatory pre-commit verification steps for the torrust-tracker project. Covers the pre-commit script (automated checks), manual review steps, and individual linter commands for debugging. Use before any commit or PR to ensure all quality gates pass. Triggers on "pre-commit checks", "run all checks", "verify before commit", or "check everything".
metadata:
  author: torrust
  version: "1.0"
---

# Run Pre-commit Checks

## Git Hook (Recommended Setup)

The repository ships a `pre-commit` Git hook that runs `./scripts/pre-commit.sh`
automatically on every `git commit`. Install it once after cloning:

```bash
./scripts/install-git-hooks.sh
```

After installation the hook fires automatically; you do not need to invoke the script
manually before each commit.

## Automated Checks

> **⏱️ Expected runtime: ~3 minutes** on a modern developer machine. AI agents must set a
> command timeout of **at least 5 minutes** before invoking `./scripts/pre-commit.sh`. Agents
> with a default per-command timeout below 5 minutes will likely time out and report a false
> failure.

Run the pre-commit script. **It must exit with code `0` before every commit.**

```bash
./scripts/pre-commit.sh
```

The script runs these steps in order:

1. `cargo machete` — unused dependency check
2. `linter all` — all linters (markdown, YAML, TOML, clippy, rustfmt, shellcheck, cspell)
3. `cargo test --doc --workspace` — documentation tests
4. `cargo test --tests --benches --examples --workspace --all-targets --all-features` — all tests

> **MySQL tests**: MySQL-specific tests require a running instance and a feature flag:
>
> ```bash
> TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test --package tracker-core
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
