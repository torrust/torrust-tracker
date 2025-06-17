//! Axum [`handlers`](axum#handlers) for the `announce` requests.
//!
//! The handlers perform the authentication and authorization of the request,
//! and resolve the client IP address.
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use bittorrent_http_tracker_core::services::scrape::ScrapeService;
use bittorrent_http_tracker_protocol::v1::requests::scrape::Scrape;
use bittorrent_http_tracker_protocol::v1::responses;
use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
use bittorrent_tracker_core::authentication::Key;
use hyper::StatusCode;
use torrust_tracker_primitives::core::ScrapeData;
use torrust_tracker_primitives::service_binding::ServiceBinding;

use crate::v1::extractors::authentication_key::Extract as ExtractKey;
use crate::v1::extractors::client_ip_sources::Extract as ExtractClientIpSources;
use crate::v1::extractors::scrape_request::ExtractRequest;

/// It handles the `scrape` request when the HTTP tracker is configured
/// to run in `public` mode.
#[allow(clippy::unused_async)]
pub async fn handle_without_key(
    State(state): State<(Arc<ScrapeService>, ServiceBinding)>,
    ExtractRequest(scrape_request): ExtractRequest,
    ExtractClientIpSources(client_ip_sources): ExtractClientIpSources,
) -> Response {
    tracing::debug!("http scrape request: {:#?}", &scrape_request);

    handle(&state.0, &scrape_request, &client_ip_sources, &state.1, None).await
}

/// It handles the `scrape` request when the HTTP tracker is configured
/// to run in `private` or `private_listed` mode.
///
/// In this case, the authentication `key` parameter is required.
#[allow(clippy::unused_async)]
pub async fn handle_with_key(
    State(state): State<(Arc<ScrapeService>, ServiceBinding)>,
    ExtractRequest(scrape_request): ExtractRequest,
    ExtractClientIpSources(client_ip_sources): ExtractClientIpSources,
    ExtractKey(key): ExtractKey,
) -> Response {
    tracing::debug!("http scrape request: {:#?}", &scrape_request);

    handle(&state.0, &scrape_request, &client_ip_sources, &state.1, Some(key)).await
}

async fn handle(
    scrape_service: &Arc<ScrapeService>,
    scrape_request: &Scrape,
    client_ip_sources: &ClientIpSources,
    server_service_binding: &ServiceBinding,
    maybe_key: Option<Key>,
) -> Response {
    let scrape_data = match scrape_service
        .handle_scrape(scrape_request, client_ip_sources, server_service_binding, maybe_key)
        .await
    {
        Ok(scrape_data) => scrape_data,
        Err(error) => {
            let error_response = responses::error::Error {
                failure_reason: error.to_string(),
            };
            return (StatusCode::OK, error_response.write()).into_response();
        }
    };

    build_response(scrape_data)
}

fn build_response(scrape_data: ScrapeData) -> Response {
    let response = responses::scrape::Bencoded::from(scrape_data);

    (StatusCode::OK, response.body()).into_response()
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;
    use std::sync::Arc;

    use bittorrent_http_tracker_core::event::bus::EventBus;
    use bittorrent_http_tracker_core::event::sender::Broadcaster;
    use bittorrent_http_tracker_core::statistics::event::listener::run_event_listener;
    use bittorrent_http_tracker_core::statistics::repository::Repository;
    use bittorrent_http_tracker_protocol::v1::requests::scrape::Scrape;
    use bittorrent_http_tracker_protocol::v1::responses;
    use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
    use bittorrent_primitives::info_hash::InfoHash;
    use bittorrent_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
    use bittorrent_tracker_core::authentication::service::AuthenticationService;
    use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
    use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use tokio_util::sync::CancellationToken;
    use torrust_tracker_configuration::{Configuration, Core};
    use torrust_tracker_test_helpers::configuration;

    struct CoreTrackerServices {
        pub core_config: Arc<Core>,
        pub scrape_handler: Arc<ScrapeHandler>,
        pub authentication_service: Arc<AuthenticationService>,
    }

    struct CoreHttpTrackerServices {
        pub http_stats_event_sender: bittorrent_http_tracker_core::event::sender::Sender,
    }

    fn initialize_private_tracker() -> (CoreTrackerServices, CoreHttpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_private())
    }

    fn initialize_listed_tracker() -> (CoreTrackerServices, CoreHttpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_listed())
    }

    fn initialize_tracker_on_reverse_proxy() -> (CoreTrackerServices, CoreHttpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_with_reverse_proxy())
    }

    fn initialize_tracker_not_on_reverse_proxy() -> (CoreTrackerServices, CoreHttpTrackerServices) {
        initialize_core_tracker_services(&configuration::ephemeral_without_reverse_proxy())
    }

    fn initialize_core_tracker_services(config: &Configuration) -> (CoreTrackerServices, CoreHttpTrackerServices) {
        let cancellation_token = CancellationToken::new();

        let core_config = Arc::new(config.core.clone());
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
        let authentication_service = Arc::new(AuthenticationService::new(&config.core, &in_memory_key_repository));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));

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

        (
            CoreTrackerServices {
                core_config,
                scrape_handler,
                authentication_service,
            },
            CoreHttpTrackerServices { http_stats_event_sender },
        )
    }

    fn sample_scrape_request() -> Scrape {
        Scrape {
            info_hashes: vec!["3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap()], // DevSkim: ignore DS173237
        }
    }

    fn sample_client_ip_sources() -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: Some(IpAddr::from_str("203.0.113.195").unwrap()),
            connection_info_socket_address: Some(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 8080)),
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

        use bittorrent_http_tracker_core::services::scrape::ScrapeService;
        use bittorrent_tracker_core::authentication;
        use torrust_tracker_primitives::core::ScrapeData;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use super::{initialize_private_tracker, sample_client_ip_sources, sample_scrape_request};

        #[tokio::test]
        async fn it_should_return_zeroed_swarm_metadata_when_the_authentication_key_is_missing() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let (core_tracker_services, core_http_tracker_services) = initialize_private_tracker();

            let scrape_request = sample_scrape_request();
            let maybe_key = None;

            let scrape_service = ScrapeService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.scrape_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let scrape_data = scrape_service
                .handle_scrape(
                    &scrape_request,
                    &sample_client_ip_sources(),
                    &server_service_binding,
                    maybe_key,
                )
                .await
                .unwrap();

            let expected_scrape_data = ScrapeData::zeroed(&scrape_request.info_hashes);

            assert_eq!(scrape_data, expected_scrape_data);
        }

        #[tokio::test]
        async fn it_should_return_zeroed_swarm_metadata_when_the_authentication_key_is_invalid() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let (core_tracker_services, core_http_tracker_services) = initialize_private_tracker();

            let scrape_request = sample_scrape_request();
            let unregistered_key = authentication::Key::from_str("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();
            let maybe_key = Some(unregistered_key);

            let scrape_service = ScrapeService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.scrape_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let scrape_data = scrape_service
                .handle_scrape(
                    &scrape_request,
                    &sample_client_ip_sources(),
                    &server_service_binding,
                    maybe_key,
                )
                .await
                .unwrap();

            let expected_scrape_data = ScrapeData::zeroed(&scrape_request.info_hashes);

            assert_eq!(scrape_data, expected_scrape_data);
        }
    }

    mod with_tracker_in_listed_mode {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use bittorrent_http_tracker_core::services::scrape::ScrapeService;
        use torrust_tracker_primitives::core::ScrapeData;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use super::{initialize_listed_tracker, sample_client_ip_sources, sample_scrape_request};

        #[tokio::test]
        async fn it_should_return_zeroed_swarm_metadata_when_the_torrent_is_not_whitelisted() {
            let (core_tracker_services, core_http_tracker_services) = initialize_listed_tracker();

            let scrape_request = sample_scrape_request();

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = ScrapeService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.scrape_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let scrape_data = scrape_service
                .handle_scrape(&scrape_request, &sample_client_ip_sources(), &server_service_binding, None)
                .await
                .unwrap();

            let expected_scrape_data = ScrapeData::zeroed(&scrape_request.info_hashes);

            assert_eq!(scrape_data, expected_scrape_data);
        }
    }

    mod with_tracker_on_reverse_proxy {

        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use bittorrent_http_tracker_core::services::scrape::ScrapeService;
        use bittorrent_http_tracker_protocol::v1::responses;
        use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use super::{initialize_tracker_on_reverse_proxy, sample_scrape_request};
        use crate::v1::handlers::scrape::tests::assert_error_response;

        #[tokio::test]
        async fn it_should_fail_when_the_right_most_x_forwarded_for_header_ip_is_not_available() {
            let (core_tracker_services, core_http_tracker_services) = initialize_tracker_on_reverse_proxy();

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: None,
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = ScrapeService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.scrape_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let response = scrape_service
                .handle_scrape(&sample_scrape_request(), &client_ip_sources, &server_service_binding, None)
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

        use bittorrent_http_tracker_core::services::scrape::ScrapeService;
        use bittorrent_http_tracker_protocol::v1::responses;
        use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};

        use super::{initialize_tracker_not_on_reverse_proxy, sample_scrape_request};
        use crate::v1::handlers::scrape::tests::assert_error_response;

        #[tokio::test]
        async fn it_should_fail_when_the_client_ip_from_the_connection_info_is_not_available() {
            let (core_tracker_services, core_http_tracker_services) = initialize_tracker_not_on_reverse_proxy();

            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: None,
            };

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let scrape_service = ScrapeService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.scrape_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let response = scrape_service
                .handle_scrape(&sample_scrape_request(), &client_ip_sources, &server_service_binding, None)
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
