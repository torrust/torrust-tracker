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
//! use bittorrent_primitives::info_hash::InfoHash;
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
//! use bittorrent_primitives::info_hash::InfoHash;
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
//! ## References:
//!
//! Refer to `BitTorrent` BEPs and other sites for more information about the `scrape` request:
//!
//! - [BEP 48. Tracker Protocol Extension: Scrape](https://www.bittorrent.org/beps/bep_0048.html)
//! - [BEP 15. UDP Tracker Protocol for `BitTorrent`. Scrape section](https://www.bittorrent.org/beps/bep_0015.html)
//! - [Vuze docs](https://wiki.vuze.com/w/Scrape)
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::core::ScrapeData;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

use super::torrent::repository::in_memory::InMemoryTorrentRepository;
use super::whitelist;
use crate::error::ScrapeError;

/// Handles scrape requests, providing torrent swarm metadata.
pub struct ScrapeHandler {
    /// Service for authorizing access to whitelisted torrents.
    whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,

    /// The in-memory torrents repository.
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
}

impl ScrapeHandler {
    /// Creates a new `ScrapeHandler` instance.
    #[must_use]
    pub fn new(
        whitelist_authorization: &Arc<whitelist::authorization::WhitelistAuthorization>,
        in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>,
    ) -> Self {
        Self {
            whitelist_authorization: whitelist_authorization.clone(),
            in_memory_torrent_repository: in_memory_torrent_repository.clone(),
        }
    }

    /// Handles a scrape request for multiple torrents.
    ///
    /// - Returns metadata for each requested torrent.
    /// - If a torrent isn't whitelisted or doesn't exist, returns zeroed stats.
    ///
    /// # Errors
    ///
    /// It does not return any errors for the time being. The error is returned
    /// to avoid breaking changes in the future if we decide to return errors.
    /// For example, a new tracker configuration option could be added to return
    /// an error if a torrent is not whitelisted instead of returning zeroed
    /// stats.
    ///
    /// # BEP Reference:
    ///
    /// [BEP 48: Scrape Protocol](https://www.bittorrent.org/beps/bep_0048.html)
    pub async fn scrape(&self, info_hashes: &Vec<InfoHash>) -> Result<ScrapeData, ScrapeError> {
        let mut scrape_data = ScrapeData::empty();

        for info_hash in info_hashes {
            let swarm_metadata = match self.whitelist_authorization.authorize(info_hash).await {
                Ok(()) => self.in_memory_torrent_repository.get_swarm_metadata(info_hash),
                Err(_) => SwarmMetadata::zeroed(),
            };
            scrape_data.add_file(info_hash, swarm_metadata);
        }

        Ok(scrape_data)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bittorrent_primitives::info_hash::InfoHash;
    use torrust_tracker_primitives::core::ScrapeData;
    use torrust_tracker_test_helpers::configuration;

    use super::ScrapeHandler;
    use crate::torrent::repository::in_memory::InMemoryTorrentRepository;
    use crate::whitelist::repository::in_memory::InMemoryWhitelist;
    use crate::whitelist::{self};

    fn scrape_handler() -> ScrapeHandler {
        let config = configuration::ephemeral_public();

        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(whitelist::authorization::WhitelistAuthorization::new(
            &config.core,
            &in_memory_whitelist.clone(),
        ));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

        ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository)
    }

    #[tokio::test]
    async fn it_should_return_a_zeroed_swarm_metadata_for_the_requested_file_if_the_tracker_does_not_have_that_torrent() {
        let scrape_handler = scrape_handler();

        let info_hashes = vec!["3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap()]; // DevSkim: ignore DS173237

        let scrape_data = scrape_handler.scrape(&info_hashes).await.unwrap();

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

        let scrape_data = scrape_handler.scrape(&info_hashes).await.unwrap();

        let mut expected_scrape_data = ScrapeData::empty();
        expected_scrape_data.add_file_with_zeroed_metadata(&info_hashes[0]);
        expected_scrape_data.add_file_with_zeroed_metadata(&info_hashes[1]);

        assert_eq!(scrape_data, expected_scrape_data);
    }
}
