//! Tracker-specific implementation of [`AuthKeyPort`].
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use torrust_tracker_core::authentication::handler::{AddKeyRequest, KeysHandler};
use torrust_tracker_core::authentication::{Key, PeerKey};
use torrust_tracker_rest_api_application::ports::auth_key::AuthKeyPort;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::forms::add_key_form::AddKeyForm;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::{AuthKey, AuthKeyError};

/// Adapter that wraps [`KeysHandler`] and implements the [`AuthKeyPort`] trait.
pub struct TrackerAuthKeyAdapter {
    keys_handler: Arc<KeysHandler>,
}

impl TrackerAuthKeyAdapter {
    /// Creates a new adapter wrapping the given keys handler.
    #[must_use]
    pub fn new(keys_handler: &Arc<KeysHandler>) -> Self {
        Self {
            keys_handler: keys_handler.clone(),
        }
    }
}

#[async_trait]
impl AuthKeyPort for TrackerAuthKeyAdapter {
    async fn add_key(&self, form: &AddKeyForm) -> Result<AuthKey, AuthKeyError> {
        let result = self
            .keys_handler
            .add_peer_key(AddKeyRequest {
                opt_key: form.opt_key.clone(),
                opt_seconds_valid: form.opt_seconds_valid,
            })
            .await;

        result.map(peer_key_to_auth_key).map_err(map_peer_key_error)
    }

    async fn generate_key(&self, seconds_valid: u64) -> Result<AuthKey, AuthKeyError> {
        let result = self
            .keys_handler
            .generate_expiring_peer_key(Some(std::time::Duration::from_secs(seconds_valid)))
            .await;

        result
            .map(peer_key_to_auth_key)
            .map_err(|e| AuthKeyError::Database(e.to_string()))
    }

    async fn delete_key(&self, key: &str) -> Result<(), AuthKeyError> {
        match Key::from_str(key) {
            Err(_) => Err(AuthKeyError::InvalidKey {
                key: key.to_string(),
                reason: "invalid key format".to_string(),
            }),
            Ok(key) => self
                .keys_handler
                .remove_peer_key(&key)
                .await
                .map_err(|e| AuthKeyError::Database(e.to_string())),
        }
    }

    async fn reload_keys(&self) -> Result<(), AuthKeyError> {
        self.keys_handler
            .load_peer_keys_from_database()
            .await
            .map_err(|e| AuthKeyError::Database(e.to_string()))
    }
}

fn map_peer_key_error(err: torrust_tracker_core::error::PeerKeyError) -> AuthKeyError {
    use torrust_tracker_core::error::PeerKeyError;

    match err {
        PeerKeyError::DurationOverflow { seconds_valid } => AuthKeyError::DurationOverflow { seconds_valid },
        PeerKeyError::InvalidKey { key, source } => AuthKeyError::InvalidKey {
            key,
            reason: source.to_string(),
        },
        PeerKeyError::DatabaseError { source } => AuthKeyError::Database(source.to_string()),
    }
}

#[allow(clippy::needless_pass_by_value)]
#[allow(deprecated)]
fn peer_key_to_auth_key(peer_key: PeerKey) -> AuthKey {
    match (peer_key.valid_until, peer_key.expiry_time()) {
        (Some(valid_until), Some(expiry_time)) => AuthKey {
            key: peer_key.key.to_string(),
            valid_until: Some(valid_until.as_secs()),
            expiry_time: Some(expiry_time.to_string()),
        },
        _ => AuthKey {
            key: peer_key.key.to_string(),
            valid_until: None,
            expiry_time: None,
        },
    }
}
