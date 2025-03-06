use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::announce_handler::{AnnounceHandler, PeersWanted};
use bittorrent_tracker_core::databases::setup::initialize_database;
use bittorrent_tracker_core::scrape_handler::ScrapeHandler;
use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use bittorrent_tracker_core::torrent::repository::persisted::DatabasePersistentTorrentRepository;
use bittorrent_tracker_core::whitelist;
use bittorrent_tracker_core::whitelist::repository::in_memory::InMemoryWhitelist;
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::DurationSinceUnixEpoch;
use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;

/// # Panics
///
/// Will panic if the temporary file path is not a valid UTF-8 string.
#[must_use]
pub fn ephemeral_configuration() -> Core {
    let mut config = Core::default();

    let temp_file = ephemeral_sqlite_database();
    temp_file.to_str().unwrap().clone_into(&mut config.database.path);

    config
}

/// # Panics
///
/// Will panic if the string representation of the info hash is not a valid infohash.
#[must_use]
pub fn sample_info_hash() -> InfoHash {
    "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0" // DevSkim: ignore DS173237
        .parse::<InfoHash>()
        .expect("String should be a valid info hash")
}

/// Sample peer whose state is not relevant for the tests.
#[must_use]
pub fn sample_peer() -> Peer {
    Peer {
        peer_id: PeerId(*b"-qB00000000000000000"),
        peer_addr: SocketAddr::new(remote_client_ip(), 8080),
        updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
        uploaded: NumberOfBytes::new(0),
        downloaded: NumberOfBytes::new(0),
        left: NumberOfBytes::new(0), // No bytes left to download
        event: AnnounceEvent::Completed,
    }
}

// The client peer IP.
#[must_use]
fn remote_client_ip() -> IpAddr {
    IpAddr::V4(Ipv4Addr::from_str("126.0.0.1").unwrap())
}

struct Container {
    pub announce_handler: Arc<AnnounceHandler>,
    pub scrape_handler: Arc<ScrapeHandler>,
}

impl Container {
    pub fn initialize(config: &Core) -> Self {
        let database = initialize_database(config);
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_torrent_repository = Arc::new(DatabasePersistentTorrentRepository::new(&database));
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(whitelist::authorization::WhitelistAuthorization::new(
            config,
            &in_memory_whitelist.clone(),
        ));
        let announce_handler = Arc::new(AnnounceHandler::new(
            config,
            &whitelist_authorization,
            &in_memory_torrent_repository,
            &db_torrent_repository,
        ));
        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));

        Self {
            announce_handler,
            scrape_handler,
        }
    }
}

#[tokio::test]
async fn test_announce_and_scrape_requests() {
    let config = ephemeral_configuration();

    let container = Container::initialize(&config);

    let info_hash = sample_info_hash();

    let mut peer = sample_peer();

    // Announce

    // First announce: download started
    peer.event = AnnounceEvent::Started;
    let announce_data = container
        .announce_handler
        .announce(&info_hash, &mut peer, &remote_client_ip(), &PeersWanted::AsManyAsPossible)
        .await
        .unwrap();

    // NOTICE: you don't get back the peer making the request.
    assert_eq!(announce_data.peers.len(), 0);
    assert_eq!(announce_data.stats.downloaded, 0);

    // Second announce: download completed
    peer.event = AnnounceEvent::Completed;
    let announce_data = container
        .announce_handler
        .announce(&info_hash, &mut peer, &remote_client_ip(), &PeersWanted::AsManyAsPossible)
        .await
        .unwrap();

    assert_eq!(announce_data.peers.len(), 0);
    assert_eq!(announce_data.stats.downloaded, 1);

    // Scrape

    let scrape_data = container.scrape_handler.scrape(&vec![info_hash]).await.unwrap();

    assert!(scrape_data.files.contains_key(&info_hash));
}

#[test]
fn test_scrape_request() {}
