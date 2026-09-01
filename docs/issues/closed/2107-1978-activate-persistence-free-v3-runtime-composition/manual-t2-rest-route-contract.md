# Manual T2 REST Route Contract Verification

## Scope

This evidence verifies the user-visible T2 behavior for disabled REST API
capabilities: authenticated key-management and whitelist requests return HTTP
`409 Conflict` with a JSON `ActionStatus::Err` response while the tracker runs
in public mode.

This is not M4's persistence-free verification. The tracker was deliberately
started with SQLite configured because the temporary compatibility bridge has
not yet been removed. M4 remains deferred until T3 creates an operational
persistence-free application graph.

## Environment

- Date: 2026-08-28 12:58 UTC
- Revision: uncommitted T2 work on
  `2107-activate-persistence-free-v3-runtime-composition`
- Configuration source:
  `share/default/config/tracker.development.sqlite3.toml`, supplied through
  `TORRUST_TRACKER_CONFIG_TOML`
- Capability configuration: `core.private = false`, `core.listed = false`
- Isolated persistence path: `./.tmp/manual-t2-rest-contract.sqlite3.db`
- REST API address: `http://127.0.0.1:1212`

## Procedure

1. Confirmed that ports `1212`, `6868`, `6969`, `7070`, and `7171` were free.
2. Started the tracker locally with the template database path replaced only by
   the isolated `.tmp` path:

   ```sh
   TORRUST_TRACKER_CONFIG_TOML="$(sed 's|path = "./storage/tracker/lib/database/sqlite3.db"|path = "./.tmp/manual-t2-rest-contract.sqlite3.db"|' share/default/config/tracker.development.sqlite3.toml)" cargo run --bin torrust-tracker
   ```

3. Sent the following authenticated requests from a second terminal:

   ```sh
   curl --silent --show-error --write-out '\nHTTP %{http_code}\n' --header 'content-type: application/json' --data '{"key":null,"seconds_valid":60}' 'http://127.0.0.1:1212/api/v1/keys?token=MyAccessToken'
   curl --silent --show-error --write-out '\nHTTP %{http_code}\n' --request POST 'http://127.0.0.1:1212/api/v1/whitelist/9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d?token=MyAccessToken'
   curl --silent --show-error --write-out '\nHTTP %{http_code}\n' 'http://127.0.0.1:1212/api/health_check?token=MyAccessToken'
   ```

4. Stopped the tracker with `Ctrl-C` and confirmed graceful shutdown in the
   tracker logs.

## Observed Results

```text
key: {"status":"err","reason":"private capability is disabled by configuration"}
HTTP 409
whitelist: {"status":"err","reason":"listed capability is disabled by configuration"}
HTTP 409
health: {"status":"Ok"}
HTTP 200
```

The tracker logs independently recorded the same HTTP status codes for both
disabled-capability requests and a clean shutdown.

## Result

PASS. A locally running tracker exposes the disabled capability contract as
HTTP `409 Conflict` with the JSON `ActionStatus::Err` shape, while unrelated
API availability remains intact. The automatic contracts provide the stronger
no-persistence-access proof by forcing database failure before each request.
