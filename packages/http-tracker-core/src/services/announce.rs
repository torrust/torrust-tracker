//! The `announce` service.
//!
//! The service is responsible for handling the `announce` requests.
//!
//! It delegates the `announce` logic to the [`AnnounceHandler`] and it returns
//! the [`AnnounceData`].
//!
//! It also sends an [`http_tracker_core::event::Event`]
//! because events are specific for the HTTP tracker.
use std::panic::Location;
use std::sync::Arc;

use bittorrent_http_tracker_protocol::v1::requests::announce::{peer_from_request, Announce};
use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{
    resolve_remote_client_addr, ClientIpSources, PeerIpResolutionError, RemoteClientAddr,
};
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::announce_handler::{AnnounceHandler, PeersWanted};
use bittorrent_tracker_core::authentication::service::AuthenticationService;
use bittorrent_tracker_core::authentication::{self, Key};
use bittorrent_tracker_core::error::{AnnounceError, TrackerCoreError, WhitelistError};
use bittorrent_tracker_core::whitelist;
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::core::AnnounceData;
use torrust_tracker_primitives::peer::PeerAnnouncement;
use torrust_tracker_primitives::service_binding::ServiceBinding;

use crate::event;
use crate::event::Event;

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
}

impl AnnounceService {
    #[must_use]
    pub fn new(
        core_config: Arc<Core>,
        announce_handler: Arc<AnnounceHandler>,
        authentication_service: Arc<AuthenticationService>,
        whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
        opt_http_stats_event_sender: event::sender::Sender,
    ) -> Self {
        Self {
            core_config,
            announce_handler,
            authentication_service,
            whitelist_authorization,
            opt_http_stats_event_sender,
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

        let remote_client_addr = resolve_remote_client_addr(&self.core_config.net.on_reverse_proxy.into(), client_ip_sources)?;

        let mut peer = peer_from_request(announce_request, &remote_client_addr.ip());

        let peers_wanted = Self::peers_wanted(announce_request);

        let announce_data = self
            .announce_handler
            .handle_announcement(
                &announce_request.info_hash,
                &mut peer,
                &remote_client_addr.ip(),
                &peers_wanted,
            )
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
        match announce_request.numwant {
            Some(numwant) => PeersWanted::only(numwant),
            None => PeersWanted::AsManyAsPossible,
        }
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
                connection: event::ConnectionContext::new(remote_client_addr, server_service_binding),
                info_hash,
                announcement,
            };

            tracing::debug!("Sending TcpAnnounce event: {:?}", event);

            http_stats_event_sender.send(event).await;
        }
    }
}

/// Errors related to announce requests.
#[derive(thiserror::Error, Debug, Clone)]
pub enum HttpAnnounceError {
    #[error("Error resolving peer IP: {source}")]
    PeerIpResolutionError { source: PeerIpResolutionError },

    #[error("Tracker core error: {source}")]
    TrackerCoreError { source: TrackerCoreError },
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

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::sync::Arc;

    use bittorrent_http_tracker_protocol::v1::requests::announce::Announce;
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
    use torrust_tracker_configuration::{Configuration, Core};
    use torrust_tracker_primitives::peer::Peer;
    use torrust_tracker_test_helpers::configuration;

    struct CoreTrackerServices {
        pub core_config: Arc<Core>,
        pub announce_handler: Arc<AnnounceHandler>,
        pub authentication_service: Arc<AuthenticationService>,
        pub whitelist_authorization: Arc<WhitelistAuthorization>,
    }

    struct CoreHttpTrackerServices {
        pub http_stats_event_sender: crate::event::sender::Sender,
    }

    fn initialize_core_tracker_services() -> (CoreTrackerServices, CoreHttpTrackerServices) {
        initialize_core_tracker_services_with_config(&configuration::ephemeral_public())
    }

    fn initialize_core_tracker_services_with_config(config: &Configuration) -> (CoreTrackerServices, CoreHttpTrackerServices) {
        let cancellation_token = CancellationToken::new();

        let core_config = Arc::new(config.core.clone());
        let database = initialize_database(&config.core);
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_downloads_metric_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&database));
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
        let authentication_service = Arc::new(AuthenticationService::new(&core_config, &in_memory_key_repository));

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

        (
            CoreTrackerServices {
                core_config,
                announce_handler,
                authentication_service,
                whitelist_authorization,
            },
            CoreHttpTrackerServices { http_stats_event_sender },
        )
    }

    fn sample_announce_request_for_peer(peer: Peer) -> (Announce, ClientIpSources) {
        let announce_request = Announce {
            info_hash: sample_info_hash(),
            peer_id: peer.peer_id,
            port: peer.peer_addr.port(),
            uploaded: Some(peer.uploaded),
            downloaded: Some(peer.downloaded),
            left: Some(peer.left),
            event: Some(peer.event.into()),
            compact: None,
            numwant: None,
        };

        let client_ip_sources = ClientIpSources {
            right_most_x_forwarded_for: None,
            connection_info_socket_address: Some(SocketAddr::new(peer.peer_addr.ip(), 8080)),
        };

        (announce_request, client_ip_sources)
    }

    use futures::future::BoxFuture;
    use mockall::mock;
    use torrust_tracker_events::sender::SendError;

    use crate::event::bus::EventBus;
    use crate::event::sender::Broadcaster;
    use crate::event::Event;
    use crate::statistics::event::listener::run_event_listener;
    use crate::statistics::repository::Repository;
    use crate::tests::sample_info_hash;

    mock! {
        HttpStatsEventSender {}
        impl torrust_tracker_events::sender::Sender for HttpStatsEventSender {
            type Event = Event;

            fn send(&self, event: Event) -> BoxFuture<'static,Option<Result<usize,SendError<Event> > > > ;
        }
    }

    mod with_tracker_in_any_mode {
        use std::future;
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
        use std::sync::Arc;

        use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::{RemoteClientAddr, ResolvedIp};
        use mockall::predicate::{self};
        use torrust_tracker_configuration::Configuration;
        use torrust_tracker_primitives::core::AnnounceData;
        use torrust_tracker_primitives::peer;
        use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};
        use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
        use torrust_tracker_test_helpers::configuration;

        use crate::event::test::announce_events_match;
        use crate::event::{ConnectionContext, Event};
        use crate::services::announce::tests::{
            initialize_core_tracker_services, initialize_core_tracker_services_with_config, sample_announce_request_for_peer,
            MockHttpStatsEventSender,
        };
        use crate::services::announce::AnnounceService;
        use crate::tests::{sample_info_hash, sample_peer, sample_peer_using_ipv4, sample_peer_using_ipv6};

        #[tokio::test]
        async fn it_should_return_the_announce_data() {
            let (core_tracker_services, core_http_tracker_services) = initialize_core_tracker_services();

            let peer = sample_peer();

            let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let announce_service = AnnounceService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.announce_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_tracker_services.whitelist_authorization.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let announce_data = announce_service
                .handle_announce(&announce_request, &client_ip_sources, &server_service_binding, None)
                .await
                .unwrap();

            let expected_announce_data = AnnounceData {
                peers: vec![],
                stats: SwarmMetadata {
                    downloaded: 0,
                    complete: 1,
                    incomplete: 0,
                },
                policy: core_tracker_services.core_config.announce_policy,
            };

            assert_eq!(announce_data, expected_announce_data);
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_4_announce_event_when_the_peer_uses_ipv4() {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();
            let peer = sample_peer_using_ipv4();
            let remote_client_ip = IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1));

            let server_service_binding_clone = server_service_binding.clone();
            let peer_copy = peer;

            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send()
                .with(predicate::function(move |event| {
                    let mut announced_peer = peer_copy;
                    announced_peer.peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080);

                    let mut announcement = peer;
                    announcement.peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080);

                    let expected_event = Event::TcpAnnounce {
                        connection: ConnectionContext::new(
                            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                            server_service_binding.clone(),
                        ),
                        info_hash: sample_info_hash(),
                        announcement,
                    };

                    announce_events_match(event, &expected_event)
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let http_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(http_stats_event_sender_mock));

            let (core_tracker_services, mut core_http_tracker_services) = initialize_core_tracker_services();

            core_http_tracker_services.http_stats_event_sender = http_stats_event_sender;

            let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

            let announce_service = AnnounceService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.announce_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_tracker_services.whitelist_authorization.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let _announce_data = announce_service
                .handle_announce(&announce_request, &client_ip_sources, &server_service_binding_clone, None)
                .await
                .unwrap();
        }

        fn tracker_with_an_ipv6_external_ip() -> Configuration {
            let mut configuration = configuration::ephemeral();
            configuration.core.net.external_ip = Some(IpAddr::V6(Ipv6Addr::new(
                0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969,
            )));
            configuration
        }

        fn peer_with_the_ipv4_loopback_ip() -> peer::Peer {
            let loopback_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
            let mut peer = sample_peer();
            peer.peer_addr = SocketAddr::new(loopback_ip, 8080);
            peer
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_4_announce_event_when_the_peer_uses_ipv4_even_if_the_tracker_changes_the_peer_ip_to_ipv6()
        {
            // Tracker changes the peer IP to the tracker external IP when the peer is using the loopback IP.

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();
            let peer = peer_with_the_ipv4_loopback_ip();
            let remote_client_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);

            let server_service_binding_clone = server_service_binding.clone();
            let peer_copy = peer;

            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send()
                .with(predicate::function(move |event| {
                    let mut announced_peer = peer_copy;
                    announced_peer.peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);

                    let mut peer_announcement = peer;
                    peer_announcement.peer_addr = SocketAddr::new(
                        IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
                        8080,
                    );

                    let expected_event = Event::TcpAnnounce {
                        connection: ConnectionContext::new(
                            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                            server_service_binding.clone(),
                        ),
                        info_hash: sample_info_hash(),
                        announcement: peer_announcement,
                    };

                    announce_events_match(event, &expected_event)
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));

            let http_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(http_stats_event_sender_mock));

            let (core_tracker_services, mut core_http_tracker_services) =
                initialize_core_tracker_services_with_config(&tracker_with_an_ipv6_external_ip());

            core_http_tracker_services.http_stats_event_sender = http_stats_event_sender;

            let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

            let announce_service = AnnounceService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.announce_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_tracker_services.whitelist_authorization.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let _announce_data = announce_service
                .handle_announce(&announce_request, &client_ip_sources, &server_service_binding_clone, None)
                .await
                .unwrap();
        }

        #[tokio::test]
        async fn it_should_send_the_tcp_6_announce_event_when_the_peer_uses_ipv6_even_if_the_tracker_changes_the_peer_ip_to_ipv4()
        {
            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();
            let peer = sample_peer_using_ipv6();
            let remote_client_ip = IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969));

            let mut http_stats_event_sender_mock = MockHttpStatsEventSender::new();
            http_stats_event_sender_mock
                .expect_send()
                .with(predicate::function(move |event| {
                    let expected_event = Event::TcpAnnounce {
                        connection: ConnectionContext::new(
                            RemoteClientAddr::new(ResolvedIp::FromSocketAddr(remote_client_ip), Some(8080)),
                            server_service_binding.clone(),
                        ),
                        info_hash: sample_info_hash(),
                        announcement: peer,
                    };
                    announce_events_match(event, &expected_event)
                }))
                .times(1)
                .returning(|_| Box::pin(future::ready(Some(Ok(1)))));
            let http_stats_event_sender: crate::event::sender::Sender = Some(Arc::new(http_stats_event_sender_mock));

            let (core_tracker_services, mut core_http_tracker_services) = initialize_core_tracker_services();
            core_http_tracker_services.http_stats_event_sender = http_stats_event_sender;

            let (announce_request, client_ip_sources) = sample_announce_request_for_peer(peer);

            let announce_service = AnnounceService::new(
                core_tracker_services.core_config.clone(),
                core_tracker_services.announce_handler.clone(),
                core_tracker_services.authentication_service.clone(),
                core_tracker_services.whitelist_authorization.clone(),
                core_http_tracker_services.http_stats_event_sender.clone(),
            );

            let server_socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7070);
            let server_service_binding = ServiceBinding::new(Protocol::HTTP, server_socket_addr).unwrap();

            let _announce_data = announce_service
                .handle_announce(&announce_request, &client_ip_sources, &server_service_binding, None)
                .await
                .unwrap();
        }
    }
}
