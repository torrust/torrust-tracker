use std::sync::Arc;

use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric_name;
use torrust_tracker_swarm_coordination_registry::event::Event;

use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::statistics::repository::Repository;
use crate::statistics::{
    TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL, TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL,
    TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL,
};

/// Handles a swarm coordination event and updates in-memory tracker statistics.
pub async fn handle_in_memory_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
    match event {
        // Torrent events
        Event::TorrentAdded { info_hash, .. } => handle_torrent_added(info_hash),
        Event::TorrentRemoved { info_hash } => handle_torrent_removed(info_hash),

        // Peer events
        Event::PeerAdded { info_hash, peer } => handle_peer_added(info_hash, peer),
        Event::PeerRemoved { info_hash, peer } => handle_peer_removed(info_hash, peer),
        Event::PeerUpdated {
            info_hash,
            old_peer,
            new_peer,
        } => handle_peer_updated(info_hash, old_peer, new_peer),
        Event::PeerDownloadCompleted { info_hash, peer } => {
            handle_peer_download_completed(info_hash, peer, stats_repository, now).await;
        }
    }
}

fn handle_torrent_added(info_hash: torrust_info_hash::InfoHash) {
    tracing::debug!(info_hash = ?info_hash, "Torrent added",);
}

fn handle_torrent_removed(info_hash: torrust_info_hash::InfoHash) {
    tracing::debug!(info_hash = ?info_hash, "Torrent removed",);
}

fn handle_peer_added(info_hash: torrust_info_hash::InfoHash, peer: torrust_tracker_primitives::peer::Peer) {
    tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer added", );
}

fn handle_peer_removed(info_hash: torrust_info_hash::InfoHash, peer: torrust_tracker_primitives::peer::Peer) {
    tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer removed", );
}

fn handle_peer_updated(
    info_hash: torrust_info_hash::InfoHash,
    old_peer: torrust_tracker_primitives::peer::Peer,
    new_peer: torrust_tracker_primitives::peer::Peer,
) {
    tracing::debug!(info_hash = ?info_hash, old_peer = ?old_peer, new_peer = ?new_peer, "Peer updated");
}

async fn handle_peer_download_completed(
    info_hash: torrust_info_hash::InfoHash,
    peer: torrust_tracker_primitives::peer::Peer,
    stats_repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer download completed", );

    increment_in_memory_download_counters(stats_repository, now).await;
}

async fn increment_in_memory_download_counters(stats_repository: &Repository, now: DurationSinceUnixEpoch) {
    let _unused = stats_repository
        .increment_counter(
            &metric_name!(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL),
            &LabelSet::default(),
            now,
        )
        .await;
    let _unused = stats_repository
        .increment_counter(
            &metric_name!(TRACKER_CORE_IN_SESSION_TORRENTS_DOWNLOADS_TOTAL),
            &LabelSet::default(),
            now,
        )
        .await;
}

/// Handles a swarm coordination event and persists completed-download statistics.
pub async fn handle_persistent_completed_statistics_event(
    event: Event,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    stats_repository: &Arc<Repository>,
    now: DurationSinceUnixEpoch,
) {
    if let Event::PeerDownloadCompleted { info_hash, .. } = event {
        increase_torrent_downloads(db_downloads_metric_repository, &info_hash).await;
        increase_global_downloads(db_downloads_metric_repository, stats_repository, now).await;
    }
}

async fn increase_torrent_downloads(
    db_downloads_metric_repository: &DatabaseDownloadsMetricRepository,
    info_hash: &torrust_info_hash::InfoHash,
) {
    match db_downloads_metric_repository.increase_downloads_for_torrent(info_hash).await {
        Ok(()) => tracing::debug!(info_hash = ?info_hash, "Number of torrent downloads increased"),
        Err(error) => {
            tracing::error!(info_hash = ?info_hash, error = ?error, "Failed to increase number of downloads for the torrent");
        }
    }
}

async fn increase_global_downloads(
    db_downloads_metric_repository: &DatabaseDownloadsMetricRepository,
    stats_repository: &Repository,
    now: DurationSinceUnixEpoch,
) {
    match db_downloads_metric_repository.increase_global_downloads().await {
        Ok(()) => {
            tracing::debug!("Global number of downloads increased");
            let _unused = stats_repository
                .increment_counter(
                    &metric_name!(TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL),
                    &LabelSet::default(),
                    now,
                )
                .await;
        }
        Err(error) => tracing::error!(error = ?error, "Failed to increase global number of downloads"),
    }
}

#[cfg(test)]
mod tests {
    use std::panic::Location;
    use std::sync::Arc;

    use rstest::rstest;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_primitives::Driver;
    use torrust_tracker_swarm_coordination_registry::event::Event;

    use super::{handle_in_memory_event, handle_persistent_completed_statistics_event};
    use crate::databases::error::Error;
    use crate::databases::setup::initialize_database;
    use crate::databases::{MockTorrentMetricsStore, TorrentMetricsStore};
    use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use crate::statistics::repository::Repository;
    use crate::test_helpers::tests::{ephemeral_configuration, sample_info_hash, sample_peer};

    fn torrent_added_event() -> Event {
        Event::TorrentAdded {
            info_hash: sample_info_hash(),
            announcement: sample_peer(),
        }
    }

    fn torrent_removed_event() -> Event {
        Event::TorrentRemoved {
            info_hash: sample_info_hash(),
        }
    }

    fn peer_added_event() -> Event {
        Event::PeerAdded {
            info_hash: sample_info_hash(),
            peer: sample_peer(),
        }
    }

    fn peer_removed_event() -> Event {
        Event::PeerRemoved {
            info_hash: sample_info_hash(),
            peer: sample_peer(),
        }
    }

    fn peer_updated_event() -> Event {
        Event::PeerUpdated {
            info_hash: sample_info_hash(),
            old_peer: sample_peer(),
            new_peer: sample_peer(),
        }
    }

    fn peer_download_completed_event() -> Event {
        Event::PeerDownloadCompleted {
            info_hash: sample_info_hash(),
            peer: sample_peer(),
        }
    }

    async fn database_downloads_repository(configuration: &Core) -> Arc<DatabaseDownloadsMetricRepository> {
        let stores = initialize_database(configuration).await;

        Arc::new(DatabaseDownloadsMetricRepository::new(&stores.torrent_metrics_store))
    }

    #[tokio::test]
    async fn it_should_increment_in_memory_completion_metrics_when_a_peer_download_is_completed() {
        // Arrange
        let stats_repository = Arc::new(Repository::default());
        let now = DurationSinceUnixEpoch::new(1, 0);

        // Act
        handle_in_memory_event(peer_download_completed_event(), &stats_repository, now).await;

        // Assert
        assert_eq!(stats_repository.get_torrents_downloads_total().await, 1);
        assert_eq!(stats_repository.get_torrents_downloads_in_session_total().await, 1);
    }

    #[rstest]
    #[case::torrent_added(torrent_added_event())]
    #[case::torrent_removed(torrent_removed_event())]
    #[case::peer_added(peer_added_event())]
    #[case::peer_removed(peer_removed_event())]
    #[case::peer_updated(peer_updated_event())]
    #[tokio::test]
    async fn it_should_change_no_in_memory_total_for_an_event_other_than_a_completed_download(#[case] event: Event) {
        // Arrange
        let stats_repository = Arc::new(Repository::default());
        let now = DurationSinceUnixEpoch::new(1, 0);

        // Act
        handle_in_memory_event(event, &stats_repository, now).await;

        // Assert
        assert_eq!(
            (
                stats_repository.get_torrents_downloads_total().await,
                stats_repository.get_torrents_downloads_in_session_total().await
            ),
            (0, 0),
            "an event other than a completed download must not change the persistent or in-session total"
        );
    }

    #[tokio::test]
    async fn it_should_persist_completion_statistics_when_a_peer_download_is_completed() {
        // Arrange
        let stats_repository = Arc::new(Repository::new(true, true));
        let ephemeral = ephemeral_configuration();
        let downloads_repository = database_downloads_repository(&ephemeral.config).await;
        let now = DurationSinceUnixEpoch::new(1, 0);

        // Act
        handle_persistent_completed_statistics_event(
            peer_download_completed_event(),
            &downloads_repository,
            &stats_repository,
            now,
        )
        .await;

        // Assert
        assert_eq!(
            downloads_repository
                .load_torrent_downloads(&sample_info_hash())
                .await
                .unwrap(),
            Some(1)
        );
        assert_eq!(downloads_repository.load_global_downloads().await.unwrap(), Some(1));
        assert_eq!(stats_repository.get_torrents_downloads_persisted_total().await, 1);
    }

    #[rstest]
    #[case::torrent_added(torrent_added_event())]
    #[case::torrent_removed(torrent_removed_event())]
    #[case::peer_added(peer_added_event())]
    #[case::peer_removed(peer_removed_event())]
    #[case::peer_updated(peer_updated_event())]
    #[tokio::test]
    async fn it_should_persist_nothing_for_an_event_other_than_a_completed_download(#[case] event: Event) {
        // Arrange
        let stats_repository = Arc::new(Repository::new(true, true));
        let ephemeral = ephemeral_configuration();
        let downloads_repository = database_downloads_repository(&ephemeral.config).await;
        let now = DurationSinceUnixEpoch::new(1, 0);

        // Act
        handle_persistent_completed_statistics_event(event, &downloads_repository, &stats_repository, now).await;

        // Assert
        assert_eq!(
            (
                downloads_repository.load_all_torrents_downloads().await.unwrap().len(),
                downloads_repository.load_global_downloads().await.unwrap(),
                stats_repository.get_torrents_downloads_persisted_total().await
            ),
            (0, None, 0),
            "an event other than a completed download must not persist a download for any torrent"
        );
    }

    #[tokio::test]
    async fn it_should_persist_global_completion_statistics_when_torrent_persistence_fails() {
        // Arrange
        let stats_repository = Arc::new(Repository::new(true, true));
        let mut store = MockTorrentMetricsStore::new();
        store.expect_load_torrent_downloads().returning(|_| {
            Box::pin(std::future::ready(Err(Error::UpdateFailed {
                location: Location::caller(),
                driver: Driver::Sqlite3,
            })))
        });
        store
            .expect_load_global_downloads()
            .returning(|| Box::pin(std::future::ready(Ok(None))));
        store
            .expect_save_global_downloads()
            .returning(|_| Box::pin(std::future::ready(Ok(()))));
        let store: Arc<dyn TorrentMetricsStore> = Arc::new(store);
        let downloads_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&store));
        let now = DurationSinceUnixEpoch::new(1, 0);

        // Act
        handle_persistent_completed_statistics_event(
            peer_download_completed_event(),
            &downloads_repository,
            &stats_repository,
            now,
        )
        .await;

        // Assert
        assert_eq!(stats_repository.get_torrents_downloads_persisted_total().await, 1);
    }

    #[tokio::test]
    async fn it_should_not_increment_the_persisted_total_when_the_global_downloads_cannot_be_increased() {
        // Arrange
        let stats_repository = Arc::new(Repository::new(true, true));
        let mut store = MockTorrentMetricsStore::new();
        store
            .expect_load_torrent_downloads()
            .returning(|_| Box::pin(std::future::ready(Ok(None))));
        store
            .expect_save_torrent_downloads()
            .times(1)
            .returning(|_, _| Box::pin(std::future::ready(Ok(()))));
        store.expect_load_global_downloads().times(1).returning(|| {
            Box::pin(std::future::ready(Err(Error::MalformedDatabaseRecord {
                message: "corrupt global downloads".to_string(),
                driver: Driver::Sqlite3,
            })))
        });
        let store: Arc<dyn TorrentMetricsStore> = Arc::new(store);
        let downloads_repository = Arc::new(DatabaseDownloadsMetricRepository::new(&store));
        let now = DurationSinceUnixEpoch::new(1, 0);

        // Act
        handle_persistent_completed_statistics_event(
            peer_download_completed_event(),
            &downloads_repository,
            &stats_repository,
            now,
        )
        .await;

        // Assert
        assert_eq!(
            stats_repository.get_torrents_downloads_persisted_total().await,
            0,
            "the persisted total must follow the database, which could not read the global downloads to increase them"
        );
    }
}
