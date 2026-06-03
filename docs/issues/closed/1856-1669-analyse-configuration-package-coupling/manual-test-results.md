# Manual Test Results — Issue #1856 Examples

**Date**: 2026-06-01
**Branch**: `1856-analyse-configuration-package-coupling`
**Tested by**: Jose Celano

This document records evidence that both Cargo examples added in Step 3 of the
[issue spec](./ISSUE.md) start a real tracker and successfully handle a client
announce request.

---

## Test 1 — UDP-only public tracker

### Start the tracker

```bash
cargo run --example udp_only_public_tracker -p torrust-tracker-udp-server
```

**Startup output** (truncated to relevant lines):

```text
Types from torrust-tracker-configuration compiled into this binary:
  Used at runtime    : Core, UdpTracker, Logging
  Full aggregate     : Configuration (required by the initialization entry point)
  Compiled but idle  : HttpTracker, HttpApi, HealthCheckApi, TslConfig, AccessTokens

2026-06-01T17:52:39.454411Z  INFO run_with_graceful_shutdown{cookie_lifetime=120s}: UDP TRACKER: Starting on: 127.0.0.1:0
2026-06-01T17:52:39.454453Z  INFO run_with_graceful_shutdown{cookie_lifetime=120s}: UDP TRACKER: Started on: udp://127.0.0.1:55078
Listening on 127.0.0.1:55078
Press Ctrl-C to stop.
```

The OS assigned port **55078**.

### Send an announce request

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- \
  udp announce udp://127.0.0.1:55078/announce \
  9c38422213e30bff212b30c360d26f9a02136422
```

**Client output**:

```json
{
  "AnnounceIpv4": {
    "transaction_id": -888840697,
    "announce_interval": 120,
    "leechers": 0,
    "seeders": 1,
    "peers": []
  }
}
```

### Result

| Check                                           | Outcome |
| ----------------------------------------------- | ------- |
| Tracker starts and binds successfully           | PASS    |
| Coupling table printed on startup               | PASS    |
| Client receives a valid `AnnounceIpv4` response | PASS    |
| `announce_interval` matches config (120 s)      | PASS    |
| Peer registered as seeder (`seeders: 1`)        | PASS    |

---

## Test 2 — HTTP-only public tracker

### Start the tracker

```bash
cargo run --example http_only_public_tracker -p torrust-tracker-axum-http-server
```

**Startup output** (truncated to relevant lines):

```text
Types from torrust-tracker-configuration compiled into this binary:
  Used at runtime    : Core, HttpTracker, Logging
  Full aggregate     : Configuration (required by the initialization entry point)
  Compiled but idle  : UdpTracker, HttpApi, AccessTokens, HealthCheckApi

Cross-layer coupling: rest-api-core imports both HttpTracker and UdpTracker
  to expose tracker status via the REST API.  A package split would not
  eliminate this dependency — the REST API needs all service config types.

2026-06-01T17:53:45.931752Z  INFO start: HTTP TRACKER: Starting on: http://127.0.0.1:35011
2026-06-01T17:53:45.931849Z  INFO start: HTTP TRACKER: Started on: http://127.0.0.1:35011
Listening on 127.0.0.1:35011
Press Ctrl-C to stop.
```

The OS assigned port **35011**.

### Send an announce request

```bash
cargo run -p torrust-tracker-client --bin tracker_client -- \
  http announce http://127.0.0.1:35011/announce \
  9c38422213e30bff212b30c360d26f9a02136422
```

**Tracker request/response log**:

```text
INFO request{...}: HTTP TRACKER: request server_socket_addr=127.0.0.1:35011 method=GET uri=/announce?info_hash=...
INFO request{...}: HTTP TRACKER: response server_socket_addr=127.0.0.1:35011 latency_ms=0 status_code=200 OK
```

**Client output**:

```json
{
  "complete": 1,
  "incomplete": 0,
  "interval": 120,
  "min interval": 120,
  "peers": []
}
```

### Result

| Check                                     | Outcome |
| ----------------------------------------- | ------- |
| Tracker starts and binds successfully     | PASS    |
| Coupling table printed on startup         | PASS    |
| Server returns HTTP 200 for `/announce`   | PASS    |
| `interval` matches config (120 s)         | PASS    |
| Peer registered as seeder (`complete: 1`) | PASS    |

---

## Summary

Both examples work as functional trackers out of the box. The coupling table
printed on startup makes the Step 3 finding tangible: even a single-protocol
binary pulls in the full `Configuration` aggregate (and all config types
compiled inside it) because `Environment::new` accepts `&Arc<Configuration>`.
