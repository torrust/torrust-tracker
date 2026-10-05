//! The `scrape` service.
//!
//! The service is responsible for handling the `scrape` requests.
//!
//! It delegates the `scrape` logic to the [`ScrapeHandler`] and it returns the
//! [`ScrapeData`].
//!
//! It also sends an [`Event`]
//! because events are specific for the UDP tracker.
use std::net::SocketAddr;
use std::ops::Range;
use std::sync::Arc;

use torrust_info_hash::InfoHash;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_core::error::{ScrapeError, WhitelistError};
use torrust_tracker_core::scrape_handler::ScrapeHandler;
use torrust_tracker_primitives::{ConfigurationInstanceId, ScrapeData};
use torrust_tracker_udp_protocol::ScrapeRequest;

use crate::connection_cookie::{ConnectionCookieError, check, gen_remote_fingerprint};
use crate::crypto::cookie_cipher::CookieCipher;
use crate::event::{ConnectionContext, Event};

/// The `ScrapeService` is responsible for handling the `scrape` requests.
///
/// The service sends an statistics event that increments:
///
/// - The number of UDP `scrape` requests handled by the UDP tracker.
pub struct ScrapeService {
    scrape_handler: Arc<ScrapeHandler>,
    cookie_cipher: Arc<CookieCipher>,
    opt_udp_stats_event_sender: crate::event::sender::Sender,
    configuration_instance_id: ConfigurationInstanceId,
    public_url: Option<String>,
}

impl ScrapeService {
    #[must_use]
    pub const fn configuration_instance_id(&self) -> ConfigurationInstanceId {
        self.configuration_instance_id
    }

    /// Creates the service. `cookie_cipher` must be the composition root's
    /// shared key, the one the connect service issues connection IDs with.
    #[must_use]
    pub fn new(
        scrape_handler: Arc<ScrapeHandler>,
        cookie_cipher: Arc<CookieCipher>,
        opt_udp_stats_event_sender: crate::event::sender::Sender,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Self {
        Self {
            scrape_handler,
            cookie_cipher,
            opt_udp_stats_event_sender,
            configuration_instance_id,
            public_url: None,
        }
    }

    #[must_use]
    pub fn with_public_url(mut self, public_url: Option<String>) -> Self {
        self.public_url = public_url;
        self
    }

    #[must_use]
    pub fn public_url(&self) -> Option<&str> {
        self.public_url.as_deref()
    }

    /// It handles the `Scrape` request.
    ///
    /// # Errors
    ///
    /// It will return an error if cookie validation fails and `validate_cookie`
    /// is `true`, or if the tracker core scrape handler returns an error.
    ///
    /// When `validate_cookie` is `false` the connection ID is not validated.
    /// The caller is responsible for any metric or event emission related to
    /// the skipped validation.
    pub async fn handle_scrape(
        &self,
        client_socket_addr: SocketAddr,
        server_service_binding: ServiceBinding,
        request: &ScrapeRequest,
        cookie_valid_range: Range<f64>,
        validate_cookie: bool,
    ) -> Result<ScrapeData, UdpScrapeError> {
        if validate_cookie {
            self.authenticate(client_socket_addr, request, cookie_valid_range)?;
        }

        let scrape_data = self
            .scrape_handler
            .handle_scrape(&Self::convert_from_wire_info_hashes(&request.info_hashes))
            .await?;

        self.send_event(client_socket_addr, server_service_binding).await;

        Ok(scrape_data)
    }

    /// Checks the request's connection ID for the client address, without
    /// handling the request.
    ///
    /// Callers that skip validation in [`Self::handle_scrape`] use it to
    /// observe invalid connection IDs.
    ///
    /// # Errors
    ///
    /// It returns an error if the connection ID was not issued to this client
    /// address within `cookie_valid_range`.
    pub fn authenticate(
        &self,
        remote_addr: SocketAddr,
        request: &ScrapeRequest,
        cookie_valid_range: Range<f64>,
    ) -> Result<f64, ConnectionCookieError> {
        check(
            &self.cookie_cipher,
            &request.connection_id,
            gen_remote_fingerprint(&remote_addr),
            cookie_valid_range,
        )
    }

    fn convert_from_wire_info_hashes(wire_info_hashes: &[torrust_tracker_udp_protocol::common::InfoHash]) -> Vec<InfoHash> {
        wire_info_hashes.iter().map(|&x| InfoHash::from(x.0)).collect()
    }

    async fn send_event(&self, client_socket_addr: SocketAddr, server_service_binding: ServiceBinding) {
        if let Some(udp_stats_event_sender) = self.opt_udp_stats_event_sender.as_deref() {
            let event = Event::UdpScrape {
                connection: ConnectionContext::new(self.configuration_instance_id, client_socket_addr, server_service_binding)
                    .with_public_url(self.public_url.clone()),
            };

            tracing::debug!(target = crate::UDP_TRACKER_LOG_TARGET, "Sending UdpScrape event: {event:?}");

            udp_stats_event_sender.send(event).await;
        }
    }
}

/// Errors related to scrape requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum UdpScrapeError {
    /// Error returned when there was an error with the connection cookie.
    #[error("Connection cookie error: {source}")]
    ConnectionCookieError { source: ConnectionCookieError },

    /// Error returned when there was an error with the tracker core scrape handler.
    #[error("Tracker core scrape error: {source}")]
    TrackerCoreScrapeError { source: ScrapeError },

    /// Error returned when there was an error with the tracker core whitelist.
    #[error("Tracker core whitelist error: {source}")]
    TrackerCoreWhitelistError { source: WhitelistError },
}

impl From<ConnectionCookieError> for UdpScrapeError {
    fn from(connection_cookie_error: ConnectionCookieError) -> Self {
        Self::ConnectionCookieError {
            source: connection_cookie_error,
        }
    }
}

impl From<ScrapeError> for UdpScrapeError {
    fn from(scrape_error: ScrapeError) -> Self {
        Self::TrackerCoreScrapeError { source: scrape_error }
    }
}

impl From<WhitelistError> for UdpScrapeError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::TrackerCoreWhitelistError { source: whitelist_error }
    }
}

#[cfg(test)]
mod tests {
    use std::future;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::ops::Range;
    use std::sync::Arc;

    use mockall::predicate::eq;
    use torrust_info_hash::InfoHash;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_core::databases::error::Error as DatabaseError;
    use torrust_tracker_core::databases::{MockTorrentMetricsStore, TorrentMetricsStore};
    use torrust_tracker_core::error::ScrapeError;
    use torrust_tracker_core::scrape_handler::ScrapeHandler;
    use torrust_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_primitives::peer::fixture::PeerBuilder;
    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
    use torrust_tracker_primitives::{ConfigurationInstanceId, Driver, ServiceRole};
    use torrust_tracker_udp_protocol::{ConnectionId, ScrapeRequest, TransactionId};

    use super::{ScrapeService, UdpScrapeError};
    use crate::connection_cookie::{ConnectionCookieError, gen_remote_fingerprint, make};
    use crate::crypto::cookie_cipher::CookieCipher;
    use crate::event::{ConnectionContext, Event};
    use crate::services::tests::MockUdpCoreStatsEventSender;

    const INSTANCE_ID: ConfigurationInstanceId = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
    const VALIDATE_COOKIE: bool = true;
    const SKIP_COOKIE_VALIDATION: bool = false;
    const COOKIE_VALID_FROM: f64 = 1_000_000_000.0;
    const COOKIE_VALID_UNTIL: f64 = 1_000_000_120.0;
    const INFO_HASH: [u8; 20] = [0x11; 20];
    const PUBLIC_URL: &str = "udp://tracker.example.test:6969/announce";

    /// The service under test plus the torrent repository its scrape handler reads from.
    struct ScrapeServiceFixture {
        service: ScrapeService,
        torrents: Arc<InMemoryTorrentRepository>,
    }

    impl ScrapeServiceFixture {
        fn public_tracker() -> Self {
            Self::new(None, None)
        }

        fn tracker_with_persisted_downloads_from(store: MockTorrentMetricsStore) -> Self {
            Self::new(Some(store), None)
        }

        fn public_tracker_publishing_events_to(event_sender: MockUdpCoreStatsEventSender) -> Self {
            Self::new(None, Some(Arc::new(event_sender)))
        }

        fn new(persisted_downloads_store: Option<MockTorrentMetricsStore>, event_sender: crate::event::sender::Sender) -> Self {
            let whitelist_authorization = Arc::new(WhitelistAuthorization::new(
                &Core::default(),
                &Arc::new(InMemoryWhitelist::default()),
            ));
            let torrents = Arc::new(InMemoryTorrentRepository::default());

            let scrape_handler = persisted_downloads_store.map_or_else(
                || ScrapeHandler::new_public(&whitelist_authorization, &torrents),
                |store| {
                    let store: Arc<dyn TorrentMetricsStore> = Arc::new(store);
                    ScrapeHandler::new_with_persistent_completed_statistics(
                        &whitelist_authorization,
                        &torrents,
                        &Arc::new(DatabaseDownloadsMetricRepository::new(&store)),
                    )
                },
            );

            let service = ScrapeService::new(
                Arc::new(scrape_handler),
                Arc::new(fixed_test_cipher()),
                event_sender,
                INSTANCE_ID,
            );

            Self { service, torrents }
        }
    }

    fn client_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 10)), 51_000)
    }

    fn server_service_binding() -> ServiceBinding {
        ServiceBinding::new(
            Protocol::UDP,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
        )
        .unwrap()
    }

    fn cookie_valid_range() -> Range<f64> {
        COOKIE_VALID_FROM..COOKIE_VALID_UNTIL
    }

    fn fixed_test_cipher() -> CookieCipher {
        CookieCipher::fixed_for_testing()
    }

    fn valid_connection_id_for(client: SocketAddr) -> ConnectionId {
        make(
            &fixed_test_cipher(),
            gen_remote_fingerprint(&client),
            COOKIE_VALID_FROM + 60.0,
        )
        .unwrap()
    }

    fn expired_connection_id_for(client: SocketAddr) -> ConnectionId {
        make(&fixed_test_cipher(), gen_remote_fingerprint(&client), COOKIE_VALID_FROM - 1.0).unwrap()
    }

    fn scrape_request(connection_id: ConnectionId) -> ScrapeRequest {
        ScrapeRequest {
            connection_id,
            transaction_id: TransactionId(0i32.into()),
            info_hashes: vec![torrust_tracker_udp_protocol::InfoHash(INFO_HASH)],
        }
    }

    #[tokio::test]
    async fn it_should_reject_an_expired_connection_id_when_cookie_validation_is_enabled() {
        // Arrange
        let fixture = ScrapeServiceFixture::public_tracker();
        let request = scrape_request(expired_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_scrape(
                client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                VALIDATE_COOKIE,
            )
            .await;

        // Assert
        assert!(matches!(
            result,
            Err(UdpScrapeError::ConnectionCookieError {
                source: ConnectionCookieError::ValueExpired { .. }
            })
        ));
    }

    #[tokio::test]
    async fn it_should_accept_an_expired_connection_id_when_cookie_validation_is_skipped() {
        // Arrange
        let fixture = ScrapeServiceFixture::public_tracker();
        let request = scrape_request(expired_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_scrape(
                client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                SKIP_COOKIE_VALIDATION,
            )
            .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn it_should_validate_the_connection_id_before_reading_any_persisted_downloads() {
        // Arrange
        let store_that_fails_the_test_if_read = MockTorrentMetricsStore::new();
        let fixture = ScrapeServiceFixture::tracker_with_persisted_downloads_from(store_that_fails_the_test_if_read);
        let request = scrape_request(expired_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_scrape(
                client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                VALIDATE_COOKIE,
            )
            .await;

        // Assert
        assert!(matches!(result, Err(UdpScrapeError::ConnectionCookieError { .. })));
    }

    #[tokio::test]
    async fn it_should_return_the_swarm_statistics_under_the_requested_info_hash() {
        // Arrange
        let fixture = ScrapeServiceFixture::public_tracker();
        fixture
            .torrents
            .handle_announcement(&InfoHash::from(INFO_HASH), &PeerBuilder::seeder().build(), None)
            .await;
        let request = scrape_request(valid_connection_id_for(client_socket_addr()));

        // Act
        let scrape_data = fixture
            .service
            .handle_scrape(
                client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                VALIDATE_COOKIE,
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(
            scrape_data.files.get(&InfoHash::from(INFO_HASH)),
            Some(&SwarmMetadata::new(0, 1, 0))
        );
    }

    #[tokio::test]
    async fn it_should_report_a_failing_persisted_downloads_lookup_as_a_tracker_core_scrape_error() {
        // Arrange
        let mut failing_store = MockTorrentMetricsStore::new();
        failing_store.expect_load_torrents_downloads().returning(|_| {
            Box::pin(future::ready(Err(DatabaseError::MalformedDatabaseRecord {
                message: "corrupt record".to_string(),
                driver: Driver::Sqlite3,
            })))
        });
        let fixture = ScrapeServiceFixture::tracker_with_persisted_downloads_from(failing_store);
        let request = scrape_request(valid_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_scrape(
                client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                VALIDATE_COOKIE,
            )
            .await;

        // Assert
        assert!(matches!(
            result,
            Err(UdpScrapeError::TrackerCoreScrapeError {
                source: ScrapeError::Database(_)
            })
        ));
    }

    #[tokio::test]
    async fn it_should_publish_a_scrape_event_with_the_connection_context() {
        // Arrange
        let expected_event = Event::UdpScrape {
            connection: ConnectionContext::new(INSTANCE_ID, client_socket_addr(), server_service_binding())
                .with_public_url(Some(PUBLIC_URL.to_string())),
        };
        let mut event_sender = MockUdpCoreStatsEventSender::new();
        event_sender
            .expect_send()
            .with(eq(expected_event))
            .times(1)
            .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
        let service = ScrapeServiceFixture::public_tracker_publishing_events_to(event_sender)
            .service
            .with_public_url(Some(PUBLIC_URL.to_string()));
        let request = scrape_request(valid_connection_id_for(client_socket_addr()));

        // Act
        let result = service
            .handle_scrape(
                client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                VALIDATE_COOKIE,
            )
            .await;

        // Assert
        assert!(result.is_ok());
    }
}
