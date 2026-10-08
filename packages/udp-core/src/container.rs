use std::sync::Arc;

use tokio::sync::RwLock;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_events::bus::SenderStatus;
use torrust_tracker_primitives::ConfigurationInstanceId;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

use crate::crypto::cookie_cipher::CookieCipher;
use crate::event::bus::EventBus;
use crate::event::sender::Broadcaster;
use crate::services::announce::AnnounceService;
use crate::services::banning::BanService;
use crate::services::connect::ConnectService;
use crate::services::scrape::ScrapeService;
use crate::statistics::repository::Repository;
use crate::{event, services, statistics};

pub struct UdpTrackerCoreContainer {
    pub udp_tracker_config: Arc<UdpTracker>,
    pub configuration_instance_id: ConfigurationInstanceId,

    pub tracker_core_container: Arc<TrackerCoreContainer>,

    // `UdpTrackerCoreServices`
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: crate::event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub ban_service: Arc<RwLock<BanService>>,
    pub connect_service: Arc<ConnectService>,
    pub announce_service: Arc<AnnounceService>,
    pub scrape_service: Arc<ScrapeService>,
}

impl UdpTrackerCoreContainer {
    /// # Panics
    ///
    /// Panics if the persistence-required tracker-core container cannot be
    /// composed from the configured database.
    #[must_use]
    pub async fn initialize(
        core_config: &Arc<Core>,
        udp_tracker_config: &Arc<UdpTracker>,
        max_connection_id_errors_per_ip: u32,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                core_config,
                &swarm_coordination_registry_container,
                core_config.database.as_ref(),
            )
            .await
            .expect("UDP tracker core initialization requires persistence"),
        );

        Self::initialize_from_tracker_core(
            &tracker_core_container,
            udp_tracker_config,
            max_connection_id_errors_per_ip,
            configuration_instance_id,
        )
    }

    #[must_use]
    pub fn initialize_from_tracker_core(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_config: &Arc<UdpTracker>,
        max_connection_id_errors_per_ip: u32,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        let udp_tracker_core_services =
            UdpTrackerCoreServices::initialize_from(tracker_core_container, max_connection_id_errors_per_ip);

        Self::initialize_from_services(
            tracker_core_container,
            &udp_tracker_core_services,
            udp_tracker_config,
            configuration_instance_id,
        )
    }

    #[must_use]
    pub fn initialize_from_services(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        udp_tracker_core_services: &Arc<UdpTrackerCoreServices>,
        udp_tracker_config: &Arc<UdpTracker>,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Arc<Self> {
        Arc::new(Self {
            udp_tracker_config: udp_tracker_config.clone(),
            configuration_instance_id,

            tracker_core_container: tracker_core_container.clone(),

            // `UdpTrackerCoreServices`
            event_bus: udp_tracker_core_services.event_bus.clone(),
            stats_event_sender: udp_tracker_core_services.stats_event_sender.clone(),
            stats_repository: udp_tracker_core_services.stats_repository.clone(),
            ban_service: udp_tracker_core_services.ban_service.clone(),
            connect_service: Arc::new(
                ConnectService::new(
                    udp_tracker_core_services.cookie_cipher.clone(),
                    udp_tracker_core_services.stats_event_sender.clone(),
                    configuration_instance_id,
                )
                .with_public_url(udp_tracker_config.public_url.as_ref().map(ToString::to_string)),
            ),
            announce_service: Arc::new(
                AnnounceService::new(
                    tracker_core_container.announce_handler.clone(),
                    tracker_core_container.whitelist_authorization.clone(),
                    udp_tracker_core_services.cookie_cipher.clone(),
                    udp_tracker_core_services.stats_event_sender.clone(),
                    configuration_instance_id,
                    udp_tracker_config.network.external_ip.map(Into::into),
                )
                .with_public_url(udp_tracker_config.public_url.as_ref().map(ToString::to_string)),
            ),
            scrape_service: Arc::new(
                ScrapeService::new(
                    tracker_core_container.scrape_handler.clone(),
                    udp_tracker_core_services.cookie_cipher.clone(),
                    udp_tracker_core_services.stats_event_sender.clone(),
                    configuration_instance_id,
                )
                .with_public_url(udp_tracker_config.public_url.as_ref().map(ToString::to_string)),
            ),
        })
    }
}

/// The UDP tracker core services shared by every UDP tracker instance built
/// from them.
///
/// The connection-cookie key stays inside this crate: code that holds these
/// services, such as the application container, cannot read it.
///
/// ```rust,compile_fail,E0616
/// use torrust_tracker_udp_core::container::UdpTrackerCoreServices;
///
/// fn key_of(services: &UdpTrackerCoreServices) {
///     let _key = services.cookie_cipher.clone();
/// }
/// ```
///
/// The other services are public:
///
/// ```rust
/// use torrust_tracker_udp_core::container::UdpTrackerCoreServices;
///
/// fn event_bus_of(services: &UdpTrackerCoreServices) {
///     let _event_bus = services.event_bus.clone();
/// }
/// ```
pub struct UdpTrackerCoreServices {
    /// The connection-cookie key. It is created here, once per set of
    /// services, so every connect, announce, and scrape service built from
    /// these services issues and accepts the same connection IDs.
    pub(crate) cookie_cipher: Arc<CookieCipher>,
    pub event_bus: Arc<event::bus::EventBus>,
    pub stats_event_sender: crate::event::sender::Sender,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub ban_service: Arc<RwLock<services::banning::BanService>>,
}

impl UdpTrackerCoreServices {
    #[must_use]
    pub fn initialize_from(
        _tracker_core_container: &Arc<TrackerCoreContainer>,
        max_connection_id_errors_per_ip: u32,
    ) -> Arc<Self> {
        let udp_core_broadcaster = Broadcaster::default();
        let udp_core_stats_repository = Arc::new(Repository::new());
        // issue: #2039
        // issue-spec: docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md
        // Events are objective facts. Per-listener metrics policy is applied by
        // the shared statistics listener, so it must not suppress publication.
        // A future consumer-demand optimization needs an inventory and benchmark
        // evidence before this can become conditional.
        let event_bus = Arc::new(EventBus::new(SenderStatus::Enabled, udp_core_broadcaster));

        let udp_core_stats_event_sender = event_bus.sender();
        let ban_service = Arc::new(RwLock::new(BanService::new(max_connection_id_errors_per_ip)));
        Arc::new(Self {
            cookie_cipher: Arc::new(CookieCipher::random()),
            event_bus,
            stats_event_sender: udp_core_stats_event_sender,
            stats_repository: udp_core_stats_repository,
            ban_service,
        })
    }
}

#[cfg(test)]
mod tests {
    //! A connection ID is only accepted by a service that holds the key it was
    //! issued with, so every service built by one container must share one key.

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::num::NonZeroU16;
    use std::ops::Range;
    use std::sync::Arc;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
    use torrust_tracker_primitives::{ConfigurationInstanceId, PeerId, ServiceRole};
    use torrust_tracker_udp_protocol::{
        AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectionId, InfoHash, NumberOfBytes, NumberOfPeers, PeerKey,
        Port, ScrapeRequest, TransactionId,
    };

    use super::UdpTrackerCoreContainer;

    const ISSUE_TIME: f64 = 1_000_000_000_f64;

    /// A container built the way a standalone UDP tracker builds it: public
    /// tracker, no database.
    async fn standalone_udp_tracker_core_container() -> Arc<UdpTrackerCoreContainer> {
        UdpTrackerCoreContainer::initialize(
            &Arc::new(Core::default()),
            &Arc::new(UdpTracker::default()),
            10,
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
        )
        .await
    }

    fn client_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(198, 51, 100, 7)), 6881)
    }

    fn server_service_binding() -> ServiceBinding {
        ServiceBinding::new(
            Protocol::UDP,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 196)), 6969),
        )
        .expect("a UDP service binding on a non-zero port")
    }

    fn valid_range() -> Range<f64> {
        (ISSUE_TIME - 120.0)..(ISSUE_TIME + 120.0)
    }

    fn announce_request_with(connection_id: ConnectionId) -> AnnounceRequest {
        AnnounceRequest {
            connection_id,
            action_placeholder: AnnounceActionPlaceholder::default(),
            transaction_id: TransactionId::new(0),
            info_hash: InfoHash([0; 20]),
            peer_id: PeerId([0; 20]),
            bytes_downloaded: NumberOfBytes::new(0),
            bytes_uploaded: NumberOfBytes::new(0),
            bytes_left: NumberOfBytes::new(0),
            event: AnnounceEvent::Started.into(),
            ip_address: Ipv4Addr::UNSPECIFIED.into(),
            key: PeerKey::new(0),
            peers_wanted: NumberOfPeers::new(1),
            port: Port::new(NonZeroU16::new(6881).expect("a non-zero port")),
        }
    }

    fn scrape_request_with(connection_id: ConnectionId) -> ScrapeRequest {
        ScrapeRequest {
            connection_id,
            transaction_id: TransactionId::new(0),
            info_hashes: vec![],
        }
    }

    #[tokio::test]
    async fn it_should_accept_in_announce_a_connection_id_issued_by_the_connect_service_of_the_same_container() {
        // Arrange
        let container = standalone_udp_tracker_core_container().await;
        let connection_id = container
            .connect_service
            .handle_connect(client_socket_addr(), server_service_binding(), ISSUE_TIME)
            .await;

        // Act
        let result =
            container
                .announce_service
                .authenticate(client_socket_addr(), &announce_request_with(connection_id), valid_range());

        // Assert
        assert_eq!(
            result,
            Ok(ISSUE_TIME),
            "the announce service rejected a connection ID issued by the same container's connect service"
        );
    }

    #[tokio::test]
    async fn it_should_accept_in_scrape_a_connection_id_issued_by_the_connect_service_of_the_same_container() {
        // Arrange
        let container = standalone_udp_tracker_core_container().await;
        let connection_id = container
            .connect_service
            .handle_connect(client_socket_addr(), server_service_binding(), ISSUE_TIME)
            .await;

        // Act
        let result =
            container
                .scrape_service
                .authenticate(client_socket_addr(), &scrape_request_with(connection_id), valid_range());

        // Assert
        assert_eq!(
            result,
            Ok(ISSUE_TIME),
            "the scrape service rejected a connection ID issued by the same container's connect service"
        );
    }
}
