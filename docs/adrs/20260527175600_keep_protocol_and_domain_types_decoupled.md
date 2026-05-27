---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md
    - docs/adrs/index.md
    - packages/primitives/src/number_of_bytes.rs
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/udp-protocol/src/common.rs
---

# Keep Protocol And Domain Types Decoupled

## Description

Several value types currently exist in more than one package with similar field
shapes. A representative example is `NumberOfBytes`, which appears in:

- `packages/primitives/src/number_of_bytes.rs` (domain-level meaning)
- `packages/http-protocol/src/v1/requests/announce.rs` (HTTP protocol DTO)
- `packages/udp-protocol/src/common.rs` (UDP protocol wire type)

At first glance this can look like accidental duplication that should be
deduplicated into one shared type. However, these types live at different
architectural boundaries and have different reasons to change.

The decision needed here is whether to enforce a single shared type across
layers/protocols, or to keep layer-local/protocol-local types and map at
boundaries.

## Agreement

Keep protocol and domain types decoupled, even when they share similar shape.

This means:

- Domain types remain domain-owned in `packages/primitives`.
- Protocol crates (`http-protocol`, `udp-protocol`) keep protocol-local types.
- Adapters perform explicit mapping at boundaries.

This is an application of single-responsibility design: each layer has one
primary reason to change.

- Domain types change when tracker domain/business policy changes.
- HTTP protocol types change when HTTP/BEP behavior or encoding constraints
  change.
- UDP protocol types change when UDP/BEP behavior or wire representation
  changes.

As a consequence, a UDP wire-format change should not force broad domain
refactors, and a domain policy change should not force protocol crates to adopt
domain-centric shape.

### Alternatives Considered

**Single shared type for all layers/protocols** (for example one global
`NumberOfBytes` used by domain + HTTP + UDP).

Rejected because:

1. It couples protocol evolution to domain internals and vice versa.
2. It increases blast radius for protocol-specific changes.
3. It weakens boundary ownership and pushes cross-layer assumptions into shared
   packages.

### Consequences

#### Positive

- Clear boundaries and ownership per layer.
- Lower coupling between protocol evolution and tracker-domain evolution.
- Easier extraction/publication of protocol crates as independently evolving
  packages.

#### Negative

- Some mapping code is required at adapter boundaries.
- Similar-looking structs may appear duplicated and require explicit
  documentation to avoid accidental re-coupling.

## Date

2026-05-27

## References

- EPIC: [docs/issues/open/1669-overhaul-packages/EPIC.md](../issues/open/1669-overhaul-packages/EPIC.md)
- Subissue SI-14: [docs/issues/open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md](../issues/open/1835-1669-14-decouple-http-protocol-from-tracker-primitives.md)
- GitHub issue #1835: <https://github.com/torrust/torrust-tracker/issues/1835>
