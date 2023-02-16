use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use log::debug;

use crate::http::axum_implementation::extractors::peer_ip::peer_ip;
use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;
use crate::http::axum_implementation::requests::announce::{Announce, Event, ExtractAnnounceRequest};
use crate::http::axum_implementation::responses;
use crate::protocol::clock::{Current, Time};
use crate::tracker::peer::Peer;
use crate::tracker::{statistics, Tracker};

#[allow(clippy::unused_async)]
pub async fn handle(
    State(tracker): State<Arc<Tracker>>,
    ExtractAnnounceRequest(announce_request): ExtractAnnounceRequest,
    remote_client_ip: RemoteClientIp,
) -> Response {
    debug!("http announce request: {:#?}", announce_request);

    let info_hash = announce_request.info_hash;

    let peer_ip = peer_ip(tracker.config.on_reverse_proxy, &remote_client_ip);

    let peer_ip = match peer_ip {
        Ok(peer_ip) => peer_ip,
        Err(err) => return err,
    };

    let mut peer = peer_from_request(&announce_request, &peer_ip);

    let response = tracker.announce(&info_hash, &mut peer, &peer_ip).await;

    match peer_ip {
        IpAddr::V4(_) => {
            tracker.send_stats_event(statistics::Event::Tcp4Announce).await;
        }
        IpAddr::V6(_) => {
            tracker.send_stats_event(statistics::Event::Tcp6Announce).await;
        }
    }

    responses::announce::Announce::from(response).into_response()
}

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
