# Experiment: Verify separate IPv4/IPv6 socket bindings at runtime

This experiment verifies that setting `IPV6_V6ONLY=1` on IPv6 sockets at the Rust
code level (via `socket2`) allows a single tracker process to bind both
`0.0.0.0:<port>` (IPv4-only) and `[::]:<port>` (IPv6-only) on the same port —
without requiring a system-wide `sysctl net.ipv6.bindv6only=1`.

## Why this matters

A tracker operator has two strategies to separate IPv4 and IPv6 traffic in metrics:

| Strategy                                               | How it works                                                            | Pros                                                                 | Cons                                  |
| ------------------------------------------------------ | ----------------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------- |
| **Client address parsing** (Task 2)                    | Parse the client's `SocketAddr` to detect `::ffff:` v4-mapped addresses | Works with current dual-stack socket, no infra changes               | Only splits after the fact in metrics |
| **Separate socket bindings** (Task 1, this experiment) | Bind `0.0.0.0` and `[::]` on same port with `IPV6_V6ONLY=1`             | True separation: metrics, performance isolation, independent sockets | Requires code change, more sockets    |

If separate bindings work, we can later add a config option so operators can choose.

## Prerequisites

- Rust toolchain
- `net.ipv6.bindv6only` must be `0` (Linux default)

```bash
sysctl net.ipv6.bindv6only
# Expected: net.ipv6.bindv6only = 0
```

If it's `1`, the experiment is invalid because the OS already separates sockets.
Set it back to `0` (requires root):

```bash
sudo sysctl -w net.ipv6.bindv6only=0
```

## How to run

```bash
cd contrib/dev-tools/experiments/dual-stack-sockets

# Run a single tracker with both IPv4 and IPv6 listeners on the same ports
cargo run --bin torrust-tracker -- --config config/tracker.dual-stack.toml
```

If `IPV6_V6ONLY=1` works at runtime, the tracker should start successfully with
both address families on the same ports. If it fails, the second bind will get
`EADDRINUSE`.

## Expected results

| Scenario                                     | Expected behaviour                  |
| -------------------------------------------- | ----------------------------------- |
| Without `IPV6_V6ONLY` change (original code) | Second bind fails with `EADDRINUSE` |
| With `IPV6_V6ONLY=1` change (current branch) | Both bindings succeed               |

## Metrics labels verification

With the tracker running, check the Prometheus metrics endpoint:

```bash
curl -s "http://127.0.0.1:1212/api/v1/metrics?token=MyAccessToken&format=prometheus" | grep server_binding_address_ip_family
```

You should see both `inet` and `inet6` entries for the same protocol+port:

```text
server_binding_address_ip_family="inet"   # from the 0.0.0.0 socket
server_binding_address_ip_family="inet6"  # from the [::] socket
```

## Results (run 2026-06-19)

### System info

```text
$ sysctl net.ipv6.bindv6only
net.ipv6.bindv6only = 0

$ uname -a
Linux josecelano-desktop 7.0.0-22-generic #22-Ubuntu SMP PREEMPT_DYNAMIC Mon May 25 15:54:34 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux
```

### Command run

```bash
cd /home/josecelano/.../torrust-tracker-agent-03
rm -f storage/tracker/lib/database/sqlite3.db
TORRUST_TRACKER_CONFIG_TOML_PATH=contrib/dev-tools/experiments/dual-stack-sockets/config/tracker.dual-stack.toml \
  cargo run --bin torrust-tracker
```

### Listening sockets (`ss`)

```text
UNCONN 0.0.0.0:6969   users:(("torrust-tracker",fd=11))    # IPv4 UDP
UNCONN [::]:6969       users:(("torrust-tracker",fd=12))    # IPv6 UDP
LISTEN 0.0.0.0:7070   users:(("torrust-tracker",fd=13))    # IPv4 HTTP
LISTEN [::]:7070       users:(("torrust-tracker",fd=14))    # IPv6 HTTP
```

All four sockets bound — no `EADDRINUSE` error. `IPV6_V6ONLY=1` set at runtime
(via `socket2` crate) is sufficient; no `sysctl` required.

### Log output (startup)

```text
UDP TRACKER: Starting on: 0.0.0.0:6969
UDP TRACKER: Started on: udp://0.0.0.0:6969
UDP TRACKER: Starting on: [::]:6969
UDP TRACKER: Started on: udp://[::]:6969
HTTP TRACKER: Starting on: http://0.0.0.0:7070
HTTP TRACKER: Started on: http://0.0.0.0:7070
HTTP TRACKER: Starting on: http://[::]:7070
HTTP TRACKER: Started on: http://[::]:7070
```

### Metrics: client address labels

After sending requests from both IPv4 and IPv6 clients, the labeled metrics show
correct separation:

**UDP — IPv4 client → IPv4 socket (`0.0.0.0:6969`):**

```prometheus
udp_tracker_core_requests_received_total{
  client_address_ip_family="inet",
  client_address_ip_type="plain",
  server_binding_address_ip_family="inet",
  ...
} 1
```

**UDP — IPv6 client → IPv6 socket (`[::]:6969`, via `::1`):**

```prometheus
udp_tracker_core_requests_received_total{
  client_address_ip_family="inet6",
  client_address_ip_type="plain",
  server_binding_address_ip_family="inet6",
  ...
} 1
```

**HTTP — IPv6 client → IPv6 socket (`[::]:7070`, via `::1`, curl -6):**

```prometheus
http_tracker_core_requests_received_total{
  client_address_ip_family="inet6",
  client_address_ip_type="plain",
  request_kind="announce",
  server_binding_address_ip_family="inet6",
  ...
} 1
```

Both `client_address_ip_family` and `client_address_ip_type` labels are present
on all per-request counters.

### Expected vs actual

| Scenario                              | Expected                     | Actual                                                           |
| ------------------------------------- | ---------------------------- | ---------------------------------------------------------------- |
| Without `IPV6_V6ONLY` change          | `EADDRINUSE`                 | Not tested (would fail)                                          |
| With `IPV6_V6ONLY=1` (current branch) | Both bindings succeed        | ✅ Both IPv4/IPv6 UDP+HTTP bind on same port                     |
| Client address labels present         | All per-request counters     | ✅ `client_address_ip_family` + `client_address_ip_type` visible |
| IPv4 → IPv4 socket labels             | `client=inet, server=inet`   | ✅ Confirmed via UDP announce to `127.0.0.1:6969`                |
| IPv6 → IPv6 socket labels             | `client=inet6, server=inet6` | ✅ Confirmed via UDP+HTTP to `[::1]:6969` and `[::1]:7070`       |

## Conclusion

Both tasks confirmed working:

1. **Task 1 — Separate socket bindings**: `IPV6_V6ONLY=1` set via `socket2` at
   the Rust code level allows a single tracker process to bind `0.0.0.0:<port>`
   and `[::]:<port>` simultaneously on the same port. No system-wide `sysctl`
   needed.
2. **Task 2 — Client address labels**: `client_address_ip_family` and
   `client_address_ip_type` labels are present on all per-request UDP and HTTP
   metric counters, correctly identifying the connecting client's address type.

### Next steps

- Consider performance benchmarks to confirm separate sockets improve throughput.
