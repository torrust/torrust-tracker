//! Axum [`handlers`](axum#handlers) for the `announce` requests.
//!
//! The handlers perform the authentication and authorization of the request,
//! and resolve the client IP address.
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_core::authentication::Key;
use torrust_tracker_http_core::services::announce::{AnnounceService, HttpAnnounceError};
use torrust_tracker_http_protocol::v1::requests::announce::{Announce, Compact};
use torrust_tracker_http_protocol::v1::responses::{self};
use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;
use torrust_tracker_primitives::AnnounceData as DomainAnnounceData;

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
    tracing::debug!("Received HTTP announce request");

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
    tracing::debug!("Received HTTP announce request");

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
            let error_response = responses::error::Error::from(error);
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
) -> Result<DomainAnnounceData, HttpAnnounceError> {
    announce_service
        .handle_announce(announce_request, client_ip_sources, server_service_binding, maybe_key)
        .await
}

fn build_response(announce_request: &Announce, announce_data: DomainAnnounceData) -> Response {
    let protocol_data = to_protocol_announce_data(announce_data);

    if announce_request.compact.as_ref().is_some_and(|f| *f == Compact::NotAccepted) {
        let response: responses::Announce<responses::Normal> = protocol_data.into();
        let bytes: Vec<u8> = response.data.into();
        (StatusCode::OK, bytes).into_response()
    } else {
        let response: responses::Announce<responses::Compact> = protocol_data.into();
        let bytes: Vec<u8> = response.data.into();
        (StatusCode::OK, bytes).into_response()
    }
}

fn to_protocol_announce_data(domain_data: DomainAnnounceData) -> responses::announce::AnnounceData {
    responses::announce::AnnounceData {
        peers: domain_data
            .peers
            .into_iter()
            .map(|peer| responses::announce::Peer {
                peer_id: peer.peer_id,
                peer_addr: peer.peer_addr,
            })
            .collect(),
        stats: responses::announce::SwarmMetadata {
            complete: domain_data.stats.complete,
            downloaded: domain_data.stats.downloaded,
            incomplete: domain_data.stats.incomplete,
        },
        policy: responses::announce::AnnouncePolicy {
            interval: domain_data.policy.interval,
            interval_min: domain_data.policy.interval_min,
        },
    }
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use axum::body::to_bytes;
    use axum::response::Response;
    use hyper::StatusCode;
    use serde::de::DeserializeOwned;
    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_core::announce_handler::AnnounceHandler;
    use torrust_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
    use torrust_tracker_core::authentication::service::AuthenticationService;
    use torrust_tracker_core::databases::setup::initialize_database;
    use torrust_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_http_core::event::bus::EventBus;
    use torrust_tracker_http_core::event::sender::Broadcaster;
    use torrust_tracker_http_core::services::announce::AnnounceService;
    use torrust_tracker_http_core::statistics::event::listener::run_event_listener;
    use torrust_tracker_http_core::statistics::repository::Repository;
    use torrust_tracker_http_protocol::v1::requests::announce::{Announce, Compact, PeerIp};
    use torrust_tracker_http_protocol::v1::responses;
    use torrust_tracker_http_protocol::v1::responses::announce::deserialization::{
        DeserializedCompact, DeserializedNormal, DictionaryPeer,
    };
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;
    use torrust_tracker_primitives::peer::fixture::PeerBuilder;
    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
    use torrust_tracker_primitives::{AnnounceData, AnnouncePolicy, ConfigurationInstanceId, PeerId, ServiceRole};
    use torrust_tracker_test_helpers::configuration;

    use crate::tests::helpers::sample_info_hash;

    struct CoreHttpTrackerServices {
        pub announce_service: Arc<AnnounceService>,
    }

    struct AnnounceResponseScenario<TExpectedResponse> {
        announce_request: Announce,
        announce_data: AnnounceData,
        expected_response: TExpectedResponse,
    }

    impl AnnounceResponseScenario<DeserializedNormal> {
        fn non_compact_response_for_one_ipv4_seeder() -> Self {
            Self {
                announce_request: Announce {
                    compact: Some(Compact::NotAccepted),
                    ..sample_announce_request()
                },
                announce_data: one_ipv4_seeder_announce_data(),
                expected_response: DeserializedNormal {
                    complete: 3,
                    incomplete: 4,
                    interval: 60,
                    min_interval: 30,
                    peers: vec![DictionaryPeer {
                        ip: "127.0.0.1".to_string(),
                        peer_id: b"-qB00000000000000001".to_vec(),
                        port: 8080,
                    }],
                },
            }
        }
    }

    impl AnnounceResponseScenario<DeserializedCompact> {
        fn compact_response_for_one_ipv4_seeder_when_omitted() -> Self {
            Self {
                announce_request: sample_announce_request(),
                announce_data: one_ipv4_seeder_announce_data(),
                expected_response: DeserializedCompact {
                    complete: 3,
                    incomplete: 4,
                    interval: 60,
                    min_interval: 30,
                    peers: vec![127, 0, 0, 1, 0x1f, 0x90],
                    peers6: Vec::new(),
                },
            }
        }

        fn compact_response_for_one_ipv4_seeder_when_accepted() -> Self {
            Self {
                announce_request: Announce {
                    compact: Some(Compact::Accepted),
                    ..sample_announce_request()
                },
                announce_data: one_ipv4_seeder_announce_data(),
                expected_response: DeserializedCompact {
                    complete: 3,
                    incomplete: 4,
                    interval: 60,
                    min_interval: 30,
                    peers: vec![127, 0, 0, 1, 0x1f, 0x90],
                    peers6: Vec::new(),
                },
            }
        }
    }

    async fn initialize_private_tracker() -> CoreHttpTrackerServices {
        initialize_core_tracker_services(&configuration::ephemeral_private()).await
    }

    async fn initialize_listed_tracker() -> CoreHttpTrackerServices {
        initialize_core_tracker_services(&configuration::ephemeral_listed()).await
    }

    async fn initialize_tracker_on_reverse_proxy() -> CoreHttpTrackerServices {
        initialize_core_tracker_services(&configuration::ephemeral_with_reverse_proxy()).await
    }

    async fn initialize_tracker_not_on_reverse_proxy() -> CoreHttpTrackerServices {
        initialize_core_tracker_services(&configuration::ephemeral_without_reverse_proxy()).await
    }

    async fn initialize_core_tracker_services(config: &Configuration) -> CoreHttpTrackerServices {
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

        // Initialize the core tracker services with the provided configuration.
        let core_config = Arc::new(config.core.clone());
        let database = initialize_database(&config.core).await;
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
        let authentication_service = Arc::new(AuthenticationService::new(&config.core, &in_memory_key_repository));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&database.torrent_metrics_store));
        let announce_handler = if config.core.tracker_policy.persistent_torrent_completed_stat {
            Arc::new(AnnounceHandler::new_with_persistent_completed_statistics(
                &config.core,
                &whitelist_authorization,
                &in_memory_torrent_repository,
                &db_downloads_metric_repository,
            ))
        } else {
            Arc::new(AnnounceHandler::new_public(
                &config.core,
                &whitelist_authorization,
                &in_memory_torrent_repository,
            ))
        };

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

        let http_tracker_config = &config
            .http_trackers
            .as_ref()
            .expect("the test configuration should contain an HTTP tracker")[0];
        let announce_service = Arc::new(AnnounceService::new_with_http_tracker_config(
            core_config.clone(),
            announce_handler.clone(),
            authentication_service.clone(),
            whitelist_authorization.clone(),
            http_stats_event_sender.clone(),
            http_tracker_config,
            configuration_instance_id,
        ));

        CoreHttpTrackerServices { announce_service }
    }

    fn sample_announce_request() -> Announce {
        Announce {
            info_hash: sample_info_hash(),
            peer_id: PeerId(*b"-qB00000000000000001"),
            port: 17548,
            ip: PeerIp::Absent,
            downloaded: None,
            uploaded: None,
            left: None,
            event: None,
            compact: None,
            numwant: None,
        }
    }

    fn sample_http_service_binding() -> ServiceBinding {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);

        ServiceBinding::new(Protocol::HTTP, address).expect("the sample HTTP service binding should be valid")
    }

    fn one_ipv4_seeder_announce_data() -> AnnounceData {
        AnnounceData {
            peers: vec![Arc::new(
                PeerBuilder::seeder()
                    .with_peer_id(&PeerId(*b"-qB00000000000000001"))
                    .with_peer_address(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080))
                    .build(),
            )],
            stats: SwarmMetadata {
                complete: 3,
                downloaded: 2, // Not represented in the announce response.
                incomplete: 4,
            },
            policy: AnnouncePolicy {
                interval: 60,
                interval_min: 30,
                max_peers_per_announce: 74, // Not represented in the announce response.
            },
        }
    }

    async fn decode_successful_bencoded_response<TExpectedResponse>(response: Response) -> TExpectedResponse
    where
        TExpectedResponse: DeserializeOwned,
    {
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "a successful announce response should use HTTP 200"
        );

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("announce response body should be readable");

        serde_bencode::from_bytes(&body).expect("announce response should be valid bencode")
    }

    fn sample_client_ip_sources() -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: None,
        }
    }

    fn assert_failure_reason_contains(error: &responses::error::Error, error_message: &str) {
        assert!(
            error.failure_reason.contains(error_message),
            "Error response does not contain message: '{error_message}'. Error: {error:?}"
        );
    }

    #[tokio::test]
    async fn it_should_encode_a_non_compact_bencoded_response_when_compact_is_not_accepted() {
        // Arrange
        let scenario = AnnounceResponseScenario::non_compact_response_for_one_ipv4_seeder();

        // Act
        let response = super::build_response(&scenario.announce_request, scenario.announce_data);
        let actual_response: DeserializedNormal = decode_successful_bencoded_response(response).await;

        // Assert
        assert_eq!(actual_response, scenario.expected_response);
    }

    #[tokio::test]
    async fn it_should_encode_a_compact_bencoded_response_when_compact_is_omitted() {
        // Arrange
        let scenario = AnnounceResponseScenario::compact_response_for_one_ipv4_seeder_when_omitted();

        // Act
        let response = super::build_response(&scenario.announce_request, scenario.announce_data);
        let actual_response: DeserializedCompact = decode_successful_bencoded_response(response).await;

        // Assert
        assert_eq!(actual_response, scenario.expected_response);
    }

    #[tokio::test]
    async fn it_should_encode_a_compact_bencoded_response_when_compact_is_accepted() {
        // Arrange
        let scenario = AnnounceResponseScenario::compact_response_for_one_ipv4_seeder_when_accepted();

        // Act
        let response = super::build_response(&scenario.announce_request, scenario.announce_data);
        let actual_response: DeserializedCompact = decode_successful_bencoded_response(response).await;

        // Assert
        assert_eq!(actual_response, scenario.expected_response);
    }

    mod with_tracker_in_private_mode {

        use std::str::FromStr;

        use torrust_tracker_core::authentication;
        use torrust_tracker_http_protocol::v1::responses;

        use super::{
            assert_failure_reason_contains, initialize_private_tracker, sample_announce_request, sample_client_ip_sources,
            sample_http_service_binding,
        };
        use crate::v1::handlers::announce::handle_announce;

        #[tokio::test]
        async fn it_should_fail_when_the_authentication_key_is_missing() {
            // Arrange
            let http_core_tracker_services = initialize_private_tracker().await;
            let maybe_key = None;

            // Act
            let actual_error = handle_announce(
                &http_core_tracker_services.announce_service,
                &sample_announce_request(),
                &sample_client_ip_sources(),
                &sample_http_service_binding(),
                maybe_key,
            )
            .await
            .unwrap_err();

            // Assert
            let actual_error_response = responses::error::Error::from(actual_error);

            assert_failure_reason_contains(
                &actual_error_response,
                "Tracker authentication error: Missing authentication key",
            );
        }

        #[tokio::test]
        async fn it_should_fail_when_the_authentication_key_is_invalid() {
            // Arrange
            let http_core_tracker_services = initialize_private_tracker().await;
            let unregistered_key = authentication::Key::from_str("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();
            let maybe_key = Some(unregistered_key);

            // Act
            let actual_error = handle_announce(
                &http_core_tracker_services.announce_service,
                &sample_announce_request(),
                &sample_client_ip_sources(),
                &sample_http_service_binding(),
                maybe_key,
            )
            .await
            .unwrap_err();

            // Assert
            let actual_error_response = responses::error::Error::from(actual_error);

            assert_failure_reason_contains(
                &actual_error_response,
                "Tracker authentication error: Failed to read key: YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ",
            );
        }
    }

    mod with_tracker_in_listed_mode {

        use torrust_tracker_http_protocol::v1::responses;

        use super::{
            assert_failure_reason_contains, initialize_listed_tracker, sample_announce_request, sample_client_ip_sources,
            sample_http_service_binding,
        };
        use crate::v1::handlers::announce::handle_announce;

        #[tokio::test]
        async fn it_should_fail_when_the_announced_torrent_is_not_whitelisted() {
            // Arrange
            let http_core_tracker_services = initialize_listed_tracker().await;
            let announce_request = sample_announce_request();

            // Act
            let actual_error = handle_announce(
                &http_core_tracker_services.announce_service,
                &announce_request,
                &sample_client_ip_sources(),
                &sample_http_service_binding(),
                None,
            )
            .await
            .unwrap_err();

            // Assert
            let actual_error_response = responses::error::Error::from(actual_error);

            assert_failure_reason_contains(
                &actual_error_response,
                &format!(
                    "Tracker whitelist error: The torrent: {}, is not whitelisted",
                    announce_request.info_hash
                ),
            );
        }
    }

    mod with_tracker_on_reverse_proxy {

        use torrust_tracker_http_protocol::v1::responses;
        use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;

        use super::{
            assert_failure_reason_contains, initialize_tracker_on_reverse_proxy, sample_announce_request,
            sample_http_service_binding,
        };
        use crate::v1::handlers::announce::handle_announce;

        #[tokio::test]
        async fn it_should_fail_when_the_right_most_x_forwarded_for_header_ip_is_not_available() {
            // Arrange
            let http_core_tracker_services = initialize_tracker_on_reverse_proxy().await;
            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: None,
            };

            // Act
            let actual_error = handle_announce(
                &http_core_tracker_services.announce_service,
                &sample_announce_request(),
                &client_ip_sources,
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
        use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;

        use super::{
            assert_failure_reason_contains, initialize_tracker_not_on_reverse_proxy, sample_announce_request,
            sample_http_service_binding,
        };
        use crate::v1::handlers::announce::handle_announce;

        #[tokio::test]
        async fn it_should_fail_when_the_client_ip_from_the_connection_info_is_not_available() {
            // Arrange
            let http_core_tracker_services = initialize_tracker_not_on_reverse_proxy().await;
            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: None,
                connection_info_socket_address: None,
            };

            // Act
            let actual_error = handle_announce(
                &http_core_tracker_services.announce_service,
                &sample_announce_request(),
                &client_ip_sources,
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
