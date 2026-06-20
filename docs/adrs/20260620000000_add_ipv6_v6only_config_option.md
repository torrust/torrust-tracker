---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - packages/configuration/src/v2_0_0/udp_tracker.rs
    - packages/configuration/src/v2_0_0/http_tracker.rs
    - packages/udp-server/src/server/bound_socket.rs
    - packages/axum-http-server/src/server.rs
    - docs/issues/open/1671-ipv4-ipv6-client-metrics/research-dual-stack-portability.md
---

# Add `ipv6_v6only` Config Option for Separate IPv4/IPv6 Sockets

## Description

The tracker currently creates IPv6 sockets in default dual-stack mode
(`IPV6_V6ONLY=0`), which means a single `[::]:<port>` bind accepts both IPv4 and
IPv6 clients. IPv4 clients appear as IPv4-mapped IPv6 addresses (`::ffff:<ipv4>`).

During the [#1671](https://github.com/torrust/torrust-tracker/issues/1671)
investigation, we confirmed that setting `IPV6_V6ONLY=1` at runtime (via `socket2`)
allows a single tracker process to bind both `0.0.0.0:<port>` and `[::]:<port>` on
the same port — giving operators true per-family socket separation.

This ADR records the decision to add an explicit config option rather than
changing the default or leaving the behaviour implicit.

## Agreement

We add a new boolean config field `ipv6_v6only` to both `UdpTracker` and
`HttpTracker` configuration structs, defaulting to `false`.

### Behaviour

| `ipv6_v6only` | Socket behaviour | Who should use this |
|---|---|---|
| `false` (default) | Dual-stack — single `[::]` socket accepts both IPv4 and IPv6 clients. | Operators with simple setups, no per-family metric needs. |
| `true` | IPv6-only — each `[::]` socket rejects IPv4. Must also bind `0.0.0.0:<port>` to serve IPv4 clients. | Operators who want separate metrics per IP family, or per-family rate limiting. |

### Config example

```toml
[[udp_trackers]]
bind_address = "0.0.0.0:6969"
ipv6_v6only = false   # irrelevant for IPv4 sockets, but accepted

[[udp_trackers]]
bind_address = "[::]:6969"
ipv6_v6only = true    # explicit IPv6-only socket
tracker_usage_statistics = true
```

### Implementation

1. Add `ipv6_v6only: bool` field to `UdpTracker` and `HttpTracker` with
   `#[serde(default = "default_false")]`.
2. In `BoundSocket::create_socket` (UDP) and `Launcher::create_tcp_listener` (HTTP),
   only call `socket.set_only_v6(true)` when `ipv6_v6only` is `true`.
3. For IPv4 sockets (`0.0.0.0`), the option is a no-op — `IPV6_V6ONLY` only applies
   to `AF_INET6` sockets.

### Platform notes

- `IPV6_V6ONLY=1` is the **default** on Windows, macOS, FreeBSD, and Solaris.
  Setting it explicitly to `true` on those platforms is a no-op.
- `IPV6_V6ONLY=0` (dual-stack) is the **default** only on Linux. Setting it to
  `true` there enables the separate-socket behaviour described above.
- On **OpenBSD**, `IPV6_V6ONLY=0` is not supported; setting `ipv6_v6only = false`
  will result in a runtime error. The config option is documented accordingly.

### Alternatives Considered

**A) Always set `IPV6_V6ONLY=1` (remove dual-stack support entirely).**

Rejected because it forces every operator to explicitly configure both address
families. While this would be consistent across platforms and simplify the code,
it breaks existing configs unnecessarily. The project will eventually release
4.0.0 with breaking changes, but this particular change does not need to be
forced on all users — those who want separate sockets can opt in.

**B) Always set `IPV6_V6ONLY=1` unconditionally + migration guide in changelog.**

Rejected for the same reason as (A). Adding the config option is minimal effort
and preserves choice.

### Consequences

- **Positive**: Operators can opt into separate IPv4/IPv6 sockets for per-family
  metrics without changing the default behaviour for everyone.
- **Positive**: The config option name (`ipv6_v6only`) matches the underlying
  socket option, making it searchable for operators familiar with the concept.
- **Positive**: The experiment code in `contrib/dev-tools/experiments/dual-stack-sockets/`
  can be removed — the `ipv6_v6only` option replaces it.
- **Negative**: Additional maintenance surface — the option needs to be
  documented and tested.
- **Negative**: Platform-dependent behaviour — OpenBSD cannot use dual-stack
  mode — must be documented.
