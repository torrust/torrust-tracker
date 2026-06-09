//! Primitive types for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the basic data structures for the [Torrust Tracker](https://docs.rs/torrust-tracker),
//! which is a `BitTorrent` tracker server. These structures are used not only
//! by the tracker server crate, but also by other crates in the Torrust
//! ecosystem.
pub mod announce;
pub mod mode;
pub mod number_of_bytes;
pub mod pagination;
pub mod peer;
#[deprecated(
    since = "3.0.0-develop",
    note = "import peer ID types from `torrust_peer_id` crate instead; \
            this module will be removed in a future release (see EPIC #1669)"
)]
pub mod peer_id;
pub mod policy;
pub mod scrape;
pub mod swarm_metadata;

use std::collections::BTreeMap;

pub use announce::{AnnounceData, AnnounceEvent, AnnouncePolicy};
pub use mode::PrivateMode;
pub use number_of_bytes::NumberOfBytes;
pub use policy::TrackerPolicy;
pub use scrape::ScrapeData;
/// Duration since the Unix Epoch.
///
/// **Deprecated**: import from [`torrust_clock::DurationSinceUnixEpoch`] instead.
/// This re-export is kept for backwards compatibility and will be removed in a
/// future release. Removal is tracked as a follow-up cleanup subissue of EPIC
/// [#1669](https://github.com/torrust/torrust-tracker/issues/1669).
#[deprecated(
    since = "3.0.0-develop",
    note = "import `DurationSinceUnixEpoch` from `torrust_clock` instead; \
            this re-export will be removed in a future release (see EPIC #1669)"
)]
pub use torrust_clock::DurationSinceUnixEpoch;
use torrust_info_hash::InfoHash;
/// **Deprecated**: import from [`torrust_peer_id`] instead via the [`peer_id`] module.
/// This re-export is kept for backwards compatibility and will be removed in a
/// future release. Removal is tracked as a follow-up cleanup subissue of EPIC
/// [#1669](https://github.com/torrust/torrust-tracker/issues/1669).
#[deprecated(
    since = "3.0.0-develop",
    note = "import peer ID types from `torrust_peer_id` crate instead; \
            this re-export will be removed in a future release (see EPIC #1669)"
)]
pub use torrust_peer_id::{PeerClient, PeerId};

/// Network service binding types.
///
/// **Deprecated**: import from [`torrust_net_primitives::service_binding`] instead.
/// This re-export is kept for backwards compatibility and will be removed in a
/// future release. Removal is tracked as a follow-up cleanup subissue of EPIC
/// [#1669](https://github.com/torrust/torrust-tracker/issues/1669).
#[deprecated(
    since = "3.0.0-develop",
    note = "import `service_binding` types from `torrust_net_primitives` instead; \
            this re-export will be removed in a future release (see EPIC #1669)"
)]
pub mod service_binding {
    pub use torrust_net_primitives::service_binding::*;
}

pub type NumberOfDownloads = u32;
pub type NumberOfDownloadsBTreeMap = BTreeMap<InfoHash, NumberOfDownloads>;
