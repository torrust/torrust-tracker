//! Peer ID parsing and client identification for `BitTorrent` crates.

#![allow(clippy::module_name_repetitions)]

mod peer_client;
mod peer_id;

pub use self::peer_client::PeerClient;
pub use self::peer_id::PeerId;
