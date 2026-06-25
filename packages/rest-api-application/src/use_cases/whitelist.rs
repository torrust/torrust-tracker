//! Use-case service for whitelist API operations.
//!
//! Orchestrates calls to the [`WhitelistCommandPort`] and adds business logic
//! such as validation, error mapping, or caching as needed.
use torrust_info_hash::InfoHash;
use torrust_tracker_rest_api_protocol::v1::context::whitelist::resources::whitelist::WhitelistError;

use crate::ports::whitelist::WhitelistCommandPort;

/// Use-case service for whitelist-related API operations.
///
/// Delegates to a [`WhitelistCommandPort`] implementation (tracker adapter)
/// and maps domain errors to protocol error types.
pub struct WhitelistApiService {
    command_port: Box<dyn WhitelistCommandPort>,
}

impl WhitelistApiService {
    /// Creates a new service backed by the given port implementation.
    #[must_use]
    pub fn new(command_port: Box<dyn WhitelistCommandPort>) -> Self {
        Self { command_port }
    }

    /// Adds a torrent to the whitelist.
    ///
    /// # Errors
    ///
    /// Returns a [`WhitelistError`] if the database operation fails.
    pub async fn add_torrent(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
        self.command_port.add_torrent(info_hash).await
    }

    /// Removes a torrent from the whitelist.
    ///
    /// # Errors
    ///
    /// Returns a [`WhitelistError`] if the database operation fails.
    pub async fn remove_torrent(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
        self.command_port.remove_torrent(info_hash).await
    }

    /// Reloads the whitelist from the database.
    ///
    /// # Errors
    ///
    /// Returns a [`WhitelistError`] if the database operation fails.
    pub async fn reload(&self) -> Result<(), WhitelistError> {
        self.command_port.reload().await
    }
}
