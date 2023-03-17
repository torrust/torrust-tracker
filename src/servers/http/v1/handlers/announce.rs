use std::net::{IpAddr, SocketAddr};
use std::panic::Location;
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use log::debug;

use crate::servers::http::v1::extractors::announce_request::ExtractRequest;
use crate::servers::http::v1::extractors::authentication_key::Extract as ExtractKey;
use crate::servers::http::v1::extractors::client_ip_sources::Extract as ExtractClientIpSources;
use crate::servers::http::v1::handlers::common::auth;
use crate::servers::http::v1::requests::announce::{Announce, Compact, Event};
use crate::servers::http::v1::responses::{self, announce};
use crate::servers::http::v1::services::peer_ip_resolver::ClientIpSources;
use crate::servers::http::v1::services::{self, peer_ip_resolver};
use crate::shared::clock::{Current, Time};
use crate::tracker::auth::Key;
use crate::tracker::peer::Peer;
use crate::tracker::{AnnounceData, Tracker};

#[allow(clippy::unused_async)]
pub async fn handle_without_key(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(announce_request): ExtractRequest,
    ExtractClientIpSources(client_ip_sources): ExtractClientIpSources,
) -> Response {
    debug!("http announce request: {:#?}", announce_request);

    handle(&tracker, &announce_request, &client_ip_sources, None).await
}

#[allow(clippy::unused_async)]
pub async fn handle_with_key(
    State(tracker): State<Arc<Tracker>>,
    ExtractRequest(announce_request): ExtractRequest,
    ExtractClientIpSources(client_ip_sources): ExtractClientIpSources,
    ExtractKey(key): ExtractKey,
) -> Response {
    debug!("http announce request: {:#?}", announce_request);

    handle(&tracker, &announce_request, &client_ip_sources, Some(key)).await
}

async fn handle(
    tracker: &Arc<Tracker>,
    announce_request: &Announce,
    client_ip_sources: &ClientIpSources,
    maybe_key: Option<Key>,
) -> Response {
    let announce_data = match handle_announce(tracker, announce_request, client_ip_sources, maybe_key).await {
        Ok(announce_data) => announce_data,
        Err(error) => return error.into_response(),
    };
    build_response(announce_request, announce_data)
}

/* code-review: authentication, authorization and peer IP resolution could be moved
   from the handler (Axum) layer into the app layer `services::announce::invoke`.
   That would make the handler even simpler and the code more reusable and decoupled from Axum.
*/

async fn handle_announce(
    tracker: &Arc<Tracker>,
    announce_request: &Announce,
    client_ip_sources: &ClientIpSources,
    maybe_key: Option<Key>,
) -> Result<AnnounceData, responses::error::Error> {
    // Authentication
    if tracker.requires_authentication() {
        match maybe_key {
            Some(key) => match tracker.authenticate(&key).await {
                Ok(_) => (),
                Err(error) => return Err(responses::error::Error::from(error)),
            },
            None => {
                return Err(responses::error::Error::from(auth::Error::MissingAuthKey {
                    location: Location::caller(),
                }))
            }
        }
    }

    // Authorization
    match tracker.authorize(&announce_request.info_hash).await {
        Ok(_) => (),
        Err(error) => return Err(responses::error::Error::from(error)),
    }

    let peer_ip = match peer_ip_resolver::invoke(tracker.config.on_reverse_proxy, client_ip_sources) {
        Ok(peer_ip) => peer_ip,
        Err(error) => return Err(responses::error::Error::from(error)),
    };

    let mut peer = peer_from_request(announce_request, &peer_ip);

    let announce_data = services::announce::invoke(tracker.clone(), announce_request.info_hash, &mut peer).await;

    Ok(announce_data)
}

fn build_response(announce_request: &Announce, announce_data: AnnounceData) -> Response {
    match &announce_request.compact {
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

#[cfg(test)]
mod tests {

    use torrust_tracker_test_helpers::configuration;

    use crate::servers::http::v1::requests::announce::Announce;
    use crate::servers::http::v1::responses;
    use crate::servers::http::v1::services::peer_ip_resolver::ClientIpSources;
    use crate::shared::bit_torrent::info_hash::InfoHash;
    use crate::tracker::services::common::tracker_factory;
    use crate::tracker::{peer, Tracker};

    fn private_tracker() -> Tracker {
        tracker_factory(configuration::ephemeral_mode_private().into())
    }

    fn whitelisted_tracker() -> Tracker {
        tracker_factory(configuration::ephemeral_mode_whitelisted().into())
    }

    fn tracker_on_reverse_proxy() -> Tracker {
        tracker_factory(configuration::ephemeral_with_reverse_proxy().into())
    }

    fn tracker_not_on_reverse_proxy() -> Tracker {
        tracker_factory(configuration::ephemeral_without_reverse_proxy().into())
    }

    fn sample_announce_request() -> Announce {
        Announce {
            info_hash: "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(),
            peer_id: "-qB00000000000000001".parse::<peer::Id>().unwrap(),
            port: 17548,
            downloaded: None,
            uploaded: None,
            left: None,
            event: None,
            compact: None,
        }
    }

    fn sample_client_ip_sources() -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_ip: None,
        }
    }

    fn assert_error_response(error: &responses::error::Error, error_message: &str) {
        assert!(
            error.failure_reason.contains(error_message),
            "Error response does not contain message: '{error_message}'. Error: {error:?}"
        );
    }

    mod with_tracker_in_private_mode {

        use std::str::FromStr;
        use std::sync::Arc;

        use super::{private_tracker, sample_announce_request, sample_client_ip_sources};
        use crate::servers::http::v1::handlers::announce::handle_announce;
        use crate::servers::http::v1::handlers::announce::tests::assert_error_response;
        use crate::tracker::auth;

        #[tokio::test]
        async fn it_should_fail_when_the_authentication_key_is_missing() {
            let tracker = Arc::new(private_tracker());

            let maybe_key = None;

            let response = handle_announce(&tracker, &sample_announce_request(), &sample_client_ip_sources(), maybe_key)
                .await
                .unwrap_err();

            assert_error_response(
                &response,
                "Authentication error: Missing authentication key param for private tracker",
            );
        }

        #[tokio::test]
        async fn it_should_fail_when_the_authentication_key_is_invalid() {
            let tracker = Arc::new(private_tracker());

            let unregistered_key = auth::Key::from_str("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();

            let maybe_key = Some(unregistered_key);

            let response = handle_announce(&tracker, &sample_announce_request(), &sample_client_ip_sources(), maybe_key)
                .await
                .unwrap_err();

            assert_error_response(&response, "Authentication error: Failed to read key");
        }
    }

    mod with_tracker_in_listed_mode {

        use std::sync::Arc;

        use super::{sample_announce_request, sample_client_ip_sources, whitelisted_tracker};
        use crate::servers::http::v1::handlers::announce::handle_announce;
        use crate::servers::http::v1::handlers::announce::tests::assert_error_response;

        #[tokio::test]
        async fn it_should_fail_when_the_announced_torrent_is_not_whitelisted() {
            let tracker = Arc::new(whitelisted_tracker());

            let announce_request = sample_announce_request();

            let response = handle_announce(&tracker, &announce_request, &sample_client_ip_sources(), None)
                .await
                .unwrap_err();

            assert_error_response(
                &response,
                &format!(
                    "Tracker error: The torrent: {}, is not whitelisted",
                    announce_request.info_hash
                ),
            );
        }
    }

    mod with_tracker_on_reverse_proxy {

        use std::sync::Arc;

        use super::{sample_announce_request, tracker_on_reverse_proxy};
        use crate::servers::http::v1::handlers::announce::handle_announce;
        use crate::servers::http::v1::handlers::announce::tests::assert_error_response;
        use crate::servers::http::v1::services::peer_ip_resolver::ClientIpSources;

        #[tokio::test]
        async fn it_should_fail_when_the_right_most_x_forwarded_for_header_ip_is_not_available() {
            let tracker = Arc::new(tracker_on_reverse_proxy());

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_ip: None,
            };

            let response = handle_announce(&tracker, &sample_announce_request(), &client_ip_sources, None)
                .await
                .unwrap_err();

            assert_error_response(
                &response,
                "Error resolving peer IP: missing or invalid the right most X-Forwarded-For IP",
            );
        }
    }

    mod with_tracker_not_on_reverse_proxy {

        use std::sync::Arc;

        use super::{sample_announce_request, tracker_not_on_reverse_proxy};
        use crate::servers::http::v1::handlers::announce::handle_announce;
        use crate::servers::http::v1::handlers::announce::tests::assert_error_response;
        use crate::servers::http::v1::services::peer_ip_resolver::ClientIpSources;

        #[tokio::test]
        async fn it_should_fail_when_the_client_ip_from_the_connection_info_is_not_available() {
            let tracker = Arc::new(tracker_not_on_reverse_proxy());

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_ip: None,
            };

            let response = handle_announce(&tracker, &sample_announce_request(), &client_ip_sources, None)
                .await
                .unwrap_err();

            assert_error_response(
                &response,
                "Error resolving peer IP: cannot get the client IP from the connection info",
            );
        }
    }
}
