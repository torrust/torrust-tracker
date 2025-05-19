use torrust_tracker_primitives::peer::fixture::PeerBuilder;
use torrust_tracker_primitives::peer::{self};

/// A torrent seeder is a peer with 0 bytes left to download which
/// has not announced it has stopped
#[must_use]
pub fn a_completed_peer(id: i32) -> peer::Peer {
    let peer_id = peer::Id::new(id);
    PeerBuilder::default()
        .with_bytes_left_to_download(0)
        .with_event_completed()
        .with_peer_id(&peer_id)
        .into()
}

/// A torrent leecher is a peer that is not a seeder.
/// Leecher: left > 0 OR event = Stopped
#[must_use]
pub fn a_started_peer(id: i32) -> peer::Peer {
    let peer_id = peer::Id::new(id);
    PeerBuilder::default()
        .with_bytes_left_to_download(1)
        .with_event_started()
        .with_peer_id(&peer_id)
        .into()
}
