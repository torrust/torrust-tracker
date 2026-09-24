---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2324-1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
last-updated-utc: 2026-09-24
---

# Manual Verification Evidence

## Environment

- Date and time (UTC): 2026-09-24 16:13
- Artifact: `./target/debug/torrust-tracker`
- Environment: Linux local Cargo workspace
- Toolchain: `rustc 1.100.0-nightly (6eeff9a52 2026-09-23)`
- Configuration: `.tmp/2324-health-check-shutdown.toml` (one HTTP tracker on
  `127.0.0.1:17072`, SQLite in `.tmp/`, no REST API or UDP tracker)
- Health-check binding: `127.0.0.1:11324`

## M1 - Token-Driven Health-Check API Shutdown

- Status: `DONE`

1. Built the tracker with `cargo build --bin torrust-tracker`.
2. Started the direct binary (`env` execs it, so `$!` is the tracker PID) and
   waited for `/health_check` to return success.
3. Confirmed PID `561982` was the `torrust-tracker` binary.
4. Sent `SIGTERM` to that exact PID and bounded exit waiting to 20 seconds.

<!-- cspell:ignore connrefused -->

```sh
env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH RUST_LOG=info \
   ./target/debug/torrust-tracker --config-toml-path .tmp/2324-health-check-shutdown.toml > .tmp/2324-health-check-first.log 2>&1 &
curl -s --fail --retry 50 --retry-connrefused --retry-delay 0 --max-time 2 http://127.0.0.1:11324/health_check
kill -TERM 561982
```

```text
first_PID=561982 (torrust-tracker)
first_HEALTH_CHECK=OK
first_SIGTERM_EXIT=0
```

The first run log showed the ordered production path (span fields trimmed):

```text
HEALTH CHECK API: Starting on: http://127.0.0.1:11324
HEALTH CHECK API: Started health check API service_role="health_check_api" instance_index=0 ...
HEALTH CHECK API: Started on: http://127.0.0.1:11324
torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
graceful_shutdown_on_cancellation{address=127.0.0.1:11324 drain_timeout=5s}: !! Shutting down health check API server on socket address: 127.0.0.1:11324 in 5s
graceful_shutdown_on_cancellation{address=127.0.0.1:11324 drain_timeout=5s}: All connections closed, shutting down server in address 127.0.0.1:11324
HEALTH CHECK API: Stopped server running on: http://127.0.0.1:11324
bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api
torrust_tracker: Torrust tracker successfully shutdown.
```

The startup log messages and target are unchanged, so the E2E log parser's
`Started on:` pattern still matches.

## M2 - Health-Check API Listener Release

- Status: `DONE`

The identical command was started immediately after M1 on the same bindings,
with output in `.tmp/2324-health-check-restart.log`. Readiness was confirmed with
the same health-check command, and `kill -TERM 562070` delivered the second
signal.

```text
restart_PID=562070 (torrust-tracker)
restart_HEALTH_CHECK=OK
restart_SIGTERM_EXIT=0
```

The restarted process bound `127.0.0.1:11324` immediately, served the health
check, logged the same token-aware drain path, and reported
`job=health_check_api` as completed after cooperative cancellation.

## M3 - Legacy Health-Check Lifecycle Compatibility

- Status: `DONE`

The migrated binary has no legacy-path switch. The unchanged
`Halted`-based `server::start` path is exercised by the package's
`environment::Started` start/stop contract tests in
`packages/axum-health-check-api-server/tests/server/contract.rs` (8 passed).

## Conclusion

The executable signal boundary cancelled the migrated health-check component.
The health-check API drained through the token-aware helper within its 5-second
budget, reported one named `health_check_api` outcome, exited with status `0`,
and released its listener for the immediate restart.
