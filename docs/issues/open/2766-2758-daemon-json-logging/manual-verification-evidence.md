---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md
last-updated-utc: "2026-11-27 06:30"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build
- Second session (R0b and R0c, task T3): 2026-11-27 06:29 UTC; the unchanged tool built from the `develop` merge commit `786b46fee` (the merge of the spec pull request, no code of this issue yet) with `rustc 1.100.0` and `cargo 1.100.0`; binary `target/debug/torrust-tracker` built with `cargo build --bin torrust-tracker` and run with stdout and stderr redirected to files; configuration: one HTTP tracker and the health check API on `127.0.0.1` ports, `trace_filter = "info"`, an SQLite database in a scratch directory

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

A configuration failure prints one plain-text line on stderr (`Tracker startup failed: ...`) and exits `1`; the contract says a configuration error is a usage error (exit `2`) and the line must be JSON. clap's argument error is plain text on stderr with exit `2`, and `--help` prints plain text on stdout: both are outside the JSON contract (Copilot's `Cli::parse()` finding). These first three cases were run on 2026-11-09. A normal run and a bind failure, which need a configuration with ports, were run later, in the second session (R0b and R0c below).

### R0b - Normal run (torrust-tracker daemon)

- Goal: show where the daemon writes its logs and in which format when it runs normally.
- Status: `DONE` (reproduction only)

#### Steps Performed

```text
$ target/debug/torrust-tracker -c tracker.toml > out 2> err &
$ curl -s -o /dev/null -w "health=%{http_code}\n" http://127.0.0.1:17313/health_check
health=200
$ kill -TERM <pid of the tracker>
stdout: 17778 bytes at the time of the check (before the stop)
stderr: 0 bytes
```

The stdout lines, with the ANSI colour codes removed and the long lines cut (the `Configuration:` record spans many lines, because it carries the indented JSON of the configuration):

```text
2026-11-27T06:29:20.573733Z  INFO torrust_tracker_configuration::v3_0_0::logging: Logging initialized
2026-11-27T06:29:20.573757Z  INFO torrust_tracker_lib::bootstrap::app: Configuration:
2026-11-27T06:29:20.578664Z  INFO initialize: METRICS: type="counter" name="swarm_coordination_registry_torrents_added_total" ...
2026-11-27T06:29:20.596891Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-11-27T06:29:24.604220Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HTTP TRACKER: request server_socket_addr=127.0.0.1:17070 service_binding=http://127.0.0.1:17070/ method=GET ur...
2026-11-27T06:29:38.873328Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-11-27T06:29:38.873395Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=25
2026-11-27T06:29:38.875257Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

#### Observed Result

Every log record goes to stdout (17778 bytes) and none to stderr (0 bytes), in the `full` text style with colour codes. The first record is `Logging initialized`: nothing before it is logged, so the records of the configuration loading are not written. The process stopped after the `SIGTERM` with the "successfully shutdown" record; its exit code was not recorded, because the process was started in the background and stopped from another shell command. The contract wants nothing on stdout, and JSON records on stderr.

### R0c - Runtime startup failure (torrust-tracker daemon)

- Goal: show how the daemon reports a failure that happens after the logging started, and its exit code.
- Status: `DONE` (reproduction only)

#### Steps Performed

```text
(a script listens on 127.0.0.1:17071, and tracker.toml has the HTTP tracker on that address)
$ target/debug/torrust-tracker -c tracker.toml > out 2> err
exit=1
stdout: 17118 bytes; its last line, with the colour codes removed and cut:
2026-11-27T06:29:28.653056Z ERROR torrust_tracker: Tracker startup failed error=Configured HTTP tracker startup failed. Correct its configuration or make its listener address available: Could not start the HTTP tracker l...
stderr: 290 bytes:
Tracker startup failed: Configured HTTP tracker startup failed. Correct its configuration or make its listener address available: Could not start the HTTP tracker listener. Check that its bind address is available: could not bind HTTP tracker listener: Address already in use (os error 98)
```

#### Observed Result

The failure is reported twice: as an `ERROR` record on stdout, after the startup records, and as a plain-text line on stderr. The exit code is `1`, which fits a runtime failure (decision D5 keeps `1` for it). The stderr line is plain text, not JSON, and the stdout record is not JSON either.
