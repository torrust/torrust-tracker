use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
use torrust_tracker_clock::clock::Time;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};

use crate::CurrentClock;

#[derive(Debug, Default)]
struct TorrentPeerBuilder {
    peer: peer::Peer,
}

#[allow(dead_code)]
impl TorrentPeerBuilder {
    #[must_use]
    fn new() -> Self {
        Self {
            peer: peer::Peer {
                updated: CurrentClock::now(),
                ..Default::default()
            },
        }
    }

    #[must_use]
    fn with_event_completed(mut self) -> Self {
        self.peer.event = AnnounceEvent::Completed;
        self
    }

    #[must_use]
    fn with_event_started(mut self) -> Self {
        self.peer.event = AnnounceEvent::Started;
        self
    }

    #[must_use]
    fn with_peer_address(mut self, peer_addr: SocketAddr) -> Self {
        self.peer.peer_addr = peer_addr;
        self
    }

    #[must_use]
    fn with_peer_id(mut self, peer_id: PeerId) -> Self {
        self.peer.peer_id = peer_id;
        self
    }

    #[must_use]
    fn with_number_of_bytes_left(mut self, left: i64) -> Self {
        self.peer.left = NumberOfBytes::new(left);
        self
    }

    #[must_use]
    fn updated_at(mut self, updated: DurationSinceUnixEpoch) -> Self {
        self.peer.updated = updated;
        self
    }

    #[must_use]
    fn into(self) -> peer::Peer {
        self.peer
    }
}

/// A torrent seeder is a peer with 0 bytes left to download which
/// has not announced it has stopped
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn a_completed_peer(id: i32) -> peer::Peer {
    let peer_id = peer::Id::new(id);
    let peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), id as u16);

    TorrentPeerBuilder::new()
        .with_number_of_bytes_left(0)
        .with_event_completed()
        .with_peer_id(*peer_id)
        .with_peer_address(peer_addr)
        .into()
}

/// A torrent leecher is a peer that is not a seeder.
/// Leecher: left > 0 OR event = Stopped
///
/// # Panics
///
/// This function panics if proved id can't be converted into a valid socket address port.
///
/// The `id` argument is used to identify the peer in both the `peer_id` and the `peer_addr`.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn a_started_peer(id: i32) -> peer::Peer {
    let peer_id = peer::Id::new(id);
    let peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), id as u16);

    TorrentPeerBuilder::new()
        .with_number_of_bytes_left(1)
        .with_event_started()
        .with_peer_id(*peer_id)
        .with_peer_address(peer_addr)
        .into()
}
