---
name: push-changes
description: Guide for pushing commits in the torrust-tracker project. Covers the push workflow, pre-push hook setup, and the SSH idle-timeout problem that can interrupt pushes when the pre-push hook runs long. Triggers on "push changes", "git push", "how to push", "push branch", "SSH timeout on push", or "Connection closed by remote host".
metadata:
  author: torrust
  version: "1.0"
---

# Pushing Changes

This skill guides you through the complete push process for the Torrust Tracker project.

## Quick Reference

```bash
# One-time setup: install the pre-push Git hook
./contrib/dev-tools/git/install-git-hooks.sh

# Push the current branch to its upstream remote
git push <remote> <branch>
```

### Restricted Agent Sandboxes

If a sandbox cannot write hook logs to `/tmp` or invokes Git hooks with a
reduced `PATH`, explicitly restore Cargo's conventional installation directory
and push with workspace-local hook logs:

```bash
PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH" \
TORRUST_GIT_HOOKS_LOG_DIR=.tmp \
git push <remote> <branch>
```

The repository hook also tries to restore Cargo. If Git still launches the hook
without usable Cargo in the restricted agent sandbox, rerun the push outside that
sandbox; do not bypass the hook. `.tmp/` is git-ignored and keeps hook logs inside
the workspace.

## Git Hook (Recommended Setup)

The repository ships a `pre-push` Git hook that runs
`./contrib/dev-tools/git/hooks/pre-push.sh` automatically on every `git push`. Install
it once after cloning:

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

When the pre-push hook is installed, `git push` itself becomes a long-running command
because it executes the full pre-push suite before uploading objects. On cold caches,
runtime can exceed the warm-cache expectation.

Recommended for AI-agent terminal execution:

- Prefer running `git push` with a **generous timeout** (at least 20 minutes).
- Do not treat sparse output as a hang too quickly; some phases can be quiet.
- Do not start a second `git push` while one is still running.
- Wait for terminal completion (exit code + final output) before retrying.

The pre-push script runs these steps in order:

1. `cargo +nightly fmt --check` — nightly format check
2. `cargo +nightly check ...` — nightly workspace check
3. `cargo +nightly doc ...` — nightly documentation build
4. `cargo +stable test --tests --benches --examples --workspace --all-targets --all-features` — all tests

Steps already covered by pre-commit (machete, linters, doc tests) are intentionally
omitted — they always run before each commit. E2E tests are excluded because they are
slow and run in CI, which is the merge authority.

## Check Tier Ownership

Check ownership is intentionally split by gate:

- Pre-commit: fast local gate (`cargo machete`, `linter all`, `cargo test --doc --workspace`)
- Pre-push: nightly toolchain checks + full stable test suite (no duplicates of pre-commit; no E2E)
- CI: merge authority with full validation and E2E matrix jobs

## SSH Idle-Timeout Problem

### Symptom

When running `git push`, you may see a connection error like:

```text
Connection to ssh.github.com closed by remote host.
fatal: the remote end hung up unexpectedly
```

### Root Cause

Git opens an SSH connection to GitHub **before** running the pre-push hook. If the hook
takes longer than GitHub's SSH idle timeout (~300 seconds), the connection is torn down
while the hook is still running. When Git tries to use the connection after the hook exits,
the push fails.

### Distinguish SSH timeout from normal long runtime

Not every quiet terminal indicates an SSH failure. Pre-push checks can run for several
minutes, especially on cold caches. Confirm failure from actual error output (for example,
"Connection to ssh.github.com closed by remote host") before concluding the push is broken.

### Fix 1 — SSH keep-alive (local developer machine)

Add the following to `~/.ssh/config` on your developer machine:

```text
Host ssh.github.com
  ServerAliveInterval 60
  ServerAliveCountMax 10
```

`ServerAliveInterval 60` sends a keep-alive packet every 60 seconds.
`ServerAliveCountMax 10` allows up to 10 unanswered keep-alives before
the client declares the connection dead (10 × 60 s = 600 s extra tolerance).

> **⚠️ Warning**: This fix is a local machine configuration change. It is not
> reproducible in automated or AI-agent environments (CI, GitHub Actions, remote
> codespaces) because those environments do not read your personal `~/.ssh/config`.
> In those environments the only reliable remedy is to ensure the pre-push hook
> completes well within 300 seconds.

### Choosing the Right Fix

| Environment                      | Recommended approach                              |
| -------------------------------- | ------------------------------------------------- |
| Personal developer machine       | Fix 1 (SSH keep-alive in `~/.ssh/config`)         |
| CI / GitHub Actions              | No fix needed — CI does not run the pre-push hook |
| AI agent / automated environment | Keep hook runtime < 300 s; do not rely on Fix 1   |

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

## Troubleshooting Output Modes

- Concise mode shows high-signal per-step summaries only. On failure, it prints the log path and
  a short failure tail.
- Verbose mode streams full command output to the terminal. Use this for deep local debugging.
- JSON mode emits one structured document to stdout; diagnostics and usage errors go to stderr.
- If concise output is too short for debugging, re-run the same command with
  `--format=text --verbosity=verbose`.
