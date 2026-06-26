//! Port trait for authentication key operations.
//!
//! Defines the boundary between the application layer and the
//! tracker-internal key management implementation. Implementations
//! live in the runtime adapter package.
use async_trait::async_trait;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::forms::add_key_form::AddKeyForm;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::{AuthKey, AuthKeyError};

/// Port for authentication key operations.
///
/// Covers both command and query operations: adding/generating/deleting
/// keys, and reloading them from the database.
#[async_trait]
pub trait AuthKeyPort: Send + Sync {
    /// Adds a new peer key (pre-generated or generated on-the-fly).
    async fn add_key(&self, form: &AddKeyForm) -> Result<AuthKey, AuthKeyError>;

    /// Generates a new expiring peer key with the given lifetime in seconds.
    async fn generate_key(&self, seconds_valid: u64) -> Result<AuthKey, AuthKeyError>;

    /// Deletes an authentication key.
    async fn delete_key(&self, key: &str) -> Result<(), AuthKeyError>;

    /// Reloads authentication keys from the database into memory.
    async fn reload_keys(&self) -> Result<(), AuthKeyError>;
}
