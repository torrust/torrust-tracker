---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/issues/open/1930-1669-si-33-rest-api-contract-first-architecture.md
    - docs/packages.md
    - packages/rest-api-protocol/
    - packages/rest-api-application/
    - packages/rest-api-runtime-adapter/
    - packages/axum-rest-api-server/
    - packages/rest-api-client/
    - docs/adrs/index.md
---

# Adopt a Contract-First Architecture for the REST API

## Description

The tracker REST API had no dedicated, reusable contract package. Request/response
DTOs were defined locally inside the Axum server package (`axum-rest-api-server`),
and the `rest-api-core` package acted as integration glue around tracker internals
rather than a clean application layer. This made package boundaries hard to enforce,
complicated generic client implementations, and blocked the path toward a future
tracker-agnostic REST API standard.

## Agreement

Adopt a **contract-first layered architecture** for the REST API, structured into
four distinct layers with enforced dependency direction:

### Layer 1 — Protocol Contract Package (`torrust-tracker-rest-api-protocol`)

A dedicated crate for versioned REST contract artifacts. It owns:

- Versioned endpoint contract modules (`v1`, `v2`, ...).
- Request/response DTOs, error schemas, and status mapping contracts.
- Auth contract surface (transport-agnostic semantics).
- Optional API capability/introspection structures for future interoperability.

> **Version coexistence**: multiple API versions coexist in the same codebase under
> versioned namespace modules (e.g., `v1/`, `v2/`) — a pattern called **version by
> namespace convention**. See ADR
> [20260629000000](20260629000000_adopt_independent_package_versioning.md) for the
> rationale and decision.

It does **not** own Axum, runtime server wiring, or tracker database logic.

### Layer 2 — Application Package (`torrust-tracker-rest-api-application`)

A use-case / port layer that defines the API's business logic boundary. It owns:

- Port traits (interfaces) for each API domain (`TorrentQueryPort`, etc.).
- Use-case services (`TorrentApiService`, etc.) that orchestrate port calls.
- Mapping of domain errors to protocol-level error categories.

It does **not** own Axum, HTTP transport, or tracker-internal implementations.

### Layer 3 — Runtime Adapter Package (`torrust-tracker-rest-api-runtime-adapter`)

A tracker-specific bridge that implements the application ports. It owns:

- Tracker-specific adapter implementations (`TrackerTorrentQueryAdapter`, etc.).
- Conversion functions between domain types (`Info`, `BasicInfo`, `peer::Peer`)
  and protocol DTOs.
- Dependency composition for the tracker runtime.

It is the only REST API layer that depends on `tracker-core` and other tracker
internals.

### Layer 4 — Transport Adapter Package (`axum-rest-api-server`, existing)

The existing Axum HTTP server refactored to be a thin transport adapter. It owns:

- HTTP routing, request extraction, response serialization, middleware.
- Binding protocol DTOs to application layer calls.

It does **not** own business logic or direct domain orchestration.

### Dependency rules

**Allowed edges:**

- `axum-rest-api-server → rest-api-application`
- `axum-rest-api-server → rest-api-protocol`
- `rest-api-client → rest-api-protocol`
- `rest-api-application → rest-api-protocol`
- `rest-api-runtime-adapter → tracker internals + rest-api-application`

**Forbidden edges (target state, once migration is complete):**

- `axum-rest-api-server → tracker-core` (direct)
- `axum-rest-api-server → http-core` (direct)
- `axum-rest-api-server → udp-core` (direct)
- `axum-rest-api-server → udp-server` (direct)

These forbidden edges are currently present and represent the coupling that this
architecture resolves by introducing the application and adapter layers.

### Long-term vision

This architecture positions the protocol contract package for potential extraction
into a standalone, tracker-agnostic REST API standard. By decoupling wire-format
contracts from tracker-internal implementation details, other tracker
implementations could adopt the same protocol surface and interoperate with
existing clients. This extraction is deferred until the API stabilizes — the
current priority is validating the boundaries within the Torrust Tracker codebase.

## Date

2026-06-23

## Alternatives Considered

### Alternative A — Keep current packages and only refactor endpoints in place

**Rejected because:** contract and implementation remain coupled, reuse by other
trackers remains weak, and repeated endpoint fixes keep accumulating architecture
debt.

### Alternative B — Mirror UDP/HTTP tracker layering (codec → core → server)

**Rejected because:** REST protocol concerns are broader than parser/codec
concerns — they include status codes, auth semantics, error schema, resource and
command modeling. A strict clone of UDP/HTTP layering does not naturally represent
REST contract governance needs. The REST API needs a protocol-contract package,
an application-layer boundary, and transport adapters — more layers than the
UDP/HTTP tracker stack.

### Alternative C — Jump directly to v2 redesign before boundary refactor

**Rejected because:** high rework risk while package boundaries are unclear, and
harder to keep v1 compatibility while extracting reusable contract assets.

## References

- Issue [#1930](https://github.com/torrust/torrust-tracker/issues/1930): Define REST API contract-first package architecture for EPIC #1669
- EPIC [#1669](https://github.com/torrust/torrust-tracker/issues/1669): Overhaul: Packages
- Draft PR [#1936](https://github.com/torrust/torrust-tracker/pull/1936): PoC branch
- Issue [#144](https://github.com/torrust/torrust-tracker/issues/144): API v2 behavior changes (future)
- ADR [20260527175600](./20260527175600_keep_protocol_and_domain_types_decoupled.md): Keep protocol and domain types decoupled
