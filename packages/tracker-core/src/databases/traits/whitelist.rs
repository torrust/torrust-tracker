//! The [`WhitelistStore`] trait — torrent whitelist context.
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use torrust_info_hash::InfoHash;

use super::super::error::Error;

/// Trait covering persistence operations for the torrent whitelist.
// `async_trait` applies `#[must_use]` to generated futures. Nightly Clippy also treats those
// futures as must-use and reports the macro expansion as redundant.
#[allow(
    clippy::double_must_use,
    reason = "async_trait generates must-use futures that Clippy also infers as must-use"
)]
#[async_trait]
#[allow(
    clippy::extra_unused_lifetimes,
    reason = "async_trait generates lifetimes outside workspace control"
)]
#[cfg_attr(test, automock)]
pub trait WhitelistStore: Sync + Send {
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
    /// Idempotent: adding a torrent that is already listed is not an error, and
    /// concurrent calls for one torrent all succeed. Returns the row count the
    /// database reports: `1` when the torrent was added, and `0` when it was already
    /// listed on `SQLite` and `PostgreSQL`. `MySQL` reports `1` in both cases (the
    /// driver connects with the found-rows flag), so do not use it to tell whether
    /// the torrent was newly added.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the database cannot be written.
    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    /// Removes a torrent from the whitelist.
    ///
    /// Idempotent: removing a torrent that is not listed is not an error. Returns
    /// the number of rows removed: `1`, or `0` when the torrent was not listed. It
    /// is atomic, so concurrent calls for one torrent all succeed and, when the
    /// torrent was listed, their row counts add up to `1`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the database cannot be written.
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
