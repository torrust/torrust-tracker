//! The [`AsyncAuthKeyStore`] trait — authentication keys context.
use async_trait::async_trait;

use crate::authentication::{self, Key};
use crate::databases::error::Error;

/// Trait covering async persistence operations for authentication keys.
#[async_trait]
pub trait AsyncAuthKeyStore: Send + Sync {
    /// Loads all authentication keys from the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the keys cannot be loaded.
    async fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error>;

    /// Retrieves a specific authentication key from the database.
    ///
    /// Returns `Some(PeerKey)` if a key corresponding to the provided [`Key`]
    /// exists, or `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be queried.
    async fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error>;

    /// Adds an authentication key to the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be saved.
    async fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error>;

    /// Removes an authentication key from the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be removed.
    async fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error>;
}
