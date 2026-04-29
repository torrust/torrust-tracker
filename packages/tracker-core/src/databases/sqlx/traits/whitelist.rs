//! The [`AsyncWhitelistStore`] trait — torrent whitelist context.
use async_trait::async_trait;
use bittorrent_primitives::info_hash::InfoHash;

use crate::databases::error::Error;

/// Trait covering async persistence operations for the torrent whitelist.
#[async_trait]
pub trait AsyncWhitelistStore: Send + Sync {
    /// Loads the whitelisted torrents from the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be loaded.
    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error>;

    /// Retrieves a whitelisted torrent from the database.
    ///
    /// Returns `Some(InfoHash)` if the torrent is in the whitelist, or `None`
    /// otherwise.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be queried.
    async fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error>;

    /// Adds a torrent to the whitelist.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the torrent cannot be added to the whitelist.
    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    /// Removes a torrent from the whitelist.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the torrent cannot be removed from the whitelist.
    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    /// Checks whether a torrent is whitelisted.
    ///
    /// This default implementation returns `true` if the infohash is included
    /// in the whitelist, or `false` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be queried.
    async fn is_info_hash_whitelisted(&self, info_hash: InfoHash) -> Result<bool, Error> {
        Ok(self.get_info_hash_from_whitelist(info_hash).await?.is_some())
    }
}
