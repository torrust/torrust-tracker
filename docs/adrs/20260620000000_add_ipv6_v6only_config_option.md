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
`HttpTracker` configuration structs, defaulting to `false` (dual-stack).

When `ipv6_v6only = true`, the socket is restricted to IPv6 only, allowing a
separate IPv4 socket (`0.0.0.0:<port>`) to bind on the same port.

Detailed implementation steps, config examples, and platform portability notes
are documented in the issue spec ([#1671](https://github.com/torrust/torrust-tracker/issues/1671))
and in the research document
[docs/issues/open/1671-ipv4-ipv6-client-metrics/research-dual-stack-portability.md](https://github.com/torrust/torrust-tracker/blob/develop/docs/issues/open/1671-ipv4-ipv6-client-metrics/research-dual-stack-portability.md).

### Alternatives Considered

**A) Always set `IPV6_V6ONLY=1` unconditionally (no config option).**

Rejected because it forces every operator to explicitly configure both address
families, breaking existing configs. While the project plans a 4.0.0 release
where breaking changes are acceptable, this particular change does not need to
be forced — operators who want separate sockets can opt in.

**B) Always set `IPV6_V6ONLY=1` in 4.0.0 with a migration guide.**

Rejected for the same reason. Adding the config option is minimal effort and
preserves operator choice without unnecessary breakage.

### Consequences

- **Positive**: Operators opt into separate IPv4/IPv6 sockets without changing
  the default for everyone.
- **Positive**: The name `ipv6_v6only` matches the underlying socket option,
  making it searchable.
- **Negative**: Small maintenance surface — the option must be documented and
  tested.
- **Negative**: Platform-dependent behaviour — OpenBSD cannot use dual-stack
  mode, must be documented.
