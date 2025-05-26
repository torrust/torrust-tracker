use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::announce_handler::PeersWanted;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use tokio::task::yield_now;
use torrust_tracker_configuration::{AnnouncePolicy, Core};
use torrust_tracker_primitives::core::AnnounceData;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::DurationSinceUnixEpoch;
use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;
use torrust_tracker_torrent_repository::container::TorrentRepositoryContainer;
use torrust_tracker_torrent_repository::Swarms;

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

async fn initialize_test_env(core_config: Core) -> (Arc<Core>, Arc<TrackerCoreContainer>, Arc<Swarms>, InfoHash, Peer) {
    let config = Arc::new(core_config);

    let info_hash = sample_info_hash();

    let peer = sample_peer();

    let (container, swarms) = start(&config).await;

    (config, container, swarms, info_hash, peer)
}

async fn start(core_config: &Arc<Core>) -> (Arc<TrackerCoreContainer>, Arc<Swarms>) {
    let torrent_repository_container = Arc::new(TorrentRepositoryContainer::initialize(
        core_config.tracker_usage_statistics.into(),
    ));

    let container = Arc::new(TrackerCoreContainer::initialize_from(
        core_config,
        &torrent_repository_container,
    ));

    let mut jobs = vec![];

    let job = torrust_tracker_torrent_repository::statistics::event::listener::run_event_listener(
        torrent_repository_container.event_bus.receiver(),
        &torrent_repository_container.stats_repository,
    );

    jobs.push(job);

    let job = bittorrent_tracker_core::statistics::event::listener::run_event_listener(
        torrent_repository_container.event_bus.receiver(),
        &container.db_torrent_repository,
    );

    jobs.push(job);

    // Give the event listeners some time to start
    // todo: they should notify when they are ready
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    (container, torrent_repository_container.swarms.clone())
}

async fn announce_peer_started(container: &Arc<TrackerCoreContainer>, peer: &mut Peer, info_hash: &InfoHash) -> AnnounceData {
    peer.event = AnnounceEvent::Started;

    let announce_data = container
        .announce_handler
        .announce(info_hash, peer, &remote_client_ip(), &PeersWanted::AsManyAsPossible)
        .await
        .unwrap();

    // Give time to the event listeners to process the event
    yield_now().await;

    announce_data
}

async fn announce_peer_completed(container: &Arc<TrackerCoreContainer>, peer: &mut Peer, info_hash: &InfoHash) -> AnnounceData {
    peer.event = AnnounceEvent::Completed;

    let announce_data = container
        .announce_handler
        .announce(info_hash, peer, &remote_client_ip(), &PeersWanted::AsManyAsPossible)
        .await
        .unwrap();

    // Give time to the event listeners to process the event
    yield_now().await;

    announce_data
}

async fn increase_number_of_downloads(container: &Arc<TrackerCoreContainer>, peer: &mut Peer, info_hash: &InfoHash) {
    let _announce_data = announce_peer_started(container, peer, info_hash).await;
    let announce_data = announce_peer_completed(container, peer, info_hash).await;

    assert_eq!(announce_data.stats.downloads(), 1);
}

#[tokio::test]
async fn it_should_handle_the_announce_request() {
    let (_config, container, _swarms, info_hash, mut peer) = initialize_test_env(ephemeral_configuration()).await;

    let announce_data = announce_peer_started(&container, &mut peer, &info_hash).await;

    assert_eq!(
        announce_data,
        AnnounceData {
            peers: vec![],
            stats: SwarmMetadata {
                downloaded: 0,
                complete: 1,
                incomplete: 0
            },
            policy: AnnouncePolicy {
                interval: 120,
                interval_min: 120
            }
        }
    );
}

#[tokio::test]
async fn it_should_not_return_the_peer_making_the_announce_request() {
    let (_config, container, _swarms, info_hash, mut peer) = initialize_test_env(ephemeral_configuration()).await;

    let announce_data = announce_peer_started(&container, &mut peer, &info_hash).await;

    assert_eq!(announce_data.peers.len(), 0);
}

#[tokio::test]
async fn it_should_handle_the_scrape_request() {
    let (_config, container, _swarms, info_hash, mut peer) = initialize_test_env(ephemeral_configuration()).await;

    let _announce_data = announce_peer_started(&container, &mut peer, &info_hash).await;

    let scrape_data = container.scrape_handler.scrape(&vec![info_hash]).await.unwrap();

    assert!(scrape_data.files.contains_key(&info_hash));
}

#[tokio::test]
async fn it_should_persist_the_number_of_completed_peers_for_all_torrents_into_the_database() {
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;

    let (_config, container, swarms, info_hash, mut peer) = initialize_test_env(core_config).await;

    increase_number_of_downloads(&container, &mut peer, &info_hash).await;

    assert!(swarms.get_swarm_metadata(&info_hash).await.unwrap().unwrap().downloads() == 1);

    swarms.remove(&info_hash).await.unwrap();

    // Make sure the swarm metadata is removed
    assert!(swarms.get_swarm_metadata(&info_hash).await.unwrap().is_none());

    // Load torrents from the database to ensure the completed stats are persisted
    container.torrents_manager.load_torrents_from_database().unwrap();

    assert!(swarms.get_swarm_metadata(&info_hash).await.unwrap().unwrap().downloads() == 1);
}
