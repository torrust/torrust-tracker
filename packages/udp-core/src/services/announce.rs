//! The `announce` service.
//!
//! The service is responsible for handling the `announce` requests.
//!
//! It delegates the `announce` logic to the [`AnnounceHandler`] and it returns
//! the [`AnnounceData`].
//!
//! It also sends an [`Event`]
//! because events are specific for the HTTP tracker.
use std::net::{IpAddr, SocketAddr};
use std::ops::Range;
use std::sync::Arc;

use torrust_info_hash::InfoHash;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_core::announce_handler::{AnnounceHandler, PeersWanted};
use torrust_tracker_core::error::{AnnounceError, WhitelistError};
use torrust_tracker_core::whitelist;
use torrust_tracker_primitives::peer::PeerAnnouncement;
use torrust_tracker_primitives::{AnnounceData, ConfigurationInstanceId};
use torrust_tracker_udp_protocol::AnnounceRequest;

use crate::connection_cookie::{ConnectionCookieError, check, gen_remote_fingerprint};
use crate::crypto::cookie_cipher::CookieCipher;
use crate::event::{ConnectionContext, Event};
use crate::peer_builder;

/// The `AnnounceService` is responsible for handling the `announce` requests.
///
/// The service sends an statistics event that increments:
///
/// - The number of UDP `announce` requests handled by the UDP tracker.
pub struct AnnounceService {
    announce_handler: Arc<AnnounceHandler>,
    whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    cookie_cipher: Arc<CookieCipher>,
    opt_udp_core_stats_event_sender: crate::event::sender::Sender,
    configuration_instance_id: ConfigurationInstanceId,
    tracker_external_ip: Option<IpAddr>,
    public_url: Option<String>,
}

impl AnnounceService {
    #[must_use]
    pub const fn configuration_instance_id(&self) -> ConfigurationInstanceId {
        self.configuration_instance_id
    }

    /// Creates the service. `cookie_cipher` must be the composition root's
    /// shared key, the one the connect service issues connection IDs with.
    #[must_use]
    pub fn new(
        announce_handler: Arc<AnnounceHandler>,
        whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
        cookie_cipher: Arc<CookieCipher>,
        opt_udp_core_stats_event_sender: crate::event::sender::Sender,
        configuration_instance_id: ConfigurationInstanceId,
        tracker_external_ip: Option<IpAddr>,
    ) -> Self {
        Self {
            announce_handler,
            whitelist_authorization,
            cookie_cipher,
            opt_udp_core_stats_event_sender,
            configuration_instance_id,
            tracker_external_ip,
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

    /// It handles the `Announce` request.
    ///
    /// # Errors
    ///
    /// It will return an error if:
    ///
    /// - Cookie validation fails and `validate_cookie` is `true`.
    /// - The tracker is running in listed mode and the torrent is not in the
    ///   whitelist.
    ///
    /// When `validate_cookie` is `false` the connection ID is not validated.
    /// The caller is responsible for any metric or event emission related to
    /// the skipped validation.
    pub async fn handle_announce(
        &self,
        client_socket_addr: SocketAddr,
        server_service_binding: ServiceBinding,
        request: &AnnounceRequest,
        cookie_valid_range: Range<f64>,
        validate_cookie: bool,
    ) -> Result<AnnounceData, UdpAnnounceError> {
        if validate_cookie {
            self.authenticate(client_socket_addr, request, cookie_valid_range)?;
        }

        let info_hash = InfoHash::from(request.info_hash.0);

        self.authorize(&info_hash).await?;

        let remote_client_ip = client_socket_addr.ip();

        let mut peer = peer_builder::from_request(request, &remote_client_ip);

        let peers_wanted = PeersWanted::from_client_request(i32::from(request.peers_wanted.0));

        let announce_data = self
            .announce_handler
            .handle_announcement(
                &info_hash,
                &mut peer,
                &remote_client_ip,
                self.tracker_external_ip,
                &peers_wanted,
            )
            .await?;

        self.send_event(info_hash, peer, client_socket_addr, server_service_binding)
            .await;

        Ok(announce_data)
    }

    /// Checks the request's connection ID for the client address, without
    /// handling the request.
    ///
    /// Callers that skip validation in [`Self::handle_announce`] use it to
    /// observe invalid connection IDs.
    ///
    /// # Errors
    ///
    /// It returns an error if the connection ID was not issued to this client
    /// address within `cookie_valid_range`.
    pub fn authenticate(
        &self,
        remote_addr: SocketAddr,
        request: &AnnounceRequest,
        cookie_valid_range: Range<f64>,
    ) -> Result<f64, ConnectionCookieError> {
        check(
            &self.cookie_cipher,
            &request.connection_id,
            gen_remote_fingerprint(&remote_addr),
            cookie_valid_range,
        )
    }

    async fn authorize(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
        self.whitelist_authorization.authorize(info_hash).await
    }

    async fn send_event(
        &self,
        info_hash: InfoHash,
        announcement: PeerAnnouncement,
        client_socket_addr: SocketAddr,
        server_service_binding: ServiceBinding,
    ) {
        if let Some(udp_stats_event_sender) = self.opt_udp_core_stats_event_sender.as_deref() {
            let event = Event::UdpAnnounce {
                connection: ConnectionContext::new(self.configuration_instance_id, client_socket_addr, server_service_binding)
                    .with_public_url(self.public_url.clone()),
                info_hash,
                announcement,
            };

            tracing::debug!(target = crate::UDP_TRACKER_LOG_TARGET, "Sending UdpAnnounce event: {event:?}");

            udp_stats_event_sender.send(event).await;
        }
    }
}

/// Errors related to announce requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum UdpAnnounceError {
    /// Error returned when there was an error with the connection cookie.
    #[error("Connection cookie error: {source}")]
    ConnectionCookieError { source: ConnectionCookieError },

    /// Error returned when there was an error with the tracker core announce handler.
    #[error("Tracker core announce error: {source}")]
    TrackerCoreAnnounceError { source: AnnounceError },

    /// Error returned when there was an error with the tracker core whitelist.
    #[error("Tracker core whitelist error: {source}")]
    TrackerCoreWhitelistError { source: WhitelistError },
}

impl From<ConnectionCookieError> for UdpAnnounceError {
    fn from(connection_cookie_error: ConnectionCookieError) -> Self {
        Self::ConnectionCookieError {
            source: connection_cookie_error,
        }
    }
}

impl From<AnnounceError> for UdpAnnounceError {
    fn from(announce_error: AnnounceError) -> Self {
        Self::TrackerCoreAnnounceError { source: announce_error }
    }
}

impl From<WhitelistError> for UdpAnnounceError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::TrackerCoreWhitelistError { source: whitelist_error }
    }
}

#[cfg(test)]
mod tests {
    use std::future;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::num::NonZeroU16;
    use std::ops::Range;
    use std::sync::Arc;

    use mockall::predicate::eq;
    use torrust_clock::clock::Time;
    use torrust_info_hash::InfoHash;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_core::announce_handler::AnnounceHandler;
    use torrust_tracker_core::databases::error::Error as DatabaseError;
    use torrust_tracker_core::databases::{MockTorrentMetricsStore, TorrentMetricsStore};
    use torrust_tracker_core::error::{AnnounceError, WhitelistError};
    use torrust_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_primitives::{AnnounceEvent, ConfigurationInstanceId, Driver, NumberOfBytes, PeerId, ServiceRole};
    use torrust_tracker_udp_protocol::{
        AnnounceActionPlaceholder, AnnounceRequest, ConnectionId, NumberOfPeers, PeerKey, Port, TransactionId,
    };

    use super::{AnnounceService, UdpAnnounceError};
    use crate::CurrentClock;
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
    const PEER_ID: [u8; 20] = [0x2d; 20];
    const PEER_PORT: u16 = 6881;
    const PUBLIC_URL: &str = "udp://tracker.example.test:6969/announce";

    /// The service under test plus the torrent repository its announce handler writes to.
    struct AnnounceServiceFixture {
        service: AnnounceService,
        torrents: Arc<InMemoryTorrentRepository>,
    }

    impl AnnounceServiceFixture {
        fn public_tracker() -> Self {
            Self::new(&Core::default(), None, None, None)
        }

        fn listed_tracker_without_whitelisted_torrents() -> Self {
            Self::new(
                &Core {
                    listed: true,
                    ..Core::default()
                },
                None,
                None,
                None,
            )
        }

        fn public_tracker_with_external_ip(external_ip: IpAddr) -> Self {
            Self::new(&Core::default(), Some(external_ip), None, None)
        }

        fn public_tracker_publishing_events_to(event_sender: MockUdpCoreStatsEventSender) -> Self {
            Self::new(&Core::default(), None, None, Some(Arc::new(event_sender)))
        }

        fn tracker_with_persisted_downloads_from(store: MockTorrentMetricsStore) -> Self {
            Self::new(&Core::default(), None, Some(store), None)
        }

        fn new(
            core: &Core,
            tracker_external_ip: Option<IpAddr>,
            persisted_downloads_store: Option<MockTorrentMetricsStore>,
            event_sender: crate::event::sender::Sender,
        ) -> Self {
            let whitelist_authorization = Arc::new(WhitelistAuthorization::new(core, &Arc::new(InMemoryWhitelist::default())));
            let torrents = Arc::new(InMemoryTorrentRepository::default());
            let announce_handler = persisted_downloads_store.map_or_else(
                || AnnounceHandler::new_public(core, &whitelist_authorization, &torrents),
                |store| {
                    let store: Arc<dyn TorrentMetricsStore> = Arc::new(store);
                    AnnounceHandler::new_with_persistent_completed_statistics(
                        core,
                        &whitelist_authorization,
                        &torrents,
                        &Arc::new(DatabaseDownloadsMetricRepository::new(&store)),
                    )
                },
            );

            let service = AnnounceService::new(
                Arc::new(announce_handler),
                whitelist_authorization,
                Arc::new(fixed_test_cipher()),
                event_sender,
                INSTANCE_ID,
                tracker_external_ip,
            );

            Self { service, torrents }
        }

        async fn announced_peer_addresses(&self) -> Vec<SocketAddr> {
            self.torrents
                .get_torrent_peers(&InfoHash::from(INFO_HASH), 10)
                .await
                .iter()
                .map(|peer| peer.peer_addr)
                .collect()
        }
    }

    fn client_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 10)), 51_000)
    }

    fn loopback_client_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 51_000)
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

    fn announce_request(connection_id: ConnectionId) -> AnnounceRequest {
        AnnounceRequest {
            connection_id,
            action_placeholder: AnnounceActionPlaceholder::default(),
            transaction_id: TransactionId(0i32.into()),
            info_hash: torrust_tracker_udp_protocol::InfoHash(INFO_HASH),
            peer_id: PeerId(PEER_ID),
            bytes_downloaded: torrust_tracker_udp_protocol::NumberOfBytes(0i64.into()),
            bytes_uploaded: torrust_tracker_udp_protocol::NumberOfBytes(0i64.into()),
            bytes_left: torrust_tracker_udp_protocol::NumberOfBytes(0i64.into()),
            event: torrust_tracker_udp_protocol::AnnounceEvent::Started.into(),
            ip_address: Ipv4Addr::UNSPECIFIED.into(),
            key: PeerKey::new(0i32),
            peers_wanted: NumberOfPeers::new(1i32),
            port: Port::new(NonZeroU16::new(PEER_PORT).unwrap()),
        }
    }

    #[tokio::test]
    async fn it_should_reject_an_expired_connection_id_when_cookie_validation_is_enabled() {
        // Arrange
        let fixture = AnnounceServiceFixture::public_tracker();
        let request = announce_request(expired_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_announce(
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
            Err(UdpAnnounceError::ConnectionCookieError {
                source: ConnectionCookieError::ValueExpired { .. }
            })
        ));
    }

    #[tokio::test]
    async fn it_should_accept_an_expired_connection_id_when_cookie_validation_is_skipped() {
        // Arrange
        let fixture = AnnounceServiceFixture::public_tracker();
        let request = announce_request(expired_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_announce(
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
    async fn it_should_reject_an_announce_for_a_torrent_that_is_not_whitelisted_on_a_listed_tracker() {
        // Arrange
        let fixture = AnnounceServiceFixture::listed_tracker_without_whitelisted_torrents();
        let request = announce_request(valid_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_announce(
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
            Err(UdpAnnounceError::TrackerCoreWhitelistError {
                source: WhitelistError::TorrentNotWhitelisted { info_hash, .. }
            }) if info_hash == InfoHash::from(INFO_HASH)
        ));
    }

    #[tokio::test]
    async fn it_should_report_a_failing_persisted_downloads_lookup_as_a_tracker_core_announce_error() {
        // Arrange
        let mut failing_store = MockTorrentMetricsStore::new();
        failing_store.expect_load_torrent_downloads().returning(|_| {
            Box::pin(future::ready(Err(DatabaseError::MalformedDatabaseRecord {
                message: "corrupt record".to_string(),
                driver: Driver::Sqlite3,
            })))
        });
        let fixture = AnnounceServiceFixture::tracker_with_persisted_downloads_from(failing_store);
        let request = announce_request(valid_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_announce(
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
            Err(UdpAnnounceError::TrackerCoreAnnounceError {
                source: AnnounceError::Database(_)
            })
        ));
    }

    #[tokio::test]
    async fn it_should_validate_the_connection_id_before_revealing_whether_the_torrent_is_whitelisted() {
        // Arrange
        let fixture = AnnounceServiceFixture::listed_tracker_without_whitelisted_torrents();
        let request = announce_request(expired_connection_id_for(client_socket_addr()));

        // Act
        let result = fixture
            .service
            .handle_announce(
                client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                VALIDATE_COOKIE,
            )
            .await;

        // Assert
        assert!(matches!(result, Err(UdpAnnounceError::ConnectionCookieError { .. })));
    }

    #[tokio::test]
    async fn it_should_announce_the_peer_with_the_client_socket_ip_instead_of_the_ip_in_the_request() {
        // Arrange
        let fixture = AnnounceServiceFixture::public_tracker();
        let mut request = announce_request(valid_connection_id_for(client_socket_addr()));
        request.ip_address = Ipv4Addr::new(198, 51, 100, 77).into();

        // Act
        fixture
            .service
            .handle_announce(
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
            fixture.announced_peer_addresses().await,
            vec![SocketAddr::new(client_socket_addr().ip(), PEER_PORT)]
        );
    }

    #[tokio::test]
    async fn it_should_announce_a_loopback_client_with_the_configured_tracker_external_ip() {
        // Arrange
        let external_ip = IpAddr::V4(Ipv4Addr::new(198, 51, 100, 1));
        let fixture = AnnounceServiceFixture::public_tracker_with_external_ip(external_ip);
        let request = announce_request(valid_connection_id_for(loopback_client_socket_addr()));

        // Act
        fixture
            .service
            .handle_announce(
                loopback_client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                VALIDATE_COOKIE,
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(
            fixture.announced_peer_addresses().await,
            vec![SocketAddr::new(external_ip, PEER_PORT)]
        );
    }

    #[tokio::test]
    async fn it_should_publish_an_announce_event_with_the_connection_context_and_the_announced_peer() {
        // Arrange
        let expected_event = Event::UdpAnnounce {
            connection: ConnectionContext::new(INSTANCE_ID, client_socket_addr(), server_service_binding())
                .with_public_url(Some(PUBLIC_URL.to_string())),
            info_hash: InfoHash::from(INFO_HASH),
            announcement: Peer {
                peer_id: PeerId(PEER_ID),
                peer_addr: SocketAddr::new(client_socket_addr().ip(), PEER_PORT),
                updated: CurrentClock::now(),
                uploaded: NumberOfBytes::new(0),
                downloaded: NumberOfBytes::new(0),
                left: NumberOfBytes::new(0),
                event: AnnounceEvent::Started,
            },
        };
        let mut event_sender = MockUdpCoreStatsEventSender::new();
        event_sender
            .expect_send()
            .with(eq(expected_event))
            .times(1)
            .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
        let service = AnnounceServiceFixture::public_tracker_publishing_events_to(event_sender)
            .service
            .with_public_url(Some(PUBLIC_URL.to_string()));
        let request = announce_request(valid_connection_id_for(client_socket_addr()));

        // Act
        let result = service
            .handle_announce(
                client_socket_addr(),
                server_service_binding(),
                &request,
                cookie_valid_range(),
                VALIDATE_COOKIE,
            )
            .await;

        // Assert: the mock verifies the single expected event when it is dropped.
        assert!(result.is_ok());
    }
}
