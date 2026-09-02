//! Axum [`handlers`](axum#handlers) for the `scrape` requests.
//!
//! The handlers perform the authentication and authorization of the request,
//! and resolve the client IP address.
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_core::authentication::Key;
use torrust_tracker_http_core::services::scrape::{HttpScrapeError, ScrapeService};
use torrust_tracker_http_protocol::v1::requests::scrape::Scrape;
use torrust_tracker_http_protocol::v1::responses;
use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;
use torrust_tracker_primitives::ScrapeData as DomainScrapeData;

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
    let scrape_data = match handle_scrape(
        scrape_service,
        scrape_request,
        client_ip_sources,
        server_service_binding,
        maybe_key,
    )
    .await
    {
        Ok(scrape_data) => scrape_data,
        Err(error) => {
            let error_response = responses::error::Error::from(error);
            return (StatusCode::OK, error_response.write()).into_response();
        }
    };

    build_response(scrape_data)
}

async fn handle_scrape(
    scrape_service: &Arc<ScrapeService>,
    scrape_request: &Scrape,
    client_ip_sources: &ClientIpSources,
    server_service_binding: &ServiceBinding,
    maybe_key: Option<Key>,
) -> Result<DomainScrapeData, HttpScrapeError> {
    scrape_service
        .handle_scrape(scrape_request, client_ip_sources, server_service_binding, maybe_key)
        .await
}

fn build_response(scrape_data: DomainScrapeData) -> Response {
    let response = responses::scrape::Bencoded::from(to_protocol_scrape_data(scrape_data));

    (StatusCode::OK, response.body()).into_response()
}

fn to_protocol_scrape_data(domain_data: DomainScrapeData) -> responses::scrape::ScrapeData {
    let mut protocol_data = responses::scrape::ScrapeData::empty();

    for (info_hash, metadata) in domain_data.files {
        protocol_data.add_file(
            &info_hash,
            responses::scrape::SwarmMetadata {
                complete: metadata.complete,
                downloaded: metadata.downloaded,
                incomplete: metadata.incomplete,
            },
        );
    }

    protocol_data
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use axum::response::Response;
    use hyper::StatusCode;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;
    use std::sync::Arc;

    use tokio_util::sync::CancellationToken;
    use torrust_info_hash::InfoHash;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
    use torrust_tracker_core::authentication::service::AuthenticationService;
    use torrust_tracker_core::scrape_handler::ScrapeHandler;
    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_http_core::event::bus::EventBus;
    use torrust_tracker_http_core::event::sender::Broadcaster;
    use torrust_tracker_http_core::services::scrape::ScrapeService;
    use torrust_tracker_http_core::statistics::event::listener::run_event_listener;
    use torrust_tracker_http_core::statistics::repository::Repository;
    use torrust_tracker_http_protocol::v1::requests::scrape::Scrape;
    use torrust_tracker_http_protocol::v1::responses;
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;
    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ScrapeData, ServiceRole};
    use torrust_tracker_test_helpers::configuration;

    struct TestServices {
        pub scrape_service: Arc<ScrapeService>,
    }

    fn initialize_private_tracker() -> TestServices {
        initialize_core_tracker_services(&configuration::ephemeral_private())
    }

    fn initialize_listed_tracker() -> TestServices {
        initialize_core_tracker_services(&configuration::ephemeral_listed())
    }

    fn initialize_tracker_on_reverse_proxy() -> TestServices {
        initialize_core_tracker_services(&configuration::ephemeral_with_reverse_proxy())
    }

    fn initialize_tracker_not_on_reverse_proxy() -> TestServices {
        initialize_core_tracker_services(&configuration::ephemeral_without_reverse_proxy())
    }

    fn initialize_core_tracker_services(config: &Configuration) -> TestServices {
        let cancellation_token = CancellationToken::new();
        let configuration_instance_id = config
            .http_trackers
            .as_deref()
            .expect("the test configuration should contain an HTTP tracker")
            .iter()
            .enumerate()
            .next()
            .map(|(index, _)| ConfigurationInstanceId::new(ServiceRole::HttpTracker, index))
            .expect("the test configuration should contain an HTTP tracker");
        let http_tracker_config = config
            .http_trackers
            .as_ref()
            .expect("the test configuration should contain an HTTP tracker")[0]
            .clone();

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
            let _unused = run_event_listener(
                http_stats_event_bus.receiver(),
                cancellation_token,
                &http_stats_repository,
                [(configuration_instance_id, true)].into(),
            );
        }

        let scrape_service = Arc::new(ScrapeService::new_with_http_tracker_config(
            core_config,
            scrape_handler,
            authentication_service,
            http_stats_event_sender,
            &http_tracker_config,
            configuration_instance_id,
        ));

        TestServices { scrape_service }
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

    fn missing_client_ip_sources() -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: None,
        }
    }

    fn sample_http_service_binding() -> ServiceBinding {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);

        ServiceBinding::new(Protocol::HTTP, address).expect("the sample HTTP service binding should be valid")
    }

    fn assert_failure_reason_contains(error: &responses::error::Error, error_message: &str) {
        assert!(
            error.failure_reason.contains(error_message),
            "Error response does not contain message: '{error_message}'. Error: {error:?}"
        );
    }

    async fn decode_successful_bencoded_response(response: Response) -> responses::scrape::deserialization::Response {
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "a successful scrape response should use HTTP 200"
        );

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("scrape response body should be readable");

        responses::scrape::deserialization::Response::try_from_bencoded(&body).expect("scrape response should be valid bencode")
    }

    async fn decode_bencoded_error_response(response: Response) -> responses::error::Error {
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "a BitTorrent scrape failure response should use HTTP 200"
        );

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("scrape failure response body should be readable");

        serde_bencode::from_bytes(&body).expect("scrape failure response should be valid bencode")
    }

    #[tokio::test]
    async fn it_should_encode_domain_scrape_data_as_a_bencoded_response() {
        // Arrange
        let info_hash = sample_scrape_request().info_hashes[0];
        let mut scrape_data = ScrapeData::empty();
        scrape_data.add_file(
            &info_hash,
            SwarmMetadata {
                complete: 3,
                downloaded: 2,
                incomplete: 4,
            },
        );

        // Act
        let response = super::build_response(scrape_data);
        let actual_response = decode_successful_bencoded_response(response).await;

        // Assert
        let expected_response = responses::scrape::deserialization::Response::with_one_file(
            info_hash,
            responses::scrape::deserialization::File {
                complete: 3,
                downloaded: 2,
                incomplete: 4,
            },
        );

        assert_eq!(actual_response, expected_response);
    }

    #[tokio::test]
    async fn it_should_encode_each_file_when_scrape_data_contains_multiple_files() {
        // Arrange
        let first_info_hash = sample_scrape_request().info_hashes[0];
        let second_info_hash = InfoHash::from([2; 20]);
        let mut scrape_data = ScrapeData::empty();
        scrape_data.add_file(
            &first_info_hash,
            SwarmMetadata {
                complete: 3,
                downloaded: 2,
                incomplete: 4,
            },
        );
        scrape_data.add_file(
            &second_info_hash,
            SwarmMetadata {
                complete: 7,
                downloaded: 11,
                incomplete: 13,
            },
        );

        // Act
        let response = super::build_response(scrape_data);
        let actual_response = decode_successful_bencoded_response(response).await;

        // Assert
        let expected_response = responses::scrape::deserialization::ResponseBuilder::default()
            .add_file(
                first_info_hash,
                responses::scrape::deserialization::File {
                    complete: 3,
                    downloaded: 2,
                    incomplete: 4,
                },
            )
            .add_file(
                second_info_hash,
                responses::scrape::deserialization::File {
                    complete: 7,
                    downloaded: 11,
                    incomplete: 13,
                },
            )
            .build();

        assert_eq!(actual_response, expected_response);
    }

    #[tokio::test]
    async fn it_should_encode_a_bencoded_failure_response_when_the_client_ip_cannot_be_resolved() {
        // Arrange
        let test_services = initialize_tracker_on_reverse_proxy();

        // Act
        let response = super::handle(
            &test_services.scrape_service,
            &sample_scrape_request(),
            &missing_client_ip_sources(),
            &sample_http_service_binding(),
            None,
        )
        .await;
        let actual_error_response = decode_bencoded_error_response(response).await;

        // Assert
        assert_failure_reason_contains(
            &actual_error_response,
            "Error resolving peer IP: missing or invalid the right most X-Forwarded-For IP",
        );
    }

    mod with_tracker_in_private_mode {
        use std::str::FromStr;

        use torrust_tracker_core::authentication;
        use torrust_tracker_primitives::ScrapeData;

        use super::{initialize_private_tracker, sample_client_ip_sources, sample_http_service_binding, sample_scrape_request};

        #[tokio::test]
        async fn it_should_return_zeroed_swarm_metadata_when_the_authentication_key_is_missing() {
            // Arrange
            let test_services = initialize_private_tracker();
            let scrape_request = sample_scrape_request();
            let maybe_key = None;

            // Act
            let actual_scrape_data = test_services
                .scrape_service
                .handle_scrape(
                    &scrape_request,
                    &sample_client_ip_sources(),
                    &sample_http_service_binding(),
                    maybe_key,
                )
                .await
                .unwrap();

            // Assert
            let expected_scrape_data = ScrapeData::zeroed(&scrape_request.info_hashes);

            assert_eq!(actual_scrape_data, expected_scrape_data);
        }

        #[tokio::test]
        async fn it_should_return_zeroed_swarm_metadata_when_the_authentication_key_is_invalid() {
            // Arrange
            let test_services = initialize_private_tracker();
            let scrape_request = sample_scrape_request();
            let unregistered_key = authentication::Key::from_str("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();
            let maybe_key = Some(unregistered_key);

            // Act
            let actual_scrape_data = test_services
                .scrape_service
                .handle_scrape(
                    &scrape_request,
                    &sample_client_ip_sources(),
                    &sample_http_service_binding(),
                    maybe_key,
                )
                .await
                .unwrap();

            // Assert
            let expected_scrape_data = ScrapeData::zeroed(&scrape_request.info_hashes);

            assert_eq!(actual_scrape_data, expected_scrape_data);
        }
    }

    mod with_tracker_in_listed_mode {

        use torrust_tracker_primitives::ScrapeData;

        use super::{initialize_listed_tracker, sample_client_ip_sources, sample_http_service_binding, sample_scrape_request};

        #[tokio::test]
        async fn it_should_return_zeroed_swarm_metadata_when_the_torrent_is_not_whitelisted() {
            // Arrange
            let test_services = initialize_listed_tracker();
            let scrape_request = sample_scrape_request();

            // Act
            let actual_scrape_data = test_services
                .scrape_service
                .handle_scrape(
                    &scrape_request,
                    &sample_client_ip_sources(),
                    &sample_http_service_binding(),
                    None,
                )
                .await
                .unwrap();

            // Assert
            let expected_scrape_data = ScrapeData::zeroed(&scrape_request.info_hashes);

            assert_eq!(actual_scrape_data, expected_scrape_data);
        }
    }

    mod with_tracker_on_reverse_proxy {

        use torrust_tracker_http_protocol::v1::responses;

        use super::{
            assert_failure_reason_contains, initialize_tracker_on_reverse_proxy, missing_client_ip_sources,
            sample_http_service_binding, sample_scrape_request,
        };

        #[tokio::test]
        async fn it_should_fail_when_the_right_most_x_forwarded_for_header_ip_is_not_available() {
            // Arrange
            let test_services = initialize_tracker_on_reverse_proxy();

            // Act
            let actual_error = test_services
                .scrape_service
                .handle_scrape(
                    &sample_scrape_request(),
                    &missing_client_ip_sources(),
                    &sample_http_service_binding(),
                    None,
                )
                .await
                .unwrap_err();

            // Assert
            let actual_error_response = responses::error::Error::from(actual_error);

            assert_failure_reason_contains(
                &actual_error_response,
                "Error resolving peer IP: missing or invalid the right most X-Forwarded-For IP",
            );
        }
    }

    mod with_tracker_not_on_reverse_proxy {

        use torrust_tracker_http_protocol::v1::responses;

        use super::{
            assert_failure_reason_contains, initialize_tracker_not_on_reverse_proxy, missing_client_ip_sources,
            sample_http_service_binding, sample_scrape_request,
        };

        #[tokio::test]
        async fn it_should_fail_when_the_client_ip_from_the_connection_info_is_not_available() {
            // Arrange
            let test_services = initialize_tracker_not_on_reverse_proxy();

            // Act
            let actual_error = test_services
                .scrape_service
                .handle_scrape(
                    &sample_scrape_request(),
                    &missing_client_ip_sources(),
                    &sample_http_service_binding(),
                    None,
                )
                .await
                .unwrap_err();

            // Assert
            let actual_error_response = responses::error::Error::from(actual_error);

            assert_failure_reason_contains(
                &actual_error_response,
                "Error resolving peer IP: cannot get the client IP from the connection info",
            );
        }
    }
}
