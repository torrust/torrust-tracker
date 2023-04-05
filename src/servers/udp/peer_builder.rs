//! Logic to extract the peer info from the announce request.
use std::net::{IpAddr, SocketAddr};

use super::request::AnnounceWrapper;
use crate::shared::clock::{Current, Time};
use crate::tracker::peer::{Id, Peer};

/// Extracts the [`Peer`](crate::tracker::peer::Peer) info from the
/// announce request.
///
/// # Arguments
///
/// * `announce_wrapper` - The announce request to extract the peer info from.
/// * `peer_ip` - The real IP address of the peer, not the one in the announce
/// request.
#[must_use]
pub fn from_request(announce_wrapper: &AnnounceWrapper, peer_ip: &IpAddr) -> Peer {
    Peer {
        peer_id: Id(announce_wrapper.announce_request.peer_id.0),
        peer_addr: SocketAddr::new(*peer_ip, announce_wrapper.announce_request.port.0),
        updated: Current::now(),
        uploaded: announce_wrapper.announce_request.bytes_uploaded,
        downloaded: announce_wrapper.announce_request.bytes_downloaded,
        left: announce_wrapper.announce_request.bytes_left,
        event: announce_wrapper.announce_request.event,
    }
}
