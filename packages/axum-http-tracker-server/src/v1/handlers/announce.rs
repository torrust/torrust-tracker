//! Axum [`handlers`](axum#handlers) for the `announce` requests.
//!
//! The handlers perform the authentication and authorization of the request,
//! and resolve the client IP address.
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use bittorrent_http_tracker_core::services::announce::{AnnounceService, HttpAnnounceError};
use bittorrent_http_tracker_protocol::v1::requests::announce::{Announce, Compact};
use bittorrent_http_tracker_protocol::v1::responses::{self};
use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
use bittorrent_tracker_core::authentication::Key;
use hyper::StatusCode;
use torrust_tracker_primitives::core::AnnounceData;
use torrust_tracker_primitives::service_binding::ServiceBinding;

use crate::v1::extractors::announce_request::ExtractRequest;
use crate::v1::extractors::authentication_key::Extract as ExtractKey;
use crate::v1::extractors::client_ip_sources::Extract as ExtractClientIpSources;

/// It handles the `announce` request when the HTTP tracker does not require
/// authentication (no PATH `key` parameter required).
#[allow(clippy::unused_async)]
pub async fn handle_without_key(
    State(state): State<(Arc<AnnounceService>, ServiceBinding)>,
    ExtractRequest(announce_request): ExtractRequest,
    ExtractClientIpSources(client_ip_sources): ExtractClientIpSources,
) -> Response {
    tracing::debug!("http announce request: {:#?}", announce_request);

    handle(&state.0, &announce_request, &client_ip_sources, &state.1, None).await
}

/// It handles the `announce` request when the HTTP tracker requires
/// authentication (PATH `key` parameter required).
#[allow(clippy::unused_async)]
pub async fn handle_with_key(
    State(state): State<(Arc<AnnounceService>, ServiceBinding)>,
    ExtractRequest(announce_request): ExtractRequest,
    ExtractClientIpSources(client_ip_sources): ExtractClientIpSources,
    ExtractKey(key): ExtractKey,
) -> Response {
    tracing::debug!("http announce request: {:#?}", announce_request);

    handle(&state.0, &announce_request, &client_ip_sources, &state.1, Some(key)).await
}

/// It handles the `announce` request.
///
/// Internal implementation that handles both the `authenticated` and
/// `unauthenticated` modes.
async fn handle(
    announce_service: &Arc<AnnounceService>,
    announce_request: &Announce,
    client_ip_sources: &ClientIpSources,
    server_service_binding: &ServiceBinding,
    maybe_key: Option<Key>,
) -> Response {
    let announce_data = match handle_announce(
        announce_service,
        announce_request,
        client_ip_sources,
        server_service_binding,
        maybe_key,
    )
    .await
    {
        Ok(announce_data) => announce_data,
        Err(error) => {
            let error_response = responses::error::Error {
                failure_reason: error.to_string(),
            };
            return (StatusCode::OK, error_response.write()).into_response();
        }
    };
    build_response(announce_request, announce_data)
}

async fn handle_announce(
    announce_service: &Arc<AnnounceService>,
    announce_request: &Announce,
    client_ip_sources: &ClientIpSources,
    server_service_binding: &ServiceBinding,
    maybe_key: Option<Key>,
) -> Result<AnnounceData, HttpAnnounceError> {
    announce_service
        .handle_announce(announce_request, client_ip_sources, server_service_binding, maybe_key)
        .await
}

fn build_response(announce_request: &Announce, announce_data: AnnounceData) -> Response {
    if announce_request.compact.as_ref().is_some_and(|f| *f == Compact::Accepted) {
        let response: responses::Announce<responses::Compact> = announce_data.into();
        let bytes: Vec<u8> = response.data.into();
        (StatusCode::OK, bytes).into_response()
    } else {
        let response: responses::Announce<responses::Normal> = announce_data.into();
        let bytes: Vec<u8> = response.data.into();
        (StatusCode::OK, bytes).into_response()
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use aquatic_udp_protocol::PeerId;
    use bittorrent_http_tracker_core::event::bus::EventBus;
    use bittorrent_http_tracker_core::event::sender::Broadcaster;
    use bittorrent_http_tracker_core::services::announce::AnnounceService;
    use bittorrent_http_tracker_core::statistics::event::listener::run_event_listener;
    use bittorrent_http_tracker_core::statistics::repository::Repository;
    use bittorrent_http_tracker_protocol::v1::requests::announce::Announce;
    use bittorrent_http_tracker_protocol::v1::responses;
    use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
    use bittorrent_tracker_core::announce_handler::AnnounceHandler;
    use bittorrent_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
    use bittorrent_tracker_core::authentication::service::AuthenticationService;
    use bittorrent_tracker_core::databases::setup::initialize_database;
    use bittorrent_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use tokio_util::sync::CancellationToken;
    use torrust_tracker_configuration::Configuration;
    use torrust_tracker_test_helpers::configuration;

    use crate::tests::helpers::sample_info_hash;

    struct CoreHttpTrackerServices {
        pub announce_service: Arc<AnnounceService>,
    }

    fn initialize_private_tracker() -> CoreHttpTrackerServices {
        initialize_core_tracker_services(&configuration::ephemeral_private())
    }

    fn initialize_listed_tracker() -> CoreHttpTrackerServices {
        initialize_core_tracker_services(&configuration::ephemeral_listed())
    }

    fn initialize_tracker_on_reverse_proxy() -> CoreHttpTrackerServices {
        initialize_core_tracker_services(&configuration::ephemeral_with_reverse_proxy())
    }

    fn initialize_tracker_not_on_reverse_proxy() -> CoreHttpTrackerServices {
        initialize_core_tracker_services(&configuration::ephemeral_without_reverse_proxy())
    }

    fn initialize_core_tracker_services(config: &Configuration) -> CoreHttpTrackerServices {
        let cancellation_token = CancellationToken::new();

        // Initialize the core tracker services with the provided configuration.
        let core_config = Arc::new(config.core.clone());
        let database = initialize_database(&config.core);
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
        let authentication_service = Arc::new(AuthenticationService::new(&config.core, &in_memory_key_repository));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&database));
        let announce_handler = Arc::new(AnnounceHandler::new(
            &config.core,
            &whitelist_authorization,
            &in_memory_torrent_repository,
            &db_downloads_metric_repository,
        ));

        // HTTP core stats
        let http_core_broadcaster = Broadcaster::default();
        let http_stats_repository = Arc::new(Repository::new());
        let http_stats_event_bus = Arc::new(EventBus::new(
            config.core.tracker_usage_statistics.into(),
            http_core_broadcaster.clone(),
        ));

        let http_stats_event_sender = http_stats_event_bus.sender();

        if config.core.tracker_usage_statistics {
            let _unused = run_event_listener(http_stats_event_bus.receiver(), cancellation_token, &http_stats_repository);
        }

        let announce_service = Arc::new(AnnounceService::new(
            core_config.clone(),
            announce_handler.clone(),
            authentication_service.clone(),
            whitelist_authorization.clone(),
            http_stats_event_sender.clone(),
        ));

        CoreHttpTrackerServices { announce_service }
    }

    fn sample_announce_request() -> Announce {
        Announce {
            info_hash: sample_info_hash(),
            peer_id: PeerId(*b"-qB00000000000000001"),
            port: 17548,
            downloaded: None,
            uploaded: None,
            left: None,
            event: None,
            compact: None,
            numwant: None,
        }
    }

    fn sample_client_ip_sources() -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: None,
        }
    }

    fn assert_error_response(error: &responses::error::Error, error_message: &str) {
        assert!(
            error.failure_reason.contains(error_message),
            "Error response does not contain message: '{error_message}'. Error: {error:?}"
        );
    }

    mod with_tracker_in_private_mode {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::str::FromStr;

        use bittorrent_http_tracker_protocol::v1::responses;
        use bittorrent_tracker_core::authentication;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use super::{initialize_private_tracker, sample_announce_request, sample_client_ip_sources};
        use crate::v1::handlers::announce::handle_announce;
        use crate::v1::handlers::announce::tests::assert_error_response;

        #[tokio::test]
        async fn it_should_fail_when_the_authentication_key_is_missing() {
            let http_core_tracker_services = initialize_private_tracker();

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let maybe_key = None;

            let response = handle_announce(
                &http_core_tracker_services.announce_service,
                &sample_announce_request(),
                &sample_client_ip_sources(),
                &server_service_binding,
                maybe_key,
            )
            .await
            .unwrap_err();

            let error_response = responses::error::Error {
                failure_reason: response.to_string(),
            };

            assert_error_response(
                &error_response,
                "Tracker core error: Tracker core authentication error: Missing authentication key",
            );
        }

        #[tokio::test]
        async fn it_should_fail_when_the_authentication_key_is_invalid() {
            let http_core_tracker_services = initialize_private_tracker();

            let unregistered_key = authentication::Key::from_str("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let maybe_key = Some(unregistered_key);

            let response = handle_announce(
                &http_core_tracker_services.announce_service,
                &sample_announce_request(),
                &sample_client_ip_sources(),
                &server_service_binding,
                maybe_key,
            )
            .await
            .unwrap_err();

            let error_response = responses::error::Error {
                failure_reason: response.to_string(),
            };

            assert_error_response(
                &error_response,
                "Tracker core error: Tracker core authentication error: Failed to read key: YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ",
            );
        }
    }

    mod with_tracker_in_listed_mode {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use bittorrent_http_tracker_protocol::v1::responses;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use super::{initialize_listed_tracker, sample_announce_request, sample_client_ip_sources};
        use crate::v1::handlers::announce::handle_announce;
        use crate::v1::handlers::announce::tests::assert_error_response;

        #[tokio::test]
        async fn it_should_fail_when_the_announced_torrent_is_not_whitelisted() {
            let http_core_tracker_services = initialize_listed_tracker();

            let announce_request = sample_announce_request();

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let response = handle_announce(
                &http_core_tracker_services.announce_service,
                &announce_request,
                &sample_client_ip_sources(),
                &server_service_binding,
                None,
            )
            .await
            .unwrap_err();

            let error_response = responses::error::Error {
                failure_reason: response.to_string(),
            };

            assert_error_response(
                &error_response,
                &format!(
                    "Tracker core error: Tracker core whitelist error: The torrent: {}, is not whitelisted",
                    announce_request.info_hash
                ),
            );
        }
    }

    mod with_tracker_on_reverse_proxy {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use bittorrent_http_tracker_protocol::v1::responses;
        use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use super::{initialize_tracker_on_reverse_proxy, sample_announce_request};
        use crate::v1::handlers::announce::handle_announce;
        use crate::v1::handlers::announce::tests::assert_error_response;

        #[tokio::test]
        async fn it_should_fail_when_the_right_most_x_forwarded_for_header_ip_is_not_available() {
            let http_core_tracker_services = initialize_tracker_on_reverse_proxy();

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: None,
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let response = handle_announce(
                &http_core_tracker_services.announce_service,
                &sample_announce_request(),
                &client_ip_sources,
                &server_service_binding,
                None,
            )
            .await
            .unwrap_err();

            let error_response = responses::error::Error {
                failure_reason: response.to_string(),
            };

            assert_error_response(
                &error_response,
                "Error resolving peer IP: missing or invalid the right most X-Forwarded-For IP",
            );
        }
    }

    mod with_tracker_not_on_reverse_proxy {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use bittorrent_http_tracker_protocol::v1::responses;
        use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use super::{initialize_tracker_not_on_reverse_proxy, sample_announce_request};
        use crate::v1::handlers::announce::handle_announce;
        use crate::v1::handlers::announce::tests::assert_error_response;

        #[tokio::test]
        async fn it_should_fail_when_the_client_ip_from_the_connection_info_is_not_available() {
            let http_core_tracker_services = initialize_tracker_not_on_reverse_proxy();

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: None,
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let response = handle_announce(
                &http_core_tracker_services.announce_service,
                &sample_announce_request(),
                &client_ip_sources,
                &server_service_binding,
                None,
            )
            .await
            .unwrap_err();

            let error_response = responses::error::Error {
                failure_reason: response.to_string(),
            };

            assert_error_response(
                &error_response,
                "Error resolving peer IP: cannot get the client IP from the connection info",
            );
        }
    }
}
