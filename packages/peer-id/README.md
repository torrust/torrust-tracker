# bittorrent-peer-id

In-house crate for BitTorrent `PeerId` parsing and `PeerClient` identification.

## Origin and In-House Maintenance

This crate was originally derived from Aquatic's `peer_id` crate:

- https://github.com/greatest-ape/aquatic/tree/master/crates/peer_id

This crate is extracted from previously duplicated in-house implementations in:

- `packages/primitives/src/peer_id.rs`
- `packages/udp-protocol/src/peer_id.rs`

It provides a shared implementation that can be consumed by both domain and protocol crates
without introducing inverted dependency directions.

Torrust keeps this package in-house because upstream maintenance appears inactive and the tracker
still needs dependency updates, security maintenance, and ongoing evolution.

Relevant upstream context:

- https://github.com/greatest-ape/aquatic/issues/224
- https://github.com/greatest-ape/aquatic/pull/235

## Licensing and Notices

The original source is Apache-2.0 licensed. The in-house package keeps the required origin and
change notices in code headers, consistent with the license terms.

An explicit copy of Apache-2.0 is included at [LICENSE-APACHE](./LICENSE-APACHE).

## Acknowledgment

Special thanks to [greatest-ape](https://github.com/greatest-ape)
(Joakim Frostegård) for his contributions to the BitTorrent ecosystem and the original
implementation this crate builds upon.
