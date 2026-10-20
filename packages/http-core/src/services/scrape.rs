//! The `scrape` service.
//!
//! The service is responsible for handling the `scrape` requests.
//!
//! It delegates the `scrape` logic to the [`ScrapeHandler`] and it returns the
//! [`ScrapeData`].
//!
//! It also sends an [`Event`]
//! because events are specific for the HTTP tracker.
use std::sync::Arc;

use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_core::authentication::service::AuthenticationService;
use torrust_tracker_core::authentication::{self, Key};
use torrust_tracker_core::error::{ScrapeError, TrackerCoreError, WhitelistError};
use torrust_tracker_core::scrape_handler::ScrapeHandler;
use torrust_tracker_http_protocol::v1::requests::scrape::Scrape;
use torrust_tracker_http_protocol::v1::responses::error::Error as HttpProtocolErrorResponse;
use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{
    ClientIpSources, PeerIpResolutionError, RemoteClientAddr, ReverseProxyMode, resolve_remote_client_addr,
};
use torrust_tracker_primitives::{ConfigurationInstanceId, ScrapeData};

use crate::event::{ConnectionContext, Event};
use crate::services::error_mapping::protocol_error_from_tracker_core_error;

/// The HTTP tracker `scrape` service.
///
/// The service sends an statistics event that increments:
///
/// - The number of TCP `announce` requests handled by the HTTP tracker.
/// - The number of TCP `scrape` requests handled by the HTTP tracker.
///
/// # Errors
///
/// This function will return an error if:
///
/// - There is an error when resolving the client IP address.
pub struct ScrapeService {
    core_config: Arc<Core>,
    scrape_handler: Arc<ScrapeHandler>,
    authentication_service: Arc<AuthenticationService>,
    opt_http_stats_event_sender: crate::event::sender::Sender,
    reverse_proxy_mode: ReverseProxyMode,
    configuration_instance_id: ConfigurationInstanceId,
    public_url: Option<String>,
}

impl ScrapeService {
    #[must_use]
    pub fn new(
        core_config: Arc<Core>,
        scrape_handler: Arc<ScrapeHandler>,
        authentication_service: Arc<AuthenticationService>,
        opt_http_stats_event_sender: crate::event::sender::Sender,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Self {
        Self {
            core_config,
            scrape_handler,
            authentication_service,
            opt_http_stats_event_sender,
            reverse_proxy_mode: ReverseProxyMode::Disabled,
            configuration_instance_id,
            public_url: None,
        }
    }

    /// Creates a service using the network policy of one configured HTTP tracker instance.
    #[must_use]
    pub fn new_with_http_tracker_config(
        core_config: Arc<Core>,
        scrape_handler: Arc<ScrapeHandler>,
        authentication_service: Arc<AuthenticationService>,
        opt_http_stats_event_sender: crate::event::sender::Sender,
        http_tracker_config: &HttpTracker,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Self {
        Self {
            core_config,
            scrape_handler,
            authentication_service,
            opt_http_stats_event_sender,
            reverse_proxy_mode: http_tracker_config.network.on_reverse_proxy.into(),
            configuration_instance_id,
            public_url: http_tracker_config.public_url.as_ref().map(ToString::to_string),
        }
    }

    /// Handles a scrape request.
    ///
    /// When the peer is not authenticated and the tracker is running in `private`
    /// mode, the tracker returns empty stats for all the torrents.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// - There is an error when resolving the client IP address.
    pub async fn handle_scrape(
        &self,
        scrape_request: &Scrape,
        client_ip_sources: &ClientIpSources,
        server_service_binding: &ServiceBinding,
        maybe_key: Option<Key>,
    ) -> Result<ScrapeData, HttpScrapeError> {
        let scrape_data = if self.authentication_is_required() && !self.is_authenticated(maybe_key).await {
            ScrapeData::zeroed(&scrape_request.info_hashes)
        } else {
            self.scrape_handler.handle_scrape(&scrape_request.info_hashes).await?
        };

        let remote_client_addr = resolve_remote_client_addr(&self.reverse_proxy_mode, client_ip_sources)?;

        self.send_event(remote_client_addr, server_service_binding.clone()).await;

        Ok(scrape_data)
    }

    fn authentication_is_required(&self) -> bool {
        self.core_config.private
    }

    async fn is_authenticated(&self, maybe_key: Option<Key>) -> bool {
        if let Some(key) = maybe_key {
            return self.authentication_service.authenticate(&key).await.is_ok();
        }

        false
    }

    async fn send_event(&self, remote_client_addr: RemoteClientAddr, server_service_binding: ServiceBinding) {
        if let Some(http_stats_event_sender) = self.opt_http_stats_event_sender.as_deref() {
            let event = Event::TcpScrape {
                connection: ConnectionContext::new(self.configuration_instance_id, remote_client_addr, server_service_binding)
                    .with_public_url(self.public_url.clone()),
            };

            tracing::debug!("Sending TcpScrape event: {:?}", event);

            http_stats_event_sender.send(event).await;
        }
    }
}

/// Errors related to scrape requests.
///
/// This internal error type is not an event payload. A future rejected-request
/// event must use the stable, bounded, consumer-safe reason types defined by
/// the [general error-events EPIC](../../../../docs/issues/drafts/generalize-error-events/EPIC.md).
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum HttpScrapeError {
    #[error("Error resolving peer IP: {source}")]
    PeerIpResolutionError { source: PeerIpResolutionError },

    #[error("Tracker core error: {source}")]
    TrackerCoreError { source: TrackerCoreError },
}

impl From<PeerIpResolutionError> for HttpScrapeError {
    fn from(peer_ip_resolution_error: PeerIpResolutionError) -> Self {
        Self::PeerIpResolutionError {
            source: peer_ip_resolution_error,
        }
    }
}

impl From<TrackerCoreError> for HttpScrapeError {
    fn from(tracker_core_error: TrackerCoreError) -> Self {
        Self::TrackerCoreError {
            source: tracker_core_error,
        }
    }
}

impl From<ScrapeError> for HttpScrapeError {
    fn from(announce_error: ScrapeError) -> Self {
        Self::TrackerCoreError {
            source: announce_error.into(),
        }
    }
}

impl From<WhitelistError> for HttpScrapeError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::TrackerCoreError {
            source: whitelist_error.into(),
        }
    }
}

impl From<authentication::key::Error> for HttpScrapeError {
    fn from(whitelist_error: authentication::key::Error) -> Self {
        Self::TrackerCoreError {
            source: whitelist_error.into(),
        }
    }
}

impl From<HttpScrapeError> for HttpProtocolErrorResponse {
    fn from(error: HttpScrapeError) -> Self {
        match error {
            HttpScrapeError::PeerIpResolutionError { source } => source.into(),
            HttpScrapeError::TrackerCoreError { source } => protocol_error_from_tracker_core_error(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::future;
    use std::net::{IpAddr, SocketAddr};
    use std::panic::Location;
    use std::sync::Arc;

    use rstest::rstest;
    use torrust_info_hash::InfoHash;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
    use torrust_tracker_configuration::v3_0_0::public_url::HttpUrl;
    use torrust_tracker_core::announce_handler::{AnnounceHandler, PeersWanted};
    use torrust_tracker_core::authentication::Key;
    use torrust_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
    use torrust_tracker_core::authentication::key::{Error as AuthenticationKeyError, generate_key};
    use torrust_tracker_core::authentication::service::AuthenticationService;
    use torrust_tracker_core::databases::error::Error as DatabaseError;
    use torrust_tracker_core::databases::{MockTorrentMetricsStore, TorrentMetricsStore};
    use torrust_tracker_core::error::{ScrapeError, TrackerCoreError, WhitelistError};
    use torrust_tracker_core::scrape_handler::ScrapeHandler;
    use torrust_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_http_protocol::v1::requests::scrape::Scrape;
    use torrust_tracker_http_protocol::v1::responses::error::Error as HttpProtocolErrorResponse;
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{
        ClientIpSources, PeerIpResolutionError, RemoteClientAddr, ResolvedIp,
    };
    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
    use torrust_tracker_primitives::{ConfigurationInstanceId, Driver, ScrapeData, ServiceRole};
    use torrust_tracker_test_helpers::configuration;

    use crate::event;
    use crate::event::{ConnectionContext, Event};
    use crate::services::error_mapping::protocol_error_from_tracker_core_error;
    use crate::services::scrape::{HttpScrapeError, ScrapeService};
    use crate::tests::{RecordingSender, sample_info_hash, sample_peer};

    /// The port of the TCP connection that the scrape arrives on.
    const CONNECTION_PORT: u16 = 54_321;

    /// The collaborators of one scrape scenario. It builds only what the service
    /// needs: no database, statistics repository or event listener.
    struct Fixture {
        core_config: Arc<Core>,
        announce_handler: Arc<AnnounceHandler>,
        scrape_handler: Arc<ScrapeHandler>,
        authentication_service: Arc<AuthenticationService>,
        in_memory_key_repository: Arc<InMemoryKeyRepository>,
        configuration_instance_id: ConfigurationInstanceId,
        http_tracker_config: HttpTracker,
    }

    impl Fixture {
        fn new(configuration: &Configuration) -> Self {
            Self::build(configuration, None)
        }

        /// A tracker that reads the persisted download counts from `store`.
        fn with_persisted_downloads_from(configuration: &Configuration, store: MockTorrentMetricsStore) -> Self {
            Self::build(configuration, Some(store))
        }

        fn build(configuration: &Configuration, persisted_downloads_store: Option<MockTorrentMetricsStore>) -> Self {
            let core_config = Arc::new(configuration.core.clone());
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
            let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
            let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&configuration.core, &in_memory_whitelist));
            let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
            let authentication_service = Arc::new(AuthenticationService::new(&configuration.core, &in_memory_key_repository));
            let announce_handler = Arc::new(AnnounceHandler::new_public(
                &configuration.core,
                &whitelist_authorization,
                &in_memory_torrent_repository,
            ));
            let scrape_handler = Arc::new(persisted_downloads_store.map_or_else(
                || ScrapeHandler::new_public(&whitelist_authorization, &in_memory_torrent_repository),
                |store| {
                    let store: Arc<dyn TorrentMetricsStore> = Arc::new(store);
                    let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&store));
                    ScrapeHandler::new_with_persistent_completed_statistics(
                        &whitelist_authorization,
                        &in_memory_torrent_repository,
                        &db_downloads_metric_repository,
                    )
                },
            ));
            let http_tracker_config = configuration
                .http_trackers
                .as_deref()
                .and_then(<[HttpTracker]>::first)
                .expect("the test configuration should contain an HTTP tracker")
                .clone();

            Self {
                core_config,
                announce_handler,
                scrape_handler,
                authentication_service,
                in_memory_key_repository,
                configuration_instance_id: ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
                http_tracker_config,
            }
        }

        /// Registers a permanent key and returns it.
        async fn register_a_key(&self) -> Key {
            let peer_key = generate_key(None);
            let key = peer_key.key();
            self.in_memory_key_repository.reset_with(vec![peer_key]).await;
            key
        }

        /// Announces the sample peer, a seeder, for the sample torrent.
        async fn announce_a_seeder(&self) {
            let mut peer = sample_peer();
            let peer_ip = peer.ip();
            self.announce_handler
                .handle_announcement(&sample_info_hash(), &mut peer, &peer_ip, None, &PeersWanted::AsManyAsPossible)
                .await
                .unwrap();
        }

        /// A service built with `ScrapeService::new`: no reverse proxy and no public URL.
        fn service(&self, sender: event::sender::Sender) -> ScrapeService {
            ScrapeService::new(
                self.core_config.clone(),
                self.scrape_handler.clone(),
                self.authentication_service.clone(),
                sender,
                self.configuration_instance_id,
            )
        }

        /// A service that takes its reverse-proxy mode and public URL from the configured HTTP tracker.
        fn service_for_http_tracker(&self, sender: event::sender::Sender) -> ScrapeService {
            ScrapeService::new_with_http_tracker_config(
                self.core_config.clone(),
                self.scrape_handler.clone(),
                self.authentication_service.clone(),
                sender,
                &self.http_tracker_config,
                self.configuration_instance_id,
            )
        }
    }

    fn ip(text: &str) -> IpAddr {
        text.parse().expect("the test IP address should be valid")
    }

    fn localhost() -> IpAddr {
        ip("127.0.0.1")
    }

    /// A torrent that nobody announced and that is not persisted.
    fn unknown_info_hash() -> InfoHash {
        "9c38422213e30bff212b30c360d26f9a02136422" // DevSkim: ignore DS173237
            .parse()
            .expect("the test info hash should be valid")
    }

    fn scrape_request_for(info_hashes: Vec<InfoHash>) -> Scrape {
        Scrape { info_hashes }
    }

    /// Client IP sources for a direct connection from `connection_ip`.
    fn direct_connection_from(connection_ip: IpAddr) -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: Some(SocketAddr::new(connection_ip, CONNECTION_PORT)),
        }
    }

    /// Client IP sources that name no address, so resolving the client IP fails.
    const fn unresolvable_client_ip_sources() -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: None,
        }
    }

    fn sample_server_service_binding() -> ServiceBinding {
        ServiceBinding::new(Protocol::HTTP, SocketAddr::new(localhost(), 7070)).unwrap()
    }

    /// The metadata of the sample torrent after one seeder announced.
    const fn one_seeder() -> SwarmMetadata {
        SwarmMetadata {
            complete: 1,
            downloaded: 0,
            incomplete: 0,
        }
    }

    const fn nothing() -> SwarmMetadata {
        SwarmMetadata {
            complete: 0,
            downloaded: 0,
            incomplete: 0,
        }
    }

    fn scrape_data_of(files: Vec<(InfoHash, SwarmMetadata)>) -> ScrapeData {
        let mut scrape_data = ScrapeData::empty();
        for (info_hash, metadata) in files {
            scrape_data.add_file(&info_hash, metadata);
        }
        scrape_data
    }

    fn expected_connection(fixture: &Fixture, remote_client_ip: ResolvedIp) -> ConnectionContext {
        ConnectionContext::new(
            fixture.configuration_instance_id,
            RemoteClientAddr::new(remote_client_ip, Some(CONNECTION_PORT)),
            sample_server_service_binding(),
        )
    }

    fn corrupt_record() -> DatabaseError {
        DatabaseError::MalformedDatabaseRecord {
            message: "corrupt record".to_string(),
            driver: Driver::Sqlite3,
        }
    }

    /// A persisted-downloads store whose lookup always fails.
    fn failing_persisted_downloads_store() -> MockTorrentMetricsStore {
        let mut store = MockTorrentMetricsStore::new();
        store
            .expect_load_torrents_downloads()
            .returning(|_| Box::pin(future::ready(Err(corrupt_record()))));
        store
    }

    #[tokio::test]
    async fn it_should_return_the_scrape_data_for_a_torrent() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_public());
        fixture.announce_a_seeder().await;
        let service = fixture.service(None);

        // Act
        let scrape_data = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(scrape_data, scrape_data_of(vec![(sample_info_hash(), one_seeder())]));
    }

    #[tokio::test]
    async fn it_should_return_the_persisted_downloads_of_a_torrent_absent_from_memory() {
        // Arrange
        let mut store = MockTorrentMetricsStore::new();
        store
            .expect_load_torrents_downloads()
            .returning(|_| Box::pin(future::ready(Ok(BTreeMap::from([(sample_info_hash(), 7)])))));
        let fixture = Fixture::with_persisted_downloads_from(&configuration::ephemeral_public(), store);
        let service = fixture.service(None);

        // Act
        let scrape_data = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await
            .unwrap();

        // Assert: the persisted downloads (7) are reported with no peers.
        let expected_metadata = SwarmMetadata {
            complete: 0,
            downloaded: 7,
            incomplete: 0,
        };
        assert_eq!(scrape_data, scrape_data_of(vec![(sample_info_hash(), expected_metadata)]));
    }

    #[tokio::test]
    async fn it_should_zero_every_requested_torrent_for_an_unauthenticated_peer_on_a_private_tracker() {
        // Arrange: one requested torrent has a swarm and the other is unknown.
        let fixture = Fixture::new(&configuration::ephemeral_private());
        fixture.announce_a_seeder().await;
        let service = fixture.service(None);
        let request = scrape_request_for(vec![sample_info_hash(), unknown_info_hash()]);

        // Act
        let scrape_data = service
            .handle_scrape(
                &request,
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await
            .unwrap();

        // Assert
        let expected = scrape_data_of(vec![(sample_info_hash(), nothing()), (unknown_info_hash(), nothing())]);
        assert_eq!(scrape_data, expected);
    }

    #[tokio::test]
    async fn it_should_zero_the_scrape_data_for_an_unregistered_key_on_a_private_tracker() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_private());
        fixture.announce_a_seeder().await;
        let service = fixture.service(None);
        let unregistered_key = Key::random();

        // Act
        let scrape_data = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                Some(unregistered_key),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(scrape_data, scrape_data_of(vec![(sample_info_hash(), nothing())]));
    }

    #[tokio::test]
    async fn it_should_return_the_scrape_data_for_a_registered_key_on_a_private_tracker() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_private());
        fixture.announce_a_seeder().await;
        let registered_key = fixture.register_a_key().await;
        let service = fixture.service(None);

        // Act
        let scrape_data = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                Some(registered_key),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(scrape_data, scrape_data_of(vec![(sample_info_hash(), one_seeder())]));
    }

    #[rstest]
    #[case::ipv4_client(ip("126.0.0.1"))]
    #[case::ipv6_client(ip("6969:6969:6969:6969:6969:6969:6969:6969"))]
    #[tokio::test]
    async fn it_should_publish_the_connection_address_of_the_client_in_the_scrape_event(#[case] connection_ip: IpAddr) {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_public());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(connection_ip),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the scrape should succeed: {result:?}");
        let expected_event = Event::TcpScrape {
            connection: expected_connection(&fixture, ResolvedIp::FromSocketAddr(connection_ip)),
        };
        assert_eq!(sender.events(), vec![expected_event]);
    }

    #[tokio::test]
    async fn it_should_publish_the_scrape_event_for_an_unauthenticated_peer_on_a_private_tracker() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_private());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the scrape should succeed: {result:?}");
        let expected_event = Event::TcpScrape {
            connection: expected_connection(&fixture, ResolvedIp::FromSocketAddr(localhost())),
        };
        assert_eq!(sender.events(), vec![expected_event]);
    }

    #[tokio::test]
    async fn it_should_ignore_the_forwarded_address_when_the_http_tracker_is_not_behind_a_reverse_proxy() {
        // Arrange
        let connection_ip = ip("192.0.2.10");
        let fixture = Fixture::new(&configuration::ephemeral_public());
        let sender = RecordingSender::new();
        let service = fixture.service_for_http_tracker(Some(sender.clone()));
        let client_ip_sources = ClientIpSources {
            right_most_x_forwarded_for: Some(ip("203.0.113.195")),
            connection_info_socket_address: Some(SocketAddr::new(connection_ip, CONNECTION_PORT)),
        };

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &client_ip_sources,
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the scrape should succeed: {result:?}");
        let expected_event = Event::TcpScrape {
            connection: expected_connection(&fixture, ResolvedIp::FromSocketAddr(connection_ip)),
        };
        assert_eq!(sender.events(), vec![expected_event]);
    }

    #[tokio::test]
    async fn it_should_use_the_http_tracker_reverse_proxy_policy() {
        // Arrange
        let proxy_ip = ip("203.0.113.195");
        let fixture = Fixture::new(&configuration::ephemeral_with_reverse_proxy());
        let sender = RecordingSender::new();
        let service = fixture.service_for_http_tracker(Some(sender.clone()));
        let client_ip_sources = ClientIpSources {
            right_most_x_forwarded_for: Some(proxy_ip),
            connection_info_socket_address: Some(SocketAddr::new(ip("192.0.2.10"), CONNECTION_PORT)),
        };

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &client_ip_sources,
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the scrape should succeed: {result:?}");
        let expected_event = Event::TcpScrape {
            connection: expected_connection(&fixture, ResolvedIp::FromXForwardedFor(proxy_ip)),
        };
        assert_eq!(sender.events(), vec![expected_event]);
    }

    #[tokio::test]
    async fn it_should_publish_the_public_url_of_the_http_tracker_in_the_scrape_event() {
        // Arrange
        let mut configuration = configuration::ephemeral_public();
        configuration
            .http_trackers
            .as_mut()
            .expect("the test configuration should contain an HTTP tracker")[0]
            .public_url = Some(HttpUrl::parse("https://tracker.example.com/").unwrap());
        let fixture = Fixture::new(&configuration);
        let sender = RecordingSender::new();
        let service = fixture.service_for_http_tracker(Some(sender.clone()));

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the scrape should succeed: {result:?}");
        let expected_event = Event::TcpScrape {
            connection: expected_connection(&fixture, ResolvedIp::FromSocketAddr(localhost()))
                .with_public_url(Some("https://tracker.example.com/".to_string())),
        };
        assert_eq!(sender.events(), vec![expected_event]);
    }

    #[tokio::test]
    async fn it_should_scrape_without_publishing_when_there_is_no_event_sender() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_public());
        fixture.announce_a_seeder().await;
        let service = fixture.service(None);

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert_eq!(result.unwrap(), scrape_data_of(vec![(sample_info_hash(), one_seeder())]));
    }

    #[tokio::test]
    async fn it_should_report_a_failing_persisted_downloads_lookup_as_a_tracker_core_scrape_error() {
        // Arrange
        let fixture =
            Fixture::with_persisted_downloads_from(&configuration::ephemeral_public(), failing_persisted_downloads_store());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(
            matches!(
                result,
                Err(HttpScrapeError::TrackerCoreError {
                    source: TrackerCoreError::ScrapeError {
                        source: ScrapeError::Database(_)
                    }
                })
            ),
            "unexpected result: {result:?}"
        );
        assert!(sender.events().is_empty(), "no event should be published");
    }

    #[rstest]
    #[case::without_a_reverse_proxy(configuration::ephemeral_public())]
    #[case::behind_a_reverse_proxy(configuration::ephemeral_with_reverse_proxy())]
    #[case::on_a_private_tracker_without_a_key(configuration::ephemeral_private())]
    #[tokio::test]
    async fn it_should_report_an_unresolvable_client_ip_without_publishing_an_event(#[case] configuration: Configuration) {
        // Arrange
        let fixture = Fixture::new(&configuration);
        let sender = RecordingSender::new();
        let service = fixture.service_for_http_tracker(Some(sender.clone()));

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &unresolvable_client_ip_sources(),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(
            matches!(result, Err(HttpScrapeError::PeerIpResolutionError { .. })),
            "unexpected result: {result:?}"
        );
        assert!(sender.events().is_empty(), "no event should be published");
    }

    #[tokio::test]
    async fn it_should_scrape_before_resolving_the_client_ip() {
        // Arrange: the persisted-downloads lookup fails and the client IP cannot be resolved.
        let fixture =
            Fixture::with_persisted_downloads_from(&configuration::ephemeral_public(), failing_persisted_downloads_store());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_scrape(
                &scrape_request_for(vec![sample_info_hash()]),
                &unresolvable_client_ip_sources(),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(
            matches!(result, Err(HttpScrapeError::TrackerCoreError { .. })),
            "unexpected result: {result:?}"
        );
        assert!(sender.events().is_empty(), "no event should be published");
    }

    #[test]
    fn it_should_wrap_a_whitelist_error_in_the_tracker_core_variant() {
        // Arrange
        let whitelist_error = WhitelistError::TorrentNotWhitelisted {
            info_hash: sample_info_hash(),
            location: Location::caller(),
        };

        // Act
        let error = HttpScrapeError::from(whitelist_error);

        // Assert
        assert!(
            matches!(
                error,
                HttpScrapeError::TrackerCoreError {
                    source: TrackerCoreError::WhitelistError { .. }
                }
            ),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn it_should_wrap_an_authentication_error_in_the_tracker_core_variant() {
        // Arrange
        let authentication_error = AuthenticationKeyError::MissingAuthKey {
            location: Location::caller(),
        };

        // Act
        let error = HttpScrapeError::from(authentication_error);

        // Assert
        assert!(
            matches!(
                error,
                HttpScrapeError::TrackerCoreError {
                    source: TrackerCoreError::AuthenticationError { .. }
                }
            ),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn it_should_wrap_a_tracker_core_error_without_changing_it() {
        // Arrange
        let tracker_core_error = TrackerCoreError::from(ScrapeError::from(corrupt_record()));

        // Act
        let error = HttpScrapeError::from(tracker_core_error);

        // Assert
        assert!(
            matches!(
                error,
                HttpScrapeError::TrackerCoreError {
                    source: TrackerCoreError::ScrapeError { .. }
                }
            ),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn it_should_report_a_peer_ip_resolution_error_with_the_protocol_failure_reason() {
        // Arrange
        let resolution_error = PeerIpResolutionError::MissingClientIp {
            location: Location::caller(),
        };
        let protocol_response = HttpProtocolErrorResponse::from(resolution_error.clone());

        // Act
        let response = HttpProtocolErrorResponse::from(HttpScrapeError::from(resolution_error));

        // Assert
        assert_eq!(response.failure_reason, protocol_response.failure_reason);
    }

    #[test]
    fn it_should_report_a_tracker_core_error_with_the_tracker_core_failure_reason() {
        // Arrange
        let tracker_core_error = TrackerCoreError::from(ScrapeError::from(corrupt_record()));
        let tracker_core_response = protocol_error_from_tracker_core_error(tracker_core_error.clone());

        // Act
        let response = HttpProtocolErrorResponse::from(HttpScrapeError::from(tracker_core_error));

        // Assert
        assert_eq!(response.failure_reason, tracker_core_response.failure_reason);
    }
}
