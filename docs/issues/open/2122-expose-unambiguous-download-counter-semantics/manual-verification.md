# Manual Verification Evidence

**Date:** 2026-09-01 12:05 UTC
**Tracker revision:** `194676344628c10a5fd34f1cb8fe5372a2a97db2`
**Issue:** #2122

## Safety

Do not record API tokens, passwords, private keys, connection strings, or other
secrets. Replace each secret with `{REDACTED}` in commands, configuration, and
HTTP requests.

## Local Environment

- Operating system: Linux
- Tracker command: `cargo run -- --config {CONFIG_PATH}`
- Working directory: repository root
- Configuration source: `TORRUST_TRACKER_CONFIG_TOML_PATH={CONFIG_PATH}`
- Configuration: local-only, API-enabled v3 configurations in `.tmp/`. M1
  omits `[core.database]` and sets `persistent_torrent_completed_stat = false`.
  M2 uses SQLite at
  `./storage/tracker/lib/database/issue-2122.sqlite3.db` and sets it to `true`.

## M1 - Disabled Persistence

**Status:** `DONE`

### Commands

```text
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/issue-2122-no-persistence.toml" cargo run --bin torrust-tracker
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp announce 127.0.0.1:16969 2122212221222122212221222122212221222122 --event started --uploaded 0 --downloaded 0 --left 1000 --port 6881 --peer-id ABCDEFGHIJKLMNOPQRST --key 1 --peers-wanted 0
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp announce 127.0.0.1:16969 2122212221222122212221222122212221222122 --event completed --uploaded 0 --downloaded 1000 --left 0 --port 6881 --peer-id ABCDEFGHIJKLMNOPQRST --key 1 --peers-wanted 0
curl --fail --silent --show-error 'http://127.0.0.1:11212/api/v1/stats?token={REDACTED}'
curl --fail --silent --show-error 'http://127.0.0.1:11212/api/v1/metrics?token={REDACTED}&format=prometheus'
{stop, restart with the same configuration, and repeat the requests}
```

### Requests And Responses

```text
GET /api/v1/stats?token={REDACTED} -> 200 OK before completion:
{"completed":0,"completed_in_session":0,"completed_persisted":0,"completed_persisted_enabled":false,...}

GET /api/v1/stats?token={REDACTED} -> 200 OK after completion:
{"completed":1,"completed_in_session":1,"completed_persisted":0,"completed_persisted_enabled":false,...}

GET /api/v1/stats?token={REDACTED} -> 200 OK after restart:
{"completed":0,"completed_in_session":0,"completed_persisted":0,"completed_persisted_enabled":false,...}

GET /api/v1/metrics?token={REDACTED}&format=prometheus -> 200 OK:
the legacy and `in_session` samples are present; no
`tracker_core_persisted_torrents_downloads_total` sample is present.
```

### Result

Passed. The completed-download transition increased the legacy and in-session
values to one. Restart reset both to zero. `completed_persisted` remained zero,
its availability flag remained false, and the persisted Prometheus sample was
absent.

## M2 - Enabled Persistence

**Status:** `DONE`

### Commands

```text
rm -f storage/tracker/lib/database/issue-2122.sqlite3.db
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/issue-2122-sqlite.toml" cargo run --bin torrust-tracker
curl --fail --silent --show-error 'http://127.0.0.1:11212/api/v1/metrics?token={REDACTED}&format=prometheus'
{send started and completed UDP announces with a distinct info hash and peer ID}
{stop, restart with the same configuration and SQLite path, then repeat the requests}
```

### Requests And Responses

```text
GET /api/v1/stats?token={REDACTED} -> 200 OK before completion:
{"completed":0,"completed_in_session":0,"completed_persisted":0,"completed_persisted_enabled":true,...}
Prometheus: tracker_core_persisted_torrents_downloads_total 0

GET /api/v1/stats?token={REDACTED} -> 200 OK after completion:
{"completed":1,"completed_in_session":1,"completed_persisted":1,"completed_persisted_enabled":true,...}
Prometheus: legacy, in-session, and persisted samples all equal 1.

GET /api/v1/stats?token={REDACTED} -> 200 OK after restart:
{"completed":1,"completed_in_session":0,"completed_persisted":1,"completed_persisted_enabled":true,...}
Prometheus: legacy and persisted samples equal 1; in-session equals 0.
```

### Result

Passed. The enabled persisted metric was exported at zero before any completion.
After completion its value became one. Restarting with the identical SQLite
path restored the legacy and persisted values to one while resetting the
in-session value to zero.

## M3 - Legacy Migration

**Status:** `DONE`

### Commands

```text
Repeat the authenticated stats and Prometheus metrics requests from M1 and M2.
```

### Requests And Responses

```text
Both requests returned 200 OK in each persistence mode. The legacy metric
description begins `Deprecated: use ...`; the explicit in-session and persisted
descriptions identify their retention behavior.
```

### Result

Passed. The legacy metric remained available with its documented conditional
value. The in-session metric reset per process; the persisted metric was omitted
when disabled and retained across the SQLite restart when enabled.
