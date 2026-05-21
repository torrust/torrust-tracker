---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/1732-replace-aquatic-udp-protocol/ISSUE.md
    - packages/primitives/
    - packages/udp-protocol/
---

# Step 3: `bittorrent-primitives` Transitive Dependency Problem

## Problem

During Step 3 (zerocopy 0.8 migration), `cargo check --workspace` fails with:

```text
error[E0599]: no associated function or constant named `read_from` found for struct
`aquatic_udp_protocol::InfoHash` in the current scope
  --> /home/josecelano/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/
       bittorrent-primitives-0.1.0/src/info_hash.rs:155:52
note: there are multiple different versions of crate `zerocopy` in the dependency graph
```

The root cause is that the crates.io package `bittorrent-primitives 0.1.0` depends on
`aquatic_udp_protocol = "0.9.0"` and calls the zerocopy 0.7 API (`read_from`) on
`aquatic_udp_protocol::InfoHash`. After our `[patch.crates-io]` entry substitutes our internal
fork (zerocopy 0.8) for `aquatic_udp_protocol`, that call becomes invalid.

```toml
# bittorrent-primitives 0.1.0 (crates.io) — relevant deps
[dependencies]
aquatic_udp_protocol = "0.9.0"
zerocopy = { version = "0.7", features = ["derive"] }
```

```rust
// bittorrent-primitives 0.1.0 — src/info_hash.rs, line 155
pub fn from_bytes(bytes: &[u8]) -> Self {
    let data = aquatic_udp_protocol::InfoHash::read_from(bytes)  // ← zerocopy 0.7 API
        .expect("it should have the exact amount of bytes");
    Self { data }
}
```

In zerocopy 0.8, `read_from` was renamed to `read_from_bytes` and its return type changed from
`Option<T>` to `Result<T, SizeError>`. The `expect` call must also be updated accordingly.

## Scope

11 workspace packages depend on `bittorrent-primitives`:

| Package                                           | Published on crates.io |
| ------------------------------------------------- | ---------------------- |
| `torrust-tracker-axum-http-server`                | No                     |
| `torrust-tracker-axum-rest-api-server`            | No                     |
| `bittorrent-http-tracker-protocol`                | No                     |
| `bittorrent-http-tracker-core`                    | No                     |
| `torrust-tracker-primitives`                      | **Yes**                |
| `torrust-tracker-swarm-coordination-registry`     | No                     |
| `torrust-tracker-torrent-repository-benchmarking` | No                     |
| `bittorrent-tracker-client`                       | No                     |
| `bittorrent-tracker-core`                         | No                     |
| `bittorrent-udp-tracker-core`                     | No                     |
| `torrust-tracker-udp-server`                      | No                     |

Also, the root workspace crate (`torrust-tracker`) has `bittorrent-primitives = "0.1.0"` in
its `[dev-dependencies]`.

Of these, only `torrust-tracker-primitives` is already published on crates.io. All others are
unpublished workspace packages with no backward-compatibility constraints on crates.io.

## Relationship Between the Crates

```text
bittorrent-primitives (crates.io 0.1.0)
  └── aquatic_udp_protocol = "0.9.0"   ← patched by our workspace to the internal fork
        └── zerocopy = "0.8"           ← our fork uses 0.8
  └── zerocopy = "0.7"                 ← crates.io version still calls 0.7 API
```

The workspace `[patch.crates-io]` already replaces `aquatic_udp_protocol` with our fork, but
the patched `bittorrent-primitives` source code itself still uses the zerocopy 0.7 call. Cargo's
patch mechanism substitutes the library, but cannot rewrite the call sites in the dependent
crate's source.

## Solution

Create an internal fork of `bittorrent-primitives` at `packages/bittorrent-primitives/`, apply
the two required changes, and add it to `[patch.crates-io]`:

### Changes required in the fork

1. **`Cargo.toml`**: Change `aquatic_udp_protocol = "0.9.0"` to
   `aquatic_udp_protocol = { path = "../aquatic-udp-protocol" }` and bump
   `zerocopy` from `"0.7"` to `"0.8"`.

2. **`src/info_hash.rs`**: Update `from_bytes` to use the zerocopy 0.8 API:

   ```rust
   // Before (zerocopy 0.7)
   use zerocopy::FromBytes;
   // ...
   let data = aquatic_udp_protocol::InfoHash::read_from(bytes)
       .expect("it should have the exact amount of bytes");

   // After (zerocopy 0.8)
   use zerocopy::FromBytes as _;
   // ...
   let data = aquatic_udp_protocol::InfoHash::read_from_bytes(bytes)
       .expect("it should have the exact amount of bytes");
   ```

### Root Cargo.toml changes

Add to `[workspace.members]`:

```toml
"packages/bittorrent-primitives",
```

Add to `[patch.crates-io]`:

```toml
bittorrent-primitives = { path = "packages/bittorrent-primitives" }
```

The existing `bittorrent-primitives = "0.1.0"` entry in `[workspace.dependencies]` stays
unchanged; the patch transparently replaces the resolved crate for all workspace members.

### Publishing considerations

The fork is marked `publish = false` because it is a temporary internal patch — not a version
intended for crates.io. When Step 4 is complete and all direct uses of
`aquatic_udp_protocol::InfoHash` are replaced by the type from `packages/udp-protocol`, the
`bittorrent-primitives` fork will need to be updated again (or, if `bittorrent-primitives` is
kept long-term as a published crate, a new version should be released that depends on the
published `bittorrent-udp-tracker-protocol` crate instead of `aquatic_udp_protocol`).

## Future Work

### Update `bittorrent-primitives` dependency after Step 4c

Once Step 4c consolidates `InfoHash` directly into `bittorrent-primitives`, the crate will no
longer depend on `aquatic_udp_protocol` at all. At that point a new version of
`bittorrent-primitives` can be published to crates.io (bumping from `0.1.0`) with the
self-contained implementation. The workspace `[patch.crates-io]` entry for
`bittorrent-primitives` and the fork in `packages/bittorrent-primitives/` can then both be
removed.

### Consolidate `InfoHash` into `bittorrent-primitives` (Step 4c)

The `bittorrent-primitives` crate currently wraps `aquatic_udp_protocol::InfoHash` inside its
own `InfoHash` newtype:

```rust
// packages/bittorrent-primitives/src/info_hash.rs
pub struct InfoHash {
    data: aquatic_udp_protocol::InfoHash,
}
```

Once Step 4a migrates the `aquatic_udp_protocol::InfoHash` bytes type into
`packages/udp-protocol` (as `bittorrent-udp-tracker-protocol`), the natural next move is to
eliminate the wrapping layer entirely: the raw `[u8; 20]` storage — and all the serialization,
formatting, and conversion logic — should live directly inside `bittorrent-primitives` with no
dependency on any UDP protocol crate at all.

This would give `bittorrent-primitives` a fully self-contained `InfoHash` type that any
BitTorrent project can use without pulling in UDP protocol machinery.

This is tracked as **Step 4c** in the issue spec.

### Re-evaluate the boundary between `bittorrent-primitives` and `torrust-tracker-primitives`

The current separation is ad-hoc:

- `bittorrent-primitives` (external crate) — originally scoped to bare BitTorrent types
  (`InfoHash`). Despite its name it currently lives in a separate repository and is published
  independently.
- `torrust-tracker-primitives` (`packages/primitives`) — a tracker-scoped library that already
  contains peer-related logic (`src/peer.rs`: `Peer`, `PeerId` usage, `PeerRole`, `PeerAnnouncement`,
  `PeerClient`), plus tracker-domain types (`DurationSinceUnixEpoch`, stats, etc.).

A cleaner long-term split would be:

| Crate                        | Should contain                                                                                                                                                                    |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bittorrent-primitives`      | Types reusable across **any** BitTorrent application or protocol: `InfoHash`, `PeerId`, `PeerClient`, announce/scrape value objects (`AnnounceEvent`, `NumberOfBytes`, `Port`, …) |
| `torrust-tracker-primitives` | Types **specific** to the Torrust Tracker domain: `Peer`, `PeerRole`, `PeerAnnouncement`, tracker stats, `DurationSinceUnixEpoch`, etc.                                           |

Concretely this means `packages/primitives/src/peer.rs` — and the peer-related logic that
currently re-exports or wraps `aquatic_udp_protocol::PeerId` — should eventually move into
`bittorrent-primitives`. This would make `InfoHash` and peer identity types available to any
BitTorrent project, not just the Torrust Tracker.

This boundary review is **out of scope for the current issue** (issue 1732 is focused on
removing `aquatic_udp_protocol`). It should be tracked as a separate issue once Step 4 is
complete and the peer/protocol types have settled into their new homes.
