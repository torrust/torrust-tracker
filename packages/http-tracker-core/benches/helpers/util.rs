use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
use bittorrent_http_tracker_core::event::bus::EventBus;
use bittorrent_http_tracker_core::event::sender::Broadcaster;
use bittorrent_http_tracker_core::event::Event;
use bittorrent_http_tracker_core::statistics::event::listener::run_event_listener;
use bittorrent_http_tracker_core::statistics::repository::Repository;
use bittorrent_http_tracker_protocol::v1::requests::announce::Announce;
use bittorrent_http_tracker_protocol::v1::services::peer_ip_resolver::ClientIpSources;
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::announce_handler::AnnounceHandler;
use bittorrent_tracker_core::authentication::key::repository::in_memory::InMemoryKeyRepository;
use bittorrent_tracker_core::authentication::service::AuthenticationService;
use bittorrent_tracker_core::databases::setup::initialize_database;
use bittorrent_tracker_core::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use bittorrent_tracker_core::whitelist::authorization::WhitelistAuthorization;
use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
use futures::future::BoxFuture;
use mockall::mock;
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::{Configuration, Core};
use torrust_tracker_events::sender::SendError;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::{peer, DurationSinceUnixEpoch};
use torrust_tracker_test_helpers::configuration;

pub struct CoreTrackerServices {
    pub core_config: Arc<Core>,
    pub announce_handler: Arc<AnnounceHandler>,
    pub authentication_service: Arc<AuthenticationService>,
    pub whitelist_authorization: Arc<WhitelistAuthorization>,
}

pub struct CoreHttpTrackerServices {
    pub http_stats_event_sender: bittorrent_http_tracker_core::event::sender::Sender,
}

pub fn initialize_core_tracker_services() -> (CoreTrackerServices, CoreHttpTrackerServices) {
    initialize_core_tracker_services_with_config(&configuration::ephemeral_public())
}

pub fn initialize_core_tracker_services_with_config(config: &Configuration) -> (CoreTrackerServices, CoreHttpTrackerServices) {
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

pub fn sample_peer() -> peer::Peer {
    peer::Peer {
        peer_id: PeerId(*b"-qB00000000000000000"),
        peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
        updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
        uploaded: NumberOfBytes::new(0),
        downloaded: NumberOfBytes::new(0),
        left: NumberOfBytes::new(0),
        event: AnnounceEvent::Started,
    }
}

pub fn sample_announce_request_for_peer(peer: Peer) -> (Announce, ClientIpSources) {
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
#[must_use]
pub fn sample_info_hash() -> InfoHash {
    "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0" // DevSkim: ignore DS173237
        .parse::<InfoHash>()
        .expect("String should be a valid info hash")
}

mock! {
    HttpStatsEventSender {}
    impl torrust_tracker_events::sender::Sender for HttpStatsEventSender {
        type Event = Event;

        fn send(&self, event: Event) -> BoxFuture<'static,Option<Result<usize,SendError<Event> > > > ;
    }
}
