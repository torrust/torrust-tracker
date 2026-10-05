//! Scrape handler.
//!
//! The `scrape` request allows clients to query metadata about the swarm in bulk.
//!
//! An `scrape` request includes a list of infohashes whose swarm metadata you
//! want to collect.
//!
//! ## Scrape Response Format
//!
//! The returned struct is:
//!
//! ```rust,no_run
//! use torrust_info_hash::InfoHash;
//! use std::collections::HashMap;
//!
//! pub struct ScrapeData {
//!     pub files: HashMap<InfoHash, SwarmMetadata>,
//! }
//!
//! pub struct SwarmMetadata {
//!     pub complete: u32,   // The number of active peers that have completed downloading (seeders)
//!     pub downloaded: u32, // The number of peers that have ever completed downloading
//!     pub incomplete: u32, // The number of active peers that have not completed downloading (leechers)
//! }
//! ```
//!
//! ## Example JSON Response
//!
//! The JSON representation of a sample `scrape` response would be like the following:
//!
//! ```json
//! {
//!     'files': {
//!       'xxxxxxxxxxxxxxxxxxxx': {'complete': 11, 'downloaded': 13772, 'incomplete': 19},
//!       'yyyyyyyyyyyyyyyyyyyy': {'complete': 21, 'downloaded': 206, 'incomplete': 20}
//!     }
//! }
//! ```
//!  
//! `xxxxxxxxxxxxxxxxxxxx` and `yyyyyyyyyyyyyyyyyyyy` are 20-byte infohash arrays.
//! There are two data structures for infohashes: byte arrays and hex strings:
//!
//! ```rust,no_run
//! use torrust_info_hash::InfoHash;
//! use std::str::FromStr;
//!
//! let info_hash: InfoHash = [255u8; 20].into();
//!
//! assert_eq!(
//!     info_hash,
//!     InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()
//! );
//! ```
//!
//! ## Reported Statistics
//!
//! A scrape reports, for each requested torrent, the swarm statistics an
//! announce would return at that moment, without any side effect (BEP 48):
//!
//! - `complete` and `incomplete` count the active peers in memory.
//! - `downloaded` is BEP 48's "ever completed" counter. The response has one
//!   field for it, so `persistent_torrent_completed_stat` decides its meaning:
//!   - disabled: completions registered while the torrent's swarm has been in
//!     memory in this process. Peerless-torrent cleanup
//!     (`remove_peerless_torrents`, enabled by default) removes the swarm and
//!     its count, so the count restarts from zero afterwards;
//!   - enabled: the persisted lifetime count. A swarm in memory already holds
//!     it, because its first announce loaded the persisted value; a torrent
//!     absent from memory reports the persisted value with zero peers.
//! - A torrent that is not whitelisted, or has neither a swarm nor a persisted
//!   count, reports zeros.
//! - A scrape never inserts a swarm, emits a swarm event, or changes retention.
//!
//! These rules are specified by tests that read as prose:
//!
//! - `packages/tracker-core/tests/integration.rs`:
//!   `it_should_scrape_the_persisted_downloads_of_a_torrent_absent_from_memory`,
//!   `it_should_not_load_a_scraped_torrent_into_memory`,
//!   `it_should_not_scrape_persisted_downloads_when_the_persistent_completed_stat_is_disabled`;
//! - the `tests` module of this file;
//! - the HTTP and UDP protocol tests named
//!   `it_should_return_the_persisted_downloads_of_*` in `http-core` and
//!   `udp-server`, and the root `persistence-scrape-after-restart` suite.
//!
//! How persisted counts are loaded (one batched, uncached query) is recorded in
//! [ADR-20261002173716](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md);
//! the domain research is in `docs/research/20261002-scrape-downloaded-semantics/`.
//!
//! ## References:
//!
//! Refer to `BitTorrent` BEPs and other sites for more information about the `scrape` request:
//!
//! - [BEP 48. Tracker Protocol Extension: Scrape](https://www.bittorrent.org/beps/bep_0048.html)
//! - [BEP 15. UDP Tracker Protocol for `BitTorrent`. Scrape section](https://www.bittorrent.org/beps/bep_0015.html)
//! - [Vuze docs](https://wiki.vuze.com/w/Scrape)
// issue: #2406
// adr: docs/adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md
use std::collections::BTreeSet;
use std::sync::Arc;

use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::ScrapeData;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

use super::torrent::repository::in_memory::InMemoryTorrentRepository;
use super::whitelist;
use crate::error::ScrapeError;
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::torrent::persisted_downloads::PersistedDownloads;

/// Handles scrape requests, providing torrent swarm metadata.
///
/// See the module documentation for the statistics a scrape reports.
pub struct ScrapeHandler {
    /// Service for authorizing access to whitelisted torrents.
    whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,

    /// The in-memory torrents repository.
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,

    /// Persisted downloads lookup for swarms absent from memory.
    persisted_downloads: PersistedDownloads,
}

impl ScrapeHandler {
    /// Creates a `ScrapeHandler` without persistent completed statistics.
    #[must_use]
    pub fn new_public(
        whitelist_authorization: &Arc<whitelist::authorization::WhitelistAuthorization>,
        in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
    ) -> Self {
        Self {
            whitelist_authorization: whitelist_authorization.clone(),
            in_memory_torrent_repository: in_memory_torrent_repository.clone(),
            persisted_downloads: PersistedDownloads::disabled(),
        }
    }

    /// Creates a `ScrapeHandler` with persistent completed statistics.
    #[must_use]
    pub fn new_with_persistent_completed_statistics(
        whitelist_authorization: &Arc<whitelist::authorization::WhitelistAuthorization>,
        in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
        db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    ) -> Self {
        Self {
            whitelist_authorization: whitelist_authorization.clone(),
            in_memory_torrent_repository: in_memory_torrent_repository.clone(),
            persisted_downloads: PersistedDownloads::enabled(db_downloads_metric_repository),
        }
    }

    /// Handles a scrape request for multiple torrents.
    ///
    /// Returns metadata for each requested torrent, following the rules in
    /// the module documentation. Torrents absent from memory are looked up in
    /// one batch when persistent completed statistics are enabled.
    ///
    /// # Errors
    ///
    /// Returns an error if loading the persisted downloads counts fails.
    ///
    /// # BEP Reference:
    ///
    /// [BEP 48: Scrape Protocol](https://www.bittorrent.org/beps/bep_0048.html)
    pub async fn handle_scrape(&self, info_hashes: &Vec<InfoHash>) -> Result<ScrapeData, ScrapeError> {
        let mut scrape_data = ScrapeData::empty();
        let mut absent_from_memory = BTreeSet::new();

        for info_hash in info_hashes {
            let swarm_metadata = match self.whitelist_authorization.authorize(info_hash).await {
                Ok(()) => self
                    .in_memory_torrent_repository
                    .get_swarm_metadata(info_hash)
                    .await
                    .unwrap_or_else(|| {
                        absent_from_memory.insert(*info_hash);
                        SwarmMetadata::zeroed()
                    }),
                Err(_) => SwarmMetadata::zeroed(),
            };
            scrape_data.add_file(info_hash, swarm_metadata);
        }

        let absent_from_memory: Vec<InfoHash> = absent_from_memory.into_iter().collect();
        for (info_hash, downloaded) in self.persisted_downloads.load_many(&absent_from_memory).await? {
            scrape_data.add_file(&info_hash, SwarmMetadata::new(downloaded, 0, 0));
        }

        Ok(scrape_data)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_info_hash::InfoHash;
    use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
    use torrust_tracker_primitives::{Driver, NumberOfDownloadsPerInfoHash, ScrapeData};
    use torrust_tracker_test_helpers::configuration;

    use super::ScrapeHandler;
    use crate::databases::error::Error as DatabaseError;
    use crate::databases::{MockTorrentMetricsStore, TorrentMetricsStore};
    use crate::error::ScrapeError;
    use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use crate::test_helpers::tests::{sample_info_hash, seeder};
    use crate::torrent::repository::in_memory::InMemoryTorrentRepository;
    use crate::whitelist::repository::in_memory::InMemoryWhitelist;
    use crate::whitelist::{self};

    fn whitelist_authorization() -> Arc<whitelist::authorization::WhitelistAuthorization> {
        let config = configuration::ephemeral_public();
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());

        Arc::new(whitelist::authorization::WhitelistAuthorization::new(
            &config.core,
            &in_memory_whitelist,
        ))
    }

    fn scrape_handler_with_persisted_downloads(
        store: MockTorrentMetricsStore,
        in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
    ) -> ScrapeHandler {
        let store: Arc<dyn TorrentMetricsStore> = Arc::new(store);

        ScrapeHandler::new_with_persistent_completed_statistics(
            &whitelist_authorization(),
            in_memory_torrent_repository,
            &Arc::new(DatabaseDownloadsMetricRepository::new(&store)),
        )
    }

    /// Scrape limits belong to the protocol parsers (UDP 74, HTTP 100), each
    /// for its own reason; core reports every info hash it receives. See
    /// `docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md`.
    #[tokio::test]
    async fn it_should_report_every_requested_torrent_without_a_limit_on_the_number_of_info_hashes() {
        // Arrange
        let scrape_handler =
            ScrapeHandler::new_public(&whitelist_authorization(), &Arc::new(InMemoryTorrentRepository::default()));
        let info_hashes: Vec<InfoHash> = (0..101u8).map(|index| InfoHash([index; 20])).collect();

        // Act
        let scrape_data = scrape_handler.handle_scrape(&info_hashes).await.unwrap();

        // Assert
        assert_eq!(scrape_data.files.len(), 101);
    }

    #[tokio::test]
    async fn it_should_report_an_in_memory_swarm_without_reading_the_persisted_downloads() {
        // Arrange
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        in_memory_torrent_repository
            .handle_announcement(&sample_info_hash(), &seeder(), Some(3))
            .await;
        let store_without_expectations = MockTorrentMetricsStore::new();
        let scrape_handler = scrape_handler_with_persisted_downloads(store_without_expectations, &in_memory_torrent_repository);

        // Act
        let scrape_data = scrape_handler.handle_scrape(&vec![sample_info_hash()]).await.unwrap();

        // Assert
        assert_eq!(
            scrape_data.files[&sample_info_hash()],
            SwarmMetadata::new(3, 1, 0),
            "an in-memory swarm (3 downloads, one seeder) should be reported as is; any database read panics the mock"
        );
    }

    #[tokio::test]
    async fn it_should_load_the_persisted_downloads_of_all_torrents_absent_from_memory_in_one_query() {
        // Arrange
        let in_memory = InfoHash([1; 20]);
        let persisted = InfoHash([2; 20]);
        let unknown = InfoHash([3; 20]);
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        in_memory_torrent_repository
            .handle_announcement(&in_memory, &seeder(), Some(3))
            .await;
        let mut store = MockTorrentMetricsStore::new();
        store
            .expect_load_torrents_downloads()
            .withf(move |info_hashes| info_hashes == [persisted, unknown])
            .times(1)
            .returning(move |_| Box::pin(std::future::ready(Ok(NumberOfDownloadsPerInfoHash::from([(persisted, 5)])))));
        let scrape_handler = scrape_handler_with_persisted_downloads(store, &in_memory_torrent_repository);
        let mut expected_scrape_data = ScrapeData::empty();
        expected_scrape_data.add_file(&in_memory, SwarmMetadata::new(3, 1, 0));
        expected_scrape_data.add_file(&persisted, SwarmMetadata::new(5, 0, 0));
        expected_scrape_data.add_file_with_zeroed_metadata(&unknown);

        // Act
        let scrape_data = scrape_handler
            .handle_scrape(&vec![in_memory, persisted, unknown])
            .await
            .unwrap();

        // Assert
        assert_eq!(
            scrape_data, expected_scrape_data,
            "in-memory swarm from memory, persisted torrent (5) and unknown torrent from one batch lookup"
        );
    }

    #[tokio::test]
    async fn it_should_look_up_a_repeated_info_hash_absent_from_memory_only_once() {
        // Arrange
        let persisted = InfoHash([2; 20]);
        let mut store = MockTorrentMetricsStore::new();
        store
            .expect_load_torrents_downloads()
            .withf(move |info_hashes| info_hashes == [persisted])
            .times(1)
            .returning(move |_| Box::pin(std::future::ready(Ok(NumberOfDownloadsPerInfoHash::from([(persisted, 5)])))));
        let scrape_handler = scrape_handler_with_persisted_downloads(store, &Arc::new(InMemoryTorrentRepository::default()));

        // Act
        let scrape_data = scrape_handler.handle_scrape(&vec![persisted, persisted]).await.unwrap();

        // Assert
        assert_eq!(
            scrape_data.files[&persisted],
            SwarmMetadata::new(5, 0, 0),
            "a repeated info-hash should be looked up once and report its persisted downloads (5)"
        );
    }

    #[tokio::test]
    async fn it_should_fail_when_loading_the_persisted_downloads_fails() {
        // Arrange
        let mut failing_store = MockTorrentMetricsStore::new();
        failing_store.expect_load_torrents_downloads().returning(|_| {
            Box::pin(std::future::ready(Err(DatabaseError::MalformedDatabaseRecord {
                message: "corrupt record".to_string(),
                driver: Driver::Sqlite3,
            })))
        });
        let scrape_handler =
            scrape_handler_with_persisted_downloads(failing_store, &Arc::new(InMemoryTorrentRepository::default()));

        // Act
        let result = scrape_handler.handle_scrape(&vec![sample_info_hash()]).await;

        // Assert
        assert!(
            matches!(result, Err(ScrapeError::Database(_))),
            "a failing persisted lookup should surface as ScrapeError::Database, got {result:?}"
        );
    }

    fn scrape_handler() -> ScrapeHandler {
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

        ScrapeHandler::new_public(&whitelist_authorization(), &in_memory_torrent_repository)
    }

    #[tokio::test]
    async fn it_should_return_a_zeroed_swarm_metadata_for_the_requested_file_if_the_tracker_does_not_have_that_torrent() {
        let scrape_handler = scrape_handler();

        let info_hashes = vec!["3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap()]; // DevSkim: ignore DS173237

        let scrape_data = scrape_handler.handle_scrape(&info_hashes).await.unwrap();

        let mut expected_scrape_data = ScrapeData::empty();

        expected_scrape_data.add_file_with_zeroed_metadata(&info_hashes[0]);

        assert_eq!(scrape_data, expected_scrape_data);
    }

    #[tokio::test]
    async fn it_should_allow_scraping_for_multiple_torrents() {
        let scrape_handler = scrape_handler();

        let info_hashes = vec![
            "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(), // DevSkim: ignore DS173237
            "99c82bb73505a3c0b453f9fa0e881d6e5a32a0c1".parse::<InfoHash>().unwrap(), // DevSkim: ignore DS173237
        ];

        let scrape_data = scrape_handler.handle_scrape(&info_hashes).await.unwrap();

        let mut expected_scrape_data = ScrapeData::empty();
        expected_scrape_data.add_file_with_zeroed_metadata(&info_hashes[0]);
        expected_scrape_data.add_file_with_zeroed_metadata(&info_hashes[1]);

        assert_eq!(scrape_data, expected_scrape_data);
    }
}
