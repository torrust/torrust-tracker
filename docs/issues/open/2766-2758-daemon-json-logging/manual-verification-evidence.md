---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md
last-updated-utc: 2026-11-09 12:12
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Verification Processes

### R0 - Reproduction of the deviation before the change (torrust-tracker daemon)

- Goal: show the current behavior that the issue changes, as `fix-bug` requires, before the code changes.
- Status: `DONE` (reproduction only; the red regression test, the fix and the final like-for-like recheck are recorded when the issue is implemented)

#### Steps Performed

```text
$ target/debug/torrust-tracker --config-toml-path /nonexistent.toml > out 2> err
exit=1
stdout (0 bytes):
(empty)
stderr (458 bytes):
Tracker startup failed: Tracker setup failed. Correct the reported configuration or dependency problem and restart: Tracker configuration could not be loaded. Correct the configuration source and restart: Could not prepare the tracker configuration source. Check `--config-toml-path`, `TORRUST_TRACKER_CONFIG_TOML_PATH`, or `TORRUST_TRACKER_CONFIG_TOML`: Unable to load explicit configuration file `/nonexistent.toml`: No such file or directory (os error 2)
```

```text
$ target/debug/torrust-tracker --unknown > out 2> err
exit=2
stdout (0 bytes):
(empty)
stderr (116 bytes):
error: unexpected argument '--unknown' found

Usage: torrust-tracker [OPTIONS]

For more information, try '--help'.
```

```text
$ target/debug/torrust-tracker --help > out 2> err
exit=0
stdout (247 bytes):
Command-line arguments accepted by the tracker executable

Usage: torrust-tracker [OPTIONS]

Options:
  -c, --config-toml-path <CONFIG_TOML_PATH>  Path to the TOML configuration file to load
  -h, --help                                 Print help
stderr (0 bytes):
(empty)
```

#### Observed Result

A configuration failure prints one plain-text line on stderr (`Tracker startup failed: ...`) and exits `1`; the contract says a configuration error is a usage error (exit `2`) and the line must be JSON. clap's argument error is plain text on stderr with exit `2`, and `--help` prints plain text on stdout: both are outside the JSON contract (Copilot's `Cli::parse()` finding). A normal run (tracing to stdout in the plain `full` style) and a bind failure were not run here: they need a tracker configuration with ports and are exercised when the issue is implemented; the stdout/plain-text tracing is read from `packages/configuration/src/v3_0_0/logging.rs` (`with_test_writer()`, ANSI).
