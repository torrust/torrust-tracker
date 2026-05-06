# bittorrent-peer-id

In-house crate for BitTorrent `PeerId` parsing and `PeerClient` identification.

This crate is extracted from previously duplicated in-house implementations in:

- `packages/primitives/src/peer_id.rs`
- `packages/udp-protocol/src/peer_id.rs`

It provides a shared implementation that can be consumed by both domain and protocol crates
without introducing inverted dependency directions.
