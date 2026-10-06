---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md
last-updated-utc: "2026-10-06 15:51"
---

# Manual Verification Evidence - Standalone UDP Environment and Example

<!-- cspell:ignore lunp -->

## Purpose

Record real, human-oriented verification of the SI-17 migration. Every command
and output below was actually run and observed.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-06 15:08-15:11 (T0)
- Artifact under test: `develop` at `693162bb8` (merge of #2451), branch
  `2448-migrate-standalone-udp-environment` before any change
- Operating system / environment: Linux 7.0.0-34-generic
- Toolchain: `rustc 1.101.0-nightly (ea137335b 2026-10-05)`,
  `cargo 1.101.0-nightly (f3865b2a4 2026-09-29)`
- Prerequisites and setup performed:
  `cargo build -p torrust-tracker-udp-server --example udp_only_public_tracker`;
  the binary `target/debug/examples/udp_only_public_tracker` is run directly so
  signals reach its own PID.

## T0 Baseline Test Results

Before any change, both affected packages pass:

```text
$ cargo test -p torrust-tracker-udp-server
test result: ok. 212 passed; 0 failed   (unit, 2.17s)
test result: ok. 12 passed; 0 failed    (integration, 5.05s)
test result: ok. 1 passed; 0 failed     (doc)
real 0m13.849s

$ cargo test -p torrust-tracker-axum-health-check-api-server
test result: ok. 3 passed; 0 failed
test result: ok. 8 passed; 0 failed     (3.05s)
```

## Verification Processes

### V1 - Baseline SIGINT and SIGTERM (M1)

- Goal: record how the example reacts to each signal before T3.
- Initial state: example started, `Listening on` printed.
- Status: `DONE`

#### Steps Performed

1. Start the binary, wait for `Listening on`, send `kill -INT <pid>`, wait,
   record the exit code.
2. Start it again, wait for `Listening on`, send `kill -TERM <pid>`; if it is
   still running after 5 s, send `kill -INT <pid>`; record the exit code.
3. Repeat step 2 and inspect the UDP socket with `ss -lunp` before and 2 s
   after SIGTERM.

#### Observed Result

SIGINT:

```text
Listening on 127.0.0.1:51273
Press Ctrl-C to stop.
Shutting down...
thread 'main' (1723184) panicked at packages/udp-server/src/testing/environment.rs:214:14:
Failed to stop the UDP tracker server: FailedToStartOrStopServer("Normal")
exit=101
```

SIGTERM, then SIGINT after 5 s:

```text
Listening on 127.0.0.1:38106
Press Ctrl-C to stop.
still running 5 s after SIGTERM; sending INT
Shutting down...
thread 'main' (1723265) panicked at packages/udp-server/src/testing/environment.rs:214:14:
Failed to stop the UDP tracker server: FailedToStartOrStopServer("Normal")
exit=101
```

Socket after SIGTERM:

```text
addr=127.0.0.1:42777
before:
127.0.0.1:42777
2 s after SIGTERM: alive=yes
(no socket listed: released)
exit=101   (after the follow-up SIGINT)
```

#### Conclusion

Matches spec fact 1. Both signals reach the legacy launcher's
`global_shutdown_signal`, which stops the UDP server first. On SIGINT the
example's own `ctrl_c()` also fires, and `Environment::stop()` panics because
the halt channel is already closed. On SIGTERM the server stops and releases
its socket, but the process keeps running; the later SIGINT panics the same
way. T1 (token-aware path) and T3 (signal handling in `main`) remove both
causes.

### V2 - Announce, then SIGTERM (M2)

- Goal: after T3, SIGTERM stops the example in order and it exits 0.
- Initial state: tree with the commit `refactor(udp-server): [#2448] migrate
  the UDP test environment to the token lifecycle` plus the uncommitted T3
  example change; same toolchain as T0; binary rebuilt.
- Status: `DONE`

#### Steps Performed

1. Start `target/debug/examples/udp_only_public_tracker`, wait for
   `Listening on`.
2. `target/debug/tracker_client udp announce "udp://<addr>/announce" 9c38422213e30bff212b30c360d26f9a02136422`
3. `kill -TERM <pid>`; wait up to 10 s; record exit code; count UDP sockets
   still bound to the port with `ss -lun`.

#### Observed Result

```text
===== SIGTERM pid=1916474 addr=127.0.0.1:36311 15:51:21
announce rc=0
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}
alive after signal: no
exit=0
Listening on 127.0.0.1:36311
Press Ctrl-C (or send SIGTERM on Unix) to stop.
Received SIGTERM. Shutting down...
Stopped.
sockets still bound: 0
```

#### Conclusion

Met: the announce succeeds, `main` receives SIGTERM, `Environment::stop()`
joins every task, and the process exits 0 with the socket released.

### V3 - Announce, then SIGINT (M3)

- Goal: SIGINT follows the same path as SIGTERM.
- Initial state: as V2.
- Status: `DONE`

#### Steps Performed

As V2, with `kill -INT <pid>`.

#### Observed Result

```text
===== SIGINT pid=1916574 addr=127.0.0.1:56282 15:51:21
announce rc=0
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}
alive after signal: no
exit=0
Listening on 127.0.0.1:56282
Press Ctrl-C (or send SIGTERM on Unix) to stop.
Received SIGINT. Shutting down...
Stopped.
sockets still bound: 0
```

#### Conclusion

Met: same orderly stop as V2; the T0 panic (exit 101) is gone.

### V4 - No direct library signal subscription (M4)

- Goal: no UDP library module subscribes to OS signals directly.
- Initial state: as V2.
- Status: `DONE`

#### Steps Performed

1. `rg -n 'tokio::signal' packages/udp-server/src`
2. `rg -n 'global_shutdown_signal|shutdown_signal' packages/udp-server/src`

#### Observed Result

```text
(no matches; rg exit 1)
packages/udp-server/src/server/launcher.rs:15:use torrust_server_lib::signals::{Halted, Started, global_shutdown_signal};
packages/udp-server/src/server/launcher.rs:523:        () = global_shutdown_signal() => ...
```

#### Conclusion

Met for direct subscriptions. The legacy launcher still subscribes indirectly
through `torrust-server-lib` until SI-19, as AC8 states; the environment and
example no longer use that path.

## Failures and Follow-up

None yet.
