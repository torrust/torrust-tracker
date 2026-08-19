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

| Case                          | Request form           | Actual result                                                                                                                |
| ----------------------------- | ---------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Absent                        | No `ip` parameter      | HTTP 200; bencoded announce success response                                                                                 |
| Empty                         | `ip=`                  | HTTP 200; bencoded announce success response                                                                                 |
| Valid IPv4                    | `ip=1.2.3.4`           | HTTP 200; `failure reason`: `Client-supplied peer IPs are disabled`                                                          |
| Valid encoded IPv6            | `ip=2001%3Adb8%3A%3A1` | HTTP 200; `failure reason`: `Client-supplied peer IPs are disabled`                                                          |
| DNS name                      | `ip=example.com`       | HTTP 200; `failure reason`: `DNS names are not supported for the announce ip parameter`                                      |
| Single-label DNS name         | `ip=localhost`         | HTTP 200; `failure reason`: `DNS names are not supported for the announce ip parameter`                                      |
| Invalid value                 | `ip=invalid_ip`        | HTTP 200; `failure reason`: `The announce ip parameter must be an IPv4 or IPv6 literal`                                      |
| Invalid numeric IP-like value | `ip=999.999.999.999`   | HTTP 200; `failure reason`: `The announce ip parameter must be an IPv4 or IPv6 literal`                                      |
| Malformed encoding            | `ip=%ZZ`               | HTTP 200; `failure reason`: `Bad request. Cannot parse query params for announce request: malformed percent encoding for ip` |

This verifies the intentional baseline change: absent and empty values remain successful, while every non-empty override is explicitly rejected until schema v3.0.0 can activate the opt-in policy.

### Observability Evidence

The tracker was restarted with a temporary local debug logging override, then received `ip=1.2.3.4` while the policy remained disabled.

```sh
TORRUST_TRACKER_CONFIG_OVERRIDE_LOGGING__THRESHOLD=debug \
  cargo +1.88.0 run --bin torrust-tracker
```

The rejection-specific debug output used only the bounded reason code and did not contain the raw submitted value (`1.2.3.4`):

```text
DEBUG torrust_tracker_http_core::statistics::event::handler: Recorded rejected HTTP announce peer IP parameter reason="override_disabled"
```

The authenticated local metrics endpoint reported one rejection with the bounded `reason="override_disabled"` label and no raw submitted-IP label:

```text
# HELP http_tracker_core_announce_peer_ip_rejections_total Total rejected HTTP announce peer IP parameters
# TYPE http_tracker_core_announce_peer_ip_rejections_total counter
http_tracker_core_announce_peer_ip_rejections_total{client_address_ip_family="inet",client_address_ip_type="plain",reason="override_disabled",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="7070",server_binding_protocol="http"}1
```

Existing HTTP request middleware logs the full request URI at `info`, including query values. That established behavior is outside #1987's rejection-observability scope and is not evidence that raw values are globally absent from tracker logs.

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

**Status:** BLOCKED

Blocked until issue #1980 activates schema v3.0.0 configuration at runtime. This phase will enable `use_ip_from_query_string` for a local HTTP tracker and verify valid overrides, fallback behavior, validation failures, and reverse-proxy precedence.
