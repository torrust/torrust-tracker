//! Logic to extract the peer info from the announce request.
use std::net::{IpAddr, SocketAddr};

use torrust_clock::clock::Time;
use torrust_tracker_primitives::peer;

use crate::CurrentClock;

/// Extracts the [`peer::Peer`] info from the
/// announce request.
///
/// # Arguments
///
/// * `peer_ip` - The real IP address of the peer, not the one in the announce request.
#[must_use]
pub fn from_request(announce_request: &torrust_tracker_udp_protocol::AnnounceRequest, peer_ip: &IpAddr) -> peer::Peer {
    let wire_event = torrust_tracker_udp_protocol::AnnounceEvent::from(announce_request.event);

    peer::Peer {
        peer_id: torrust_tracker_primitives::PeerId(announce_request.peer_id.0),
        peer_addr: SocketAddr::new(*peer_ip, announce_request.port.0.into()).into(),
        updated: CurrentClock::now(),
        uploaded: torrust_tracker_primitives::NumberOfBytes::new(announce_request.bytes_uploaded.0.get()),
        downloaded: torrust_tracker_primitives::NumberOfBytes::new(announce_request.bytes_downloaded.0.get()),
        left: torrust_tracker_primitives::NumberOfBytes::new(announce_request.bytes_left.0.get()),
        event: match wire_event {
            torrust_tracker_udp_protocol::AnnounceEvent::Completed => torrust_tracker_primitives::AnnounceEvent::Completed,
            torrust_tracker_udp_protocol::AnnounceEvent::Started => torrust_tracker_primitives::AnnounceEvent::Started,
            torrust_tracker_udp_protocol::AnnounceEvent::Stopped => torrust_tracker_primitives::AnnounceEvent::Stopped,
            torrust_tracker_udp_protocol::AnnounceEvent::None => torrust_tracker_primitives::AnnounceEvent::None,
        },
    }
}
