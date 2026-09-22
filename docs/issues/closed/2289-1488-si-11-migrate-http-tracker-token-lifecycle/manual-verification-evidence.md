---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2289-1488-si-11-migrate-http-tracker-token-lifecycle/ISSUE.md
last-updated-utc: 2026-09-22 17:43
---

# Manual Verification Evidence

## Environment

- Date and time (UTC): 2026-09-22 11:13
- Artifact: `./target/debug/torrust-tracker`
- Environment: Linux local Cargo workspace
- Configuration: `.tmp/2274-tracker-shutdown.toml`
- HTTP tracker binding: `127.0.0.1:17070`
- Health-check binding: `127.0.0.1:11313`

## V1 - Direct Tracker SIGTERM

- Status: `DONE`

1. Built the tracker with `cargo build --bin torrust-tracker`.
2. Started the direct binary with the isolated configuration and waited for the
   health check to return success.
3. Sent `SIGTERM` to the exact binary PID and waited for exit.
4. Repeated the same start, readiness, signal, and exit sequence on the same
   bindings.

```text
first_PID=572447
first_HEALTH_CHECK=OK
first_SIGTERM_EXIT=0
```

The first run log showed the ordered production path:

```text
Tracker shutdown signal handlers installed.
Torrust tracker shutting down (SIGTERM) ...
graceful_shutdown_on_cancellation ... Shutting down HTTP server on socket address: 127.0.0.1:17070 in 90s
All connections closed, shutting down server in address 127.0.0.1:17070
Torrust tracker successfully shutdown.
```

## V2 - HTTP Listener Release

- Status: `DONE`

The same direct-binary command was started again immediately after V1 on the
same HTTP binding.

```text
restart_PID=572558
restart_HEALTH_CHECK=OK
restart_SIGTERM_EXIT=0
HTTP_LISTENER_REBIND=OK
```

The restarted process logged the same token-aware HTTP drain and clean shutdown
after receiving `SIGTERM`.

## Conclusion

The executable signal boundary cancelled the migrated HTTP component. The
HTTP tracker used the token-aware drain helper, drained cleanly, exited with
status `0`, and released its listener for the immediate restart.

## V3 - Legacy HTTP Lifecycle Compatibility (Automated)

- Status: `AUTOMATED_ONLY`
- Verification: automated, because the running tracker now deliberately uses
   the token-aware path and has no executable configuration switch for the
   retained legacy API.
- Command: `cargo test -p torrust-tracker-axum-http-server it_should_preserve_the_launcher_bind_address_after_starting_and_stopping`
- Toolchain: nightly Rust `1.100.0`

The unchanged `HttpServer::start` and `HttpServer::stop` path started and
stopped successfully while preserving its configured bind address.
