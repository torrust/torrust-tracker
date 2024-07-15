//! Primitive types for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the basic data structures for the [Torrust Tracker](https://docs.rs/torrust-tracker),
//! which is a `BitTorrent` tracker server. These structures are used not only
//! by the tracker server crate, but also by other crates in the Torrust
//! ecosystem.
use std::collections::BTreeMap;
use std::time::Duration;

use info_hash::InfoHash;

pub mod info_hash;
pub mod pagination;
pub mod peer;
pub mod swarm_metadata;
pub mod torrent_metrics;

/// Duration since the Unix Epoch.
pub type DurationSinceUnixEpoch = Duration;

/// IP version used by the peer to connect to the tracker: IPv4 or IPv6
#[derive(PartialEq, Eq, Debug)]
pub enum IPVersion {
    /// <https://en.wikipedia.org/wiki/Internet_Protocol_version_4>
    IPv4,
    /// <https://en.wikipedia.org/wiki/IPv6>
    IPv6,
}

pub type PersistentTorrents = BTreeMap<InfoHash, u32>;
