use super::common::AUTH_KEY_LENGTH;
use crate::utils::current_time;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use serde::Serialize;
use log::debug;
use derive_more::{Display, Error};

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

#[derive(Debug, Display, PartialEq, Error)]
#[allow(dead_code)]
pub enum Error {
    #[display(fmt = "Key is invalid.")]
    KeyVerificationError,
    #[display(fmt = "Key has expired.")]
    KeyExpired
}

impl From<r2d2_sqlite::rusqlite::Error> for Error {
    fn from(e: r2d2_sqlite::rusqlite::Error) -> Self {
        eprintln!("{}", e);
        Error::KeyVerificationError
    }
}

pub struct KeyManager;

impl KeyManager {
    pub fn generate_auth_key(&self, seconds_valid: u64) -> AuthKey {
        let key: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(AUTH_KEY_LENGTH)
            .map(char::from)
            .collect();

        debug!("Generated key: {}, valid for: {} seconds", key, seconds_valid);

        AuthKey {
            key,
            valid_until: Some(current_time() + seconds_valid),
        }
    }

    pub async fn verify_auth_key(&self, auth_key: &AuthKey) -> Result<(), Error> {
        let current_time = current_time();
        if auth_key.valid_until.is_none() { return Err(Error::KeyVerificationError) }
        if &auth_key.valid_until.unwrap() < &current_time { return Err(Error::KeyExpired) }

        Ok(())
    }
}
