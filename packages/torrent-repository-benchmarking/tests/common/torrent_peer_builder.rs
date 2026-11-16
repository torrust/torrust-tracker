use torrust_tracker_primitives::peer::fixture::{PeerBuilder, qbittorrent_peer_id};
use torrust_tracker_primitives::peer::{self};

/// The number of a peer that no test scenario makes: the scenarios number their peers from 0 to
/// 407, so this peer is always a new one.
pub const OTHER_PEER: u32 = 1000;

/// A torrent seeder is a peer with 0 bytes left to download which
/// has not announced it has stopped
#[must_use]
pub fn a_completed_peer(number: u32) -> peer::Peer {
    let peer_id = qbittorrent_peer_id(number);
    PeerBuilder::default()
        .with_bytes_left_to_download(0)
        .with_event_completed()
        .with_peer_id(&peer_id)
        .into()
}

/// A torrent leecher is a peer that is not a seeder.
/// Leecher: left > 0 OR event = Stopped
#[must_use]
pub fn a_started_peer(number: u32) -> peer::Peer {
    let peer_id = qbittorrent_peer_id(number);
    PeerBuilder::default()
        .with_bytes_left_to_download(1)
        .with_event_started()
        .with_peer_id(&peer_id)
        .into()
}
