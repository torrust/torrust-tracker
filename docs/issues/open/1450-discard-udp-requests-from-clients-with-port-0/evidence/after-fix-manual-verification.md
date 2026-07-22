# After-Fix Manual Verification — Issue #1450

**Date**: 2026-07-21
**Branch**: `1450-discard-udp-requests-from-clients-with-port-0`
**Tracker version**: `3.0.0-develop` (commit `86bb083b`)
**Config**: `share/default/config/tracker.development.sqlite3.toml`

## Setup

```sh
cargo build --bin torrust-tracker
TORRUST_TRACKER_CONFIG_TOML_FILE=share/default/config/tracker.development.sqlite3.toml \
  ./target/debug/torrust-tracker > .tmp/tracker-run-fixed.log 2>&1 &
```

## Packet sent

```sh
sudo python3 .tmp/send_port0_udp.py
# Sent BEP 15 connect request  src=127.0.0.1:0  dst=127.0.0.1:6969
```

The script crafts a raw IP/UDP datagram with source port 0 using `socket.IPPROTO_RAW`
and `IP_HDRINCL`, bypassing the OS socket API which would otherwise assign a non-zero
ephemeral port.

## Result

### No WARN log (fixed)

```sh
grep -i "warn\|error 22\|failed to send" .tmp/tracker-run-fixed.log
# (no output — WARN is gone)
```

Before the fix the following line appeared in the tracker log every time a port-0
datagram arrived:

```text
WARN process_request:send_response{...}: torrust_udp_tracker_server::server::processor:
failed to send bytes_count=16 error=Invalid argument (os error 22)
```

After the fix, no such line appears.

### Stats counter incremented

```sh
curl -s "http://localhost:1212/api/v1/stats?token=MyAccessToken" | python3 -m json.tool
```

Relevant fields from the response after one port-0 datagram:

```json
{
  "udp_requests_discarded": 1,
  "udp_requests_aborted": 0,
  "udp4_requests": 1,
  "udp4_responses": 0
}
```

| Field                    | Value | Meaning                                                  |
| ------------------------ | ----- | -------------------------------------------------------- |
| `udp_requests_discarded` | **1** | Request was counted and discarded                        |
| `udp4_requests`          | 1     | Datagram was received by the socket                      |
| `udp4_responses`         | **0** | No response was sent (correct — port 0 is undeliverable) |
| `udp_requests_aborted`   | 0     | Not aborted — discarded before any processing            |

## Summary

The fix works as designed:

- The WARN log no longer pollutes production logs.
- The request is discarded silently before any parsing or handler invocation.
- The `udp_requests_discarded` counter gives operators a clean signal via the stats API.
