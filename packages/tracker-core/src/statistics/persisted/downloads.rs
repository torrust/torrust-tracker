//! The repository that stored persistent torrents' data into the database.
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use crate::databases::error::Error;
use crate::databases::TorrentMetricsStore;

/// It persists torrent metrics in a database.
///
/// This repository persists only a subset of the torrent data: the torrent
/// metrics, specifically the number of downloads (or completed counts) for each
/// torrent. It relies on a database driver (either `SQLite3` or `MySQL`) that
/// implements the [`TorrentMetricsStore`] trait to perform the actual persistence
/// operations.
///
/// # Note
///
/// Not all in-memory torrent data is persisted; only the aggregate metrics are
/// stored.
pub struct DatabaseDownloadsMetricRepository {
    /// A shared reference to the torrent metrics store implementation.
    ///
    /// This allows for different underlying implementations (e.g., `SQLite3`
    /// or `MySQL`) to be used interchangeably.
    database: Arc<dyn TorrentMetricsStore>,
}

impl DatabaseDownloadsMetricRepository {
    /// Creates a new instance of `DatabaseDownloadsMetricRepository`.
    ///
    /// # Arguments
    ///
    /// * `database` - A shared reference to a torrent metrics store
    ///   implementing the [`TorrentMetricsStore`] trait.
    ///
    /// # Returns
    ///
    /// A new `DatabaseDownloadsMetricRepository` instance with a cloned
    /// reference to the provided store.
    #[must_use]
    pub fn new(database: &Arc<dyn TorrentMetricsStore>) -> DatabaseDownloadsMetricRepository {
        Self {
            database: database.clone(),
        }
    }

    // Single Torrent Metrics

    /// Increases the number of downloads for a given torrent.
    ///
    /// If the torrent is not found, it creates a new entry.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - The info hash of the torrent.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the database operation fails.
    pub(crate) async fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        let torrent = self.load_torrent_downloads(info_hash).await?;

        match torrent {
            Some(_number_of_downloads) => self.database.increase_downloads_for_torrent(info_hash).await,
            None => self.save_torrent_downloads(info_hash, 1).await,
        }
    }

    /// Loads all persistent torrent metrics from the database.
    ///
    /// This function retrieves the torrent metrics (e.g., download counts) from the persistent store
    /// and returns them as a [`PersistentTorrents`] map.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the underlying database query fails.
    pub(crate) async fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        self.database.load_all_torrents_downloads().await
    }

    /// Loads one persistent torrent metrics from the database.
    ///
    /// This function retrieves the torrent metrics (e.g., download counts) from the persistent store
    /// and returns them as a [`PersistentTorrents`] map.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the underlying database query fails.
    pub(crate) async fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        self.database.load_torrent_downloads(info_hash).await
    }

    /// Saves the persistent torrent metric into the database.
    ///
    /// This function stores or updates the download count for the torrent
    /// identified by the provided infohash.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - The info hash of the torrent.
    /// * `downloaded` - The number of times the torrent has been downloaded.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the database operation fails.
    pub(crate) async fn save_torrent_downloads(&self, info_hash: &InfoHash, downloaded: u32) -> Result<(), Error> {
        self.database.save_torrent_downloads(info_hash, downloaded).await
    }

    // Aggregate Metrics

    /// Increases the global number of downloads for all torrent.
    ///
    /// If the metric is not found, it creates it.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the database operation fails.
    pub(crate) async fn increase_global_downloads(&self) -> Result<(), Error> {
        let torrent = self.database.load_global_downloads().await?;

        match torrent {
            Some(_number_of_downloads) => self.database.increase_global_downloads().await,
            None => self.database.save_global_downloads(1).await,
        }
    }

    /// Loads the global number of downloads for all torrents from the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the underlying database query fails.
    pub(crate) async fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.database.load_global_downloads().await
    }
}

#[cfg(test)]
mod tests {

    use torrust_tracker_primitives::NumberOfDownloadsBTreeMap;

    use super::DatabaseDownloadsMetricRepository;
    use crate::databases::setup::initialize_database;
    use crate::test_helpers::tests::{ephemeral_configuration, sample_info_hash, sample_info_hash_one, sample_info_hash_two};

    async fn initialize_db_persistent_torrent_repository() -> DatabaseDownloadsMetricRepository {
        let config = ephemeral_configuration();
        let stores = initialize_database(&config).await;
        DatabaseDownloadsMetricRepository::new(&stores.torrent_metrics_store)
    }

    #[tokio::test]
    async fn it_saves_the_numbers_of_downloads_for_a_torrent_into_the_database() {
        let repository = initialize_db_persistent_torrent_repository().await;

        let infohash = sample_info_hash();

        repository.save_torrent_downloads(&infohash, 1).await.unwrap();

        let torrents = repository.load_all_torrents_downloads().await.unwrap();

        assert_eq!(torrents.get(&infohash), Some(1).as_ref());
    }

    #[tokio::test]
    async fn it_increases_the_numbers_of_downloads_for_a_torrent_into_the_database() {
        let repository = initialize_db_persistent_torrent_repository().await;

        let infohash = sample_info_hash();

        repository.increase_downloads_for_torrent(&infohash).await.unwrap();

        let torrents = repository.load_all_torrents_downloads().await.unwrap();

        assert_eq!(torrents.get(&infohash), Some(1).as_ref());
    }

    #[tokio::test]
    async fn it_loads_the_numbers_of_downloads_for_all_torrents_from_the_database() {
        let repository = initialize_db_persistent_torrent_repository().await;

        let infohash_one = sample_info_hash_one();
        let infohash_two = sample_info_hash_two();

        repository.save_torrent_downloads(&infohash_one, 1).await.unwrap();
        repository.save_torrent_downloads(&infohash_two, 2).await.unwrap();

        let torrents = repository.load_all_torrents_downloads().await.unwrap();

        let mut expected_torrents = NumberOfDownloadsBTreeMap::new();
        expected_torrents.insert(infohash_one, 1);
        expected_torrents.insert(infohash_two, 2);

        assert_eq!(torrents, expected_torrents);
    }
}
