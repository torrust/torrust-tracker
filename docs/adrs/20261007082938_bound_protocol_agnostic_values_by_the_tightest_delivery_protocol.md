---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - "issue #1978"
    - "issue #2245"
    - packages/primitives/src/announce.rs
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md
---

<!-- skill-link: create-adr -->

# Bound Protocol-Agnostic Values by the Tightest Delivery Protocol

## Scope

Repository-level. The decision spans the configuration schema, the domain types in `primitives`,
and every delivery-protocol package (`udp-server`, `http-protocol`, `axum-http-server`), so it
belongs in `docs/adrs/`.

## Description

The tracker serves the same swarm through several delivery protocols: UDP (BEP 15), HTTP (BEP 3
and BEP 23), and possibly others later, such as WebTorrent. Some configured values are sent to
clients by every protocol, but each protocol encodes them differently. The announce interval is the
first case: BEP 15 encodes it as a signed 32-bit integer, while HTTP encodes it as a bencode
integer (`i64` in `http-protocol`).

The configuration and domain types held the interval as a plain `u32`, so a value above
`i32::MAX` was accepted. Before #2245, UDP sent such a value as a negative number. #2245 clamped
it on the UDP wire, which hid the problem: UDP and HTTP clients then received different intervals
for the same configuration.

The [value-invariant ADR](20260723184019_separate_configuration_value_invariants_from_consistency_validation.md)
says *how* to reject a single-value bound: a typed newtype, rejected during deserialization. It
does not say *which* bound to choose when the delivery protocols disagree, or where such a type
lives.

## Agreement

A configuration value that every delivery protocol sends to clients is **one protocol-agnostic
value**, not one value per protocol.

Its bound is the **tightest limit among the supported delivery protocols**. For the announce
interval that is `i32::MAX` seconds (about 68 years), from BEP 15; realistic values are minutes to
hours.

The bound is a domain rule (the value must be representable on every supported protocol), so it is
enforced by a validated newtype in `primitives`. The configuration uses that domain type directly,
as it already uses `AnnouncePolicy`. Following the value-invariant ADR, construction and
`Deserialize` reject out-of-range values, so an invalid configuration fails at load.

Delivery adapters then convert the value to their wire type without clamping or failing. Clamping
remains appropriate only for values that no configuration bounds, such as peer counts.

When a new delivery protocol has a tighter limit, the shared bound is lowered. That is a breaking
configuration change and must be released as one.

This rule covers values the operator configures once and the tracker sends through every protocol.
It does not cover limits on what clients send, which differ per protocol for their own reasons; see
[Cap scrape info hashes per protocol](20261005124222_cap_scrape_info_hashes_per_protocol.md).

## Alternatives Considered

### One configuration value per delivery protocol

Rejected. It duplicates a single concept across protocols and lets operators configure different
intervals for the same swarm by accident.

### Validate the shared value per delivery protocol

Rejected. A value could be valid for HTTP and invalid for UDP, so enabling a protocol would turn a
working configuration into an invalid one. It also needs consistency rules that depend on which
protocols are enabled.

### Clamp on the wire only

Rejected. This was the state after #2245. It silently changes the configured value for some
clients, and the protocols then disagree.

### An arbitrary "sensible" cap

Rejected. A cap such as one day has no source to derive it from, and choosing it is a separate
policy decision. The protocol limit is objective.

### A bounded schema type only in `configuration`

Rejected. It keeps `primitives` unaware of the bound, but the domain type stays unbounded, so the
delivery adapters still need a fallible conversion or a clamp.

## Consequences

- **Positive**: one unambiguous value per concept; all delivery protocols report the same value.
- **Positive**: invalid configurations fail at load with a clear error instead of being clamped.
- **Positive**: delivery adapters convert infallibly, with no clamp to keep in sync.
- **Negative**: the bound is lower than some protocols need. For the announce interval this costs
  nothing in practice.
- **Negative**: adding a protocol with a tighter limit is a breaking configuration change.
- **Negative**: changing a `primitives` field type is a breaking public API change for its
  consumers.

## Affected Code

- [`packages/primitives/src/announce.rs`](../../packages/primitives/src/announce.rs):
  `AnnouncePolicy::interval` and `interval_min`, which #2466 changes to the bounded type.
- [`packages/udp-server/src/handlers/announce.rs`](../../packages/udp-server/src/handlers/announce.rs):
  the UDP response encodes the interval through `saturating_wire_i32` until #2466 converts it
  without clamping.

Issue #2466 adds module-level doc comments in both places that link back to this ADR.

## Date

2026-10-07

## References

- [Configuration Overhaul EPIC #1978](https://github.com/torrust/torrust-tracker/issues/1978)
- [Issue #2245](https://github.com/torrust/torrust-tracker/issues/2245) — review numeric protocol wire conversions (introduced the UDP clamp)
- [BEP 15: UDP Tracker Protocol](https://www.bittorrent.org/beps/bep_0015.html)
- [Separate Configuration Value Invariants from Consistency Validation](20260723184019_separate_configuration_value_invariants_from_consistency_validation.md)
- [Use Newtypes for Domain-Constrained Configuration Field Types](20260721100000_use_newtypes_for_constrained_configuration_field_types.md)
