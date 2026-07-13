# Analysis: Should `announce_builder::Query` Be Merged with `announce::Announce`?

**Date**: 2026-07-13
**Status**: Open for discussion — updated after user feedback
**Context**: [PR #1974](https://github.com/torrust/torrust-tracker/pull/1974) — EPIC 1669 SI-34: Consolidate Duplicate HTTP Types

## The Two Structs

### Client-side: `announce_builder::Query`

```rust
pub struct Query {
    pub info_hash: InfoHash,
    pub peer_addr: IpAddr,       // ← BEP 3 "ip" parameter
    pub downloaded: BaseTenASCII, // u64, always present, default 0
    pub uploaded: BaseTenASCII,   // u64, always present, default 0
    pub peer_id: PeerId,
    pub port: PortNumber,         // u16
    pub left: BaseTenASCII,       // u64, always present, default 0
    pub event: Option<Event>,
    pub compact: Option<Compact>,
    pub numwant: Option<u32>,
}
```

- **Purpose**: Build outgoing announce URLs (client-side)
- **Construction**: Fluent builder (`QueryBuilder::with_default_values().with_*().query()`)
- **Consumption**: `.to_string()` / `.build()` / `.params()` → URL query string

### Server-side: `announce::Announce`

```rust
pub struct Announce {
    pub info_hash: InfoHash,
    pub peer_id: PeerId,
    pub port: u16,
    pub downloaded: Option<NumberOfBytes>, // Option<i64>, truly optional
    pub uploaded: Option<NumberOfBytes>,   // Option<i64>, truly optional
    pub left: Option<NumberOfBytes>,       // Option<i64>, truly optional
    pub event: Option<Event>,
    pub compact: Option<Compact>,
    pub numwant: Option<u32>,
    // MISSING: peer_addr — BEP 3 "ip" parameter
}
```

- **Purpose**: Parse incoming announce requests (server-side)
- **Construction**: `TryFrom<crate::v1::query::Query>` — fallible parsing from raw URL query string
- **Consumption**: Passed to `AnnounceService::handle_announce()`

## Data-Flow Diagram

```text
CLIENT SIDE (outgoing):                    SERVER SIDE (incoming):
QueryBuilder → Query → .to_string()        URL string → crate::v1::query::Query → TryFrom → Announce
                 ↓                          ↓
           URL query string ────────────→  HTTP request
```

These are **two different points in the pipeline**. Merging them would force one direction's
concerns into the other.

## Semantic Differences

### 1. `peer_addr` — NOT a Genuine Difference (Updated)

| Aspect           | `Query` (client) | `Announce` (server) |
| ---------------- | ---------------- | ------------------- |
| Has `peer_addr`? | Yes (`IpAddr`)   | **No — but should** |

**BEP 3** defines `ip` as a standard optional announce parameter:

> **ip** — An optional parameter giving the IP (or dns name) which this peer is at.
> Generally used for the origin if it's on the same machine as the tracker.

The current `Announce` doc comment says: _"The struct does not contain the IP of the peer.
It's not mandatory and it's not used by the tracker. The IP is obtained from the request itself."_

However:

- The `tracker-client` crate is planned for publication on crates.io and should follow the
  protocol specification
- Users have requested a tracker configuration option to use the peer address from announce
  requests instead of the connection IP (see
  [discussion #532](https://github.com/torrust/torrust-tracker/discussions/532#issuecomment-1836642956))
- `peer_addr` should be added to `Announce` regardless of whether the two types are merged

**Conclusion**: `peer_addr` is no longer a reason to keep the types separate. It should exist
in both.

### 2. Byte Counters — NOT a Genuine Difference (Updated)

| Aspect      | `Query` (client)               | `Announce` (server)                          |
| ----------- | ------------------------------ | -------------------------------------------- |
| Type        | `u64` (raw integer)            | `Option<NumberOfBytes>` (newtype over `i64`) |
| Optionality | Always present (defaults to 0) | Truly optional (may be absent from request)  |
| Signedness  | Unsigned                       | Signed                                       |

**BEP 3** defines `uploaded`, `downloaded`, and `left` as standard parameters but does not
mandate that they are always present. The protocol-level semantics are that they are optional.

The current `Query` makes them always-present with a default of 0, but this is a builder
convenience, not a protocol requirement. The `Announce` type correctly models them as
`Option<NumberOfBytes>`.

The builder's `u64` type and non-optional default of 0 is only used in **2 files** (the
`console/tracker-client` CLI apps), where it simply passes through CLI arguments. Changing
the builder to use `Option<NumberOfBytes>` would be a trivial update to those 2 call sites.

**Conclusion**: Byte counter types are no longer a reason to keep the types separate. The
builder should adopt `Option<NumberOfBytes>` to match the protocol semantics and align with
`Announce`.

### 3. Construction Patterns — Fundamentally Different

| Aspect         | `Query` (client)              | `Announce` (server)               |
| -------------- | ----------------------------- | --------------------------------- |
| Pattern        | Fluent builder                | Fallible `TryFrom`                |
| Error handling | Infallible (defaults)         | Fallible (invalid params → error) |
| Use case       | Ergonomic client construction | Robust server parsing             |

## Usage Across the Codebase

### `announce_builder::Query` consumers (client-side)

| File                                                                | How used                                                                      |
| ------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `packages/tracker-client/src/http/client/mod.rs`                    | `announce(&self, query: &Query)` — builds URL from query                      |
| `packages/axum-http-server/tests/server/client.rs`                  | `announce(&self, query: &Query)` — test client (duplicate, should be removed) |
| `console/tracker-client/src/console/clients/checker/checks/http.rs` | Constructed via `QueryBuilder`, passed to client                              |
| `console/tracker-client/src/console/clients/http/app.rs`            | Constructed via `QueryBuilder`, passed to client                              |
| `console/tracker-client/src/console/clients/unified/http.rs`        | Constructed via `QueryBuilder`, passed to client                              |
| `packages/axum-http-server/tests/server/v1/contract.rs`             | ~47 occurrences via `QueryBuilder::default().query()`                         |
| `tests/servers/api/contract/stats/mod.rs`                           | Constructed via `QueryBuilder`, passed to client                              |

### `announce::Announce` consumers (server-side)

| File                                                              | How used                                                   |
| ----------------------------------------------------------------- | ---------------------------------------------------------- |
| `packages/axum-http-server/src/v1/extractors/announce_request.rs` | Axum extractor: `TryFrom<Query>`                           |
| `packages/axum-http-server/src/v1/handlers/announce.rs`           | Passed to `AnnounceService::handle_announce()`             |
| `packages/http-core/src/services/announce.rs`                     | `handle_announce(&self, announce_request: &Announce, ...)` |

**There are zero conversions between `announce_builder::Query` and `announce::Announce` anywhere
in the codebase.** They are completely separate types with no shared code path.

## Alignment with Issue Design Decisions

The issue spec's **DD1** already anticipated this question:

> **DD1: Merge Strategy — Add Builder Types Alongside Parsers (Iteration 1)**
>
> In the first iteration, add builder types to `http-protocol` alongside the existing parser types.
> After consolidation, a second iteration can evaluate whether a unified data model for both
> parsing and building makes sense.

This analysis is that "second iteration" evaluation.

## Final Decision: Merge Into a Single `Announce` Struct

**Decision**: Merge `announce_builder::Query` into `announce::Announce`. Remove the
`announce_builder` module entirely.

### Rationale

All three original blockers have been resolved by aligning with the BEP 3 protocol specification:

| Blocker               | Resolution                                                                                                                                                                                |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `peer_addr`           | BEP 3 defines `ip` as a standard optional parameter. `Announce` should have `peer_addr: Option<IpAddr>`.                                                                                  |
| Byte counters         | BEP 3 treats `uploaded`/`downloaded`/`left` as optional. Both sides should use `Option<NumberOfBytes>`.                                                                                   |
| Construction patterns | The builder pattern can coexist with `TryFrom<Query>` on the same struct — they serve different use cases (client-side construction vs server-side parsing) but operate on the same data. |

### Implementation Plan

1. Add `peer_addr: Option<IpAddr>` to `Announce` (per BEP 3)
2. Add a `Display` impl to `Announce` that serializes it to a URL query string (replacing `QueryParams`)
3. Add an `AnnounceBuilder` that produces `Announce` directly (replacing the current `announce_builder::QueryBuilder`), with builder methods accepting `u64` and converting to `NumberOfBytes` internally for ergonomics
4. Remove the `announce_builder` module entirely
5. Update all call sites (~47 in contract tests, ~5 in CLI apps, 2 client implementations)

### Impact

- **`Announce`** gains: `peer_addr` field, `Display` impl (URL serialization), `AnnounceBuilder`
- **Removed**: `announce_builder::Query`, `announce_builder::QueryBuilder`, `announce_builder::QueryParams`, `BaseTenASCII`, `PortNumber` type aliases
- **Call sites**: `announce_builder::Query` → `Announce`, `QueryBuilder` → `AnnounceBuilder`
- **Duplicate test client**: `packages/axum-http-server/tests/server/client.rs` should be removed in favor of `packages/tracker-client/src/http/client/mod.rs` (tracked separately)
