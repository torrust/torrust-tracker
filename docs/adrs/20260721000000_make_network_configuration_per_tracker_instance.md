---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - issue #1640
    - issue #1978
    - packages/configuration/src/v3_0_0/network.rs
    - packages/configuration/src/v3_0_0/http_tracker.rs
    - packages/configuration/src/v3_0_0/udp_tracker.rs
    - packages/configuration/src/v3_0_0/core.rs
    - docs/adrs/20260617093046_reject_wildcard_external_ip.md
    - docs/adrs/20260620000000_add_ipv6_v6only_config_option.md
---

# Make Network Configuration Per Tracker Instance

## Description

Schema v2 placed `external_ip` and `on_reverse_proxy` in the global `[core.net]`
section, while `ipv6_v6only` was duplicated as a flat field on each HTTP and UDP
tracker. This model cannot represent trackers with distinct public addresses,
reverse-proxy trust policies, or socket behavior.

## Agreement

Schema v3 places one optional `network: Network` value on each `HttpTracker` and
`UdpTracker`. The corresponding TOML `[*.network]` block contains:

- `external_ip`
- `on_reverse_proxy`
- `ipv6_v6only`

When the block is omitted, it defaults to `external_ip = None`,
`on_reverse_proxy = false`, and `ipv6_v6only = false`.

Schema v3 removes `[core.net]` and the flat tracker `ipv6_v6only` fields. It
does not accept those removed fields, fall back to them, or define precedence
between the old and new layouts. Schema v2 remains separately available for
backward compatibility; application-wide migration to v3 is deferred to EPIC
subissue #1980.

When application consumers migrate to schema v3 in EPIC subissue #1980,
`AnnounceHandler` will receive the applicable instance's external IP as a
parameter instead of owning global network-topology configuration.

## Alternatives Considered

### Keep global `[core.net]`

Rejected because a global setting cannot model independent tracker instances.

### Support old and new fields in schema v3

Rejected because it would make a breaking schema ambiguous, require a precedence
rule, and leave obsolete configuration behavior in production code.

### Keep `ipv6_v6only` flat on each tracker

Rejected because all three values describe the same per-instance network topology
and socket behavior.

## Consequences

- **Positive**: Each listener has an explicit, independently configurable network identity.
- **Positive**: Reverse-proxy trust is correctly scoped to the HTTP listener handling a request.
- **Positive**: The v3 schema has one clear configuration layout with no hidden fallback.
- **Negative**: Operators must migrate v2 configuration files before using schema v3.

## Date

2026-07-21
