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

pub type PersistentTorrents = BTreeMap<InfoHash, u32>;
