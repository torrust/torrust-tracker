//! Torrents manager.
use std::sync::Arc;
use std::time::Duration;

use torrust_tracker_clock::clock::Time;
use torrust_tracker_configuration::Core;

use super::repository::in_memory::InMemoryTorrentRepository;
use super::repository::persisted::DatabasePersistentTorrentRepository;
use crate::{databases, CurrentClock};

/// The `TorrentsManager` is responsible for managing torrent entries by
/// integrating persistent storage and in-memory state. It provides methods to
/// load torrent data from the database into memory, and to periodically clean
/// up stale torrent entries by removing inactive peers or entire torrent
/// entries that no longer have active peers.
///
/// This manager relies on two repositories:
///
/// - An **in-memory repository** to provide fast access to the current torrent
///   state.
/// - A **persistent repository** that stores aggregate torrent metrics (e.g.,
///   seeders count) across tracker restarts.
pub struct TorrentsManager {
    /// The tracker configuration.
    config: Core,

    /// The in-memory torrents repository.
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,

    /// The persistent torrents repository.
    #[allow(dead_code)]
    db_torrent_repository: Arc<DatabasePersistentTorrentRepository>,
}

impl TorrentsManager {
    /// Creates a new instance of `TorrentsManager`.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the tracker configuration.
    /// * `in_memory_torrent_repository` - A shared reference to the in-memory
    ///   repository of torrents.
    /// * `db_torrent_repository` - A shared reference to the persistent
    ///   repository for torrent metrics.
    ///
    /// # Returns
    ///
    /// A new `TorrentsManager` instance with cloned references of the provided dependencies.
    #[must_use]
    pub fn new(
        config: &Core,
        in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
        db_torrent_repository: &Arc<DatabasePersistentTorrentRepository>,
    ) -> Self {
        Self {
            config: config.clone(),
            in_memory_torrent_repository: in_memory_torrent_repository.clone(),
            db_torrent_repository: db_torrent_repository.clone(),
        }
    }

    /// Loads torrents from the persistent database into the in-memory repository.
    ///
    /// This function retrieves the list of persistent torrent entries (which
    /// include only the aggregate metrics, not the detailed peer lists) from
    /// the database, and then imports that data into the in-memory repository.
    ///
    /// # Errors
    ///
    /// Returns a `databases::error::Error` if unable to load the persistent
    /// torrent data.
    #[allow(dead_code)]
    pub(crate) fn load_torrents_from_database(&self) -> Result<(), databases::error::Error> {
        let persistent_torrents = self.db_torrent_repository.load_all()?;

        self.in_memory_torrent_repository.import_persistent(&persistent_torrents);

        Ok(())
    }

    /// Cleans up torrent entries by removing inactive peers and, optionally,
    /// torrents with no active peers.
    ///
    /// This function performs two cleanup tasks:
    ///
    /// 1. It removes peers from torrent entries that have not been updated
    ///    within a cutoff time. The cutoff time is calculated as the current
    ///    time minus the maximum allowed peer timeout, as specified in the
    ///    tracker policy.
    /// 2. If the tracker is configured to remove peerless torrents
    ///    (`remove_peerless_torrents` is set), it removes entire torrent
    ///    entries that have no active peers.
    pub fn cleanup_torrents(&self) {
        self.log_aggregate_swarm_metadata();

        self.remove_inactive_peers();

        self.log_aggregate_swarm_metadata();

        self.remove_peerless_torrents();

        self.log_aggregate_swarm_metadata();
    }

    fn remove_inactive_peers(&self) {
        let current_cutoff = CurrentClock::now_sub(&Duration::from_secs(u64::from(self.config.tracker_policy.max_peer_timeout)))
            .unwrap_or_default();

        self.in_memory_torrent_repository.remove_inactive_peers(current_cutoff);
    }

    fn remove_peerless_torrents(&self) {
        if self.config.tracker_policy.remove_peerless_torrents {
            self.in_memory_torrent_repository
                .remove_peerless_torrents(&self.config.tracker_policy);
        }
    }

    fn log_aggregate_swarm_metadata(&self) {
        // Pre-calculated data
        let aggregate_swarm_metadata = self.in_memory_torrent_repository.get_aggregate_swarm_metadata();

        tracing::info!(name: "pre_calculated_aggregate_swarm_metadata",
            torrents = aggregate_swarm_metadata.total_torrents,
            downloads = aggregate_swarm_metadata.total_downloaded,
            seeders = aggregate_swarm_metadata.total_complete,
            leechers = aggregate_swarm_metadata.total_incomplete,
        );

        // Hot data (iterating over data structures)
        let peerless_torrents = self.in_memory_torrent_repository.count_peerless_torrents();
        let peers = self.in_memory_torrent_repository.count_peers();

        tracing::info!(name: "hot_aggregate_swarm_metadata",
            peerless_torrents = peerless_torrents,
            peers = peers,
        );
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use torrust_tracker_configuration::Core;
    use torrust_tracker_torrent_repository::LockTrackedTorrent;

    use super::{DatabasePersistentTorrentRepository, TorrentsManager};
    use crate::databases::setup::initialize_database;
    use crate::test_helpers::tests::{ephemeral_configuration, sample_info_hash};
    use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

    struct TorrentsManagerDeps {
        config: Arc<Core>,
        in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
        database_persistent_torrent_repository: Arc<DatabasePersistentTorrentRepository>,
    }

    fn initialize_torrents_manager() -> (Arc<TorrentsManager>, Arc<TorrentsManagerDeps>) {
        let config = ephemeral_configuration();
        initialize_torrents_manager_with(config.clone())
    }

    fn initialize_torrents_manager_with(config: Core) -> (Arc<TorrentsManager>, Arc<TorrentsManagerDeps>) {
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let database = initialize_database(&config);
        let database_persistent_torrent_repository = Arc::new(DatabasePersistentTorrentRepository::new(&database));

        let torrents_manager = Arc::new(TorrentsManager::new(
            &config,
            &in_memory_torrent_repository,
            &database_persistent_torrent_repository,
        ));

        (
            torrents_manager,
            Arc::new(TorrentsManagerDeps {
                config: Arc::new(config),
                in_memory_torrent_repository,
                database_persistent_torrent_repository,
            }),
        )
    }

    #[test]
    fn it_should_load_the_numbers_of_downloads_for_all_torrents_from_the_database() {
        let (torrents_manager, services) = initialize_torrents_manager();

        let infohash = sample_info_hash();

        services.database_persistent_torrent_repository.save(&infohash, 1).unwrap();

        torrents_manager.load_torrents_from_database().unwrap();

        assert_eq!(
            services
                .in_memory_torrent_repository
                .get(&infohash)
                .unwrap()
                .lock_or_panic()
                .metadata()
                .downloaded,
            1
        );
    }

    mod cleaning_torrents {
        use std::ops::Add;
        use std::sync::Arc;
        use std::time::Duration;

        use bittorrent_primitives::info_hash::InfoHash;
        use torrust_tracker_clock::clock::stopped::Stopped;
        use torrust_tracker_clock::clock::{self};
        use torrust_tracker_primitives::DurationSinceUnixEpoch;

        use crate::test_helpers::tests::{ephemeral_configuration, sample_info_hash, sample_peer};
        use crate::torrent::manager::tests::{initialize_torrents_manager, initialize_torrents_manager_with};
        use crate::torrent::repository::in_memory::InMemoryTorrentRepository;

        #[test]
        fn it_should_remove_peers_that_have_not_been_updated_after_a_cutoff_time() {
            let (torrents_manager, services) = initialize_torrents_manager();

            let infohash = sample_info_hash();

            clock::Stopped::local_set(&Duration::from_secs(0));

            // Add a peer to the torrent
            let mut peer = sample_peer();
            peer.updated = DurationSinceUnixEpoch::new(0, 0);
            let _number_of_downloads_increased = services.in_memory_torrent_repository.upsert_peer(&infohash, &peer, None);

            // Simulate the time has passed 1 second more than the max peer timeout.
            clock::Stopped::local_add(&Duration::from_secs(
                (services.config.tracker_policy.max_peer_timeout + 1).into(),
            ))
            .unwrap();

            torrents_manager.cleanup_torrents();

            assert!(services.in_memory_torrent_repository.get(&infohash).is_none());
        }

        fn add_a_peerless_torrent(infohash: &InfoHash, in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>) {
            // Add a peer to the torrent
            let mut peer = sample_peer();
            peer.updated = DurationSinceUnixEpoch::new(0, 0);
            let _number_of_downloads_increased = in_memory_torrent_repository.upsert_peer(infohash, &peer, None);

            // Remove the peer. The torrent is now peerless.
            in_memory_torrent_repository.remove_inactive_peers(peer.updated.add(Duration::from_secs(1)));
        }

        #[test]
        fn it_should_remove_torrents_that_have_no_peers_when_it_is_configured_to_do_so() {
            let mut config = ephemeral_configuration();
            config.tracker_policy.remove_peerless_torrents = true;

            let (torrents_manager, services) = initialize_torrents_manager_with(config);

            let infohash = sample_info_hash();

            add_a_peerless_torrent(&infohash, &services.in_memory_torrent_repository);

            torrents_manager.cleanup_torrents();

            assert!(services.in_memory_torrent_repository.get(&infohash).is_none());
        }

        #[test]
        fn it_should_retain_peerless_torrents_when_it_is_configured_to_do_so() {
            let mut config = ephemeral_configuration();
            config.tracker_policy.remove_peerless_torrents = false;

            let (torrents_manager, services) = initialize_torrents_manager_with(config);

            let infohash = sample_info_hash();

            add_a_peerless_torrent(&infohash, &services.in_memory_torrent_repository);

            torrents_manager.cleanup_torrents();

            assert!(services.in_memory_torrent_repository.get(&infohash).is_some());
        }
    }
}
