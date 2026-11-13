---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2844-1488-si-25-remove-unused-error-variants/ISSUE.md
last-updated-utc: "2026-11-13 23:06"
---

<!-- cspell:ignore lntu -->

# Verification Evidence - Remove Unused Shutdown Error Variants (#2844)

> **Status**: Verified on 2026-11-13 (M1 to M3).

## Environment

- OS: Linux 7.0.0-43-generic
- Tracker commit/branch: `2844-remove-unused-error-variants` (based on `torrust/develop` at `24230dd01`)

## M1 - Variants are gone

T1 on the base (`torrust/develop` at `24230dd01`) found each variant only at its definition and in the `udp-server` doc comment. At this head:

```text
$ git grep -nE "StartupNotification|FailedToStartOrStopServer|UdpError::Launcher|Error::Stop\b" -- '*.rs' '*.md' ':!docs/issues' ':!docs/pr-reviews'
(exit 1)
```

The `enum UdpError` and each package's `enum Error` were read after the edit: they hold only `Bind`, `Listener`
(HTTP and REST) and `Registration`. The `Launcher` structs and `Stopped` states are unrelated and untouched.

## M2 - Startup and shutdown unchanged

Configuration: `share/default/config/tracker.development.sqlite3.toml` with only the database path changed to a
temporary file (`.tmp/m/db.sqlite`). It enables two UDP trackers (6868, 6969), two HTTP trackers (7070, 7171), the REST
API (1212) and, always, the health check API (127.0.0.1:1313): six listeners.

Output below is trimmed, not verbatim: log lines are the message part of the relevant `Started on` lines (timestamps and
span fields removed, the UDP lines omitted); `ss -lntu` columns are reduced to protocol and local address (it also prints
state, queues and peer); the `HH:MM:SS` stamps come from `date -u +%T` calls run at the start and again after the signal and wait of each run (two stamps per run are kept), and are shown as trailing comments.

```text
$ export TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/m/t.toml
$ date -u +%T; ./target/debug/torrust-tracker & P=$!; sleep 4        # 22:50:04 start
  log: API: Started on: http://0.0.0.0:1212
       HEALTH CHECK API: Started on: http://127.0.0.1:1313
       HTTP TRACKER: Started on: http://0.0.0.0:7070
       HTTP TRACKER: Started on: http://0.0.0.0:7171
$ kill -TERM $P; wait $P; echo exit=$?                              # 22:50:08
exit=0

# restart on the same ports
$ ./target/debug/torrust-tracker & P=$!; sleep 4                     # 22:50:14
$ ss -lntu | grep -E ":(6868|6969|7070|7171|1212|1313) "
udp 0.0.0.0:6868
udp 0.0.0.0:6969
tcp 0.0.0.0:1212
tcp 0.0.0.0:7171
tcp 0.0.0.0:7070
tcp 127.0.0.1:1313
$ kill -TERM $P; wait $P; echo exit=$?                              # 22:50:18
exit=0
```

All six listeners rebound, including the health check, and both runs exited 0 on `SIGTERM`.

## M3 - Startup errors still reported

```text
$ python3 -c "import socket,time;s=socket.socket();s.setsockopt(1,2,1);s.bind(('0.0.0.0',1212));s.listen();time.sleep(10)" &
$ ./target/debug/torrust-tracker; echo exit=$?
ERROR torrust_tracker: Tracker startup failed error=Configured tracker API startup failed. ... could not bind tracker API listener: Address already in use (os error 98)
exit=1
```

## Automatic checks

Toolchain: `rustc` stable of the repository toolchain on Linux 7.0.0-43-generic. `cargo check --workspace --all-targets`
warning-free; `cargo test --workspace`: 4024 passed, 0 failed; `linter all` and the pre-push checks passed.
