//! The `announce` service.
//!
//! The service is responsible for handling the `announce` requests.
//!
//! It delegates the `announce` logic to the [`AnnounceHandler`] and it returns
//! the [`AnnounceData`].
//!
//! It also sends an [`Event`]
//! because events are specific for the HTTP tracker.
use std::panic::Location;
use std::sync::Arc;

use torrust_info_hash::InfoHash;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_core::announce_handler::{AnnounceHandler, PeersWanted};
use torrust_tracker_core::authentication::service::AuthenticationService;
use torrust_tracker_core::authentication::{self, Key};
use torrust_tracker_core::error::{AnnounceError, TrackerCoreError, WhitelistError};
use torrust_tracker_core::whitelist;
use torrust_tracker_http_protocol::v1::requests::announce::{
    Announce, Event as ProtocolAnnounceEvent, NumberOfBytes as ProtocolNumberOfBytes, PeerIp,
};
use torrust_tracker_http_protocol::v1::responses::error::Error as HttpProtocolErrorResponse;
use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{
    ClientIpSources, PeerIpResolutionError, RemoteClientAddr, ReverseProxyMode, resolve_remote_client_addr,
};
use torrust_tracker_primitives::peer::PeerAnnouncement;
use torrust_tracker_primitives::{AnnounceData, AnnounceEvent, ConfigurationInstanceId, NumberOfBytes};

use crate::event;
use crate::event::Event;
use crate::services::error_mapping::protocol_error_from_tracker_core_error;

/// The HTTP tracker `announce` service.
///
/// The service sends an statistics event that increments:
///
/// - The number of TCP `announce` requests handled by the HTTP tracker.
/// - The number of TCP `scrape` requests handled by the HTTP tracker.
pub struct AnnounceService {
    core_config: Arc<Core>,
    announce_handler: Arc<AnnounceHandler>,
    authentication_service: Arc<AuthenticationService>,
    whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    opt_http_stats_event_sender: event::sender::Sender,
    reverse_proxy_mode: ReverseProxyMode,
    peer_ip_selection_policy: PeerIpSelectionPolicy,
    configuration_instance_id: ConfigurationInstanceId,
    public_url: Option<String>,
}

/// Controls whether an HTTP announce may override its peer IP with BEP 3's
/// non-empty `ip` parameter. Enabling this trusts client-supplied addresses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PeerIpSelectionPolicy {
    use_ip_from_query_string: bool,
    external_ip: Option<std::net::IpAddr>,
}

#[derive(Clone, Debug)]
struct HttpTrackerPolicy {
    reverse_proxy_mode: ReverseProxyMode,
    peer_ip_selection: PeerIpSelectionPolicy,
    public_url: Option<String>,
}

impl From<&HttpTracker> for HttpTrackerPolicy {
    fn from(http_tracker_config: &HttpTracker) -> Self {
        Self {
            reverse_proxy_mode: http_tracker_config.network.on_reverse_proxy.into(),
            peer_ip_selection: http_tracker_config.into(),
            public_url: http_tracker_config.public_url.as_ref().map(ToString::to_string),
        }
    }
}

impl PeerIpSelectionPolicy {
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            use_ip_from_query_string: false,
            external_ip: None,
        }
    }

    #[must_use]
    pub const fn enabled() -> Self {
        Self {
            use_ip_from_query_string: true,
            external_ip: None,
        }
    }
}

impl From<&HttpTracker> for PeerIpSelectionPolicy {
    fn from(http_tracker_config: &HttpTracker) -> Self {
        Self {
            use_ip_from_query_string: http_tracker_config.use_ip_from_query_string,
            external_ip: http_tracker_config.network.external_ip.map(Into::into),
        }
    }
}

impl AnnounceService {
    #[must_use]
    pub fn new(
        core_config: Arc<Core>,
        announce_handler: Arc<AnnounceHandler>,
        authentication_service: Arc<AuthenticationService>,
        whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
        opt_http_stats_event_sender: event::sender::Sender,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Self {
        Self::new_with_peer_ip_selection_policy(
            core_config,
            announce_handler,
            authentication_service,
            whitelist_authorization,
            opt_http_stats_event_sender,
            PeerIpSelectionPolicy::disabled(),
            configuration_instance_id,
        )
    }

    /// Creates a service using the policy of one configured HTTP tracker instance.
    #[must_use]
    pub fn new_with_http_tracker_config(
        core_config: Arc<Core>,
        announce_handler: Arc<AnnounceHandler>,
        authentication_service: Arc<AuthenticationService>,
        whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
        opt_http_stats_event_sender: event::sender::Sender,
        http_tracker_config: &HttpTracker,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Self {
        Self::new_with_policies(
            core_config,
            announce_handler,
            authentication_service,
            whitelist_authorization,
            opt_http_stats_event_sender,
            http_tracker_config.into(),
            configuration_instance_id,
        )
    }

    #[must_use]
    pub fn new_with_peer_ip_selection_policy(
        core_config: Arc<Core>,
        announce_handler: Arc<AnnounceHandler>,
        authentication_service: Arc<AuthenticationService>,
        whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
        opt_http_stats_event_sender: event::sender::Sender,
        peer_ip_selection_policy: PeerIpSelectionPolicy,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Self {
        Self::new_with_policies(
            core_config,
            announce_handler,
            authentication_service,
            whitelist_authorization,
            opt_http_stats_event_sender,
            HttpTrackerPolicy {
                reverse_proxy_mode: ReverseProxyMode::Disabled,
                peer_ip_selection: peer_ip_selection_policy,
                public_url: None,
            },
            configuration_instance_id,
        )
    }

    fn new_with_policies(
        core_config: Arc<Core>,
        announce_handler: Arc<AnnounceHandler>,
        authentication_service: Arc<AuthenticationService>,
        whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
        opt_http_stats_event_sender: event::sender::Sender,
        http_tracker_policy: HttpTrackerPolicy,
        configuration_instance_id: ConfigurationInstanceId,
    ) -> Self {
        Self {
            core_config,
            announce_handler,
            authentication_service,
            whitelist_authorization,
            opt_http_stats_event_sender,
            reverse_proxy_mode: http_tracker_policy.reverse_proxy_mode,
            peer_ip_selection_policy: http_tracker_policy.peer_ip_selection,
            configuration_instance_id,
            public_url: http_tracker_policy.public_url,
        }
    }

    /// Handles an announce request.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// - The tracker is running in `listed` mode and the torrent is not whitelisted.
    /// - There is an error when resolving the client IP address.
    pub async fn handle_announce(
        &self,
        announce_request: &Announce,
        client_ip_sources: &ClientIpSources,
        server_service_binding: &ServiceBinding,
        maybe_key: Option<Key>,
    ) -> Result<AnnounceData, HttpAnnounceError> {
        self.authenticate(maybe_key).await?;

        self.authorize(announce_request.info_hash).await?;

        let remote_client_addr = resolve_remote_client_addr(&self.reverse_proxy_mode, client_ip_sources)?;

        let peer_ip = self.select_peer_ip(announce_request, remote_client_addr.ip())?;

        let mut peer = Self::peer_from_request(announce_request, &peer_ip);

        let peers_wanted = Self::peers_wanted(announce_request);

        let announce_data = self
            .announce_handler
            .handle_announcement(&announce_request.info_hash, &mut peer, &peer_ip, None, &peers_wanted)
            .await?;

        self.send_event(
            announce_request.info_hash,
            remote_client_addr,
            server_service_binding.clone(),
            peer,
        )
        .await;

        Ok(announce_data)
    }

    fn peer_from_request(announce_request: &Announce, peer_ip: &std::net::IpAddr) -> PeerAnnouncement {
        // Intentional adapter boundary: map protocol-owned request DTOs into
        // domain announcements here instead of sharing domain types with the
        // protocol crate. This limits coupling and keeps protocol evolution
        // from forcing domain-wide refactors.
        let uploaded = announce_request.uploaded.unwrap_or(ProtocolNumberOfBytes::new(0));
        let downloaded = announce_request.downloaded.unwrap_or(ProtocolNumberOfBytes::new(0));
        let left = announce_request.left.unwrap_or(ProtocolNumberOfBytes::new(0));

        PeerAnnouncement {
            peer_id: announce_request.peer_id,
            peer_addr: std::net::SocketAddr::new(*peer_ip, announce_request.port),
            updated: <crate::CurrentClock as torrust_clock::clock::Time>::now(),
            uploaded: NumberOfBytes::new(uploaded.0),
            downloaded: NumberOfBytes::new(downloaded.0),
            left: NumberOfBytes::new(left.0),
            event: announce_request
                .event
                .as_ref()
                .map_or(AnnounceEvent::None, |event| match event {
                    ProtocolAnnounceEvent::Started => AnnounceEvent::Started,
                    ProtocolAnnounceEvent::Stopped => AnnounceEvent::Stopped,
                    ProtocolAnnounceEvent::Completed => AnnounceEvent::Completed,
                    ProtocolAnnounceEvent::Empty => AnnounceEvent::None,
                }),
        }
    }

    fn select_peer_ip(
        &self,
        announce_request: &Announce,
        connection_peer_ip: std::net::IpAddr,
    ) -> Result<std::net::IpAddr, HttpAnnounceError> {
        Self::select_peer_ip_with_policy(self.peer_ip_selection_policy, announce_request, connection_peer_ip)
    }

    fn select_peer_ip_with_policy(
        peer_ip_selection_policy: PeerIpSelectionPolicy,
        announce_request: &Announce,
        connection_peer_ip: std::net::IpAddr,
    ) -> Result<std::net::IpAddr, HttpAnnounceError> {
        match &announce_request.ip {
            PeerIp::Absent | PeerIp::Empty => Ok(Self::external_ip_for_loopback_connection(
                peer_ip_selection_policy,
                connection_peer_ip,
            )),
            PeerIp::Literal(_) if !peer_ip_selection_policy.use_ip_from_query_string => {
                Err(HttpAnnounceError::PeerIpOverrideDisabled)
            }
            PeerIp::Literal(ip) => Ok(*ip),
            PeerIp::DnsName => Err(HttpAnnounceError::PeerIpDnsNameUnsupported),
            PeerIp::Invalid => Err(HttpAnnounceError::PeerIpInvalid),
        }
    }

    fn external_ip_for_loopback_connection(
        peer_ip_selection_policy: PeerIpSelectionPolicy,
        connection_peer_ip: std::net::IpAddr,
    ) -> std::net::IpAddr {
        if connection_peer_ip.is_loopback() {
            peer_ip_selection_policy.external_ip.unwrap_or(connection_peer_ip)
        } else {
            connection_peer_ip
        }
    }

    async fn authenticate(&self, maybe_key: Option<Key>) -> Result<(), authentication::key::Error> {
        if self.core_config.private {
            let key = maybe_key.ok_or(authentication::key::Error::MissingAuthKey {
                location: Location::caller(),
            })?;

            self.authentication_service.authenticate(&key).await?;
        }

        Ok(())
    }

    async fn authorize(&self, info_hash: InfoHash) -> Result<(), WhitelistError> {
        self.whitelist_authorization.authorize(&info_hash).await
    }

    /// Determines how many peers the client wants in the response
    fn peers_wanted(announce_request: &Announce) -> PeersWanted {
        announce_request
            .numwant
            .map_or(PeersWanted::AsManyAsPossible, PeersWanted::only)
    }

    async fn send_event(
        &self,
        info_hash: InfoHash,
        remote_client_addr: RemoteClientAddr,
        server_service_binding: ServiceBinding,
        announcement: PeerAnnouncement,
    ) {
        if let Some(http_stats_event_sender) = self.opt_http_stats_event_sender.as_deref() {
            let event = Event::TcpAnnounce {
                connection: event::ConnectionContext::new(
                    self.configuration_instance_id,
                    remote_client_addr,
                    server_service_binding,
                )
                .with_public_url(self.public_url.clone()),
                info_hash,
                announcement,
            };

            tracing::debug!("Sending TcpAnnounce event: {:?}", event);

            http_stats_event_sender.send(event).await;
        }
    }
}

/// Errors related to announce requests.
///
/// This internal error type is not an event payload: variants may compose
/// implementation errors and client-visible text. Any future rejected-request
/// event must use a stable, bounded, consumer-safe reason type defined by the
/// [general error-events EPIC](../../../../docs/issues/drafts/generalize-error-events/EPIC.md).
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum HttpAnnounceError {
    #[error("Error resolving peer IP: {source}")]
    PeerIpResolutionError { source: PeerIpResolutionError },

    #[error("Tracker core error: {source}")]
    TrackerCoreError { source: TrackerCoreError },

    #[error("Client-supplied peer IPs are disabled")]
    PeerIpOverrideDisabled,

    #[error("DNS names are not supported for the announce ip parameter")]
    PeerIpDnsNameUnsupported,

    #[error("The announce ip parameter must be an IPv4 or IPv6 literal")]
    PeerIpInvalid,
}

impl From<PeerIpResolutionError> for HttpAnnounceError {
    fn from(peer_ip_resolution_error: PeerIpResolutionError) -> Self {
        Self::PeerIpResolutionError {
            source: peer_ip_resolution_error,
        }
    }
}

impl From<TrackerCoreError> for HttpAnnounceError {
    fn from(tracker_core_error: TrackerCoreError) -> Self {
        Self::TrackerCoreError {
            source: tracker_core_error,
        }
    }
}

impl From<AnnounceError> for HttpAnnounceError {
    fn from(announce_error: AnnounceError) -> Self {
        Self::TrackerCoreError {
            source: announce_error.into(),
        }
    }
}

impl From<WhitelistError> for HttpAnnounceError {
    fn from(whitelist_error: WhitelistError) -> Self {
        Self::TrackerCoreError {
            source: whitelist_error.into(),
        }
    }
}

impl From<authentication::key::Error> for HttpAnnounceError {
    fn from(whitelist_error: authentication::key::Error) -> Self {
        Self::TrackerCoreError {
            source: whitelist_error.into(),
        }
    }
}

impl From<HttpAnnounceError> for HttpProtocolErrorResponse {
    fn from(error: HttpAnnounceError) -> Self {
        match error {
            HttpAnnounceError::PeerIpResolutionError { source } => source.into(),
            HttpAnnounceError::TrackerCoreError { source } => protocol_error_from_tracker_core_error(source),
            HttpAnnounceError::PeerIpOverrideDisabled
            | HttpAnnounceError::PeerIpDnsNameUnsupported
            | HttpAnnounceError::PeerIpInvalid => Self {
                failure_reason: error.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::panic::Location;
    use std::sync::Arc;

    use rstest::rstest;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_clock::clock::stopped::Stopped as _;
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
    use torrust_tracker_core::error::{AnnounceError, TrackerCoreError, WhitelistError};
    use torrust_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::whitelist::authorization::WhitelistAuthorization;
    use torrust_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
    use torrust_tracker_http_protocol::v1::requests::announce::{
        Announce, Event as ProtocolAnnounceEvent, NumberOfBytes as ProtocolNumberOfBytes, PeerIp,
    };
    use torrust_tracker_http_protocol::v1::responses::error::Error as HttpProtocolErrorResponse;
    use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::{
        ClientIpSources, PeerIpResolutionError, RemoteClientAddr, ResolvedIp,
    };
    use torrust_tracker_primitives::peer::{Peer, PeerAnnouncement};
    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
    use torrust_tracker_primitives::{
        AnnounceData, AnnounceEvent, AnnouncePolicy, ConfigurationInstanceId, Driver, NumberOfBytes, ServiceRole,
    };
    use torrust_tracker_test_helpers::configuration;

    use crate::event::{ConnectionContext, Event};
    use crate::services::announce::{AnnounceService, HttpAnnounceError, PeerIpSelectionPolicy};
    use crate::services::error_mapping::protocol_error_from_tracker_core_error;
    use crate::tests::{RecordingSender, sample_info_hash, sample_peer};
    use crate::{CurrentClock, event};

    /// The port that the peer announces in every request.
    const ANNOUNCED_PEER_PORT: u16 = 8080;

    /// The port of the TCP connection that the announce arrives on. It differs
    /// from the announced port so that a test can tell them apart.
    const CONNECTION_PORT: u16 = 54_321;

    /// The collaborators of one announce scenario. It builds only what the
    /// service needs: no database, statistics repository or event listener.
    struct Fixture {
        core_config: Arc<Core>,
        announce_handler: Arc<AnnounceHandler>,
        authentication_service: Arc<AuthenticationService>,
        whitelist_authorization: Arc<WhitelistAuthorization>,
        in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
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
            let authentication_service = Arc::new(AuthenticationService::new(&core_config, &in_memory_key_repository));
            let announce_handler = Arc::new(persisted_downloads_store.map_or_else(
                || AnnounceHandler::new_public(&configuration.core, &whitelist_authorization, &in_memory_torrent_repository),
                |store| {
                    let store: Arc<dyn TorrentMetricsStore> = Arc::new(store);
                    let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&store));
                    AnnounceHandler::new_with_persistent_completed_statistics(
                        &configuration.core,
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
                authentication_service,
                whitelist_authorization,
                in_memory_torrent_repository,
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

        /// Whether the swarm of the sample torrent holds any peer.
        fn torrent_is_registered(&self) -> bool {
            self.in_memory_torrent_repository.contains(&sample_info_hash())
        }

        /// A service built with `AnnounceService::new`: the peer IP override is disabled.
        fn service(&self, sender: event::sender::Sender) -> AnnounceService {
            AnnounceService::new(
                self.core_config.clone(),
                self.announce_handler.clone(),
                self.authentication_service.clone(),
                self.whitelist_authorization.clone(),
                sender,
                self.configuration_instance_id,
            )
        }

        /// A service with an explicit peer IP selection policy.
        fn service_with_policy(&self, sender: event::sender::Sender, policy: PeerIpSelectionPolicy) -> AnnounceService {
            AnnounceService::new_with_peer_ip_selection_policy(
                self.core_config.clone(),
                self.announce_handler.clone(),
                self.authentication_service.clone(),
                self.whitelist_authorization.clone(),
                sender,
                policy,
                self.configuration_instance_id,
            )
        }

        /// A service that takes its policies from the configured HTTP tracker.
        fn service_for_http_tracker(&self, sender: event::sender::Sender) -> AnnounceService {
            AnnounceService::new_with_http_tracker_config(
                self.core_config.clone(),
                self.announce_handler.clone(),
                self.authentication_service.clone(),
                self.whitelist_authorization.clone(),
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
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    }

    /// Stops the clock at a known time and returns it, so a test can write the
    /// announcement timestamp it expects.
    fn freeze_the_clock() -> DurationSinceUnixEpoch {
        let now = DurationSinceUnixEpoch::new(1_669_397_478, 0);
        CurrentClock::local_set(&now);
        now
    }

    /// A started announce from the sample peer, with no `ip`, no `numwant` and zero byte counters.
    fn sample_announce_request() -> Announce {
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

    /// Client IP sources for a direct connection from `connection_ip`.
    fn direct_connection_from(connection_ip: IpAddr) -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: Some(SocketAddr::new(connection_ip, CONNECTION_PORT)),
        }
    }

    fn sample_server_service_binding() -> ServiceBinding {
        ServiceBinding::new(Protocol::HTTP, SocketAddr::new(localhost(), 7070)).unwrap()
    }

    /// The announcement that the sample request produces for a peer reached at `peer_ip`.
    fn expected_announcement(peer_ip: IpAddr, updated: DurationSinceUnixEpoch) -> PeerAnnouncement {
        Peer {
            peer_addr: SocketAddr::new(peer_ip, ANNOUNCED_PEER_PORT),
            updated,
            ..sample_peer()
        }
    }

    fn expected_connection(fixture: &Fixture, remote_client_ip: ResolvedIp) -> ConnectionContext {
        ConnectionContext::new(
            fixture.configuration_instance_id,
            RemoteClientAddr::new(remote_client_ip, Some(CONNECTION_PORT)),
            sample_server_service_binding(),
        )
    }

    fn expected_announce_event(fixture: &Fixture, remote_client_ip: ResolvedIp, announcement: PeerAnnouncement) -> Event {
        Event::TcpAnnounce {
            connection: expected_connection(fixture, remote_client_ip),
            info_hash: sample_info_hash(),
            announcement,
        }
    }

    /// Client IP sources that name no address, so resolving the client IP fails.
    const fn unresolvable_client_ip_sources() -> ClientIpSources {
        ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: None,
        }
    }

    #[rstest]
    #[case::absent(PeerIp::Absent)]
    #[case::empty(PeerIp::Empty)]
    fn it_should_select_the_connection_address_for_an_absent_or_empty_peer_ip(#[case] peer_ip: PeerIp) {
        // Arrange
        let connection_ip = localhost();
        let request = Announce {
            ip: peer_ip,
            ..sample_announce_request()
        };

        // Act
        let selected = AnnounceService::select_peer_ip_with_policy(PeerIpSelectionPolicy::enabled(), &request, connection_ip);

        // Assert
        assert_eq!(selected.unwrap(), connection_ip);
    }

    #[rstest]
    #[case::absent(PeerIp::Absent)]
    #[case::empty(PeerIp::Empty)]
    fn it_should_replace_a_loopback_connection_address_with_the_external_ip_for_an_absent_or_empty_peer_ip(
        #[case] peer_ip: PeerIp,
    ) {
        // Arrange
        let external_ip = ip("203.0.113.195");
        let policy = PeerIpSelectionPolicy {
            use_ip_from_query_string: false,
            external_ip: Some(external_ip),
        };
        let request = Announce {
            ip: peer_ip,
            ..sample_announce_request()
        };

        // Act
        let selected = AnnounceService::select_peer_ip_with_policy(policy, &request, localhost());

        // Assert
        assert_eq!(selected.unwrap(), external_ip);
    }

    #[rstest]
    #[case::ipv4(ip("192.0.2.1"))]
    #[case::ipv6(ip("2001:db8::1"))]
    fn it_should_select_a_literal_peer_ip_when_the_policy_allows_the_override(#[case] literal_ip: IpAddr) {
        // Arrange
        let request = Announce {
            ip: PeerIp::Literal(literal_ip),
            ..sample_announce_request()
        };

        // Act
        let selected = AnnounceService::select_peer_ip_with_policy(PeerIpSelectionPolicy::enabled(), &request, localhost());

        // Assert
        assert_eq!(selected.unwrap(), literal_ip);
    }

    #[rstest]
    #[case::ipv4(ip("192.0.2.1"))]
    #[case::ipv6(ip("2001:db8::1"))]
    fn it_should_reject_a_literal_peer_ip_when_the_policy_disables_the_override(#[case] literal_ip: IpAddr) {
        // Arrange
        let request = Announce {
            ip: PeerIp::Literal(literal_ip),
            ..sample_announce_request()
        };

        // Act
        let selected = AnnounceService::select_peer_ip_with_policy(PeerIpSelectionPolicy::disabled(), &request, localhost());

        // Assert
        assert!(
            matches!(selected, Err(HttpAnnounceError::PeerIpOverrideDisabled)),
            "unexpected result: {selected:?}"
        );
    }

    #[rstest]
    #[case::overrides_enabled(PeerIpSelectionPolicy::enabled())]
    #[case::overrides_disabled(PeerIpSelectionPolicy::disabled())]
    fn it_should_reject_a_dns_name_peer_ip_under_any_override_policy(#[case] policy: PeerIpSelectionPolicy) {
        // Arrange
        let request = Announce {
            ip: PeerIp::DnsName,
            ..sample_announce_request()
        };

        // Act
        let selected = AnnounceService::select_peer_ip_with_policy(policy, &request, localhost());

        // Assert
        assert!(
            matches!(selected, Err(HttpAnnounceError::PeerIpDnsNameUnsupported)),
            "unexpected result: {selected:?}"
        );
    }

    #[rstest]
    #[case::overrides_enabled(PeerIpSelectionPolicy::enabled())]
    #[case::overrides_disabled(PeerIpSelectionPolicy::disabled())]
    fn it_should_reject_an_invalid_peer_ip_under_any_override_policy(#[case] policy: PeerIpSelectionPolicy) {
        // Arrange
        let request = Announce {
            ip: PeerIp::Invalid,
            ..sample_announce_request()
        };

        // Act
        let selected = AnnounceService::select_peer_ip_with_policy(policy, &request, localhost());

        // Assert
        assert!(
            matches!(selected, Err(HttpAnnounceError::PeerIpInvalid)),
            "unexpected result: {selected:?}"
        );
    }

    #[tokio::test]
    async fn it_should_return_the_announce_data() {
        // Arrange
        let configured_policy = AnnouncePolicy {
            interval: 1800.into(),
            interval_min: 900.into(),
            max_peers_per_announce: 50,
        };
        let mut configuration = configuration::ephemeral_public();
        configuration.core.announce_policy = configured_policy;
        let fixture = Fixture::new(&configuration);
        let service = fixture.service(None);

        // Act
        let announce_data = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await
            .unwrap();

        // Assert: the announcing peer is a seeder, and the response excludes it.
        let expected_announce_data = AnnounceData {
            peers: vec![],
            stats: SwarmMetadata {
                downloaded: 0,
                complete: 1,
                incomplete: 0,
            },
            policy: configured_policy,
        };
        assert_eq!(announce_data, expected_announce_data);
    }

    #[tokio::test]
    async fn it_should_use_the_http_tracker_policy_for_query_string_and_reverse_proxy_ips() {
        // Arrange
        let now = freeze_the_clock();
        let query_string_peer_ip = ip("198.51.100.42");
        let x_forwarded_for_ip = ip("203.0.113.195");
        let mut configuration = configuration::ephemeral_with_reverse_proxy();
        configuration
            .http_trackers
            .as_mut()
            .expect("the test configuration should contain an HTTP tracker")[0]
            .use_ip_from_query_string = true;
        let fixture = Fixture::new(&configuration);
        let sender = RecordingSender::new();
        let service = fixture.service_for_http_tracker(Some(sender.clone()));
        let request = Announce {
            ip: PeerIp::Literal(query_string_peer_ip),
            ..sample_announce_request()
        };
        let client_ip_sources = ClientIpSources {
            right_most_x_forwarded_for: Some(x_forwarded_for_ip),
            connection_info_socket_address: Some(SocketAddr::new(ip("192.0.2.10"), CONNECTION_PORT)),
        };

        // Act
        let result = service
            .handle_announce(&request, &client_ip_sources, &sample_server_service_binding(), None)
            .await;

        // Assert
        assert!(result.is_ok(), "the announce should succeed: {result:?}");
        let expected_event = expected_announce_event(
            &fixture,
            ResolvedIp::FromXForwardedFor(x_forwarded_for_ip),
            expected_announcement(query_string_peer_ip, now),
        );
        assert_eq!(sender.events(), vec![expected_event]);
    }

    #[tokio::test]
    async fn it_should_prefer_a_query_string_peer_ip_over_the_external_ip_for_a_loopback_client_when_overrides_are_enabled() {
        // Arrange
        let now = freeze_the_clock();
        let external_ip = ip("203.0.113.195");
        let query_string_peer_ip = ip("198.51.100.42");
        let fixture = Fixture::new(&configuration::ephemeral_public());
        let sender = RecordingSender::new();
        let policy = PeerIpSelectionPolicy {
            use_ip_from_query_string: true,
            external_ip: Some(external_ip),
        };
        let service = fixture.service_with_policy(Some(sender.clone()), policy);
        let request = Announce {
            ip: PeerIp::Literal(query_string_peer_ip),
            ..sample_announce_request()
        };

        // Act
        let result = service
            .handle_announce(
                &request,
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the announce should succeed: {result:?}");
        let expected_event = expected_announce_event(
            &fixture,
            ResolvedIp::FromSocketAddr(localhost()),
            expected_announcement(query_string_peer_ip, now),
        );
        assert_eq!(sender.events(), vec![expected_event]);
    }

    /// The remote client address in the event is the connection address, even
    /// when the tracker replaces the peer IP with its external IP. Only a
    /// loopback connection is replaced.
    #[rstest]
    #[case::ipv4_client(ip("126.0.0.1"), None, ip("126.0.0.1"))]
    #[case::ipv6_client(
        ip("6969:6969:6969:6969:6969:6969:6969:6969"),
        None,
        ip("6969:6969:6969:6969:6969:6969:6969:6969")
    )]
    #[case::ipv4_loopback_client_and_an_ipv6_external_ip(
        ip("127.0.0.1"),
        Some(ip("6969:6969:6969:6969:6969:6969:6969:6969")),
        ip("6969:6969:6969:6969:6969:6969:6969:6969")
    )]
    #[case::ipv6_loopback_client_and_an_ipv4_external_ip(ip("::1"), Some(ip("203.0.113.195")), ip("203.0.113.195"))]
    #[tokio::test]
    async fn it_should_publish_the_connection_address_and_the_selected_peer_ip_in_the_announce_event(
        #[case] connection_ip: IpAddr,
        #[case] external_ip: Option<IpAddr>,
        #[case] expected_peer_ip: IpAddr,
    ) {
        // Arrange
        let now = freeze_the_clock();
        let configuration = external_ip.map_or_else(configuration::ephemeral_public, configuration::ephemeral_with_external_ip);
        let fixture = Fixture::new(&configuration);
        let sender = RecordingSender::new();
        let service = fixture.service_for_http_tracker(Some(sender.clone()));

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(connection_ip),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the announce should succeed: {result:?}");
        let expected_event = expected_announce_event(
            &fixture,
            ResolvedIp::FromSocketAddr(connection_ip),
            expected_announcement(expected_peer_ip, now),
        );
        assert_eq!(sender.events(), vec![expected_event]);
    }

    #[test]
    fn it_should_not_replace_a_loopback_literal_peer_ip_with_the_external_ip() {
        // Arrange: the client supplies a loopback literal and the tracker has an external IP.
        let loopback_literal = ip("127.0.0.2");
        let policy = PeerIpSelectionPolicy {
            use_ip_from_query_string: true,
            external_ip: Some(ip("203.0.113.195")),
        };
        let request = Announce {
            ip: PeerIp::Literal(loopback_literal),
            ..sample_announce_request()
        };

        // Act
        let selected = AnnounceService::select_peer_ip_with_policy(policy, &request, localhost());

        // Assert
        assert_eq!(selected.unwrap(), loopback_literal);
    }

    #[rstest]
    #[case::ipv4(ip("126.0.0.1"))]
    #[case::ipv6(ip("6969:6969:6969:6969:6969:6969:6969:6969"))]
    fn it_should_keep_the_connection_address_of_a_non_loopback_client_when_an_external_ip_is_configured(
        #[case] connection_ip: IpAddr,
    ) {
        // Arrange
        let policy = PeerIpSelectionPolicy {
            use_ip_from_query_string: false,
            external_ip: Some(ip("203.0.113.195")),
        };

        // Act
        let selected = AnnounceService::select_peer_ip_with_policy(policy, &sample_announce_request(), connection_ip);

        // Assert
        assert_eq!(selected.unwrap(), connection_ip);
    }

    #[tokio::test]
    async fn it_should_reject_an_announce_without_a_key_on_a_private_tracker() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_private());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(
            matches!(
                result,
                Err(HttpAnnounceError::TrackerCoreError {
                    source: TrackerCoreError::AuthenticationError {
                        source: AuthenticationKeyError::MissingAuthKey { .. }
                    }
                })
            ),
            "unexpected result: {result:?}"
        );
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[tokio::test]
    async fn it_should_reject_an_announce_with_an_unregistered_key_on_a_private_tracker() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_private());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));
        let unregistered_key = Key::random();

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                Some(unregistered_key),
            )
            .await;

        // Assert
        assert!(
            matches!(
                result,
                Err(HttpAnnounceError::TrackerCoreError {
                    source: TrackerCoreError::AuthenticationError {
                        source: AuthenticationKeyError::UnableToReadKey { .. }
                    }
                })
            ),
            "unexpected result: {result:?}"
        );
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[tokio::test]
    async fn it_should_accept_an_announce_with_a_registered_key_on_a_private_tracker() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_private());
        let registered_key = fixture.register_a_key().await;
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                Some(registered_key),
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the announce should succeed: {result:?}");
        assert!(fixture.torrent_is_registered());
        assert_eq!(sender.events().len(), 1);
    }

    #[tokio::test]
    async fn it_should_ignore_a_presented_key_on_a_public_tracker() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_public());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));
        let unregistered_key = Key::random();

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                Some(unregistered_key),
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the announce should succeed: {result:?}");
        assert!(fixture.torrent_is_registered());
        assert_eq!(sender.events().len(), 1);
    }

    #[tokio::test]
    async fn it_should_reject_a_torrent_that_is_not_whitelisted_on_a_listed_tracker() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_listed());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(
            matches!(
                result,
                Err(HttpAnnounceError::TrackerCoreError {
                    source: TrackerCoreError::WhitelistError {
                        source: WhitelistError::TorrentNotWhitelisted { .. }
                    }
                })
            ),
            "unexpected result: {result:?}"
        );
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[tokio::test]
    async fn it_should_authenticate_before_authorizing_the_torrent() {
        // Arrange: both checks would fail, the key is missing and the torrent is not whitelisted.
        let fixture = Fixture::new(&configuration::ephemeral_private_and_listed());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

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
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[tokio::test]
    async fn it_should_authenticate_before_resolving_the_client_ip() {
        // Arrange: the key is missing and the client IP cannot be resolved.
        let fixture = Fixture::new(&configuration::ephemeral_private());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &unresolvable_client_ip_sources(),
                &sample_server_service_binding(),
                None,
            )
            .await;

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
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[tokio::test]
    async fn it_should_authorize_the_torrent_before_resolving_the_client_ip() {
        // Arrange: the torrent is not whitelisted and the client IP cannot be resolved.
        let fixture = Fixture::new(&configuration::ephemeral_listed());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &unresolvable_client_ip_sources(),
                &sample_server_service_binding(),
                None,
            )
            .await;

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
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[tokio::test]
    async fn it_should_resolve_the_client_ip_before_selecting_the_peer_ip() {
        // Arrange: the client IP cannot be resolved and the `ip` parameter is a DNS name.
        let fixture = Fixture::new(&configuration::ephemeral_public());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));
        let request = Announce {
            ip: PeerIp::DnsName,
            ..sample_announce_request()
        };

        // Act
        let result = service
            .handle_announce(
                &request,
                &unresolvable_client_ip_sources(),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(
            matches!(result, Err(HttpAnnounceError::PeerIpResolutionError { .. })),
            "unexpected result: {result:?}"
        );
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[tokio::test]
    async fn it_should_select_the_peer_ip_before_announcing() {
        // Arrange: the `ip` parameter is a literal and the tracker does not accept one.
        let fixture = Fixture::new(&configuration::ephemeral_public());
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));
        let request = Announce {
            ip: PeerIp::Literal(ip("198.51.100.42")),
            ..sample_announce_request()
        };

        // Act
        let result = service
            .handle_announce(
                &request,
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(
            matches!(result, Err(HttpAnnounceError::PeerIpOverrideDisabled)),
            "unexpected result: {result:?}"
        );
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[rstest]
    #[case::started(Some(ProtocolAnnounceEvent::Started), AnnounceEvent::Started)]
    #[case::stopped(Some(ProtocolAnnounceEvent::Stopped), AnnounceEvent::Stopped)]
    #[case::completed(Some(ProtocolAnnounceEvent::Completed), AnnounceEvent::Completed)]
    #[case::empty(Some(ProtocolAnnounceEvent::Empty), AnnounceEvent::None)]
    #[case::absent(None, AnnounceEvent::None)]
    fn it_should_map_the_announce_event_to_the_announcement_event(
        #[case] protocol_event: Option<ProtocolAnnounceEvent>,
        #[case] expected_event: AnnounceEvent,
    ) {
        // Arrange
        let request = Announce {
            event: protocol_event,
            ..sample_announce_request()
        };

        // Act
        let announcement = AnnounceService::peer_from_request(&request, &localhost());

        // Assert
        assert_eq!(announcement.event, expected_event);
    }

    #[test]
    fn it_should_default_absent_byte_counters_to_zero_in_the_announcement() {
        // Arrange
        let request = Announce {
            uploaded: None,
            downloaded: None,
            left: None,
            ..sample_announce_request()
        };

        // Act
        let announcement = AnnounceService::peer_from_request(&request, &localhost());

        // Assert
        assert_eq!(
            (announcement.uploaded, announcement.downloaded, announcement.left),
            (NumberOfBytes::new(0), NumberOfBytes::new(0), NumberOfBytes::new(0))
        );
    }

    #[test]
    fn it_should_copy_the_peer_id_address_byte_counters_and_time_into_the_announcement() {
        // Arrange
        let now = freeze_the_clock();
        let peer_ip = ip("198.51.100.7");
        let request = Announce {
            uploaded: Some(ProtocolNumberOfBytes::new(1)),
            downloaded: Some(ProtocolNumberOfBytes::new(2)),
            left: Some(ProtocolNumberOfBytes::new(3)),
            ..sample_announce_request()
        };

        // Act
        let announcement = AnnounceService::peer_from_request(&request, &peer_ip);

        // Assert
        let expected_announcement = Peer {
            peer_id: sample_peer().peer_id,
            peer_addr: SocketAddr::new(peer_ip, ANNOUNCED_PEER_PORT),
            updated: now,
            uploaded: NumberOfBytes::new(1),
            downloaded: NumberOfBytes::new(2),
            left: NumberOfBytes::new(3),
            event: AnnounceEvent::Started,
        };
        assert_eq!(announcement, expected_announcement);
    }

    #[rstest]
    #[case::absent(None, PeersWanted::AsManyAsPossible)]
    #[case::a_number(Some(30), PeersWanted::only(30))]
    fn it_should_translate_numwant_into_the_number_of_peers_wanted(
        #[case] numwant: Option<u32>,
        #[case] expected_peers_wanted: PeersWanted,
    ) {
        // Arrange
        let request = Announce {
            numwant,
            ..sample_announce_request()
        };

        // Act
        let peers_wanted = AnnounceService::peers_wanted(&request);

        // Assert
        assert_eq!(peers_wanted, expected_peers_wanted);
    }

    #[tokio::test]
    async fn it_should_announce_without_publishing_when_there_is_no_event_sender() {
        // Arrange
        let fixture = Fixture::new(&configuration::ephemeral_public());
        let service = fixture.service(None);

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the announce should succeed: {result:?}");
        assert!(fixture.torrent_is_registered());
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
        let fixture = Fixture::with_persisted_downloads_from(&configuration::ephemeral_public(), failing_store);
        let sender = RecordingSender::new();
        let service = fixture.service(Some(sender.clone()));

        // Act
        let result = service
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(
            matches!(
                result,
                Err(HttpAnnounceError::TrackerCoreError {
                    source: TrackerCoreError::AnnounceError {
                        source: AnnounceError::Database(_)
                    }
                })
            ),
            "unexpected result: {result:?}"
        );
        assert_nothing_was_announced(&fixture, &sender);
    }

    #[tokio::test]
    async fn it_should_publish_the_public_url_of_the_http_tracker_in_the_announce_event() {
        // Arrange
        let now = freeze_the_clock();
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
            .handle_announce(
                &sample_announce_request(),
                &direct_connection_from(localhost()),
                &sample_server_service_binding(),
                None,
            )
            .await;

        // Assert
        assert!(result.is_ok(), "the announce should succeed: {result:?}");
        let expected_event = Event::TcpAnnounce {
            connection: expected_connection(&fixture, ResolvedIp::FromSocketAddr(localhost()))
                .with_public_url(Some("https://tracker.example.com/".to_string())),
            info_hash: sample_info_hash(),
            announcement: expected_announcement(localhost(), now),
        };
        assert_eq!(sender.events(), vec![expected_event]);
    }

    #[rstest]
    #[case::override_disabled(HttpAnnounceError::PeerIpOverrideDisabled, "Client-supplied peer IPs are disabled")]
    #[case::dns_name(
        HttpAnnounceError::PeerIpDnsNameUnsupported,
        "DNS names are not supported for the announce ip parameter"
    )]
    #[case::invalid(
        HttpAnnounceError::PeerIpInvalid,
        "The announce ip parameter must be an IPv4 or IPv6 literal"
    )]
    fn it_should_report_a_rejected_peer_ip_with_its_failure_reason(
        #[case] error: HttpAnnounceError,
        #[case] expected_reason: &str,
    ) {
        // Arrange / Act
        let response = HttpProtocolErrorResponse::from(error);

        // Assert
        assert_eq!(response.failure_reason, expected_reason);
    }

    #[test]
    fn it_should_report_a_peer_ip_resolution_error_with_the_protocol_failure_reason() {
        // Arrange
        let resolution_error = PeerIpResolutionError::MissingClientIp {
            location: Location::caller(),
        };
        let protocol_response = HttpProtocolErrorResponse::from(resolution_error.clone());

        // Act
        let response = HttpProtocolErrorResponse::from(HttpAnnounceError::from(resolution_error));

        // Assert
        assert_eq!(response.failure_reason, protocol_response.failure_reason);
    }

    #[test]
    fn it_should_report_a_tracker_core_error_with_the_tracker_core_failure_reason() {
        // Arrange
        let tracker_core_error = TrackerCoreError::from(WhitelistError::TorrentNotWhitelisted {
            info_hash: sample_info_hash(),
            location: Location::caller(),
        });
        let tracker_core_response = protocol_error_from_tracker_core_error(tracker_core_error.clone());

        // Act
        let response = HttpProtocolErrorResponse::from(HttpAnnounceError::from(tracker_core_error));

        // Assert
        assert_eq!(response.failure_reason, tracker_core_response.failure_reason);
    }

    #[test]
    fn it_should_wrap_a_tracker_core_error_without_changing_it() {
        // Arrange
        let tracker_core_error = TrackerCoreError::from(WhitelistError::TorrentNotWhitelisted {
            info_hash: sample_info_hash(),
            location: Location::caller(),
        });

        // Act
        let error = HttpAnnounceError::from(tracker_core_error);

        // Assert
        assert!(
            matches!(
                error,
                HttpAnnounceError::TrackerCoreError {
                    source: TrackerCoreError::WhitelistError { .. }
                }
            ),
            "unexpected error: {error:?}"
        );
    }

    /// An announce that fails a check must stop before the tracker registers the
    /// peer and before it publishes an event.
    fn assert_nothing_was_announced(fixture: &Fixture, sender: &RecordingSender) {
        assert!(sender.events().is_empty(), "no event should be published");
        assert!(!fixture.torrent_is_registered(), "no peer should be registered");
    }
}
