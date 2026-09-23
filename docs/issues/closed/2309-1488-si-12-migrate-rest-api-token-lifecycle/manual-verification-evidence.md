---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2309-1488-si-12-migrate-rest-api-token-lifecycle/ISSUE.md
last-updated-utc: 2026-09-23
---

# Manual Verification Evidence

## Environment

- Date and time (UTC): 2026-09-23 09:33
- Artifact: `./target/debug/torrust-tracker`
- Environment: Linux local Cargo workspace
- Toolchain: `rustc 1.100.0-nightly (1303417c4 2026-09-21)`
- Configuration: `.tmp/2309-rest-api-shutdown.toml`
- REST API binding: `127.0.0.1:17121`
- Health-check binding: `127.0.0.1:11314`

## V1 - Direct Tracker SIGTERM

- Status: `DONE`

1. Built the tracker with `cargo build --bin torrust-tracker`.
2. Started the direct binary with the isolated configuration and waited for the
   health check to return success.
3. Confirmed PID `2175094` was the `torrust-tracker` binary.
4. Sent `SIGTERM` to that exact PID and bounded exit waiting to 20 seconds.

<!-- cspell:ignore connrefused -->

```sh
env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH RUST_LOG=info \
   ./target/debug/torrust-tracker --config-toml-path .tmp/2309-rest-api-shutdown.toml > .tmp/2309-rest-api-first.log 2>&1 &
curl --fail --retry 50 --retry-connrefused http://127.0.0.1:11314/health_check
kill -TERM 2175094
```

```text
first_PID=2175094
first_HEALTH_CHECK=OK
first_SIGTERM_EXIT=0
```

The first run log showed the ordered production path:

```text
Torrust tracker shutting down (SIGTERM) ...
graceful_shutdown_on_cancellation ... Shutting down tracker API server on socket address: 127.0.0.1:17121 in 90s
Torrust tracker successfully shutdown.
```

## V2 - REST API Listener Release

- Status: `DONE`

The same direct-binary command was started immediately after V1 on the same
REST API and health-check bindings.

The identical command was rerun with output in
`.tmp/2309-rest-api-restart.log`, readiness was confirmed with the same
health-check command, and `kill -TERM 2175197` delivered the second signal.

```text
restart_PID=2175197
restart_HEALTH_CHECK=OK
restart_SIGTERM_EXIT=0
REST_API_LISTENER_REBIND=OK
```

The restarted process again logged the REST API token-aware drain and exited
cleanly after receiving `SIGTERM`.

## Conclusion

The executable signal boundary cancelled the migrated REST API component. The
REST API used the token-aware drain helper, drained cleanly, exited with status
`0`, and released its listener for the immediate restart.
