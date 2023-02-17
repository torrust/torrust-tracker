use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use log::debug;

use crate::http::axum_implementation::extractors::announce_request::ExtractRequest;
use crate::http::axum_implementation::extractors::peer_ip::assign_ip_address_to_peer;
use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::requests::announce::{Announce, Compact, Event};
use crate::http::axum_implementation::responses::announce;
use crate::http::axum_implementation::services;
use crate::protocol::clock::{Current, Time};
use crate::tracker::peer::Peer;
use crate::tracker::Tracker;

#[allow(clippy::unused_async)]
pub async fn handle(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(announce_request): ExtractRequest,
    remote_client_ip: RemoteClientIp,
) -> Response {
    debug!("http announce request: {:#?}", announce_request);

    let peer_ip = match assign_ip_address_to_peer(tracker.config.on_reverse_proxy, &remote_client_ip) {
        Ok(peer_ip) => peer_ip,
        Err(err) => return err,
    };

    let mut peer = peer_from_request(&announce_request, &peer_ip);

    let announce_data = services::announce::invoke(tracker.clone(), announce_request.info_hash, &mut peer).await;

    match announce_request.compact {
        Some(compact) => match compact {
            Compact::Accepted => announce::Compact::from(announce_data).into_response(),
            Compact::NotAccepted => announce::NonCompact::from(announce_data).into_response(),
        },
        // Default response format non compact
        None => announce::NonCompact::from(announce_data).into_response(),
    }
}

/// It ignores the peer address in the announce request params.
#[must_use]
fn peer_from_request(announce_request: &Announce, peer_ip: &IpAddr) -> Peer {
    Peer {
        peer_id: announce_request.peer_id,
        peer_addr: SocketAddr::new(*peer_ip, announce_request.port),
        updated: Current::now(),
        uploaded: NumberOfBytes(announce_request.uploaded.unwrap_or(0)),
        downloaded: NumberOfBytes(announce_request.downloaded.unwrap_or(0)),
        left: NumberOfBytes(announce_request.left.unwrap_or(0)),
        event: map_to_aquatic_event(&announce_request.event),
    }
}

fn map_to_aquatic_event(event: &Option<Event>) -> AnnounceEvent {
    match event {
        Some(event) => match &event {
            Event::Started => aquatic_udp_protocol::AnnounceEvent::Started,
            Event::Stopped => aquatic_udp_protocol::AnnounceEvent::Stopped,
            Event::Completed => aquatic_udp_protocol::AnnounceEvent::Completed,
        },
        None => aquatic_udp_protocol::AnnounceEvent::None,
    }
}
