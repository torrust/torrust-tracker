---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
last-updated-utc: 2026-09-26
---

# Manual Verification Evidence

## Environment

- Date and time (UTC): 2026-09-26 13:11-13:25
- Artifacts: `./target/debug/torrust-tracker`, `./target/debug/tracker_client`,
  and `./target/debug/examples/udp_only_public_tracker`
- Environment: Linux local Cargo workspace
- Toolchain: `rustc 1.100.0-nightly (6eeff9a52 2026-09-23)`
- Configuration: `.tmp/2342-udp-shutdown.toml` (one UDP tracker on
  `127.0.0.1:16969`, health check on `127.0.0.1:11342`, SQLite in `.tmp/`,
  `info` logging)

## M1 - Token-Driven UDP Shutdown

- Status: `DONE`

1. Built the tracker and client with `cargo build --bin torrust-tracker` and
   `cargo build -p torrust-tracker-client --bin tracker_client`.
2. Started the direct binary (`env` execs it, so `$!` is the tracker PID),
   waited for `/health_check`, and sent a real UDP announce.
3. Confirmed PID `2582491` was the `torrust-tracker` binary.
4. Sent `SIGTERM` to that exact PID and bounded exit waiting to 20 seconds.

<!-- cspell:ignore connrefused -->

```sh
env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH RUST_LOG=info \
   ./target/debug/torrust-tracker --config-toml-path .tmp/2342-udp-shutdown.toml > .tmp/2342-udp-first.log 2>&1 &
curl -s --fail --retry 50 --retry-connrefused --retry-delay 0 --max-time 2 http://127.0.0.1:11342/health_check
./target/debug/tracker_client udp announce udp://127.0.0.1:16969/announce 9c38422213e30bff212b30c360d26f9a02136422
kill -TERM 2582491
```

```text
first_PID=2582491 (torrust-tracker)
first_HEALTH_CHECK=OK
first_UDP_ANNOUNCE=OK
first_SIGTERM_EXIT=0
```

Announce response:

```json
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}
```

The first run log showed the ordered production path (span fields trimmed):

```text
UDP TRACKER: Starting on: 127.0.0.1:16969
UDP TRACKER: Started on: udp://127.0.0.1:16969
UDP TRACKER: Started UDP tracker service_binding=udp://127.0.0.1:16969
torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_instance_0_127.0.0.1:16969
torrust_tracker: Torrust tracker successfully shutdown.
```

The startup messages and target are unchanged, so the E2E log parser's
`Started on: udp://` pattern still matches. The `JobManager` recorded the UDP
component's cooperative-cancellation outcome, which it reports only when the
receive loop returns after observing its token. Log lines matching `error`
were configuration keys and metric names, not errors.

## M2 - UDP Listener Release

- Status: `DONE`

The identical command was started immediately after M1 on the same bindings,
with output in `.tmp/2342-udp-restart.log`. Readiness and a UDP announce were
confirmed again, and `kill -TERM 2582682` delivered the second signal.

```text
restart_PID=2582682 (torrust-tracker)
restart_HEALTH_CHECK=OK
restart_UDP_ANNOUNCE=OK
restart_SIGTERM_EXIT=0
```

The restarted process bound `127.0.0.1:16969` immediately, served the announce,
and logged the same startup lines, cooperative-cancellation outcome, and clean
shutdown.

## M3 - Legacy UDP Lifecycle

- Status: `DONE` (with a pre-existing example failure recorded for SI-17)

Automated coverage: the unchanged legacy contract and environment tests in
`packages/udp-server` pass (11 contract tests, plus the `Server::start` /
`Server::stop` unit tests), and the new legacy launcher regression tests pass.

Example run: started `udp_only_public_tracker`, announced against its ephemeral
port, and sent `SIGINT` (its Ctrl-C path) to PID `2596738`.

```text
PID=2596738 (udp_only_public) ADDR=127.0.0.1:54795
EXAMPLE_ANNOUNCE=OK
EXAMPLE_STOPPED=OK
EXAMPLE_REBIND=OK
```

The example served the announce, exited, and released its port, but its output
ended with a panic:

```text
Shutting down...
thread 'main' panicked at packages/udp-server/src/testing/environment.rs:214:14:
Failed to stop the UDP tracker server: FailedToStartOrStopServer("Normal")
```

The same run against a build of the baseline `develop` commit `0f1dcd28`
produced the identical panic, so SI-14 did not introduce it. Cause: `SIGINT`
reaches both the example's `ctrl_c()` wait and the legacy launcher's global
OS-signal branch. The launcher stops first and drops the halt receiver, so the
environment's later `Server::stop()` cannot send its halt message. This is the
EPIC's double-signal problem; SI-17 (migrating the environment and example to
the token-aware path) and SI-19 (removing library OS signals) remove it. The
note was added to the SI-17 draft.

## Conclusion

The executable signal boundary cancelled each migrated UDP component. The
receive loop stopped cooperatively, the component reported cooperative
cancellation, the tracker exited with status `0`, and the UDP socket was free
for an immediate restart. The legacy path keeps its behavior, including one
pre-existing example panic now tracked by SI-17.
