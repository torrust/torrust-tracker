---
name: run-pre-push-checks
description: Run all mandatory pre-push verification steps for the torrust-tracker project. Covers the pre-push script (automated checks), output modes, and log-directory configuration. Use before pushing or when running the nightly toolchain checks and the full stable test suite. Triggers on "pre-push checks", "run pre-push", "verify before push", or "push checks".
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
> ./contrib/dev-tools/git/check-git-hooks.sh
> ```
>
> If installed, skip the manual run — `git push` will trigger it automatically.
> Running both would execute every check twice.

## Automated Checks

> **⏱️ Expected runtime: ~5 minutes** on a modern developer machine with warm caches.
> AI agents should set a command timeout of **at least 15 minutes** before invoking
> `./contrib/dev-tools/git/hooks/pre-push.sh`.
>
> **For AI agents — `git push` is a long-running command:**
> When the pre-push hook is installed, `git push` runs the full check suite (~5 minutes)
> before sending objects to the remote. Do **not** poll, retry, or issue a second `git push`
> while the first is still running. Wait for the IDE terminal-completion notification
> (exit code + output) before taking any follow-up action.
>
> Use a **generous timeout** for `git push` itself (at least 20 minutes), because cold-cache
> runs can be significantly slower than warm-cache runs. Quiet output during tests is normal;
> do not cancel early unless there is concrete failure output.
>
> To avoid parsing shared terminal history (which other commands or the user may have
> populated), redirect the output to a dedicated file and read that file for results:
>
> ```bash
> git push <remote> <branch> > .tmp/push-output.txt 2>&1; echo "Exit: $?" >> .tmp/push-output.txt
> ```
>
> The `.tmp/` directory is git-ignored. Clean stale files periodically.

Run the pre-push script. **It must exit with code `0` before every push.**

```bash
./contrib/dev-tools/git/hooks/pre-push.sh
```

The script runs these steps in order:

1. `cargo +nightly fmt --check` - nightly format check
2. `cargo +nightly check ...` - nightly workspace check
3. `cargo +nightly doc ...` - nightly documentation build
4. `cargo +stable test --tests --benches --examples --workspace --all-targets --all-features` - all tests

Steps already covered by pre-commit (machete, linters, doc tests) are intentionally
omitted — they always run before each commit. E2E tests are excluded because they are
slow and run in CI, which is the merge authority.

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
- Pre-push: nightly toolchain checks + full stable test suite (no duplicates of pre-commit; no E2E)
- CI: merge authority with full validation and E2E matrix jobs

E2E tests are intentionally excluded from both pre-commit and pre-push. They run only in CI.
Pre-push does not repeat pre-commit steps — since every push is preceded by a commit, those
checks have already passed.

## Troubleshooting Output Modes

- Concise mode shows high-signal per-step summaries only. On failure, it prints the log path and
  a short failure tail.
- Verbose mode streams full command output to the terminal. Use this for deep local debugging.
- JSON mode emits one structured document to stdout; diagnostics and usage errors go to stderr.
- If concise output is too short for debugging, re-run the same command with
  `--format=text --verbosity=verbose`.

## Troubleshooting Long `git push` Runs

- If `git push` appears quiet, check whether the pre-push suite is still running before retrying.
- Do not assume SSH/GPG/passphrase prompts are the only cause of delay; long test phases are common.
- Only treat it as SSH idle-timeout after seeing explicit connection-close errors.
