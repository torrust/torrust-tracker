//! Primitive types for [Torrust Tracker](https://docs.rs/torrust-tracker).
//!
//! This module contains the basic data structures for the [Torrust Tracker](https://docs.rs/torrust-tracker),
//! which is a `BitTorrent` tracker server. These structures are used not only
//! by the tracker server crate, but also by other crates in the Torrust
//! ecosystem.
use serde::{Deserialize, Serialize};

/// The database management system used by the tracker.
///
/// Refer to:
///
/// - [Torrust Tracker Configuration](https://docs.rs/torrust-tracker-configuration).
/// - [Torrust Tracker](https://docs.rs/torrust-tracker).
///
/// For more information about persistence.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, derive_more::Display, Clone)]
pub enum DatabaseDriver {
    // TODO: Move to the database crate once that gets its own crate.
    /// The Sqlite3 database driver.
    Sqlite3,
    /// The `MySQL` database driver.
    MySQL,
}

/// The mode the tracker will run in.
///
/// Refer to [Torrust Tracker Configuration](https://docs.rs/torrust-tracker-configuration)
/// to know how to configure the tracker to run in each mode.
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum TrackerMode {
    /// Will track every new info hash and serve every peer.
    #[serde(rename = "public")]
    Public,

    /// Will only track whitelisted info hashes.
    #[serde(rename = "listed")]
    Listed,

    /// Will only serve authenticated peers
    #[serde(rename = "private")]
    Private,

    /// Will only track whitelisted info hashes and serve authenticated peers
    #[serde(rename = "private_listed")]
    PrivateListed,
}
