use std::net::IpAddr;
use std::sync::Arc;

use crate::protocol::info_hash::InfoHash;
use crate::tracker::peer::Peer;
use crate::tracker::{statistics, AnnounceData, Tracker};

pub async fn invoke(tracker: Arc<Tracker>, info_hash: InfoHash, peer: &mut Peer) -> AnnounceData {
    let original_peer_ip = peer.peer_addr.ip();

    // The tracker could change the original peer ip
    let announce_data = tracker.announce(&info_hash, peer, &original_peer_ip).await;

    match original_peer_ip {
        IpAddr::V4(_) => {
            tracker.send_stats_event(statistics::Event::Tcp4Announce).await;
        }
        IpAddr::V6(_) => {
            tracker.send_stats_event(statistics::Event::Tcp6Announce).await;
        }
    }

    announce_data
}
