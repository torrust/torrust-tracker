use super::common::AUTH_KEY_LENGTH;
use crate::utils::current_time;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use serde::Serialize;
use log::debug;
use derive_more::{Display, Error};

pub fn generate_auth_key(seconds_valid: u64) -> AuthKey {
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

pub fn verify_auth_key(auth_key: &AuthKey) -> Result<(), Error> {
    let current_time = current_time();
    if auth_key.valid_until.is_none() { return Err(Error::KeyInvalid) }
    if auth_key.valid_until.unwrap() < current_time { return Err(Error::KeyExpired) }

    Ok(())
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct AuthKey {
    pub key: String,
    pub valid_until: Option<u64>,
}

impl AuthKey {
    pub fn from_buffer(key_buffer: [u8; AUTH_KEY_LENGTH]) -> Option<AuthKey> {
        if let Ok(key) = String::from_utf8(Vec::from(key_buffer)) {
            Some(AuthKey {
                key,
                valid_until: None,
            })
        } else {
            None
        }
    }

    pub fn from_string(key: &str) -> Option<AuthKey> {
        if key.len() != AUTH_KEY_LENGTH {
            None
        } else {
            Some(AuthKey {
                key: key.to_string(),
                valid_until: None,
            })
        }
    }
}

#[derive(Debug, Display, PartialEq, Error)]
#[allow(dead_code)]
pub enum Error {
    #[display(fmt = "Key could not be verified.")]
    KeyVerificationError,
    #[display(fmt = "Key is invalid.")]
    KeyInvalid,
    #[display(fmt = "Key has expired.")]
    KeyExpired
}

impl From<r2d2_sqlite::rusqlite::Error> for Error {
    fn from(e: r2d2_sqlite::rusqlite::Error) -> Self {
        eprintln!("{}", e);
        Error::KeyVerificationError
    }
}

#[cfg(test)]
mod tests {
    use crate::key_manager;

    #[test]
    fn auth_key_from_buffer() {
        let auth_key = key_manager::AuthKey::from_buffer(
            [
                89, 90, 83, 108,
                52, 108, 77, 90,
                117, 112, 82, 117,
                79, 112, 83, 82,
                67, 51, 107, 114,
                73, 75, 82, 53,
                66, 80, 66, 49,
                52, 110, 114, 74]
        );

        assert!(auth_key.is_some());
        assert_eq!(auth_key.unwrap().key, "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ");
    }

    #[test]
    fn auth_key_from_string() {
        let key_string = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ";
        let auth_key = key_manager::AuthKey::from_string(key_string);

        assert!(auth_key.is_some());
        assert_eq!(auth_key.unwrap().key, key_string);
    }

    #[test]
    fn generate_valid_auth_key() {
        let auth_key = key_manager::generate_auth_key(9999);

        assert!(key_manager::verify_auth_key(&auth_key).is_ok());
    }

    #[test]
    fn generate_expired_auth_key() {
        let mut auth_key = key_manager::generate_auth_key(0);
        auth_key.valid_until = Some(0);

        assert!(key_manager::verify_auth_key(&auth_key).is_err());
    }
}
