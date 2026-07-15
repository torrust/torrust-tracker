# Analysis: Should `announce_deserialization` Types Be Merged with `announce` Response Types?

**Date**: 2026-07-13
**Status**: Open for discussion — updated after architectural review
**Context**: [PR #1974](https://github.com/torrust/torrust-tracker/pull/1974) — EPIC 1669 SI-34: Consolidate Duplicate HTTP Types
**Related**: [`analysis-announce-query-vs-announce.md`](./analysis-announce-query-vs-announce.md) — same issue, request-side analysis

## Architectural Layers

Unlike the request side (which has a single layer — parse URL string → DTO), the response
side has **two layers of abstraction** within the HTTP protocol crate:

```text
                        DOMAIN LAYER
                  primitives::AnnounceData
                            │
                  to_protocol_announce_data()
                            │
              ┌─────────────┴─────────────┐
              │    PROTOCOL DTO LAYER      │  ← transport-agnostic
              │    announce::AnnounceData  │     "what" data goes in the response
              └─────────────┬─────────────┘
                            │
              ┌─────────────┴─────────────┐
              │    ENCODING LAYER          │  ← format-specific
              │    Normal / Compact        │     "how" data is serialized
              └─────────────┬─────────────┘
                            │
                      bencode bytes
                            │
              ┌─────────────┴─────────────┐
              │    CLIENT DESERIALIZATION  │  ← reverse of DTO layer
              │    announce_deserialization│
              └───────────────────────────┘
```

The extra layer exists because the wire accepts **two formats** (Normal per BEP 3, Compact
per BEP 23). `AnnounceData` abstracts over both — it says _what_ data goes in the response
without binding to _how_ it's encoded. `Normal` and `Compact` are encoding strategies that
take that DTO and produce the wire format.

The client-side `announce_deserialization` types are the **reverse of the DTO layer** — they
represent the same conceptual data as `AnnounceData`, just coming from the opposite direction
(deserialization instead of construction).

## The Two Modules

### Server-side: `announce.rs` — DTO + Encoding

Located at `packages/http-protocol/src/v1/responses/announce.rs`.

Contains both the DTO layer and the encoding layer. Used in exactly **one place** outside
its own crate: `packages/axum-http-server/src/v1/handlers/announce.rs`.

**DTO layer types** (transport-agnostic, "what" data):

| Type             | Purpose                                     |
| ---------------- | ------------------------------------------- |
| `AnnounceData`   | DTO: peers + stats + policy                 |
| `AnnouncePolicy` | `interval` + `interval_min`                 |
| `SwarmMetadata`  | `complete` + `downloaded` + `incomplete`    |
| `Peer`           | `peer_id: PeerId` + `peer_addr: SocketAddr` |

**Encoding layer types** (format-specific, "how" to serialize):

| Type                 | Purpose                                                                      |
| -------------------- | ---------------------------------------------------------------------------- |
| `Announce<E>`        | Generic wrapper: `E: From<AnnounceData> + Into<Vec<u8>>`                     |
| `Normal`             | Non-compact encoding: `i64` fields + `Vec<NormalPeer>`                       |
| `Compact`            | Compact encoding: `i64` fields + `peers: Vec<u8>` + `peers6: Vec<u8>`        |
| `NormalPeer`         | `peer_id: [u8; 20]`, `ip: IpAddr`, `port: u16`                               |
| `CompactPeer`        | **Enum**: `V4(CompactPeerData<Ipv4Addr>)` or `V6(CompactPeerData<Ipv6Addr>)` |
| `CompactPeerData<V>` | Generic: `ip: V`, `port: u16`                                                |

Data flow:

```text
Domain (primitives::AnnounceData)
    │
    ▼  to_protocol_announce_data()  [axum-http-server handler]
    │
announce::AnnounceData  (DTO layer)
    │
    ├──►  announce::Announce<announce::Normal>   (encoding layer)  ──►  bencode bytes
    └──►  announce::Announce<announce::Compact>  (encoding layer)  ──►  bencode bytes
```

### Client-side: `announce_deserialization.rs` — Reverse DTO Layer

Located at `packages/http-protocol/src/v1/responses/announce_deserialization.rs`.

Deserializes bencode-encoded announce responses. These types are the **reverse of the DTO
layer** — they represent the same conceptual data as `AnnounceData`, just coming from the
opposite direction.

Used in:

- `console/tracker-client/` — CLI tracker client (3 files)
- `packages/axum-http-server/tests/` — integration test assertions (2 files)

Key types:

| Type                  | Purpose                                                          | Equivalent DTO concept           |
| --------------------- | ---------------------------------------------------------------- | -------------------------------- |
| `Announce`            | Non-compact response: `u32` fields + `Vec<DictionaryPeer>`       | `AnnounceData` (non-compact)     |
| `DictionaryPeer`      | `peer_id: Vec<u8>`, `ip: String`, `port: u16`                    | `Peer`                           |
| `DeserializedCompact` | Raw compact response: `u32` fields + `peers: Vec<u8>`            | `AnnounceData` (compact, raw)    |
| `Compact`             | Parsed compact response: `u32` fields + `peers: CompactPeerList` | `AnnounceData` (compact, parsed) |
| `CompactPeerList`     | Wrapper: `peers: Vec<CompactPeer>`                               | `Vec<Peer>`                      |
| `CompactPeer`         | **Struct**: `ip: Ipv4Addr`, `port: u16` (IPv4 only)              | `CompactPeer` (but incomplete)   |

Data flow:

```text
bencode bytes
    │
    ▼  serde_bencode::from_bytes()
    │
    ├──►  announce_deserialization::Announce          (non-compact DTO)
    └──►  announce_deserialization::DeserializedCompact  ──►  announce_deserialization::Compact  (compact DTO)
```

## The Real Question

The question isn't "should we merge the encoding layer with the deserialization types?" —
those are at different layers. The question is:

**Should the client-side deserialization types be unified with the server-side DTO types
(`AnnounceData`)?**

They represent the same conceptual data — peers, stats, policy — just with different type
choices (wire-friendly vs domain-friendly).

## Naming Collision

There is a **direct naming collision** between the two modules:

| Name          | `announce::` (server)                                       | `announce_deserialization::` (client)                 |
| ------------- | ----------------------------------------------------------- | ----------------------------------------------------- |
| `Announce`    | Generic wrapper `Announce<E>` (encoding layer)              | Non-compact response struct (DTO layer)               |
| `Compact`     | `struct Compact { i64, Vec<u8>, Vec<u8> }` (encoding layer) | `struct Compact { u32, CompactPeerList }` (DTO layer) |
| `CompactPeer` | `enum CompactPeer { V4(...), V6(...) }` (encoding layer)    | `struct CompactPeer { Ipv4Addr, u16 }` (DTO layer)    |

The `mod.rs` re-exports `pub use announce::{Announce, Compact, Normal}`, so bare
`responses::Compact` refers to the **server-side encoding** type. The client-side types must
be accessed via the full path `announce_deserialization::Compact`.

## Semantic Differences (DTO Layer vs Deserialization)

### 1. Integer Types: `u32` vs `u32` (Already Aligned)

| Field          | `AnnounceData` (server DTO) | `announce_deserialization::Announce` (client) |
| -------------- | --------------------------- | --------------------------------------------- |
| `complete`     | `u32`                       | `u32`                                         |
| `incomplete`   | `u32`                       | `u32`                                         |
| `interval`     | `u32`                       | `u32`                                         |
| `min_interval` | `u32`                       | `u32`                                         |

The DTO layer already uses `u32`. The encoding layer (`Normal`/`Compact`) uses `i64` for
bencode compatibility, but that's an encoding concern, not a DTO concern. **No conflict.**

### 2. Peer Representations

#### Non-compact peers

| Aspect    | `Peer` (server DTO)                | `DictionaryPeer` (client)           |
| --------- | ---------------------------------- | ----------------------------------- |
| `peer_id` | `PeerId` (newtype over `[u8; 20]`) | `Vec<u8>` (variable, `serde_bytes`) |
| `ip`      | `SocketAddr` (parsed)              | `String` (raw)                      |
| `port`    | `u16` (via `SocketAddr`)           | `u16`                               |

**Can they be unified?** The server DTO uses domain-friendly types (`PeerId`, `SocketAddr`)
because it's constructed from domain data. The client uses wire-friendly types (`Vec<u8>`,
`String`) because it's deserialized from bencode. This is the same protocol-vs-domain
decoupling we accept elsewhere. A unified type would need to handle both construction paths,
or we accept that the DTO and deserialization types use different representations.

#### Compact peers

| Aspect | `announce::CompactPeer` (server encoding)         | `announce_deserialization::CompactPeer` (client)      |
| ------ | ------------------------------------------------- | ----------------------------------------------------- |
| Kind   | **Enum** (V4/V6)                                  | **Struct** (IPv4 only)                                |
| IPv6   | ✅ Supported                                      | ❌ Panics: `"IPV6 is not supported for compact peer"` |
| Fields | `V4(CompactPeerData { ip: Ipv4Addr, port: u16 })` | `ip: Ipv4Addr`, `port: u16` (private)                 |

**Can they be unified?** The server-side enum is the correct representation — it supports
both IPv4 and IPv6 per BEP 7/BEP 23. The client-side struct is incomplete and should be
upgraded to support IPv6 regardless of whether we merge. `CompactPeerData<V>` from the
server side could be shared directly.

### 3. Serialization Strategy (Encoding Layer Only)

| Aspect    | Server encoding (`Normal`/`Compact`)                             | Client deserialization                        |
| --------- | ---------------------------------------------------------------- | --------------------------------------------- |
| Approach  | Manual bencode via `ben_map!` / `ben_int!` / `ben_bytes!` macros | `serde_bencode` with `#[derive(Deserialize)]` |
| Direction | `Into<Vec<u8>>` (serialize only)                                 | `Deserialize` (deserialize only)              |

**This is NOT a blocker for DTO unification.** The encoding layer (`Normal`/`Compact`) and
the deserialization types are at different layers. The encoding layer stays as-is. The
question is only about the DTO layer.

### 4. IPv6 Support Gap

The server-side `Compact` (encoding layer) includes `peers6: Vec<u8>` for IPv6 peers
(BEP 7). The client-side `DeserializedCompact` and `Compact` have **no `peers6` field**.

This is a bug/limitation in the client-side types that should be fixed regardless of
whether we merge.

### 5. `Announce` Name Collision

| Module                     | Type          | Layer                                         |
| -------------------------- | ------------- | --------------------------------------------- |
| `announce`                 | `Announce<E>` | Encoding layer (generic wrapper)              |
| `announce_deserialization` | `Announce`    | DTO layer (non-compact deserialized response) |

The server-side `Announce<E>` is a generic wrapper at the encoding layer. The client-side
`Announce` is a concrete non-compact response at the DTO layer. These are different
concepts at different layers sharing the same name.

## Usage Across the Codebase

### Server-side DTO + Encoding (`announce`) consumers

| File                                                    | How used                                                                                                      |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `packages/axum-http-server/src/v1/handlers/announce.rs` | `to_protocol_announce_data()` → `AnnounceData`; `build_response()` → `Announce<Normal>` / `Announce<Compact>` |

Only **one** production consumer. Very tightly scoped.

### Client-side deserialization (`announce_deserialization`) consumers

| File                                                                | How used                                                                                |
| ------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `console/tracker-client/src/console/clients/checker/checks/http.rs` | `serde_bencode::from_bytes::<Announce>(&response)`                                      |
| `console/tracker-client/src/console/clients/http/app.rs`            | `serde_bencode::from_bytes::<Announce>(&body)` + fallback to `DeserializedCompact`      |
| `console/tracker-client/src/console/clients/unified/http.rs`        | Same pattern as `app.rs`                                                                |
| `packages/axum-http-server/tests/server/asserts.rs`                 | Test assertions using `Announce`, `DeserializedCompact`, `Compact`                      |
| `packages/axum-http-server/tests/server/v1/contract.rs`             | Constructing expected responses with `DictionaryPeer`, `CompactPeerList`, `CompactPeer` |

## Recommendation: Partial Merge — Unify DTO Layer, Keep Encoding Layer Separate

### What to merge (DTO layer)

The client-side deserialization types and the server-side DTO types represent the same
conceptual data. They should live in the same module with clear naming:

- `announce_deserialization::Announce` → rename to `announce::DeserializedNormal` and move into `announce.rs`
- `announce_deserialization::DeserializedCompact` → move into `announce.rs`
- `announce_deserialization::Compact` → rename to `announce::DeserializedCompactParsed` and move into `announce.rs`
- `announce_deserialization::CompactPeerList` → move into `announce.rs`
- `announce_deserialization::CompactPeer` → replace with `announce::CompactPeer` (the enum), upgrade to support IPv6
- `announce_deserialization::DictionaryPeer` → keep separate from `announce::Peer` (different type choices: wire-friendly vs domain-friendly)

### What to keep separate (encoding layer)

- `announce::Announce<E>` — generic wrapper, encoding layer concern
- `announce::Normal` — non-compact encoding, stays as-is
- `announce::Compact` — compact encoding, stays as-is
- `announce::NormalPeer` — encoding-specific peer representation, stays as-is

### What to fix regardless

1. **Add IPv6 support** to client-side compact types: add `peers6` field to
   `DeserializedCompact`, upgrade `CompactPeer` to use the server-side enum
2. **Fix naming**: eliminate the `Announce`/`Compact`/`CompactPeer` collisions
3. **Remove `announce_deserialization.rs`** as a separate module — consolidate into
   `announce.rs`

### Why not a full merge

The encoding layer (`Normal`/`Compact`/`Announce<E>`) uses `torrust_bencode` with manual
macro-based construction and `Into<Vec<u8>>`. The deserialization types use `serde_bencode`
with derive macros. These are fundamentally different serialization strategies serving
different directions (serialize vs deserialize). They should not be forced onto the same
structs.

## Module Structure: Making the Architecture Visible

The current flat file naming hides the layered architecture:

```text
responses/
    announce.rs                    ← DTO + Encoding mashed together
    announce_deserialization.rs    ← sounds like "serde for announce.rs" (misleading)
```

A newcomer reads this and thinks: "Why is deserialization in a separate file? Why not just
put `#[derive(Deserialize)]` on the types in `announce.rs`?" — which is exactly the wrong
conclusion, because the encoding layer uses `torrust_bencode` macros, not serde.

### Proposed Structure

```text
responses/
    announce/
        mod.rs               ← re-exports public API
        data.rs              ← DTO layer: transport-agnostic "what"
        encoding.rs          ← Encoding layer: format-specific "how"
        deserialization.rs   ← Client-side: reverse of DTO layer
```

The directory name `announce/` says "everything about announce responses." The three files
inside immediately reveal the three concerns:

| File                 | Layer           | Direction     | Question it answers               |
| -------------------- | --------------- | ------------- | --------------------------------- |
| `data.rs`            | DTO             | Neutral       | _What_ data goes in the response? |
| `encoding.rs`        | Encoding        | Server → Wire | _How_ is it serialized?           |
| `deserialization.rs` | Deserialization | Wire → Client | _How_ is it parsed?               |

No more confusion about why deserialization is separate — the file structure _is_ the
documentation.

### What goes where

**`announce/data.rs`** — The DTO layer. Transport-agnostic. Single source of truth for what
an announce response contains. Uses domain-friendly types (`PeerId`, `SocketAddr`):

```rust
// announce/data.rs
pub struct AnnounceData { pub peers: Vec<Peer>, pub stats: SwarmMetadata, pub policy: AnnouncePolicy }
pub struct AnnouncePolicy { pub interval: u32, pub interval_min: u32 }
pub struct SwarmMetadata { pub complete: u32, pub downloaded: u32, pub incomplete: u32 }
pub struct Peer { pub peer_id: PeerId, pub peer_addr: SocketAddr }
```

**`announce/encoding.rs`** — Format-specific serialization. "How" to turn the DTO into
bencode. Uses `torrust_bencode` macros:

```rust
// announce/encoding.rs
pub struct Announce<E: From<AnnounceData> + Into<Vec<u8>>> { pub data: E }
pub struct Normal { complete: i64, incomplete: i64, interval: i64, min_interval: i64, peers: Vec<NormalPeer> }
pub struct Compact { complete: i64, incomplete: i64, interval: i64, min_interval: i64, peers: Vec<u8>, peers6: Vec<u8> }
pub struct NormalPeer { pub peer_id: [u8; 20], pub ip: IpAddr, pub port: u16 }
pub enum CompactPeer { V4(CompactPeerData<Ipv4Addr>), V6(CompactPeerData<Ipv6Addr>) }
pub struct CompactPeerData<V> { pub ip: V, pub port: u16 }
```

**`announce/deserialization.rs`** — Client-side. Reverse of the DTO layer. Deserializes from
bencode wire format using `serde_bencode` derives. Uses wire-friendly types (`Vec<u8>`,
`String`):

```rust
// announce/deserialization.rs
pub struct DeserializedNormal { pub complete: u32, pub incomplete: u32, pub interval: u32, pub min_interval: u32, pub peers: Vec<DictionaryPeer> }
pub struct DictionaryPeer { pub ip: String, pub peer_id: Vec<u8>, pub port: u16 }
pub struct DeserializedCompact { pub complete: u32, pub incomplete: u32, pub interval: u32, pub min_interval: u32, pub peers: Vec<u8>, pub peers6: Vec<u8> }
pub struct DeserializedCompactParsed { pub complete: u32, pub incomplete: u32, pub interval: u32, pub min_interval: u32, pub peers: CompactPeerList }
pub struct CompactPeerList { peers: Vec<CompactPeer> }
// CompactPeer re-exported from encoding.rs (shared enum)
```

**`announce/mod.rs`** — Re-exports for backward compatibility:

```rust
// announce/mod.rs
pub mod data;
pub mod encoding;
pub mod deserialization;

// Re-export commonly used types at the module level
pub use data::{AnnounceData, AnnouncePolicy, Peer, SwarmMetadata};
pub use encoding::{Announce, Compact, CompactPeer, CompactPeerData, Normal, NormalPeer};
```

### Naming Changes Summary

| Old Name                                        | New Name                                               | Rationale                                                                            |
| ----------------------------------------------- | ------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| `announce_deserialization::Announce`            | `announce::deserialization::DeserializedNormal`        | Avoids collision with `encoding::Announce<E>`; mirrors `encoding::Normal`            |
| `announce_deserialization::Compact`             | `announce::deserialization::DeserializedCompactParsed` | Avoids collision with `encoding::Compact`; "Parsed" = bytes already split into peers |
| `announce_deserialization::DeserializedCompact` | `announce::deserialization::DeserializedCompact`       | Unchanged (already well-named)                                                       |
| `announce_deserialization::CompactPeer`         | `announce::encoding::CompactPeer` (shared)             | Client uses the server-side enum; gains IPv6 support                                 |
| `announce_deserialization::CompactPeerList`     | `announce::deserialization::CompactPeerList`           | Unchanged                                                                            |
| `announce_deserialization::DictionaryPeer`      | `announce::deserialization::DictionaryPeer`            | Unchanged; kept separate from `data::Peer` (wire vs domain types)                    |

### Same Pattern for Scrape

The scrape response types have the same problem (`scrape.rs` + `scrape_deserialization.rs`)
and should follow the same pattern:

```text
responses/
    scrape/
        mod.rs
        data.rs              ← DTO layer
        encoding.rs          ← Encoding layer
        deserialization.rs   ← Client-side deserialization
```

### Migration Path

1. Create `responses/announce/` directory
2. Move DTO types from `announce.rs` → `announce/data.rs`
3. Move encoding types from `announce.rs` → `announce/encoding.rs`
4. Move deserialization types from `announce_deserialization.rs` → `announce/deserialization.rs`
5. Create `announce/mod.rs` with re-exports for backward compatibility
6. Delete old `announce.rs` and `announce_deserialization.rs`
7. Update imports across the workspace
8. Repeat for scrape types

## Decision Pending

- [ ] Restructure into `announce/{data,encoding,deserialization}.rs` + partial merge (recommended)
- [ ] Full merge: unify everything including encoding layer (not recommended — incompatible serialization strategies)
- [ ] Keep separate: fix naming collision, add IPv6 support, align types
- [ ] Leave as-is: no changes to response types
