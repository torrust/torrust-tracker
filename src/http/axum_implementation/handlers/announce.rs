use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum_client_ip::SecureClientIp;

use crate::http::axum_implementation::requests::announce::{Announce, ExtractAnnounceRequest};
use crate::http::axum_implementation::responses;
use crate::protocol::clock::{Current, Time};
use crate::tracker::peer::Peer;
use crate::tracker::{statistics, Tracker};

/// WIP
#[allow(clippy::unused_async)]
pub async fn handle(
    State(tracker): State<Arc<Tracker>>,
    ExtractAnnounceRequest(announce_request): ExtractAnnounceRequest,
    secure_ip: SecureClientIp,
) -> Response {
    // todo: compact response and optional params

    let info_hash = announce_request.info_hash;
    let remote_client_ip = secure_ip.0;

    let mut peer = peer_from_request(&announce_request, &remote_client_ip);

    let response = tracker.announce(&info_hash, &mut peer, &remote_client_ip).await;

    match remote_client_ip {
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
    #[allow(clippy::cast_possible_truncation)]
    Peer {
        peer_id: announce_request.peer_id,
        peer_addr: SocketAddr::new(*peer_ip, announce_request.port),
        updated: Current::now(),
        // todo: optional parameters not included in the announce request yet
        uploaded: NumberOfBytes(i128::from(0) as i64),
        downloaded: NumberOfBytes(i128::from(0) as i64),
        left: NumberOfBytes(i128::from(0) as i64),
        event: AnnounceEvent::None,
    }
}
