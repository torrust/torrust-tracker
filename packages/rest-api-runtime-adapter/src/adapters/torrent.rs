//! Tracker-specific implementation of [`TorrentQueryPort`].
use std::sync::Arc;

use async_trait::async_trait;
use torrust_info_hash::InfoHash;
use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use torrust_tracker_core::torrent::services;
use torrust_tracker_primitives::pagination::Pagination;
use torrust_tracker_rest_api_application::ports::torrent::TorrentQueryPort;
use torrust_tracker_rest_api_protocol::v1::resources::torrent::{ListItem, Torrent};

use super::super::conversion;

/// Adapter that queries the in-memory torrent repository
/// and converts domain types to protocol DTOs.
pub struct TrackerTorrentQueryAdapter {
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
}

impl TrackerTorrentQueryAdapter {
    /// Creates a new adapter wrapping the in-memory repository.
    #[must_use]
    pub fn new(in_memory_torrent_repository: &Arc<InMemoryTorrentRepository>) -> Self {
        Self {
            in_memory_torrent_repository: in_memory_torrent_repository.clone(),
        }
    }
}

#[async_trait]
impl TorrentQueryPort for TrackerTorrentQueryAdapter {
    async fn get_torrent_info(&self, info_hash: &InfoHash) -> Option<Torrent> {
        services::get_torrent_info(&self.in_memory_torrent_repository, info_hash)
            .await
            .map(conversion::from_domain_info)
    }

    async fn get_torrents_page(&self, pagination: &Pagination) -> Vec<ListItem> {
        let result = services::get_torrents_page(&self.in_memory_torrent_repository, Some(pagination)).await;
        conversion::list_items_from_domain(&result)
    }

    async fn get_torrents(&self, info_hashes: &[InfoHash]) -> Vec<ListItem> {
        let result = services::get_torrents(&self.in_memory_torrent_repository, info_hashes).await;
        conversion::list_items_from_domain(&result)
    }
}
