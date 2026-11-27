---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md
last-updated-utc: "2026-11-27 06:47"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build
- Second session (R0b and R0c, task T3): 2026-11-27 06:29 UTC; the unchanged tool built from the `develop` merge commit `786b46fee` (the merge of the spec pull request, no code of this issue yet) with `rustc 1.100.0` and `cargo 1.100.0`; binary `target/debug/torrust-tracker` built with `cargo build --bin torrust-tracker` and run with stdout and stderr redirected to files; configuration: one HTTP tracker and the health check API on `127.0.0.1` ports, `trace_filter = "info"`, an SQLite database in a scratch directory

### Setup of R0b, R0c and R1

A scratch directory with two subdirectories, `ok` and `busy`, each with a `tracker.toml` (below, with `<SCRATCH>` standing for the scratch directory; no secret is involved), and a variant `off.toml` of the first with `trace_filter = "off"` and the ports `17072` and `17315`.

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "<SCRATCH>/ok/sqlite3.db"

[[http_trackers]]
bind_address = "127.0.0.1:17070"
tracker_usage_statistics = false

[health_check_api]
bind_address = "127.0.0.1:17313"
```

The file of `busy` is the same with the database path in `busy` and the ports `17071` (HTTP tracker) and `17314` (health check API). The commands, from the scratch directory, with `B` the absolute path of `target/debug/torrust-tracker`. The directory is changed in its own command, not in the list that starts the tracker: in `cd ok && $B … &` the `&` applies to the whole list, `$!` is the shell that runs it, and the `kill` stops that shell and leaves the tracker running (this happened with the first run of R0b, which had to be stopped by its process id).

```text
$ cd ok
$ $B -c tracker.toml > out 2> err & P=$!
$ sleep 3; curl -s -o /dev/null -w "health=%{http_code}\n" http://127.0.0.1:17313/health_check
$ kill -TERM $P; wait $P; echo "exit=$?"          # only in R1; R0b did not record the exit code

# R0c: a listener holds the port of the HTTP tracker of `busy` for 8 seconds, then the tracker starts
$ cd ../busy
$ python3 -c 'import socket,time; s=socket.socket(); s.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1); s.bind(("127.0.0.1",17071)); s.listen(); time.sleep(8)' &
$ sleep 1                                          # what the runs used to let the listener bind
$ timeout 20 $B -c tracker.toml > out 2> err; echo "exit=$?"
```

The one-second pause is not a readiness check, and the 8-second listener of the recorded runs can end before a reader who types the commands one by one starts the tracker. To repeat R0c without either problem, one script binds the port first, runs the tracker while it holds the port, and prints the exit code; there is nothing to wait for and nothing to time. It was not the way the recorded runs were made:

```text
$ cd ../busy
$ B=<absolute path of target/debug/torrust-tracker> python3 - <<'PY'
import os, socket, subprocess
listener = socket.socket()
listener.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
listener.bind(("127.0.0.1", 17071))
listener.listen()
with open("out", "w") as out, open("err", "w") as err:
    result = subprocess.run([os.environ["B"], "-c", "tracker.toml"], stdout=out, stderr=err, timeout=20)
print("exit=%d" % result.returncode)
PY
```

The script fails with an error, and does not start the tracker, if the port cannot be bound; the tracker is stopped by the timeout if it does not exit within 20 seconds.

Nothing needs to be cleaned up: the listener of the recorded runs ends by itself after 8 seconds, the script above ends its listener when it exits, and the tracker of R0b and R1 is stopped by the `SIGTERM`.

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

### R1 - Logging installed first: red and green tests, and the recheck of R0b and R0c (first pull request)

- Goal: show that the tests of the new `bootstrap::logging` module and of the instrumentation fail before the change and pass after it, and that the real binary writes the same lines as in R0b and R0c, plus the records of the configuration loading.
- Status: `DONE` for the first pull request (tasks T2 to T4). The other scenarios of this file belong to the later pull requests.

#### Red

The module was first written as a stub that builds a subscriber without formatters and ignores the configuration (`cargo test --lib bootstrap::logging`):

```text
test result: FAILED. 4 passed; 9 failed; 0 ignored; 0 measured; 180 filtered out
```

The four that passed are the ones that expect nothing to be written (no debug record before the configuration, nothing when the filter is off), the mapping of the thresholds and the call of `configure` without an installed logging; the nine failures are the ones that expect a record. After the implementation, one test still failed, because the ANSI colour codes split `answer=42` in the captured text; the test helper now removes them. With the field formatter of the `compact` style temporarily replaced by the one of the `full` style, the test of the fields of a span fails (`left: 2`, `right: 1`: the fields are written twice), which is the reason for the separate field formatter in the module.

The two tests that guard the instrumentation, with the instrumentation as it was (`cargo test --lib bootstrap::app::tests`):

```text
the span records the fields ["configuration"]
the span records the fields ["explicit_config_toml_path"]
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 191 filtered out
```

The configuration appeared as a span field because the logging, now installed first, records the span of `initialize_global_services`: a real run showed the record `Logging initialized` prefixed with `setup{explicit_config_toml_path=Some("tracker.toml")}:initialize_global_services{configuration=Configuration { metadata: …`, the `Debug` form of the whole configuration, which holds the API access tokens.

#### Green

```text
$ cargo test --lib bootstrap
test result: ok. 101 passed; 0 failed
$ cargo test -p torrust-tracker-configuration -p torrust-tracker-udp-server -p torrust-tracker-axum-http-server -p torrust-tracker-axum-rest-api-server -p torrust-tracker-test-helpers -p torrust-tracker-http-core -p torrust-tracker-udp-core
(every test result: ok, 0 failed)
$ cargo test --test lifecycle-signals --test lifecycle-shutdown-policy --test lifecycle-fixture-shutdown-assertion --test cli-configuration --test metrics-statistics-disabled-logs --test metrics-port-zero
test result: ok. (35, 1, 35, 30, 1 and 27 tests) 0 failed
$ cargo clippy --workspace --all-targets   (no warning)
$ cargo machete   (nothing unused)
```

The six root test binaries start the real binary and read its output; they ran with the first code commit and again, with the same results, on the final tree of this pull request (2026-11-27 06:46 UTC).

#### Recheck of the real binary

Same configuration and commands as R0b and R0c, with the binary built from the final tree of this pull request (2026-11-27 06:45 UTC):

```text
normal run (stopped with SIGTERM, started with `& P=$!` so the exit code is known):
health=200
exit=0, stdout 27232 bytes, stderr 0 bytes
2026-11-27T06:45:04.694309Z  INFO torrust_tracker_configuration: Loading base configuration from explicit configuration file path="tracker.toml"
2026-11-27T06:45:04.694495Z  INFO torrust_tracker_lib::bootstrap::logging: Logging initialized
2026-11-27T06:45:04.694512Z  INFO torrust_tracker_lib::bootstrap::app: Configuration:
2026-11-27T06:45:07.701890Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-11-27T06:45:07.702096Z  INFO torrust_tracker: Torrust tracker successfully shutdown.

bind failure (127.0.0.1:17071 in use):
exit=1, stdout 17305 bytes, stderr 290 bytes
ERROR torrust_tracker: Tracker startup failed error=Configured HTTP tracker startup failed. …
stderr: Tracker startup failed: Configured HTTP tracker startup failed. …

trace_filter = "off":
exit=0, stdout 190 bytes, stderr 0 bytes
2026-11-27T06:45:08.765914Z  INFO torrust_tracker_configuration: Loading base configuration from explicit configuration file path="off.toml"
```

#### Observed Result

The lines are those of R0b and R0c: same stream (stdout), same `full` style, no span prefix on the records written during the setup (`Logging initialized`, `Configuration:`), the same exit code for the bind failure (`1` in R0c and in the recheck) and the same double report of the failure. The exit code of the normal run was not recorded in R0b, so this comparison does not cover it; the recheck records `0` after `SIGTERM`, and the lifecycle test `lifecycle-signals`, which asserts exit code `0` after `SIGTERM` and passes on the final tree, covers the same behavior. The difference is the first record, `Loading base configuration from explicit configuration file`, which was lost before. With `trace_filter = "off"` that one record is still written, because the level is `info` until the configuration is applied (decision D2); before this change the tracker wrote nothing in that case. Nothing of the configuration, secrets included, appears in a span.
