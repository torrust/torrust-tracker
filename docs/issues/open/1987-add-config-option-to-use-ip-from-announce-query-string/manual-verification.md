# Manual Verification — Issue #1987

This evidence file records three comparable local verification phases:

1. Baseline behavior before implementation.
2. Behavior after implementation while the internal policy remains disabled.
3. Behavior after #1980 activates configuration schema v3.0.0 and the setting is enabled.

## Phase 1 — Baseline Before Implementation

**Status:** DONE

### Environment

| Item                  | Value                                                                   |
| --------------------- | ----------------------------------------------------------------------- |
| Date/time (UTC)       | 2026-08-18; exact time not captured                                     |
| Commit                | `4005cca5518d3ce8b1556cf10abcd7db146ae18e`                              |
| OS                    | Linux                                                                   |
| Rust toolchain        | Rust `1.88.0` (`rustc 1.88.0`, `cargo 1.88.0`)                          |
| Tracker configuration | `share/default/config/tracker.development.sqlite3.toml` (schema v2.0.0) |
| Local HTTP tracker    | `http://127.0.0.1:7070`                                                 |

### Request Matrix

| Case          | Request form      | Expected baseline behavior                            | Actual result                       |
| ------------- | ----------------- | ----------------------------------------------------- | ----------------------------------- |
| Absent        | No `ip` parameter | Announce succeeds using connection-derived address    | HTTP 200; bencoded success response |
| Empty         | `ip=`             | Announce succeeds using connection-derived address    | HTTP 200; bencoded success response |
| Valid IPv4    | `ip=1.2.3.4`      | Announce succeeds; supplied value is silently ignored | HTTP 200; bencoded success response |
| Valid IPv6    | `ip=2001:db8::1`  | Announce succeeds; supplied value is silently ignored | HTTP 200; bencoded success response |
| DNS name      | `ip=example.com`  | Announce succeeds; supplied value is silently ignored | HTTP 200; bencoded success response |
| Invalid value | `ip=invalid_ip`   | Announce succeeds; supplied value is silently ignored | HTTP 200; bencoded success response |

### Commands and Output

The local tracker was started with:

```sh
cargo +1.88.0 run --bin torrust-tracker
```

The raw HTTP matrix used a single valid announce query with each `ip` suffix below:

```text
(absent)
&ip=
&ip=1.2.3.4
&ip=2001%3Adb8%3A%3A1
&ip=example.com
&ip=invalid_ip
```

All six requests returned HTTP 200 and the same bencoded announce success response:

```text
d8:completei0e10:incompletei1e8:intervali120e12:min intervali120e5:peers0:6:peers6e
```

This confirms the pre-implementation behavior: the tracker does not distinguish absent, empty, valid, DNS-name, and invalid `ip` values at the HTTP response boundary; every supplied value is silently ignored.

The local typed tracker client also confirmed that a valid supplied address is ignored:

```sh
cargo +1.88.0 run -p torrust-tracker-client --bin tracker_client -- \
  http announce http://127.0.0.1:7070 \
  9c38422213e30bff212b30c360d26f9a02136422 \
  --ip 1.2.3.4
```

It returned a successful JSON announce response whose peer list contains the connection address, not `1.2.3.4`:

```json
{
  "complete": 1,
  "incomplete": 1,
  "interval": 120,
  "min interval": 120,
  "peers": [
    {
      "ip": "127.0.0.1",
      "peer id": [
        45, 77, 86, 48, 48, 48, 49, 45, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
        48, 49
      ],
      "port": 6881
    }
  ]
}
```

## Phase 2 — Post-Implementation Disabled Policy

**Status:** DONE

### Environment

| Item                  | Value                                                                   |
| --------------------- | ----------------------------------------------------------------------- |
| Date/time (UTC)       | 2026-08-18 to 2026-08-19; exact time not captured                       |
| Commit                | Uncommitted #1987 implementation after `4005cca`                        |
| OS                    | Linux                                                                   |
| Rust toolchain        | Rust `1.88.0`                                                           |
| Tracker configuration | `share/default/config/tracker.development.sqlite3.toml` (schema v2.0.0) |
| Local HTTP tracker    | `http://127.0.0.1:7070`                                                 |

### Address-Selection Request Matrix

The same raw HTTP announce matrix from Phase 1 was run after rebuilding the tracker. Every response used HTTP 200, as required by the BitTorrent HTTP tracker failure-response convention; failed announces carry a bencoded `failure reason`.

| Case                          | Request form           | Actual result                                                                                                                                 |
| ----------------------------- | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Absent                        | No `ip` parameter      | HTTP 200; bencoded announce success response                                                                                                  |
| Empty                         | `ip=`                  | HTTP 200; bencoded announce success response                                                                                                  |
| Valid IPv4                    | `ip=1.2.3.4`           | HTTP 200; `failure reason`: `Client-supplied peer IPs are disabled`                                                                           |
| Valid encoded IPv6            | `ip=2001%3Adb8%3A%3A1` | HTTP 200; `failure reason`: `Client-supplied peer IPs are disabled`                                                                           |
| DNS name                      | `ip=example.com`       | HTTP 200; `failure reason`: `DNS names are not supported for the announce ip parameter`                                                       |
| Single-label DNS name         | `ip=localhost`         | HTTP 200; `failure reason`: `DNS names are not supported for the announce ip parameter`                                                       |
| Invalid value                 | `ip=invalid_ip`        | HTTP 200; `failure reason`: `The announce ip parameter must be an IPv4 or IPv6 literal`                                                       |
| Invalid numeric IP-like value | `ip=999.999.999.999`   | HTTP 200; `failure reason`: `The announce ip parameter must be an IPv4 or IPv6 literal`                                                       |
| Malformed encoding            | `ip=%ZZ`               | HTTP 200; `failure reason`: `Bad request. Cannot parse query params for announce request: malformed percent encoding or invalid UTF-8 for ip` |

This verifies the intentional baseline change: absent and empty values remain successful, while every non-empty override is explicitly rejected until schema v3.0.0 can activate the opt-in policy.

### Observability Decision

The initially tested rejection-specific event and metric were deliberately
removed under Option B after architectural review. The tracker therefore has no
dedicated aggregate counter for rejected announce `ip` parameters in #1987.
Existing request logs and normal diagnostics remain available to investigate
client compatibility. A future general error-event contract may introduce a
counter only when it is consistent with the documented cross-service design in
[`generalize-error-events.md`](../../drafts/generalize-error-events.md).

### Local Tracker-Client Result

The local typed client was run against the rebuilt tracker:

```sh
cargo +1.88.0 run -p torrust-tracker-client --bin tracker_client -- \
  http announce http://127.0.0.1:7070 \
  9c38422213e30bff212b30c360d26f9a02136422 \
  --ip 1.2.3.4
```

The client displayed the expected tracker failure reason:

```json
{ "failure reason": "Client-supplied peer IPs are disabled" }
```

It then returned its existing generic client-side error, `unrecognized announce response from tracker`. The tracker response itself is correct and matches the raw HTTP evidence above; this client-side classification behavior is not changed by #1987.

## Phase 3 — Active v3 Enabled Policy

**Status:** DONE

### Environment

| Item                  | Value                                                                 |
| --------------------- | --------------------------------------------------------------------- |
| Date/time (UTC)       | 2026-08-26; exact time captured in local logs                         |
| Commit                | `af890d927578d5f60dc70d2da87dae92416e4f5c`                            |
| OS                    | Linux                                                                 |
| Rust toolchain        | Rust `1.98.0` (`rustc 1.98.0`, `cargo 1.98.0`)                        |
| Tracker configuration | Isolated v3 TOML in `.tmp/issue-1987-enabled-v3/config.toml`          |
| HTTP tracker          | `http://127.0.0.1:18070`                                              |
| REST API              | `http://127.0.0.1:18121`                                              |
| Health API            | `http://127.0.0.1:18122`                                              |

The isolated v3 configuration enabled
`use_ip_from_query_string = true`, set the HTTP listener's loopback fallback
to `network.external_ip = "198.51.100.77"`, and used an isolated SQLite
database. The health endpoint returned `status: "Ok"`, confirming the HTTP
tracker and REST API were healthy before the request matrix ran.

### Request Matrix

| Case          | Request form                        | Actual result |
| ------------- | ----------------------------------- | ------------- |
| Valid IPv4    | Tracker client with `--ip 1.2.3.4`  | Successful announce; REST reported `peer_addr: "1.2.3.4:6881"`. |
| Absent        | Raw HTTP request without `ip`       | Successful announce; REST reported fallback `peer_addr: "198.51.100.77:6882"`. |
| Empty         | Raw HTTP request with `ip=`         | Successful announce; REST reported fallback `peer_addr: "198.51.100.77:6882"`. |
| DNS name      | Raw HTTP request with `ip=example.com` | Bencoded failure: `DNS names are not supported for the announce ip parameter`; no peer added. |
| Invalid value | Raw HTTP request with `ip=invalid_ip` | Bencoded failure: `The announce ip parameter must be an IPv4 or IPv6 literal`; no peer added. |
| Precedence    | Loopback request with `ip=1.2.3.4`  | Successful announce; REST reported `peer_addr: "1.2.3.4:6882"`, overriding `external_ip`. |

### Commands and Output

The valid override used the local typed client:

```sh
cargo run -p torrust-tracker-client --bin tracker_client -- \
  http announce http://127.0.0.1:18070 \
  0123456789abcdef0123456789abcdef01234567 \
  --ip 1.2.3.4 \
  --port 6881 \
  --peer-id=-MV0001-123456789012 \
  --event started
```

It returned a successful announce response:

```json
{"complete":1,"incomplete":0,"interval":120,"min interval":120,"peers":[]}
```

The REST peer observation confirmed that the tracker registered
`1.2.3.4:6881`. Raw local HTTP requests covered absent, empty, DNS, invalid,
and loopback-precedence forms because the typed client cannot construct each
raw request state. The tracker was stopped with `SIGINT`; its logs confirmed
graceful shutdown of the HTTP tracker, REST API, health API, and jobs, and no
listeners remained on the three test ports.

The ignored reproducibility artifacts, including the effective configuration
and tracker logs, are retained locally in `.tmp/issue-1987-enabled-v3/`.
