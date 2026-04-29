//! The [`AuthKeyStore`] trait — authentication keys context.
use async_trait::async_trait;
use mockall::automock;

use super::super::error::Error;
use crate::authentication::{self, Key};

/// Trait covering persistence operations for authentication keys.
// The `automock` macro generates a struct whose fields all end with `keys`,
// which triggers `clippy::struct_field_names` (pedantic). Suppressed here
// because the generated mock struct is outside our control.
#[async_trait]
#[allow(clippy::struct_field_names)]
#[automock]
pub trait AuthKeyStore: Sync + Send {
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
