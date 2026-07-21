---
doc-type: issue
issue-type: bug
status: open
priority: p2
github-issue: 1450
spec-path: docs/issues/open/1450-discard-udp-requests-from-clients-with-port-0/ISSUE.md
branch: "1450-discard-udp-requests-from-clients-with-port-0"
related-pr: null
last-updated-utc: 2026-07-21 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/statistics/mod.rs
    - packages/udp-server/src/statistics/event/handler/
---

# Issue #1450 - Discard UDP requests from clients with port 0

## Goal

Prevent the UDP tracker from processing requests that arrive from a client address
where the source port is 0. Such requests produce an OS-level error when the tracker
tries to send the response, and the error is currently surfaced as a noisy `WARN` log.

## Background

### Why can a UDP client have port 0?

Unlike TCP, UDP is a **connectionless protocol**. The tracker never establishes a
handshake — it simply calls `recvfrom()` and reads whatever datagram arrives. The
"client port" is whatever value happens to be in the **source port field of the
incoming UDP header**, a 16-bit number entirely under the sender's control.

RFC 768 (the UDP specification) explicitly permits port 0:

> _"Source Port is an optional field, when meaningful, it indicates the port of the
> sending process... If not used, a value of zero is inserted."_

In practice, port 0 in a UDP source can originate from:

- A **buggy BitTorrent client** that fails to bind before sending.
- A **raw-socket tool or scanner** that crafts datagrams with an intentionally zeroed
  source port (e.g., to probe the tracker without revealing a real port).
- A **broken middlebox** (NAT/firewall) that strips or zeroes the source port.

The tracker has no way to prevent these datagrams from arriving — the OS delivers
them just like any other UDP packet.

### Current behaviour

The tracker received UDP packets from clients whose source port is `0` in the UDP
header. Although RFC 768 does not forbid source port 0, no response can ever be
delivered to `<ip>:0`. The current code processes the request fully (parses it,
executes the handler, serializes the response) and only discovers the problem when
it calls `send_to`, which returns `EINVAL` (OS error 22). The failure is then
logged as a `WARN`, polluting production logs.

Example from the demo tracker logs:

```text
tracker  | 2025-04-14T08:52:42.491940Z  WARN process_request:send_response{client_socket_addr=*.*.*.*:0 response=Connect(...) ...}: torrust_udp_tracker_server::server::processor: failed to send bytes_count=16 error=Invalid argument (os error 22) payload=[...]
```

This happens at least for `Connect` requests and could in theory happen for any
request type. It has been observed multiple times in the demo tracker logs:

- 2025-04-14 (first observation)
- 2025-06-18 (two additional occurrences)

Whether these are malformed clients, scanner tools, or deliberate abuse (port-0
spam) is unknown. Regardless, the tracker should not waste resources processing
them and should not fill logs with OS-level errors caused by user-space input.

## Design

### Detection point

The check happens at the very start of `Processor::process_request`, before any
packet parsing or handler invocation. If `client_socket_addr.port() == 0`, the
request is **discarded immediately**:

```rust
pub async fn process_request(self, request: RawRequest) {
    let client_socket_addr = request.from;

    if client_socket_addr.port() == 0 {
        // Discard: cannot send a response to port 0.
        // Emit a stats event so operators can detect abuse / misconfigured clients.
        ...
        return;
    }
    ...
}
```

### Logging

**No per-request `WARN` log.** The existing `WARN` log is removed (it came from the
send failure, which no longer occurs). A per-request log for bad-user traffic would
add uncontrollable noise to production logs. Operators who want visibility should
use the metrics/stats endpoint.

A single `tracing::trace!` line may be emitted for debugging purposes (enabled only
at trace level, never in default production configurations).

### Statistics

A new stats event `UdpRequestDiscarded` is introduced (not reusing
`UdpRequestAborted`, which represents a different lifecycle stage). A matching
metric counter is added:

```text
udp_tracker_server_requests_discarded_total
```

This counter increments for every discarded request, providing operators with
a signal to detect scanner activity or abuse without exposing it in logs.

## Acceptance Criteria

- [ ] Requests with client port 0 are discarded before any handler is invoked.
- [ ] The existing `WARN` log ("failed to send ... error=Invalid argument (os error 22)")
      no longer appears for this case.
- [ ] A new `UdpRequestDiscarded` event is defined in `event.rs`.
- [ ] The event is emitted from `process_request` when the client port is 0.
- [ ] A new metric counter `udp_tracker_server_requests_discarded_total` is described
      and handled by the statistics event handler.
- [ ] Unit tests cover:
  - The handler for `UdpRequestDiscarded` increments the counter.
  - The processor discards the request (no response sent, counter incremented).

## Verification

### Automated tests

The unit tests in `packages/udp-server/src/server/processor.rs` are the deepest
automated coverage possible for this scenario. They work by injecting a `RawRequest`
with `from = <ip>:0` directly into `Processor::process_request`, bypassing the
network layer entirely.

**Why a network-level integration test is not feasible:**

When a process opens a normal UDP socket and binds to port 0, the OS always assigns
a real ephemeral source port (e.g., `54321`). It is impossible to make the kernel
send a datagram with source port 0 through a normal socket API. The only way to
produce such a datagram on the wire is to use a **raw socket**, which requires
`CAP_NET_RAW` / `root` privileges — not acceptable in standard CI environments.

Therefore:

- Unit tests cover the discard logic and the stats counter increment.
- No separate integration or E2E test is added for this path.

### Manual verification

To verify the fix end-to-end on a running tracker, you need a tool that can craft
raw UDP packets with an explicit source port. Two options:

**Option A — `nping` (from the nmap suite)**

```sh
sudo nping --udp --dest-port 6969 --source-port 0 <tracker-ip>
```

**Option B — `scapy` (Python)**

```sh
sudo python3 - <<'EOF'
from scapy.all import *
# BEP 15 connect request (magic + action=0 + transaction_id)
payload = b'\x00\x00\x04\x17\x27\x10\x19\x80\x00\x00\x00\x00\xde\xad\xbe\xef'
send(IP(dst="<tracker-ip>") / UDP(sport=0, dport=6969) / Raw(load=payload))
EOF
```

Both require root / `sudo` because they use raw sockets.

**What to check after sending the packet:**

1. **No `WARN` log** — the line
   `"failed to send ... error=Invalid argument (os error 22)"` must not appear.
2. **Counter increment** — query the REST API stats endpoint and confirm
   `udp_requests_discarded` has increased by 1:

   ```sh
   curl -s http://localhost:1212/api/v1/stats | jq .udp_requests_discarded
   ```

3. **No response sent to the client** — `nping` or `scapy` should report no reply.

## Implementation Notes

- `ConnectionContext::new(client_addr_with_port_0, server_service_binding)` is valid;
  only the server binding is required to have a non-zero port.
- The `process_request` function already has `self.server_service_binding` available
  to construct the `ConnectionContext` for the stats event.
- Follow the existing pattern in
  `packages/udp-server/src/statistics/event/handler/request_aborted.rs` for the new
  handler file.
