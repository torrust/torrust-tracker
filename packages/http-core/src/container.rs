use std::sync::Arc;

use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_events::bus::SenderStatus;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

use crate::event::bus::EventBus;
use crate::event::sender::Broadcaster;
use crate::services::announce::AnnounceService;
use crate::services::scrape::ScrapeService;
use crate::statistics::repository::Repository;
use crate::{event, statistics};

pub struct HttpTrackerCoreContainer {
    pub http_tracker_config: Arc<HttpTracker>,

    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // `HttpTrackerCoreServices`
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl HttpTrackerCoreContainer {
    /// # Panics
    ///
    /// Panics if the persistence-required tracker-core container cannot be
    /// composed from the configured database or SQLite fallback.
    #[must_use]
    pub async fn initialize(
        core_config: &Arc<Core>,
        http_tracker_config: &Arc<HttpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let database_compatibility_bridge = core_config.database.clone().unwrap_or_default();
        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                core_config,
                &swarm_coordination_registry_container,
                Some(&database_compatibility_bridge),
            )
            .await
            .expect("HTTP tracker core initialization requires a configured database or SQLite fallback"),
        );

        Self::initialize_from_tracker_core(&tracker_core_container, http_tracker_config, configuration_instance_id)
    }

    #[must_use]
    pub fn initialize_from_tracker_core(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_config: &Arc<HttpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        let http_tracker_core_services = HttpTrackerCoreServices::initialize_from(tracker_core_container);

        Self::initialize_from_services(
            tracker_core_container,
            &http_tracker_core_services,
            http_tracker_config,
            configuration_instance_id,
        )
    }

    #[must_use]
    pub fn initialize_from_services(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_core_services: &Arc<HttpTrackerCoreServices>,
        http_tracker_config: &Arc<HttpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        Arc::new(Self {
            tracker_core_container: tracker_core_container.clone(),
            http_tracker_config: http_tracker_config.clone(),
            event_bus: http_tracker_core_services.event_bus.clone(),
            stats_event_sender: http_tracker_core_services.stats_event_sender.clone(),
            stats_repository: http_tracker_core_services.stats_repository.clone(),
            announce_service: Arc::new(AnnounceService::new_with_http_tracker_config(
                tracker_core_container.core_config.clone(),
                tracker_core_container.announce_handler.clone(),
                tracker_core_container.authentication_service.clone(),
                tracker_core_container.whitelist_authorization.clone(),
                http_tracker_core_services.stats_event_sender.clone(),
                http_tracker_config,
                configuration_instance_id,
            )),
            scrape_service: Arc::new(ScrapeService::new_with_http_tracker_config(
                tracker_core_container.core_config.clone(),
                tracker_core_container.scrape_handler.clone(),
                tracker_core_container.authentication_service.clone(),
                http_tracker_core_services.stats_event_sender.clone(),
                http_tracker_config,
                configuration_instance_id,
            )),
        })
    }
}

pub struct HttpTrackerCoreServices {
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
}

impl HttpTrackerCoreServices {
    #[must_use]
    pub fn initialize_from(_tracker_core_container: &Arc<TrackerCoreContainer>) -> Arc<Self> {
        // HTTP core stats
        let http_core_broadcaster = Broadcaster::default();
        let http_stats_repository = Arc::new(Repository::new());
        // issue: #2039
        // issue-spec: docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md
        // Events are objective facts. Per-listener metrics policy is applied by
        // the shared statistics listener, so it must not suppress publication.
        // A future consumer-demand optimization needs an inventory and benchmark
        // evidence before this can become conditional.
        let http_stats_event_bus = Arc::new(EventBus::new(SenderStatus::Enabled, http_core_broadcaster));

        let http_stats_event_sender = http_stats_event_bus.sender();

        Arc::new(Self {
            event_bus: http_stats_event_bus,
            stats_event_sender: http_stats_event_sender,
            stats_repository: http_stats_repository,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, SocketAddr};
    use std::sync::Arc;

    use rstest::rstest;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_clock::clock::stopped::Stopped as _;
    use torrust_info_hash::InfoHash;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::database::Database;
    use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
    use torrust_tracker_configuration::v3_0_0::public_url::HttpUrl;
    use torrust_tracker_core::announce_handler::AnnounceHandler;
    use torrust_tracker_core::authentication::Key;
    use torrust_tracker_core::authentication::key::generate_key;
    use torrust_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
    use torrust_tracker_core::authentication::service::AuthenticationService;
    use torrust_tracker_core::container::TrackerCoreContainer;
    use torrust_tracker_core::error::TrackerCoreError;
    use torrust_tracker_core::scrape_handler::ScrapeHandler;
    use torrust_tracker_core::statistics::repository::Repository as TrackerCoreStatisticsRepository;
    use torrust_tracker_core::torrent::manager::TorrentsManager;
    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_events::receiver::TryRecvError;
    use torrust_tracker_http_protocol::v1::requests::announce::{
        Announce, Event as ProtocolAnnounceEvent, NumberOfBytes as ProtocolNumberOfBytes, PeerIp,
    };
    use torrust_tracker_http_protocol::v1::requests::scrape::Scrape;
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{ClientIpSources, RemoteClientAddr, ResolvedIp};
    use torrust_tracker_primitives::peer::{Peer, PeerAnnouncement};
    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ScrapeData, ServiceRole};
    use torrust_tracker_test_helpers::configuration;

    use super::{HttpTrackerCoreContainer, HttpTrackerCoreServices};
    use crate::CurrentClock;
    use crate::event::bus::EventBus;
    use crate::event::{ConnectionContext, Event};
    use crate::services::announce::HttpAnnounceError;
    use crate::statistics::repository::Repository;
    use crate::tests::{RecordingSender, sample_info_hash, sample_peer};

    /// The port of the TCP connection that every request arrives on.
    const CONNECTION_PORT: u16 = 54_321;

    /// The port that the peer announces in every request.
    const ANNOUNCED_PEER_PORT: u16 = 8080;

    /// The tracker-core collaborators that the container composes. They are
    /// built by hand, without persistence, so no test depends on a production
    /// container factory.
    struct TrackerCore {
        container: Arc<TrackerCoreContainer>,
        in_memory_key_repository: Arc<InMemoryKeyRepository>,
    }

    impl TrackerCore {
        fn new(core_config: &Core) -> Self {
            let core_config = Arc::new(core_config.clone());
            let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
            let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&core_config, &in_memory_whitelist));
            let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
            let authentication_service = Arc::new(AuthenticationService::new(&core_config, &in_memory_key_repository));
            let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
            let container = Arc::new(TrackerCoreContainer {
                core_config: core_config.clone(),
                announce_handler: Arc::new(AnnounceHandler::new_public(
                    &core_config,
                    &whitelist_authorization,
                    &in_memory_torrent_repository,
                )),
                scrape_handler: Arc::new(ScrapeHandler::new_public(
                    &whitelist_authorization,
                    &in_memory_torrent_repository,
                )),
                authentication_service,
                in_memory_whitelist,
                whitelist_authorization,
                torrents_manager: Arc::new(TorrentsManager::new(&core_config, &in_memory_torrent_repository)),
                in_memory_torrent_repository,
                stats_repository: Arc::new(TrackerCoreStatisticsRepository::new(
                    core_config.tracker_usage_statistics,
                    core_config.tracker_policy.persistent_torrent_completed_stat,
                )),
                persistence: None,
            });

            Self {
                container,
                in_memory_key_repository,
            }
        }

        /// Registers a permanent key and returns it.
        async fn register_a_key(&self) -> Key {
            let peer_key = generate_key(None);
            let key = peer_key.key();
            self.in_memory_key_repository.reset_with(vec![peer_key]).await;
            key
        }
    }

    /// One composed container whose services publish to a recording sender.
    struct Fixture {
        tracker_core: TrackerCore,
        services: Arc<HttpTrackerCoreServices>,
        sender: Arc<RecordingSender>,
        http_tracker_config: Arc<HttpTracker>,
        container: Arc<HttpTrackerCoreContainer>,
    }

    impl Fixture {
        fn new(configuration: &Configuration) -> Self {
            let tracker_core = TrackerCore::new(&configuration.core);
            let sender = RecordingSender::new();
            let services = Arc::new(HttpTrackerCoreServices {
                event_bus: Arc::new(EventBus::default()),
                stats_event_sender: Some(sender.clone()),
                stats_repository: Arc::new(Repository::new()),
            });
            let http_tracker_config = Arc::new(first_http_tracker_of(configuration));
            let container = HttpTrackerCoreContainer::initialize_from_services(
                &tracker_core.container,
                &services,
                &http_tracker_config,
                instance_id(),
            );

            Self {
                tracker_core,
                services,
                sender,
                http_tracker_config,
                container,
            }
        }

        /// Announces the sample peer, reached at `connection_ip`, through the container.
        async fn announce(&self, connection_ip: IpAddr, key: Option<Key>) -> Result<(), HttpAnnounceError> {
            self.container
                .announce_service
                .handle_announce(
                    &announce_request(),
                    &direct_connection_from(connection_ip),
                    &server_binding(),
                    key,
                )
                .await
                .map(|_| ())
        }

        /// The swarm metadata that the container's scrape service reports for the sample torrent.
        async fn scraped_metadata(&self, key: Option<Key>) -> SwarmMetadata {
            let scrape_data = self
                .container
                .scrape_service
                .handle_scrape(
                    &Scrape {
                        info_hashes: vec![sample_info_hash()],
                    },
                    &direct_connection_from(ip("192.0.2.10")),
                    &server_binding(),
                    key,
                )
                .await
                .expect("the scrape should succeed");
            scrape_data.files[&sample_info_hash()]
        }
    }

    fn ip(text: &str) -> IpAddr {
        text.parse().expect("the test IP address should be valid")
    }

    /// An instance id that differs from the first one, so a service wired to a default id fails.
    fn instance_id() -> ConfigurationInstanceId {
        ConfigurationInstanceId::new(ServiceRole::HttpTracker, 7)
    }

    fn first_http_tracker_of(configuration: &Configuration) -> HttpTracker {
        configuration
            .http_trackers
            .as_deref()
            .and_then(<[HttpTracker]>::first)
            .expect("the test configuration should contain an HTTP tracker")
            .clone()
    }

    fn with_public_url(mut configuration: Configuration, public_url: &str) -> Configuration {
        configuration
            .http_trackers
            .as_mut()
            .expect("the test configuration should contain an HTTP tracker")[0]
            .public_url = Some(HttpUrl::parse(public_url).expect("the test public URL should be valid"));
        configuration
    }

    fn with_ip_from_query_string(mut configuration: Configuration, enabled: bool) -> Configuration {
        configuration
            .http_trackers
            .as_mut()
            .expect("the test configuration should contain an HTTP tracker")[0]
            .use_ip_from_query_string = enabled;
        configuration
    }

    fn server_binding() -> ServiceBinding {
        ServiceBinding::new(Protocol::HTTP, SocketAddr::new(ip("127.0.0.1"), 7070)).unwrap()
    }

    /// Stops the clock at a known time and returns it, so a test can write the
    /// announcement timestamp it expects.
    fn freeze_the_clock() -> DurationSinceUnixEpoch {
        let now = DurationSinceUnixEpoch::new(1_669_397_478, 0);
        CurrentClock::local_set(&now);
        now
    }

    /// A started announce from the sample peer, with no `ip` and zero byte counters.
    fn announce_request() -> Announce {
        Announce {
            info_hash: sample_info_hash(),
            peer_id: sample_peer().peer_id,
            port: ANNOUNCED_PEER_PORT,
            ip: PeerIp::Absent,
            uploaded: Some(ProtocolNumberOfBytes::new(0)),
            downloaded: Some(ProtocolNumberOfBytes::new(0)),
            left: Some(ProtocolNumberOfBytes::new(0)),
            event: Some(ProtocolAnnounceEvent::Started),
            compact: None,
            numwant: None,
        }
    }

    fn direct_connection_from(connection_ip: IpAddr) -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: Some(SocketAddr::new(connection_ip, CONNECTION_PORT)),
        }
    }

    /// The announcement that the sample request produces for a peer reached at `peer_ip`.
    fn expected_announcement(peer_ip: IpAddr, updated: DurationSinceUnixEpoch) -> PeerAnnouncement {
        Peer {
            peer_addr: SocketAddr::new(peer_ip, ANNOUNCED_PEER_PORT),
            updated,
            ..sample_peer()
        }
    }

    fn expected_announce_event(connection: ConnectionContext, announcement: PeerAnnouncement) -> Event {
        Event::TcpAnnounce {
            connection,
            info_hash: sample_info_hash(),
            announcement,
        }
    }

    fn a_connection() -> ConnectionContext {
        expected_connection(ResolvedIp::FromSocketAddr(ip("192.0.2.10")), None)
    }

    /// The connection context that a request from `remote_client_ip` is published with.
    fn expected_connection(remote_client_ip: ResolvedIp, public_url: Option<&str>) -> ConnectionContext {
        ConnectionContext::new(
            instance_id(),
            RemoteClientAddr::new(remote_client_ip, Some(CONNECTION_PORT)),
            server_binding(),
        )
        .with_public_url(public_url.map(str::to_string))
    }

    fn one_seeder() -> SwarmMetadata {
        SwarmMetadata {
            complete: 1,
            downloaded: 0,
            incomplete: 0,
        }
    }

    fn nothing() -> SwarmMetadata {
        SwarmMetadata {
            complete: 0,
            downloaded: 0,
            incomplete: 0,
        }
    }

    fn scrape_data_of(info_hash: &InfoHash, metadata: SwarmMetadata) -> ScrapeData {
        let mut scrape_data = ScrapeData::empty();
        scrape_data.add_file(info_hash, metadata);
        scrape_data
    }

    mod the_http_tracker_core_services {
        use super::*;

        #[tokio::test]
        async fn it_should_deliver_the_events_of_the_services_sender_to_the_receivers_of_the_services_event_bus() {
            // Arrange
            let tracker_core = TrackerCore::new(&Core::default());
            let services = HttpTrackerCoreServices::initialize_from(&tracker_core.container);
            let mut receiver = services.event_bus.receiver();
            let sender = services
                .stats_event_sender
                .clone()
                .expect("the services should always publish events");
            let event = Event::TcpScrape {
                connection: a_connection(),
            };

            // Act
            sender.send(event.clone()).await;

            // Assert
            let delivered = receiver.try_recv().expect("the event bus should have delivered the event");
            assert_eq!(delivered, event);
        }

        #[tokio::test]
        async fn it_should_not_share_the_event_bus_between_two_sets_of_services() {
            // Arrange
            let tracker_core = TrackerCore::new(&Core::default());
            let first = HttpTrackerCoreServices::initialize_from(&tracker_core.container);
            let second = HttpTrackerCoreServices::initialize_from(&tracker_core.container);
            let mut second_receiver = second.event_bus.receiver();
            let event = Event::TcpScrape {
                connection: a_connection(),
            };

            // Act
            first
                .stats_event_sender
                .clone()
                .expect("the services should always publish events")
                .send(event)
                .await;

            // Assert
            assert_eq!(second_receiver.try_recv(), Err(TryRecvError::Empty));
        }

        #[test]
        fn it_should_not_share_the_statistics_repository_between_two_sets_of_services() {
            // Arrange
            let tracker_core = TrackerCore::new(&Core::default());

            // Act
            let first = HttpTrackerCoreServices::initialize_from(&tracker_core.container);
            let second = HttpTrackerCoreServices::initialize_from(&tracker_core.container);

            // Assert
            assert!(!Arc::ptr_eq(&first.stats_repository, &second.stats_repository));
        }

        #[tokio::test]
        async fn it_should_start_with_a_fresh_statistics_repository() {
            // Arrange
            let tracker_core = TrackerCore::new(&Core::default());

            // Act
            let services = HttpTrackerCoreServices::initialize_from(&tracker_core.container);

            // Assert
            let metrics = services.stats_repository.read_metrics(Clone::clone).await;
            let fresh_metrics = Repository::new().read_metrics(Clone::clone).await;
            assert_eq!(metrics, fresh_metrics);
        }
    }

    mod the_container_composed_from_services {
        use super::*;

        #[test]
        fn it_should_expose_the_collaborators_it_was_composed_from() {
            // Arrange
            let fixture = Fixture::new(&configuration::ephemeral_public());

            // Act
            let container = &fixture.container;

            // Assert
            assert!(Arc::ptr_eq(
                &container.tracker_core_container,
                &fixture.tracker_core.container
            ));
            assert!(Arc::ptr_eq(&container.http_tracker_config, &fixture.http_tracker_config));
            assert!(Arc::ptr_eq(&container.event_bus, &fixture.services.event_bus));
            assert!(Arc::ptr_eq(&container.stats_repository, &fixture.services.stats_repository));
            assert!(Arc::ptr_eq(
                container
                    .stats_event_sender
                    .as_ref()
                    .expect("the container should have a sender"),
                fixture
                    .services
                    .stats_event_sender
                    .as_ref()
                    .expect("the services should have a sender"),
            ));
        }

        #[tokio::test]
        async fn it_should_publish_the_announce_event_of_its_instance_with_the_public_url_to_the_services_sender() {
            // Arrange
            let now = freeze_the_clock();
            let public_url = "https://tracker.example.com/";
            let fixture = Fixture::new(&with_public_url(configuration::ephemeral_public(), public_url));
            let connection_ip = ip("192.0.2.10");

            // Act
            let result = fixture.announce(connection_ip, None).await;

            // Assert
            assert!(result.is_ok(), "the announce should succeed: {result:?}");
            let expected_event = expected_announce_event(
                expected_connection(ResolvedIp::FromSocketAddr(connection_ip), Some(public_url)),
                expected_announcement(connection_ip, now),
            );
            assert_eq!(fixture.sender.events(), vec![expected_event]);
        }

        #[tokio::test]
        async fn it_should_publish_the_scrape_event_of_its_instance_with_the_public_url_to_the_services_sender() {
            // Arrange
            let public_url = "https://tracker.example.com/";
            let fixture = Fixture::new(&with_public_url(configuration::ephemeral_public(), public_url));

            // Act
            fixture.scraped_metadata(None).await;

            // Assert
            let expected_event = Event::TcpScrape {
                connection: expected_connection(ResolvedIp::FromSocketAddr(ip("192.0.2.10")), Some(public_url)),
            };
            assert_eq!(fixture.sender.events(), vec![expected_event]);
        }

        #[rstest]
        #[case::behind_a_reverse_proxy(
            configuration::ephemeral_with_reverse_proxy(),
            ResolvedIp::FromXForwardedFor(ip("203.0.113.195")),
            ip("203.0.113.195")
        )]
        #[case::not_behind_a_reverse_proxy(
            configuration::ephemeral_without_reverse_proxy(),
            ResolvedIp::FromSocketAddr(ip("192.0.2.10")),
            ip("192.0.2.10")
        )]
        #[tokio::test]
        async fn it_should_resolve_the_client_of_an_announce_by_the_reverse_proxy_policy_of_the_http_tracker(
            #[case] configuration: Configuration,
            #[case] expected_client: ResolvedIp,
            #[case] expected_peer_ip: IpAddr,
        ) {
            // Arrange
            let now = freeze_the_clock();
            let fixture = Fixture::new(&configuration);
            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: Some(ip("203.0.113.195")),
                connection_info_socket_address: Some(SocketAddr::new(ip("192.0.2.10"), CONNECTION_PORT)),
            };

            // Act
            let result = fixture
                .container
                .announce_service
                .handle_announce(&announce_request(), &client_ip_sources, &server_binding(), None)
                .await;

            // Assert
            assert!(result.is_ok(), "the announce should succeed: {result:?}");
            let expected_event = expected_announce_event(
                expected_connection(expected_client, None),
                expected_announcement(expected_peer_ip, now),
            );
            assert_eq!(fixture.sender.events(), vec![expected_event]);
        }

        #[rstest]
        #[case::behind_a_reverse_proxy(
            configuration::ephemeral_with_reverse_proxy(),
            ResolvedIp::FromXForwardedFor(ip("203.0.113.195"))
        )]
        #[case::not_behind_a_reverse_proxy(
            configuration::ephemeral_without_reverse_proxy(),
            ResolvedIp::FromSocketAddr(ip("192.0.2.10"))
        )]
        #[tokio::test]
        async fn it_should_resolve_the_client_of_a_scrape_by_the_reverse_proxy_policy_of_the_http_tracker(
            #[case] configuration: Configuration,
            #[case] expected_client: ResolvedIp,
        ) {
            // Arrange
            let fixture = Fixture::new(&configuration);
            let client_ip_sources = ClientIpSources {
                right_most_x_forwarded_for: Some(ip("203.0.113.195")),
                connection_info_socket_address: Some(SocketAddr::new(ip("192.0.2.10"), CONNECTION_PORT)),
            };

            // Act
            let result = fixture
                .container
                .scrape_service
                .handle_scrape(
                    &Scrape {
                        info_hashes: vec![sample_info_hash()],
                    },
                    &client_ip_sources,
                    &server_binding(),
                    None,
                )
                .await;

            // Assert
            assert!(result.is_ok(), "the scrape should succeed: {result:?}");
            let expected_event = Event::TcpScrape {
                connection: expected_connection(expected_client, None),
            };
            assert_eq!(fixture.sender.events(), vec![expected_event]);
        }

        #[tokio::test]
        async fn it_should_honor_an_ip_in_the_query_string_when_the_http_tracker_is_configured_to_use_it() {
            // Arrange
            let now = freeze_the_clock();
            let query_string_peer_ip = ip("198.51.100.42");
            let connection_ip = ip("192.0.2.10");
            let fixture = Fixture::new(&with_ip_from_query_string(configuration::ephemeral_public(), true));
            let request = Announce {
                ip: PeerIp::Literal(query_string_peer_ip),
                ..announce_request()
            };

            // Act
            let result = fixture
                .container
                .announce_service
                .handle_announce(&request, &direct_connection_from(connection_ip), &server_binding(), None)
                .await;

            // Assert
            assert!(result.is_ok(), "the announce should succeed: {result:?}");
            let expected_event = expected_announce_event(
                expected_connection(ResolvedIp::FromSocketAddr(connection_ip), None),
                expected_announcement(query_string_peer_ip, now),
            );
            assert_eq!(fixture.sender.events(), vec![expected_event]);
        }

        #[tokio::test]
        async fn it_should_reject_an_ip_in_the_query_string_when_the_http_tracker_is_not_configured_to_use_it() {
            // Arrange
            let fixture = Fixture::new(&with_ip_from_query_string(configuration::ephemeral_public(), false));
            let request = Announce {
                ip: PeerIp::Literal(ip("198.51.100.42")),
                ..announce_request()
            };

            // Act
            let result = fixture
                .container
                .announce_service
                .handle_announce(&request, &direct_connection_from(ip("192.0.2.10")), &server_binding(), None)
                .await;

            // Assert
            assert!(
                matches!(result, Err(HttpAnnounceError::PeerIpOverrideDisabled)),
                "unexpected result: {result:?}"
            );
            assert_eq!(fixture.sender.events(), vec![]);
        }

        #[tokio::test]
        async fn it_should_reject_an_announce_without_a_key_when_the_tracker_core_is_private() {
            // Arrange
            let fixture = Fixture::new(&configuration::ephemeral_private());

            // Act
            let result = fixture.announce(ip("192.0.2.10"), None).await;

            // Assert
            assert!(
                matches!(
                    result,
                    Err(HttpAnnounceError::TrackerCoreError {
                        source: TrackerCoreError::AuthenticationError { .. }
                    })
                ),
                "unexpected result: {result:?}"
            );
            assert!(
                !fixture
                    .tracker_core
                    .container
                    .in_memory_torrent_repository
                    .contains(&sample_info_hash())
            );
        }

        #[tokio::test]
        async fn it_should_zero_a_scrape_without_a_key_when_the_tracker_core_is_private() {
            // Arrange
            let fixture = Fixture::new(&configuration::ephemeral_private());
            let key = fixture.tracker_core.register_a_key().await;
            fixture.announce(ip("192.0.2.10"), Some(key)).await.unwrap();

            // Act
            let metadata = fixture.scraped_metadata(None).await;

            // Assert
            assert_eq!(metadata, nothing());
        }

        #[tokio::test]
        async fn it_should_authenticate_both_services_with_the_keys_of_the_tracker_core() {
            // Arrange
            let fixture = Fixture::new(&configuration::ephemeral_private());
            let key = fixture.tracker_core.register_a_key().await;

            // Act
            let announced = fixture.announce(ip("192.0.2.10"), Some(key.clone())).await;
            let metadata = fixture.scraped_metadata(Some(key)).await;

            // Assert
            assert!(announced.is_ok(), "the announce should succeed: {announced:?}");
            assert_eq!(metadata, one_seeder());
        }

        #[tokio::test]
        async fn it_should_reject_an_announce_of_a_torrent_missing_from_the_whitelist_of_the_tracker_core() {
            // Arrange
            let fixture = Fixture::new(&configuration::ephemeral_listed());

            // Act
            let result = fixture.announce(ip("192.0.2.10"), None).await;

            // Assert
            assert!(
                matches!(
                    result,
                    Err(HttpAnnounceError::TrackerCoreError {
                        source: TrackerCoreError::WhitelistError { .. }
                    })
                ),
                "unexpected result: {result:?}"
            );
        }

        #[tokio::test]
        async fn it_should_accept_an_announce_of_a_torrent_added_to_the_whitelist_of_the_tracker_core() {
            // Arrange
            let fixture = Fixture::new(&configuration::ephemeral_listed());
            fixture
                .tracker_core
                .container
                .in_memory_whitelist
                .add(&sample_info_hash())
                .await;

            // Act
            let result = fixture.announce(ip("192.0.2.10"), None).await;

            // Assert
            assert!(result.is_ok(), "the announce should succeed: {result:?}");
        }

        #[tokio::test]
        async fn it_should_scrape_the_swarm_that_the_announce_service_registered_in_the_tracker_core() {
            // Arrange
            let fixture = Fixture::new(&configuration::ephemeral_public());
            fixture.announce(ip("192.0.2.10"), None).await.unwrap();

            // Act
            let scrape_data = fixture
                .container
                .scrape_service
                .handle_scrape(
                    &Scrape {
                        info_hashes: vec![sample_info_hash()],
                    },
                    &direct_connection_from(ip("192.0.2.10")),
                    &server_binding(),
                    None,
                )
                .await
                .unwrap();

            // Assert
            assert_eq!(scrape_data, scrape_data_of(&sample_info_hash(), one_seeder()));
            assert!(
                fixture
                    .tracker_core
                    .container
                    .in_memory_torrent_repository
                    .contains(&sample_info_hash())
            );
        }
    }

    mod the_container_composed_from_tracker_core {
        use super::*;

        #[tokio::test]
        async fn it_should_publish_the_events_of_both_services_to_its_own_event_bus() {
            // Arrange
            let now = freeze_the_clock();
            let connection_ip = ip("192.0.2.10");
            let tracker_core = TrackerCore::new(&Core::default());
            let http_tracker_config = Arc::new(first_http_tracker_of(&configuration::ephemeral_public()));
            let container = HttpTrackerCoreContainer::initialize_from_tracker_core(
                &tracker_core.container,
                &http_tracker_config,
                instance_id(),
            );
            let mut receiver = container.event_bus.receiver();

            // Act
            container
                .announce_service
                .handle_announce(
                    &announce_request(),
                    &direct_connection_from(connection_ip),
                    &server_binding(),
                    None,
                )
                .await
                .unwrap();
            container
                .scrape_service
                .handle_scrape(
                    &Scrape {
                        info_hashes: vec![sample_info_hash()],
                    },
                    &direct_connection_from(connection_ip),
                    &server_binding(),
                    None,
                )
                .await
                .unwrap();

            // Assert
            let connection = ConnectionContext::new(
                instance_id(),
                RemoteClientAddr::new(ResolvedIp::FromSocketAddr(connection_ip), Some(CONNECTION_PORT)),
                server_binding(),
            );
            assert_eq!(
                receiver.try_recv(),
                Ok(expected_announce_event(
                    connection.clone(),
                    expected_announcement(connection_ip, now)
                ))
            );
            assert_eq!(receiver.try_recv(), Ok(Event::TcpScrape { connection }));
            assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
        }
    }

    mod the_container_initialized_from_the_core_configuration {
        use super::*;

        #[tokio::test]
        async fn it_should_compose_the_tracker_core_over_the_configured_database_with_the_given_core_configuration() {
            // Arrange
            let configuration = configuration::ephemeral_public();
            let core_config = Arc::new(configuration.core.clone());
            let http_tracker_config = Arc::new(first_http_tracker_of(&configuration));

            // Act
            let container = HttpTrackerCoreContainer::initialize(&core_config, &http_tracker_config, instance_id()).await;

            // Assert
            assert!(container.tracker_core_container.persistence.is_some());
            assert!(Arc::ptr_eq(&container.tracker_core_container.core_config, &core_config));
        }

        #[tokio::test]
        async fn it_should_publish_the_events_of_its_instance_with_the_public_url_to_its_own_event_bus() {
            // Arrange
            let now = freeze_the_clock();
            let public_url = "https://tracker.example.com/";
            let connection_ip = ip("192.0.2.10");
            let configuration = with_public_url(configuration::ephemeral_public(), public_url);
            let core_config = Arc::new(configuration.core.clone());
            let http_tracker_config = Arc::new(first_http_tracker_of(&configuration));
            let container = HttpTrackerCoreContainer::initialize(&core_config, &http_tracker_config, instance_id()).await;
            let mut receiver = container.event_bus.receiver();

            // Act
            container
                .announce_service
                .handle_announce(
                    &announce_request(),
                    &direct_connection_from(connection_ip),
                    &server_binding(),
                    None,
                )
                .await
                .unwrap();

            // Assert
            let expected_event = expected_announce_event(
                expected_connection(ResolvedIp::FromSocketAddr(connection_ip), Some(public_url)),
                expected_announcement(connection_ip, now),
            );
            assert_eq!(receiver.try_recv(), Ok(expected_event));
        }

        #[tokio::test]
        #[should_panic(expected = "HTTP tracker core initialization requires a configured database or SQLite fallback")]
        async fn it_should_panic_when_the_configured_database_cannot_be_initialized() {
            // Arrange: the parent of the database file is a random name that does not exist.
            let mut configuration = configuration::ephemeral_public();
            configuration.core.database = Some(Database::Sqlite3 {
                path: configuration::ephemeral_sqlite_database()
                    .join("tracker.sqlite3")
                    .to_string_lossy()
                    .into_owned(),
            });
            let core_config = Arc::new(configuration.core.clone());
            let http_tracker_config = Arc::new(first_http_tracker_of(&configuration));

            // Act
            let _container = HttpTrackerCoreContainer::initialize(&core_config, &http_tracker_config, instance_id()).await;
        }
    }
}
