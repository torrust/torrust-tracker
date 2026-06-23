//! Use-case service for torrent API operations.
use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_rest_api_protocol::v1::resources::torrent::{ListItem, Torrent};

use crate::ports::torrent::TorrentQueryPort;

/// Use-case service for torrent-related API operations.
///
/// Orchestrates calls to the [`TorrentQueryPort`] and adds business logic
/// such as validation, error mapping, or caching as needed.
pub struct TorrentApiService {
    query_port: Box<dyn TorrentQueryPort>,
}

impl TorrentApiService {
    /// Creates a new service backed by the given port implementation.
    #[must_use]
    pub fn new(query_port: Box<dyn TorrentQueryPort>) -> Self {
        Self { query_port }
    }

    /// Returns full torrent info including peers.
    pub async fn get_torrent(&self, info_hash: &InfoHash) -> Option<Torrent> {
        self.query_port.get_torrent_info(info_hash).await
    }

    /// Returns a paginated list of torrents.
    pub async fn get_torrents_page(&self, pagination: &Pagination) -> Vec<ListItem> {
        self.query_port.get_torrents_page(pagination).await
    }

    /// Returns torrents for specific infohashes.
    pub async fn get_torrents(&self, info_hashes: &[InfoHash]) -> Vec<ListItem> {
        self.query_port.get_torrents(info_hashes).await
    }
}

// Manual Send + Sync: the service is Send+Sync if its inner port is.
// Since TorrentQueryPort: Send + Sync, and Box<dyn TorrentQueryPort>
// is Send + Sync, this is automatically satisfied.
