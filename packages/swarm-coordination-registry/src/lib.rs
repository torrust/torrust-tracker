pub mod container;
pub mod event;
pub mod statistics;
pub mod swarm;

use std::sync::Arc;

use tokio::sync::Mutex;
use torrust_tracker_clock::clock;

pub type Registry = swarm::registry::Registry;
pub type CoordinatorHandle = Arc<Mutex<Coordinator>>;
pub type Coordinator = swarm::coordinator::Coordinator;

/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;

pub const SWARM_COORDINATION_REGISTRY_LOG_TARGET: &str = "SWARM_COORDINATION_REGISTRY";

#[cfg(test)]
pub(crate) mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
    use bittorrent_primitives::info_hash::InfoHash;
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    /// # Panics
    ///
    /// Will panic if the string representation of the info hash is not a valid info hash.
    #[must_use]
    pub fn sample_info_hash() -> InfoHash {
        "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0" // DevSkim: ignore DS173237
            .parse::<InfoHash>()
            .expect("String should be a valid info hash")
    }

    /// # Panics
    ///
    /// Will panic if the string representation of the info hash is not a valid info hash.
    #[must_use]
    pub fn sample_info_hash_one() -> InfoHash {
        "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0" // DevSkim: ignore DS173237
            .parse::<InfoHash>()
            .expect("String should be a valid info hash")
    }

    /// # Panics
    ///
    /// Will panic if the string representation of the info hash is not a valid info hash.
    #[must_use]
    pub fn sample_info_hash_alphabetically_ordered_after_sample_info_hash_one() -> InfoHash {
        "99c82bb73505a3c0b453f9fa0e881d6e5a32a0c1" // DevSkim: ignore DS173237
            .parse::<InfoHash>()
            .expect("String should be a valid info hash")
    }

    /// Sample peer whose state is not relevant for the tests.
    #[must_use]
    pub fn sample_peer() -> Peer {
        Peer {
            peer_id: PeerId(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(0), // No bytes left to download
            event: AnnounceEvent::Completed,
        }
    }

    #[must_use]
    pub fn sample_peer_one() -> Peer {
        Peer {
            peer_id: PeerId(*b"-qB00000000000000001"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8081),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(0), // No bytes left to download
            event: AnnounceEvent::Completed,
        }
    }

    #[must_use]
    pub fn sample_peer_two() -> Peer {
        Peer {
            peer_id: PeerId(*b"-qB00000000000000002"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 2)), 8082),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(0), // No bytes left to download
            event: AnnounceEvent::Completed,
        }
    }

    #[must_use]
    pub fn seeder() -> Peer {
        complete_peer()
    }

    #[must_use]
    pub fn leecher() -> Peer {
        incomplete_peer()
    }

    /// A peer that counts as `complete` is swarm metadata
    /// IMPORTANT!: it only counts if the it has been announce at least once before
    /// announcing the `AnnounceEvent::Completed` event.
    #[must_use]
    pub fn complete_peer() -> Peer {
        Peer {
            peer_id: PeerId(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(0), // No bytes left to download
            event: AnnounceEvent::Completed,
        }
    }

    /// A peer that counts as `incomplete` is swarm metadata
    #[must_use]
    pub fn incomplete_peer() -> Peer {
        Peer {
            peer_id: PeerId(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes::new(0),
            downloaded: NumberOfBytes::new(0),
            left: NumberOfBytes::new(1000), // Still bytes to download
            event: AnnounceEvent::Started,
        }
    }
}
