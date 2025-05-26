use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::DurationSinceUnixEpoch;
use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;

/// # Panics
///
/// Will panic if the temporary file path is not a valid UTF-8 string.
#[must_use]
pub fn ephemeral_configuration() -> Core {
    let mut config = Core::default();

    let temp_file = ephemeral_sqlite_database();
    temp_file.to_str().unwrap().clone_into(&mut config.database.path);

    config
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
