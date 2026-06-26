//! Use-case service for authentication key API operations.
//!
//! Orchestrates calls to the [`AuthKeyPort`] and adds business logic
//! such as validation, error mapping, or caching as needed.
use torrust_tracker_rest_api_protocol::v1::context::auth_key::forms::add_key_form::AddKeyForm;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::{AuthKey, AuthKeyError};

use crate::ports::auth_key::AuthKeyPort;

/// Use-case service for auth-key-related API operations.
///
/// Delegates to an [`AuthKeyPort`] implementation (tracker adapter)
/// and maps domain errors to protocol error types.
pub struct AuthKeyApiService {
    port: Box<dyn AuthKeyPort>,
}

impl AuthKeyApiService {
    /// Creates a new service backed by the given port implementation.
    #[must_use]
    pub fn new(port: Box<dyn AuthKeyPort>) -> Self {
        Self { port }
    }

    /// Adds a new peer key.
    ///
    /// # Errors
    ///
    /// Returns an [`AuthKeyError`] if the operation fails.
    pub async fn add_key(&self, form: &AddKeyForm) -> Result<AuthKey, AuthKeyError> {
        self.port.add_key(form).await
    }

    /// Generates a new expiring peer key.
    ///
    /// # Errors
    ///
    /// Returns an [`AuthKeyError`] if the operation fails.
    pub async fn generate_key(&self, seconds_valid: u64) -> Result<AuthKey, AuthKeyError> {
        self.port.generate_key(seconds_valid).await
    }

    /// Deletes an authentication key.
    ///
    /// # Errors
    ///
    /// Returns an [`AuthKeyError`] if the operation fails.
    pub async fn delete_key(&self, key: &str) -> Result<(), AuthKeyError> {
        self.port.delete_key(key).await
    }

    /// Reloads authentication keys from the database.
    ///
    /// # Errors
    ///
    /// Returns an [`AuthKeyError`] if the operation fails.
    pub async fn reload_keys(&self) -> Result<(), AuthKeyError> {
        self.port.reload_keys().await
    }
}
