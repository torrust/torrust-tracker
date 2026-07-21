# Evidence: Original (Pre-Fix) Behaviour — Manual Verification

**Date**: 2026-07-21
**Tracker version**: 3.0.0-develop, commit `c0fb3895`
**Branch**: `develop` (before the fix branch was applied)
**Environment**: Local development machine, Linux

## Purpose

This document captures the evidence that confirms the buggy behaviour described in
issue #1450: the tracker processes a UDP connect request from a client with source
port 0 and then emits a `WARN` log when it fails to send the response back.

## Steps to Reproduce

### 1. Build the tracker from the pre-fix commit

```sh
git checkout c0fb3895
cargo build --bin torrust-tracker
```

### 2. Start the tracker with the development config

```sh
TORRUST_TRACKER_CONFIG_TOML_FILE=share/default/config/tracker.development.sqlite3.toml \
  ./target/debug/torrust-tracker > /tmp/tracker.log 2>&1 &
```

### 3. Send a UDP datagram with source port 0 using a raw socket

The script below constructs a BEP 15 connect request with source port 0
and sends it via a raw IP socket (requires root):

```sh
sudo python3 .tmp/send_port0_udp.py
```

The script content (`send_port0_udp.py`):

```python
import socket, struct

DST_IP, DST_PORT, SRC_PORT = "127.0.0.1", 6969, 0
PAYLOAD = struct.pack("!qII", 0x0000041727101980, 0, 0xDEADBEEF)

def checksum(data):
    if len(data) % 2: data += b"\x00"
    s = sum((data[i] << 8) + data[i+1] for i in range(0, len(data), 2))
    s = (s >> 16) + (s & 0xFFFF); s += s >> 16
    return ~s & 0xFFFF

def build_udp(sp, dp, payload, sip, dip):
    ln = 8 + len(payload)
    pseudo = socket.inet_aton(sip) + socket.inet_aton(dip) + struct.pack("!BBH", 0, socket.IPPROTO_UDP, ln)
    raw = struct.pack("!HHHH", sp, dp, ln, 0) + payload
    return struct.pack("!HHHH", sp, dp, ln, checksum(pseudo + raw)) + payload

def build_ip(sip, dip, udp):
    tl = 20 + len(udp)
    return struct.pack("!BBHHHBBH4s4s", 0x45, 0, tl, 0xABCD, 0, 64,  # cspell:disable-line
                       socket.IPPROTO_UDP, 0, socket.inet_aton(sip), socket.inet_aton(dip)) + udp

pkt = build_ip(DST_IP, DST_IP, build_udp(SRC_PORT, DST_PORT, PAYLOAD, DST_IP, DST_IP))
with socket.socket(socket.AF_INET, socket.SOCK_RAW, socket.IPPROTO_RAW) as s:
    s.setsockopt(socket.IPPROTO_IP, socket.IP_HDRINCL, 1)
    s.sendto(pkt, (DST_IP, 0))
print(f"Sent BEP 15 connect request  src={DST_IP}:{SRC_PORT}  dst={DST_IP}:{DST_PORT}")
```

## Observed Behaviour (Bug Confirmed)

The tracker received the request, processed it fully (parsed the connect request,
generated a connect response), then tried to send the response back to `127.0.0.1:0`
and received `EINVAL` (OS error 22). The failure was surfaced as a `WARN` log:

```text
2026-07-21T16:58:50.032701Z  WARN process_request:send_response{client_socket_addr=127.0.0.1:0 response=Connect(ConnectResponse { transaction_id: TransactionId(I32(-559038737)), connection_id: ConnectionId(I64(-4357419529092936579)) }) opt_req_kind=Some(Connect) req_processing_time=54.673µs}: torrust_tracker_udp_server::server::processor: failed to send bytes_count=16 error=Invalid argument (os error 22) payload=[0, 0, 0, 0, 222, 173, 190, 239, 195, 135, 86, 6, 95, 16, 204, 125]
```

### Key observations from the log line

| Field                 | Value                              | Meaning                                                  |
| --------------------- | ---------------------------------- | -------------------------------------------------------- |
| `client_socket_addr`  | `127.0.0.1:0`                      | Source port is 0 — undeliverable                         |
| `response`            | `Connect(ConnectResponse { ... })` | Request was fully processed before the error             |
| `req_processing_time` | `54.673µs`                         | CPU was spent on a request that can never be answered    |
| `error`               | `Invalid argument (os error 22)`   | `EINVAL` from `sendto(2)` — OS refuses to send to port 0 |
| `bytes_count`         | `16`                               | Full 16-byte connect response was serialized             |

### What should happen instead (after the fix)

The request should be discarded **before** any parsing or processing. No response
is serialized, no `WARN` is emitted. The `udp_requests_discarded` statistics
counter increments by 1.
