//! Primitive types for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the basic data structures for the [Torrust Tracker](https://docs.rs/torrust-tracker),
//! which is a `BitTorrent` tracker server. These structures are used not only
//! by the tracker server crate, but also by other crates in the Torrust
//! ecosystem.
pub mod announce;
pub mod announce_event;
pub mod core;
pub mod number_of_bytes;
pub mod pagination;
pub mod peer;
pub mod peer_id;
pub mod scrape;
pub mod service_binding;
pub mod swarm_metadata;

use std::collections::BTreeMap;
use std::time::Duration;

use bittorrent_primitives::info_hash::InfoHash;

/// Duration since the Unix Epoch.
pub type DurationSinceUnixEpoch = Duration;

pub use announce::AnnounceData;
pub use announce_event::AnnounceEvent;
pub use number_of_bytes::NumberOfBytes;
pub use peer_id::{PeerClient, PeerId};

pub type NumberOfDownloads = u32;
pub type NumberOfDownloadsBTreeMap = BTreeMap<InfoHash, NumberOfDownloads>;
