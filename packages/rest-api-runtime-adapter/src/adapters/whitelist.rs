//! Tracker-specific implementation of [`WhitelistCommandPort`].
use std::sync::Arc;

use async_trait::async_trait;
use torrust_info_hash::InfoHash;
use torrust_tracker_core::whitelist::manager::WhitelistManager;
use torrust_tracker_rest_api_application::ports::whitelist::WhitelistCommandPort;
use torrust_tracker_rest_api_protocol::v1::context::whitelist::resources::whitelist::WhitelistError;

/// Adapter that wraps [`WhitelistManager`] and implements the
/// [`WhitelistCommandPort`] trait.
pub struct TrackerWhitelistAdapter {
    whitelist_manager: Arc<WhitelistManager>,
}

impl TrackerWhitelistAdapter {
    /// Creates a new adapter wrapping the given whitelist manager.
    #[must_use]
    pub fn new(whitelist_manager: &Arc<WhitelistManager>) -> Self {
        Self {
            whitelist_manager: whitelist_manager.clone(),
        }
    }
}

#[async_trait]
impl WhitelistCommandPort for TrackerWhitelistAdapter {
    async fn add_torrent(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
        self.whitelist_manager
            .add_torrent_to_whitelist(info_hash)
            .await
            .map_err(|e| WhitelistError::Database(e.to_string()))
    }

    async fn remove_torrent(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
        self.whitelist_manager
            .remove_torrent_from_whitelist(info_hash)
            .await
            .map_err(|e| WhitelistError::Database(e.to_string()))
    }

    async fn reload(&self) -> Result<(), WhitelistError> {
        self.whitelist_manager
            .load_whitelist_from_database()
            .await
            .map_err(|e| WhitelistError::Database(e.to_string()))
    }
}
