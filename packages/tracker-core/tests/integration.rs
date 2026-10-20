mod common;

use common::fixtures::{ephemeral_configuration, remote_client_ip, sample_info_hash, sample_peer};
use common::test_env::TestEnv;
use tokio::task::yield_now;
use tokio::time::{Duration, Instant};
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{AnnounceData, AnnouncePolicy};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_swarm_coordination_registry::event::Event;

/// Publishes a harmless swarm event and returns how many receivers got it: the number of
/// event listeners still subscribed to the swarm bus.
async fn number_of_swarm_bus_listeners(container: &SwarmCoordinationRegistryContainer) -> usize {
    let sender = container
        .event_bus
        .sender()
        .expect("the swarm event bus must be enabled for this test");

    let probe = Event::TorrentRemoved {
        info_hash: sample_info_hash(),
    };

    sender
        .send(probe)
        .await
        .expect("an enabled sender always reports a result")
        .unwrap_or_default()
}

/// Waits, within one absolute deadline, until no listener is left on the swarm bus, and
/// returns how many were still there when the wait ended.
async fn number_of_swarm_bus_listeners_once_they_finish(container: &SwarmCoordinationRegistryContainer) -> usize {
    let deadline = Instant::now() + Duration::from_secs(5);

    loop {
        let remaining = number_of_swarm_bus_listeners(container).await;

        if remaining == 0 || Instant::now() >= deadline {
            return remaining;
        }

        yield_now().await;
    }
}

#[tokio::test]
async fn it_should_handle_the_announce_request() {
    let test_env = TestEnv::started(ephemeral_configuration()).await;

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
                interval_min: 120,
                max_peers_per_announce: 74,
            }
        }
    );
}

#[tokio::test]
async fn it_should_not_return_the_peer_making_the_announce_request() {
    let test_env = TestEnv::started(ephemeral_configuration()).await;

    let announce_data = test_env
        .announce_peer_started(sample_peer(), &remote_client_ip(), &sample_info_hash())
        .await;

    assert_eq!(announce_data.peers.len(), 0);
}

#[tokio::test]
async fn it_should_handle_the_scrape_request() {
    let test_env = TestEnv::started(ephemeral_configuration()).await;

    let info_hash = sample_info_hash();

    let _announce_data = test_env
        .announce_peer_started(sample_peer(), &remote_client_ip(), &info_hash)
        .await;

    let scrape_data = test_env.scrape(&info_hash).await;

    assert!(scrape_data.files.contains_key(&info_hash));
}

// issue: #2406
#[tokio::test]
async fn it_should_scrape_the_persisted_downloads_of_a_torrent_absent_from_memory() {
    // Arrange
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;
    let test_env = TestEnv::started(core_config).await;
    let info_hash = sample_info_hash();
    test_env.persist_torrent_downloads(&info_hash, 7).await;

    // Act
    let scrape_data = test_env.scrape(&info_hash).await;

    // Assert
    assert_eq!(
        scrape_data.files[&info_hash],
        SwarmMetadata {
            downloaded: 7,
            complete: 0,
            incomplete: 0
        },
        "scrape of a torrent absent from memory should report its persisted downloads (7) with persistence enabled"
    );
}

#[tokio::test]
async fn it_should_not_load_a_scraped_torrent_into_memory() {
    // Arrange
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;
    let test_env = TestEnv::started(core_config).await;
    let info_hash = sample_info_hash();
    test_env.persist_torrent_downloads(&info_hash, 7).await;

    // Act
    let _scrape_data = test_env.scrape(&info_hash).await;

    // Assert
    assert!(
        test_env.get_swarm_metadata(&info_hash).await.is_none(),
        "scrape must not insert a swarm for a torrent that was only persisted"
    );
}

#[tokio::test]
async fn it_should_not_scrape_persisted_downloads_when_the_persistent_completed_stat_is_disabled() {
    // Arrange
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = false;
    let test_env = TestEnv::started(core_config).await;
    let info_hash = sample_info_hash();
    test_env.persist_torrent_downloads(&info_hash, 7).await;

    // Act
    let scrape_data = test_env.scrape(&info_hash).await;

    // Assert
    assert_eq!(
        scrape_data.files[&info_hash],
        SwarmMetadata::zeroed(),
        "scrape should ignore the persisted downloads (7) when persistent_torrent_completed_stat is disabled"
    );
}

#[tokio::test]
async fn it_should_persist_the_number_of_completed_peers_for_each_torrent_into_the_database() {
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;

    let test_env = TestEnv::started(core_config).await;

    let info_hash = sample_info_hash();

    test_env
        .increase_number_of_downloads(sample_peer(), &remote_client_ip(), &info_hash)
        .await;

    assert_eq!(test_env.get_swarm_metadata(&info_hash).await.unwrap().downloads(), 1);

    test_env.remove_swarm(&info_hash).await;

    // Ensure the swarm metadata is removed
    assert!(test_env.get_swarm_metadata(&info_hash).await.is_none());

    // Load torrents from the database to ensure the completed stats are persisted.
    // Bound the wait with a timeout instead of a fixed iteration count so the
    // test fails loudly on a stalled system rather than after an arbitrary
    // number of immediate retries. Re-check the desired state (`downloads == 1`)
    // inside the retry condition so an intermediate observation does not
    // panic the test before the background listener has finished applying
    // the persisted value.
    let restored = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            test_env
                .tracker_core_container
                .torrents_manager
                .load_torrents_from_database(
                    &test_env
                        .tracker_core_container
                        .persistence
                        .as_ref()
                        .expect("torrent restoration test requires persistence")
                        .db_downloads_metric_repository,
                )
                .await
                .unwrap();

            if let Some(swarm_metadata) = test_env.get_swarm_metadata(&info_hash).await
                && swarm_metadata.downloads() == 1
            {
                break true;
            }

            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap_or(false);

    assert!(restored);
}

#[tokio::test]
async fn it_should_persist_the_global_number_of_completed_peers_into_the_database() {
    let mut core_config = ephemeral_configuration();

    core_config.tracker_policy.persistent_torrent_completed_stat = true;

    let test_env = TestEnv::started(core_config.clone()).await;

    test_env
        .increase_number_of_downloads(sample_peer(), &remote_client_ip(), &sample_info_hash())
        .await;

    // Wait for the event listener to persist the download count to the database
    // before simulating a restart. Without this, the new test environment may
    // start before the background task has written to the database, causing a
    // flaky failure under high-concurrency environments such as Docker builds.
    test_env.wait_for_global_downloads_persisted(1).await;
    assert_eq!(
        test_env
            .tracker_core_container
            .stats_repository
            .get_torrents_downloads_persisted_total()
            .await,
        1
    );

    // We run a new instance of the test environment to simulate a restart.
    // The new instance uses the same underlying database.

    let new_test_env = TestEnv::started(core_config).await;

    assert_eq!(
        new_test_env
            .get_counter_value("tracker_core_persistent_torrents_downloads_total")
            .await,
        1
    );
    assert_eq!(
        new_test_env
            .tracker_core_container
            .stats_repository
            .get_torrents_downloads_persisted_total()
            .await,
        1
    );
}

#[tokio::test]
async fn it_should_reset_in_session_completed_downloads_after_a_persistence_free_restart() {
    // Arrange
    let mut core_config = ephemeral_configuration();
    core_config.database = None;
    let test_env = TestEnv::started(core_config.clone()).await;

    test_env
        .increase_number_of_downloads(sample_peer(), &remote_client_ip(), &sample_info_hash())
        .await;

    // Act
    let restarted_test_env = TestEnv::started(core_config).await;

    // Assert
    assert_eq!(
        test_env
            .tracker_core_container
            .stats_repository
            .get_torrents_downloads_in_session_total()
            .await,
        1
    );
    assert_eq!(
        restarted_test_env
            .tracker_core_container
            .stats_repository
            .get_torrents_downloads_in_session_total()
            .await,
        0
    );
    assert_eq!(
        restarted_test_env
            .tracker_core_container
            .stats_repository
            .get_torrents_downloads_persisted_total()
            .await,
        0
    );
}

#[tokio::test]
async fn it_should_release_every_listener_receiver_when_the_environment_is_stopped() {
    // Arrange
    // Persistence for completed downloads is on, so the environment runs three swarm-bus
    // listeners.
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;
    let mut test_env = TestEnv::started(core_config).await;
    let listeners_before_stop = number_of_swarm_bus_listeners(&test_env.swarm_coordination_registry_container).await;

    // Act
    let stopped = test_env.stop().await;

    // Assert
    // A finished listener drops its receiver, so an event published after the stop reaches nobody.
    assert_eq!(
        stopped,
        Ok(()),
        "stopping the environment must join every listener without failure"
    );
    assert_eq!(
        listeners_before_stop, 3,
        "an environment with persistent completed statistics runs three swarm-bus listeners"
    );
    assert_eq!(
        number_of_swarm_bus_listeners(&test_env.swarm_coordination_registry_container).await,
        0,
        "no listener may remain subscribed to the swarm bus after the environment is stopped"
    );
}

#[tokio::test]
async fn it_should_cancel_every_listener_when_the_environment_is_dropped_without_being_stopped() {
    // Arrange
    // The environment runs three swarm-bus listeners. The test keeps a handle to the swarm
    // container, so it can still observe the bus after the environment is gone.
    let mut core_config = ephemeral_configuration();
    core_config.tracker_policy.persistent_torrent_completed_stat = true;
    let test_env = TestEnv::started(core_config).await;
    let swarm_container = test_env.swarm_coordination_registry_container.clone();
    let listeners_before_drop = number_of_swarm_bus_listeners(&swarm_container).await;

    // Act
    drop(test_env);

    // Assert
    // Dropping a cancellation token does not cancel it, so the cancellation comes from the
    // environment's drop guard. The listeners finish after the drop, so the test waits for them.
    assert_eq!(
        listeners_before_drop, 3,
        "an environment with persistent completed statistics runs three swarm-bus listeners"
    );
    assert_eq!(
        number_of_swarm_bus_listeners_once_they_finish(&swarm_container).await,
        0,
        "dropping the environment without stop() must still cancel every listener"
    );
}
