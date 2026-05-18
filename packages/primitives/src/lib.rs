//! Primitive types for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the basic data structures for the [Torrust Tracker](https://docs.rs/torrust-tracker),
//! which is a `BitTorrent` tracker server. These structures are used not only
//! by the tracker server crate, but also by other crates in the Torrust
//! ecosystem.
pub mod announce;
pub mod number_of_bytes;
pub mod pagination;
pub mod peer;
pub mod peer_id;
pub mod scrape;
pub mod service_binding;
pub mod swarm_metadata;

use std::collections::BTreeMap;

pub use announce::{AnnounceData, AnnounceEvent};
use bittorrent_primitives::info_hash::InfoHash;
pub use number_of_bytes::NumberOfBytes;
pub use peer_id::{PeerClient, PeerId};
pub use scrape::ScrapeData;
/// Duration since the Unix Epoch.
///
/// **Deprecated**: import from [`torrust_tracker_clock::DurationSinceUnixEpoch`] instead.
/// This re-export is kept for backwards compatibility and will be removed in a
/// future release. Removal is tracked in issue
/// [#1790](https://github.com/torrust/torrust-tracker/issues/1790).
#[deprecated(
    since = "3.0.0-develop",
    note = "import `DurationSinceUnixEpoch` from `torrust_tracker_clock` instead; \
            this re-export will be removed in a future release (see #1790)"
)]
pub use torrust_tracker_clock::DurationSinceUnixEpoch;

pub type NumberOfDownloads = u32;
pub type NumberOfDownloadsBTreeMap = BTreeMap<InfoHash, NumberOfDownloads>;
