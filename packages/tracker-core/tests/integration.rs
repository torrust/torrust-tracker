mod common;

use common::fixtures::{ephemeral_configuration, remote_client_ip, sample_info_hash, sample_peer};
use common::test_env::TestEnv;
use torrust_tracker_configuration::AnnouncePolicy;
use torrust_tracker_primitives::core::AnnounceData;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

#[tokio::test]
async fn it_should_handle_the_announce_request() {
    let mut test_env = TestEnv::started(ephemeral_configuration()).await;

    let announce_data = test_env
        .announce_peer_started(sample_peer(), &remote_client_ip(), &sample_info_hash())
        .await;

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
    let mut test_env = TestEnv::started(ephemeral_configuration()).await;

    let announce_data = test_env
        .announce_peer_started(sample_peer(), &remote_client_ip(), &sample_info_hash())
        .await;

    assert_eq!(announce_data.peers.len(), 0);
}

#[tokio::test]
async fn it_should_handle_the_scrape_request() {
    let mut test_env = TestEnv::started(ephemeral_configuration()).await;

    let info_hash = sample_info_hash();

    let _announce_data = test_env
        .announce_peer_started(sample_peer(), &remote_client_ip(), &info_hash)
        .await;

    let scrape_data = test_env.scrape(&info_hash).await;

    assert!(scrape_data.files.contains_key(&info_hash));
}

#[tokio::test]
async fn it_should_persist_the_number_of_completed_peers_for_each_torrent_into_the_database() {
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;

    let mut test_env = TestEnv::started(core_config).await;

    let info_hash = sample_info_hash();

    test_env
        .increase_number_of_downloads(sample_peer(), &remote_client_ip(), &info_hash)
        .await;

    assert!(test_env.get_swarm_metadata(&info_hash).await.unwrap().downloads() == 1);

    test_env.remove_swarm(&info_hash).await;

    // Ensure the swarm metadata is removed
    assert!(test_env.get_swarm_metadata(&info_hash).await.is_none());

    // Load torrents from the database to ensure the completed stats are persisted
    test_env
        .tracker_core_container
        .torrents_manager
        .load_torrents_from_database()
        .unwrap();

    assert!(test_env.get_swarm_metadata(&info_hash).await.unwrap().downloads() == 1);
}

#[tokio::test]
async fn it_should_persist_the_global_number_of_completed_peers_into_the_database() {
    let mut core_config = ephemeral_configuration();

    core_config.tracker_policy.persistent_torrent_completed_stat = true;

    let mut test_env = TestEnv::started(core_config.clone()).await;

    test_env
        .increase_number_of_downloads(sample_peer(), &remote_client_ip(), &sample_info_hash())
        .await;

    // We run a new instance of the test environment to simulate a restart.
    // The new instance uses the same underlying database.

    let new_test_env = TestEnv::started(core_config).await;

    assert_eq!(
        new_test_env
            .get_counter_value("tracker_core_persistent_torrents_downloads_total")
            .await,
        1
    );
}
