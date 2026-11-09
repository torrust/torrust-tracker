---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2769-2758-e2e-runners-json/ISSUE.md
last-updated-utc: 2026-11-09 12:12
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Verification Processes

### R0 - Reproduction of the deviation before the change (e2e_tests_runner)

- Goal: show the current behavior that the issue changes, as `fix-bug` requires, before the code changes.
- Status: `DONE` (reproduction only; the red regression test, the fix and the final like-for-like recheck are recorded when the issue is implemented)

#### Steps Performed

```text
$ target/debug/e2e_tests_runner --config-toml-path /nonexistent.toml --skip-build > out 2> err
exit=1
stdout (311 bytes):
2026-11-09T12:12:38.673505Z  INFO torrust_tracker_lib::console::ci::e2e::runner: Logging initialized
2026-11-09T12:12:38.673576Z  INFO torrust_tracker_lib::console::ci::e2e::runner: Reading tracker configuration from file: /nonexistent.toml ...
stderr (104 bytes):
Error: CSan't read config file /nonexistent.toml

Caused by:
    No such file or directory (os error 2)
```

#### Observed Result

`tracing` output (with ANSI colors) goes to stdout, and the failure is an `anyhow` `Error:` line in plain text on stderr, exit `1`. The Docker, Compose and checker child processes and the qBittorrent runner were not run here (they need Docker and a built image); the inherited streams are read from `src/console/ci/e2e/docker.rs`, `src/console/ci/compose.rs` and `src/console/ci/e2e/tracker_checker.rs` (`.status()` calls).
