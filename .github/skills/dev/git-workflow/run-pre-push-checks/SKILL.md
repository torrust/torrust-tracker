---
name: run-pre-push-checks
description: Run all mandatory pre-push verification steps for the torrust-tracker project. Covers the pre-push script (automated checks), output modes, and log-directory configuration. Use before pushing or when running the comprehensive developer gate including nightly checks and E2E tests. Triggers on "pre-push checks", "run pre-push", "verify before push", or "push checks".
metadata:
  author: torrust
  version: "1.0"
---

# Run Pre-push Checks

## Git Hook (Recommended Setup)

The repository ships a `pre-push` Git hook that runs `./contrib/dev-tools/git/hooks/pre-push.sh`
automatically on every `git push`. Install it once after cloning:

```bash
./contrib/dev-tools/git/install-git-hooks.sh
```

After installation the hook fires automatically; you do not need to invoke the script
manually before each push.

> **For AI agents**: before invoking the script manually, check whether the hook is installed:
>
> ```bash
> [[ -x "$(git rev-parse --git-path hooks)/pre-push" ]] && echo "installed" || echo "not installed"
> ```
>
> If installed, skip the manual run — `git push` will trigger it automatically.
> Running both would execute every check twice.

## Automated Checks

> **⏱️ Expected runtime: ~15 minutes** on a modern developer machine with warm caches.
> AI agents should set a command timeout of **at least 30 minutes** before invoking
> `./contrib/dev-tools/git/hooks/pre-push.sh`.

Run the pre-push script. **It must exit with code `0` before every push.**

```bash
./contrib/dev-tools/git/hooks/pre-push.sh
```

The script runs these steps in order:

1. `cargo +stable machete` - unused dependency check
2. `linter all` - all linters (markdown, YAML, TOML, clippy, rustfmt, shellcheck, cspell)
3. `cargo +nightly fmt --check` - nightly format check
4. `cargo +nightly check ...` - nightly workspace check
5. `cargo +nightly doc ...` - nightly documentation build
6. `cargo +stable test --doc --workspace` - documentation tests
7. `cargo +stable test --tests --benches --examples --workspace --all-targets --all-features` - all tests
8. `cargo +stable run --bin e2e_tests_runner ...` - end-to-end tests

## Output Modes

The pre-push script supports concise human output, verbose human output, and JSON output for
automation.

```bash
# Default: text + concise
./contrib/dev-tools/git/hooks/pre-push.sh

# Explicit text + concise
./contrib/dev-tools/git/hooks/pre-push.sh --format=text --verbosity=concise

# Text + verbose streaming command output
./contrib/dev-tools/git/hooks/pre-push.sh --format=text --verbosity=verbose

# Compatibility alias
./contrib/dev-tools/git/hooks/pre-push.sh --format=text --verbose

# Structured output (single JSON document to stdout)
./contrib/dev-tools/git/hooks/pre-push.sh --format=json
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
TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-push.sh
```

The `.tmp/` directory is git-ignored.
Because `.tmp/` is workspace-local, clean stale `pre-push-*.log` files periodically.

## Check Tier Ownership

Check ownership is intentionally split by gate:

- Pre-commit: fast local gate (`cargo machete`, `linter all`, `cargo test --doc --workspace`)
- Pre-push: comprehensive developer gate (nightly format/check/doc + stable tests + E2E)
- CI: merge authority with full validation and E2E matrix jobs

E2E is intentionally excluded from pre-commit and remains a pre-push/CI responsibility.

## Troubleshooting Output Modes

- Concise mode shows high-signal per-step summaries only. On failure, it prints the log path and
  a short failure tail.
- Verbose mode streams full command output to the terminal. Use this for deep local debugging.
- JSON mode emits one structured document to stdout; diagnostics and usage errors go to stderr.
- If concise output is too short for debugging, re-run the same command with
  `--format=text --verbosity=verbose`.
