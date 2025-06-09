//! The repository that stored persistent torrents' data into the database.
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use crate::databases::error::Error;
use crate::databases::Database;

/// It persists torrent metrics in a database.
///
/// This repository persists only a subset of the torrent data: the torrent
/// metrics, specifically the number of downloads (or completed counts) for each
/// torrent. It relies on a database driver (either `SQLite3` or `MySQL`) that
/// implements the [`Database`] trait to perform the actual persistence
/// operations.
///
/// # Note
///
/// Not all in-memory torrent data is persisted; only the aggregate metrics are
/// stored.
pub struct DatabaseDownloadsMetricRepository {
    /// A shared reference to the database driver implementation.
    ///
    /// The driver must implement the [`Database`] trait. This allows for
    /// different underlying implementations (e.g., `SQLite3` or `MySQL`) to be
    /// used interchangeably.
    database: Arc<Box<dyn Database>>,
}

impl DatabaseDownloadsMetricRepository {
    /// Creates a new instance of `DatabasePersistentTorrentRepository`.
    ///
    /// # Arguments
    ///
    /// * `database` - A shared reference to a boxed database driver
    ///   implementing the [`Database`] trait.
    ///
    /// # Returns
    ///
    /// A new `DatabasePersistentTorrentRepository` instance with a cloned
    /// reference to the provided database.
    #[must_use]
    pub fn new(database: &Arc<Box<dyn Database>>) -> DatabaseDownloadsMetricRepository {
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
    pub(crate) fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        let torrent = self.load_torrent_downloads(info_hash)?;

        match torrent {
            Some(_number_of_downloads) => self.database.increase_downloads_for_torrent(info_hash),
            None => self.save_torrent_downloads(info_hash, 1),
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
    pub(crate) fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        self.database.load_all_torrents_downloads()
    }

    /// Loads one persistent torrent metrics from the database.
    ///
    /// This function retrieves the torrent metrics (e.g., download counts) from the persistent store
    /// and returns them as a [`PersistentTorrents`] map.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the underlying database query fails.
    pub(crate) fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        self.database.load_torrent_downloads(info_hash)
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
    pub(crate) fn save_torrent_downloads(&self, info_hash: &InfoHash, downloaded: u32) -> Result<(), Error> {
        self.database.save_torrent_downloads(info_hash, downloaded)
    }

    // Aggregate Metrics

    /// Increases the global number of downloads for all torrent.
    ///
    /// If the metric is not found, it creates it.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the database operation fails.
    pub(crate) fn increase_global_downloads(&self) -> Result<(), Error> {
        let torrent = self.database.load_global_downloads()?;

        match torrent {
            Some(_number_of_downloads) => self.database.increase_global_downloads(),
            None => self.database.save_global_downloads(1),
        }
    }

    /// Loads the global number of downloads for all torrents from the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the underlying database query fails.
    pub(crate) fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.database.load_global_downloads()
    }
}

#[cfg(test)]
mod tests {

    use torrust_tracker_primitives::NumberOfDownloadsBTreeMap;

    use super::DatabaseDownloadsMetricRepository;
    use crate::databases::setup::initialize_database;
    use crate::test_helpers::tests::{ephemeral_configuration, sample_info_hash, sample_info_hash_one, sample_info_hash_two};

    fn initialize_db_persistent_torrent_repository() -> DatabaseDownloadsMetricRepository {
        let config = ephemeral_configuration();
        let database = initialize_database(&config);
        DatabaseDownloadsMetricRepository::new(&database)
    }

    #[test]
    fn it_saves_the_numbers_of_downloads_for_a_torrent_into_the_database() {
        let repository = initialize_db_persistent_torrent_repository();

        let infohash = sample_info_hash();

        repository.save_torrent_downloads(&infohash, 1).unwrap();

        let torrents = repository.load_all_torrents_downloads().unwrap();

        assert_eq!(torrents.get(&infohash), Some(1).as_ref());
    }

    #[test]
    fn it_increases_the_numbers_of_downloads_for_a_torrent_into_the_database() {
        let repository = initialize_db_persistent_torrent_repository();

        let infohash = sample_info_hash();

        repository.increase_downloads_for_torrent(&infohash).unwrap();

        let torrents = repository.load_all_torrents_downloads().unwrap();

        assert_eq!(torrents.get(&infohash), Some(1).as_ref());
    }

    #[test]
    fn it_loads_the_numbers_of_downloads_for_all_torrents_from_the_database() {
        let repository = initialize_db_persistent_torrent_repository();

        let infohash_one = sample_info_hash_one();
        let infohash_two = sample_info_hash_two();

        repository.save_torrent_downloads(&infohash_one, 1).unwrap();
        repository.save_torrent_downloads(&infohash_two, 2).unwrap();

        let torrents = repository.load_all_torrents_downloads().unwrap();

        let mut expected_torrents = NumberOfDownloadsBTreeMap::new();
        expected_torrents.insert(infohash_one, 1);
        expected_torrents.insert(infohash_two, 2);

        assert_eq!(torrents, expected_torrents);
    }
}
