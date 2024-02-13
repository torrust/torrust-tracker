//! Logic to extract the peer info from the announce request.
use std::net::{IpAddr, SocketAddr};

use torrust_tracker_primitives::announce_event::AnnounceEvent;
use torrust_tracker_primitives::{peer, NumberOfBytes};

use super::request::AnnounceWrapper;
use crate::shared::clock::{Current, Time};

/// Extracts the [`peer::Peer`] info from the
/// announce request.
///
/// # Arguments
///
/// * `announce_wrapper` - The announce request to extract the peer info from.
/// * `peer_ip` - The real IP address of the peer, not the one in the announce
/// request.
#[must_use]
pub fn from_request(announce_wrapper: &AnnounceWrapper, peer_ip: &IpAddr) -> peer::Peer {
    peer::Peer {
        peer_id: peer::Id(announce_wrapper.announce_request.peer_id.0),
        peer_addr: SocketAddr::new(*peer_ip, announce_wrapper.announce_request.port.0),
        updated: Current::now(),
        uploaded: NumberOfBytes(announce_wrapper.announce_request.bytes_uploaded.0),
        downloaded: NumberOfBytes(announce_wrapper.announce_request.bytes_downloaded.0),
        left: NumberOfBytes(announce_wrapper.announce_request.bytes_left.0),
        event: AnnounceEvent::from_i32(announce_wrapper.announce_request.event.to_i32()),
    }
}
