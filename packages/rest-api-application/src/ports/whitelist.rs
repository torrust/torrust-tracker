//! Port trait for whitelist command operations.
//!
//! Defines the boundary between the application layer and the
//! tracker-internal whitelist implementation. Implementations
//! live in the runtime adapter package.
use async_trait::async_trait;
use torrust_info_hash::InfoHash;
use torrust_tracker_rest_api_protocol::v1::context::whitelist::resources::whitelist::WhitelistError;

/// Port for whitelist command operations.
///
/// All whitelist operations are pure commands with no query/read
/// operations. They return either success or an error.
#[async_trait]
pub trait WhitelistCommandPort: Send + Sync {
    /// Adds a torrent to the whitelist.
    async fn add_torrent(&self, info_hash: &InfoHash) -> Result<(), WhitelistError>;

    /// Removes a torrent from the whitelist.
    async fn remove_torrent(&self, info_hash: &InfoHash) -> Result<(), WhitelistError>;

    /// Reloads the whitelist from the database into memory.
    async fn reload(&self) -> Result<(), WhitelistError>;
}
