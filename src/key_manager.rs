use super::common::AUTH_KEY_LENGTH;
use crate::utils::current_time;
use crate::database::SqliteDatabase;
use std::sync::Arc;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use serde::Serialize;
use log::debug;

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct AuthKey {
    pub key: String,
    pub valid_until: Option<u64>,
}

impl AuthKey {
    pub fn from_buffer(key_buffer: [u8; AUTH_KEY_LENGTH]) -> Option<AuthKey> {
        Some(AuthKey {
            key: String::from_utf8(Vec::from(key_buffer)).unwrap(),
            valid_until: None,
        })
    }

    pub fn from_string(key: &str) -> Option<AuthKey> {
        if key.len() != AUTH_KEY_LENGTH { return None }

        Some(AuthKey {
            key: key.to_string(),
            valid_until: None,
        })
    }
}

pub struct KeyManager {
    database: Arc<SqliteDatabase>,
}

impl KeyManager {
    pub fn new(database: Arc<SqliteDatabase>) -> KeyManager {
        KeyManager {
            database
        }
    }

    pub async fn generate_auth_key(&self, seconds_valid: u64) -> Result<AuthKey, ()> {
        let key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(AUTH_KEY_LENGTH)
            .map(char::from)
            .collect();

        debug!("Generated key: {}, valid for: {} seconds", key, seconds_valid);

        let auth_key = AuthKey {
            key: key.clone(),
            valid_until: Some(current_time() + seconds_valid),
        };

        // add key to database
        match self.database.add_key_to_keys(auth_key.clone()).await {
            Ok(_) => Ok(auth_key),
            Err(_) => Err(())
        }
    }

    pub async fn remove_auth_key(&self, key: String) -> Result<(), ()> {
        match self.database.remove_key_from_keys(key).await {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub async fn verify_auth_key(&self, auth_key: &AuthKey) -> bool {
        let current_time = current_time();

        match self.database.get_key_from_keys(auth_key.key.to_string()).await {
            Ok(auth_key) => {
                match auth_key.valid_until {
                    // should not be possible, valid_until is required
                    None => false,
                    Some(valid_until) => valid_until > current_time
                }
            }
            Err(e) => {
                debug!{"{:?}", e}
                false
            }
        }
    }
}
