//! Port trait for querying torrent data.
use async_trait::async_trait;
use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_rest_api_protocol::v1::resources::torrent::{ListItem, Torrent};

/// Port for querying torrent data from the tracker runtime.
///
/// Implementations of this trait adapt tracker-internal data sources
/// (e.g., `InMemoryTorrentRepository`) into protocol-level DTOs.
#[async_trait]
pub trait TorrentQueryPort: Send + Sync {
    /// Returns full torrent info including peers for the given infohash.
    async fn get_torrent_info(&self, info_hash: &InfoHash) -> Option<Torrent>;

    /// Returns a paginated list of basic torrent info (no peers).
    async fn get_torrents_page(&self, pagination: &Pagination) -> Vec<ListItem>;

    /// Returns basic torrent info for the given infohashes.
    async fn get_torrents(&self, info_hashes: &[InfoHash]) -> Vec<ListItem>;
}
