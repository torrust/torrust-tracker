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
- [x] The maintainer's approval ending the 0.5.x window (chat, 2026-11-10) and the `torrust-server-lib` 0.6.0
      release notes: torrust/torrust-server-lib#5, PR torrust/torrust-server-lib/pull/6 (`CHANGELOG.md`).
- [x] Q4/Q5 decisions and the deployment policy: `docs/features/shutdown-process/questions.md` and
      `docs/containers.md#stopping-the-tracker`.

## Source and Deterministic Tests

### Test 1: Legacy symbols and library signal handling are gone

- [x] Search the workspace for removed `Halted` shutdown symbols and legacy signal helpers.
- [x] Remaining matches are only archived migration history, ADRs, analysis and the migration guide.
- [x] No server-library module subscribes to SIGINT or SIGTERM.

**Evidence:**

```text
$ git grep -nE "Halted|shutdown_signal_with_message|global_shutdown_signal|HaltSignal|NestedServerTask|run_with_graceful_shutdown|spawn_launcher|LaunchRequest|RunningHttpServer|RunningApiServer|RunningUdpServer" -- '*.rs' '*.toml'
(no output, exit 1)

$ git grep -nE "tokio::signal|SignalKind|ctrl_c" -- '*.rs' | grep -vE "^(src/main.rs|src/console/profiling.rs|packages/[a-z-]+/examples/)"
console/tracker-client/src/console/clients/checker/monitor/udp.rs:178:  tokio::signal::ctrl_c()
console/tracker-client/src/console/clients/checker/monitor/udp.rs:231:  tokio::signal::ctrl_c()

$ git grep -nlE "Halted|HaltSignal|NestedServerTask|global_shutdown_signal|run_with_graceful_shutdown" -- . ':!docs/issues' ':!docs/pr-reviews'
docs/adrs/ (3 files), docs/analysis/..., docs/features/shutdown-process/ (README, legacy-api-migration, questions, task-inventory),
docs/media/*.svg, docs/research/...
```

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

**Evidence:** the script `e2e.sh` execs the binary in the background (so `$!` is the direct PID), waits for readiness,
sends the signal to that PID, waits at most 20 s, then records the exit status and the listeners left for the PID.

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
