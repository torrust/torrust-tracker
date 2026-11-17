use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use tempfile::TempDir;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_info_hash::InfoHash;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId};

/// A core configuration whose SQLite database lives in a temporary directory owned by this value.
///
/// Dropping the value removes the directory and the database in it, also when the test fails by panicking, so a
/// test run leaves no database file behind. Keep the value alive for as long as the configuration is used: once it
/// is dropped, the database path points into a removed directory and opening it fails.
pub struct EphemeralConfiguration {
    pub config: Core,
    _database_directory: TempDir,
}

/// # Panics
///
/// Will panic if the temporary directory cannot be created or its path is not valid UTF-8.
#[must_use]
pub fn ephemeral_configuration() -> EphemeralConfiguration {
    let database_directory = tempfile::tempdir().expect("a temporary directory can be created");

    let mut config = Core::default();
    let database = config.database.get_or_insert_with(Database::default);
    let Database::Sqlite3 { path } = database else {
        unreachable!("default core configuration uses SQLite persistence");
    };
    database_directory
        .path()
        .join("data.db")
        .to_str()
        .expect("the temporary database path is valid UTF-8")
        .clone_into(path);

    EphemeralConfiguration {
        config,
        _database_directory: database_directory,
    }
}

/// # Panics
///
/// Will panic if the string representation of the info hash is not a valid infohash.
#[must_use]
pub fn sample_info_hash() -> InfoHash {
    "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0" // DevSkim: ignore DS173237
        .parse::<InfoHash>()
        .expect("String should be a valid info hash")
}

/// Sample peer whose state is not relevant for the tests.
#[must_use]
pub fn sample_peer() -> Peer {
    Peer {
        peer_id: PeerId(*b"-qB00000000000000000"),
        peer_addr: SocketAddr::new(remote_client_ip(), 8080),
        updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
        uploaded: NumberOfBytes::new(0),
        downloaded: NumberOfBytes::new(0),
        left: NumberOfBytes::new(0), // No bytes left to download
        event: AnnounceEvent::Completed,
    }
}

// The client peer IP.
#[must_use]
pub fn remote_client_ip() -> IpAddr {
    IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap())
}
