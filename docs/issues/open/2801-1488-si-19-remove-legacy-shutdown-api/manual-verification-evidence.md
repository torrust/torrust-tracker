<!-- cspell:ignore ltnup -->

# Verification Evidence — Legacy Shutdown API Removal (#2801)

> **Status**: Verified on 2026-11-10 (M1 to M5).

## Environment

- Date: 2026-11-10
- OS: Linux 7.0.0-43-generic
- Rust version (`rustc --version`): rustc 1.99.1 (58cf8f9c3 2026-10-13)
- Tracker commit/branch: `2801-remove-legacy-shutdown-api` (head recorded in the PR)
- `torrust-server-lib` breaking release/version: 0.6.0 (crates.io `max_version` 0.6.0)

## Start-Gate Evidence

- [x] SI-18 deprecation evidence: issue #2751, PRs 2752, 2793 and close-out 2797;
      `docs/issues/closed/2751-1488-si-18-deprecate-legacy-shutdown-api/manual-verification-evidence.md`.
- [x] Token-lifecycle migrations: SI-11 to SI-17, SI-23 (#2449) and SI-24 (#2450), all Done in the EPIC table
      `docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md`.
- [x] #1588 (closed) final task inventory.
- [x] The maintainer's approval ending the 0.5.x window (given in chat on 2026-11-10, with no issue or PR comment of
      its own) and the `torrust-server-lib` 0.6.0 release notes: torrust/torrust-server-lib#5, PR
      torrust/torrust-server-lib/pull/6 (`CHANGELOG.md`). The maintainer merged the spec PR torrust/torrust-tracker#2803, whose start gate records the approval.
- [x] Q4/Q5 decisions and the deployment policy: `docs/features/shutdown-process/questions.md` and
      `docs/containers.md#stopping-the-tracker`.

## Source and Deterministic Tests

### Test 1: Legacy symbols and library signal handling are gone

- [x] Search the workspace for removed `Halted` shutdown symbols and legacy signal helpers.
- [x] No use of the removed API remains. Descriptions of it match in archived migration history, ADRs, analysis and the migration guide, and in five lines of four live specs of other issues (listed below).
- [x] No server-library module subscribes to SIGINT or SIGTERM.

**Evidence:** output at this head. The last search covers the live specs under `docs/issues/open` and
`docs/issues/drafts`; its matches are old descriptions of the legacy API in specs of other issues, not uses of it.

```text
$ git grep -nE "Halted|shutdown_signal_with_message|global_shutdown_signal|HaltSignal|NestedServerTask|run_with_graceful_shutdown|spawn_launcher|LaunchRequest|RunningHttpServer|RunningApiServer|RunningUdpServer" -- "*.rs" "*.toml"
(exit 1)

$ git grep -nE "tokio::signal|SignalKind|ctrl_c" -- "*.rs" | grep -vE "^(src/main.rs|src/console/profiling.rs|packages/[a-z-]+/examples/)"
console/tracker-client/src/console/clients/checker/monitor/udp.rs:178:            _ = tokio::signal::ctrl_c() => {
console/tracker-client/src/console/clients/checker/monitor/udp.rs:231:            _ = tokio::signal::ctrl_c() => {

$ git grep -nlE "Halted|HaltSignal|NestedServerTask|global_shutdown_signal|run_with_graceful_shutdown" -- . ":!docs/issues" ":!docs/pr-reviews"
docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md
docs/analysis/20260716-shutdown-process/README.md
docs/features/shutdown-process/README.md
docs/features/shutdown-process/legacy-api-migration.md
docs/features/shutdown-process/questions.md
docs/features/shutdown-process/task-inventory.md
docs/media/flamegraph.svg
docs/media/flamegraph_generated_without_sudo.svg
docs/research/20260716-console-shutdown-patterns/README.md

$ git grep -nE "Halted|HaltSignal|NestedServerTask|global_shutdown_signal|run_with_graceful_shutdown|shutdown_signal_with_message|spawn_launcher|LaunchRequest" -- docs/issues/open docs/issues/drafts ":!docs/issues/open/2801*" ":!docs/issues/open/1488*" | sed 's/[[:space:]]*$//'
docs/issues/drafts/1488-si-9-improve-udp-shutdown/ISSUE.md:48:Replace the UDP server's shutdown `Halted` oneshot and OS-signal wait with a
docs/issues/drafts/1488-si-9-improve-udp-shutdown/ISSUE.md:112:      shutdown `Halted` oneshot or OS-signal listener.
docs/issues/drafts/simplify-udp-server-main-loop/ISSUE.md:99:- Changes to `run_with_graceful_shutdown`.
docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md:420:server wrappers already forward the manager token to their private `Halted::Normal` channels, but
docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/completion-plan.md:45:`CancellationToken`. Each wrapper forwards cancellation to its private `Halted::Normal` channel
```

The last search also finds the legacy draft `docs/issues/drafts/simplify-udp-server-main-loop/ISSUE.md`, which lists
`run_with_graceful_shutdown` as out of scope for a UDP main-loop refactor. The pre-commit frontmatter validator refuses to
stage that file (it has the legacy frontmatter shape and needs a full migration), so the line stays; it describes a
removed function and is not a use of it.

The two `ctrl_c` hits belong to the `tracker_client check` monitor, a command-line executable and not a server
library, so they stay at an executable boundary. `src/main.rs`, `src/console/profiling.rs` and the four examples
are executables.

### Test 2: Token lifecycle owns completion

- [x] Deterministic component tests using injected cancellation tokens run and pass.
- [x] Every component joins or deliberately aborts its owned child tasks (the token-aware tests of each package).
- [x] No test needs an OS signal to prove component cancellation.

**Evidence:**

```text
$ cargo test --workspace
exit=0; 119 test result lines; 3947 passed, 0 failed
$ cargo check --workspace --all-targets
(no warnings)
```

## End-to-End Evidence

- [x] Tracker binary: SIGINT and SIGTERM produce one application-owned shutdown sequence (one
      `Torrust tracker shutting down (SIGTERM|SIGINT)` line) with no duplicate library signal handling.
- [x] Standalone HTTP example (both variants): SIGINT and SIGTERM use the in-process stop path.
- [x] Standalone UDP example (both variants): SIGINT and SIGTERM use the in-process stop path.
- [x] Container/service-manager evidence uses the deadlines specified by Q4 (M5).
- [x] Process results match the Q3 exit-code policy (exit 0 for each stop).

**Evidence:** the script below (`e2e.sh`, run from the repository root as
`e2e.sh <name> <SIGINT|SIGTERM as INT|TERM> <command...>`) execs the binary in the background (so `$!` is the direct
PID), waits for readiness, sends the signal to that PID, waits at most 20 s, then records the exit status and the
listeners left for the PID. The tracker used `.tmp/2801-tracker.toml`, a copy of the development template with the
ports `28969`, `28070`, `28212` and `28313` on `127.0.0.1` and a temporary SQLite file.

```sh
#!/usr/bin/env bash
# usage: e2e.sh <name> <SIGNAL> <command...>   (run from repo root; command is exec'd so $! is the direct PID)
name=$1; sig=$2; shift 2
log=.tmp/2801-$name-$sig.log
env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH RUST_LOG=info "$@" > "$log" 2>&1 &
pid=$!
for i in $(seq 1 100); do
  if grep -qE "Listening on|Started health check API|started: " "$log" 2>/dev/null; then break; fi
  sleep 0.2
done
sleep 1
echo "$name/$sig: PID=$pid comm=$(cat /proc/$pid/comm 2>/dev/null)"
ss -ltnup 2>/dev/null | grep "pid=$pid," | awk '{print "  bound: "$1" "$5}'
kill -s "$sig" "$pid"
start=$(date +%s.%N)
for i in $(seq 1 200); do kill -0 "$pid" 2>/dev/null || break; sleep 0.1; done
if kill -0 "$pid" 2>/dev/null; then echo "  STILL RUNNING after 20s"; kill -KILL "$pid"; fi
wait "$pid"; code=$?
end=$(date +%s.%N)
LC_ALL=C printf "  exit=%s elapsed=%.1fs\n" "$code" "$(echo "$end - $start" | bc)"
echo "  remaining listeners for pid: $(ss -ltnup 2>/dev/null | grep -c "pid=$pid,")"
echo "  shutdown log lines: $(grep -ciE 'shutting down|shutdown' "$log")"
```

```text
tracker/INT: PID=2792986 comm=torrust-tracker
  bound: udp 127.0.0.1:28969
  bound: tcp 127.0.0.1:28212
  bound: tcp 127.0.0.1:28313
  bound: tcp 127.0.0.1:28070
  exit=0 elapsed=0.1s
  remaining listeners for pid: 0
  shutdown log lines: 11
http_only_public_tracker/INT: PID=2793876 comm=http_only_pub
  bound: tcp 127.0.0.1:56697
  exit=0 elapsed=0.0s
  remaining listeners for pid: 0
  shutdown log lines: 1
http_only_public_tracker_production_style/INT: PID=2794572 comm=http_only_pub
  bound: tcp 127.0.0.1:36379
  exit=0 elapsed=0.0s
  remaining listeners for pid: 0
  shutdown log lines: 1
udp_only_public_tracker/INT: PID=2794645 comm=udp_only_pub
  bound: udp 127.0.0.1:42783
  exit=0 elapsed=0.1s
  remaining listeners for pid: 0
  shutdown log lines: 1
udp_only_public_tracker_production_style/INT: PID=2796755 comm=udp_only_pub
  bound: udp 127.0.0.1:37830
  exit=0 elapsed=0.1s
  remaining listeners for pid: 0
  shutdown log lines: 1
tracker/TERM: PID=2796853 comm=torrust-tracker
  bound: udp 127.0.0.1:28969
  bound: tcp 127.0.0.1:28212
  bound: tcp 127.0.0.1:28313
  bound: tcp 127.0.0.1:28070
  exit=0 elapsed=0.1s
  remaining listeners for pid: 0
  shutdown log lines: 11
http_only_public_tracker/TERM: PID=2796969 comm=http_only_pub
  bound: tcp 127.0.0.1:48169
  exit=0 elapsed=0.0s
  remaining listeners for pid: 0
  shutdown log lines: 1
http_only_public_tracker_production_style/TERM: PID=2797137 comm=http_only_pub
  bound: tcp 127.0.0.1:37251
  exit=0 elapsed=0.0s
  remaining listeners for pid: 0
  shutdown log lines: 1
udp_only_public_tracker/TERM: PID=2797194 comm=udp_only_pub
  bound: udp 127.0.0.1:55070
  exit=0 elapsed=0.1s
  remaining listeners for pid: 0
  shutdown log lines: 1
udp_only_public_tracker_production_style/TERM: PID=2797268 comm=udp_only_pub
  bound: udp 127.0.0.1:36351
  exit=0 elapsed=0.1s
  remaining listeners for pid: 0
  shutdown log lines: 1

```

The ten invocations (the script removes the two config environment variables, so the tracker reads its configuration
only from `--config-toml-path`; `target/debug` holds the artifacts of `cargo build --bin torrust-tracker` and
`cargo build -p torrust-tracker-axum-http-server -p torrust-tracker-udp-server --examples`):

```sh
for s in INT TERM; do
  e2e.sh tracker $s ./target/debug/torrust-tracker --config-toml-path .tmp/2801-tracker.toml
  for ex in http_only_public_tracker http_only_public_tracker_production_style \
            udp_only_public_tracker udp_only_public_tracker_production_style; do
    e2e.sh $ex $s ./target/debug/examples/$ex
  done
done
```

The configuration file `.tmp/2801-tracker.toml`:

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false

[core.database]
driver = "sqlite3"
path = ".tmp/2801-sqlite3.db"

[application.shutdown]
deadline_in_secs = 25
http_drain_in_secs = 15
udp_request_drain_in_secs = 5
event_listener_drain_in_secs = 5

[[udp_trackers]]
bind_address = "127.0.0.1:28969"
tracker_usage_statistics = true

[[http_trackers]]
bind_address = "127.0.0.1:28070"
tracker_usage_statistics = true

[http_api]
bind_address = "127.0.0.1:28212"

[http_api.access_tokens]
admin = "MyAccessToken"

[health_check_api]
bind_address = "127.0.0.1:28313"
```

Tracker shutdown sequence for SIGTERM (filtered; one `shutting down` line):

```text
 INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
 INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_api
 INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api
 INFO torrust_tracker: Torrust tracker successfully shutdown.
```

Each example log shows exactly one `Received SIG...` and one `Stopped.` line.

### M5: Container

The image was built with `docker build --target debug --tag torrust-tracker:2801-debug --file Containerfile .`; the
in-image `cargo nextest run` ran 1778 tests, 1778 passed. An earlier build attempt failed once on
`server::contract::http::it_should_return_error_when_http_service_was_stopped_after_registration`
(`axum-health-check-api-server`), which passed in seven local runs of its package and in the rebuild: a flaky test, not
a regression (the test only reads a stopped service's port).

The tracker is container process 1; `docker stop -t 35` follows the deadline policy of
`docs/containers.md#stopping-the-tracker` (at least `deadline_in_secs` plus 5 s):

```text
TERM: running=true pid1=torrust-tracker
  exit=0 oom=false elapsed=0.2s shutdown_lines=1
2026-11-10T15:46:09.823733Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-11-10T15:46:09.824063Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
INT: running=true pid1=torrust-tracker
  exit=0 oom=false elapsed=0.1s shutdown_lines=1
2026-11-10T15:46:10.801573Z  INFO torrust_tracker: Torrust tracker shutting down (SIGINT) ...
2026-11-10T15:46:10.801837Z  INFO torrust_tracker: Torrust tracker successfully shutdown.
```

Exit code 0 matches the Q3 policy; the container is not killed (`oom=false`, no `137`).

## Summary

| Check                           | Result  | Evidence link or note |
| ------------------------------- | ------- | --------------------- |
| Start gate                      | Pass    | Start-Gate Evidence   |
| Legacy shutdown symbols removed | Pass    | Test 1                |
| No library OS signals           | Pass    | Test 1                |
| Deterministic lifecycle tests   | Pass    | Test 2                |
| Tracker signal handling         | Pass    | End-to-End Evidence   |
| Standalone signals              | Pass    | End-to-End Evidence   |
| Deployment verification         | Pass    | M5 (container)        |
