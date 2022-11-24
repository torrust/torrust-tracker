use std::time::Duration;

use derive_more::{Display, Error};
use log::debug;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Serialize;

use crate::protocol::clock::{Current, DurationSinceUnixEpoch, Time, TimeNow};
use crate::protocol::common::AUTH_KEY_LENGTH;

#[must_use]
/// # Panics
///
/// It would panic if the `lifetime: Duration` + Duration is more than `Duration::MAX`.
pub fn generate(lifetime: Duration) -> Auth {
    let key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(AUTH_KEY_LENGTH)
        .map(char::from)
        .collect();

    debug!("Generated key: {}, valid for: {:?} seconds", key, lifetime);

    Auth {
        key,
        valid_until: Some(Current::add(&lifetime).unwrap()),
    }
}

/// # Errors
///
/// Will return `Error::KeyExpired` if `auth_key.valid_until` is past the `current_time`.
///
/// Will return `Error::KeyInvalid` if `auth_key.valid_until` is past the `None`.
pub fn verify(auth_key: &Auth) -> Result<(), Error> {
    let current_time: DurationSinceUnixEpoch = Current::now();

    match auth_key.valid_until {
        Some(valid_untill) => {
            if valid_untill < current_time {
                Err(Error::KeyExpired)
            } else {
                Ok(())
            }
        }
        None => Err(Error::KeyInvalid),
    }
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Auth {
    pub key: String,
    pub valid_until: Option<DurationSinceUnixEpoch>,
}

impl Auth {
    #[must_use]
    pub fn from_buffer(key_buffer: [u8; AUTH_KEY_LENGTH]) -> Option<Auth> {
        if let Ok(key) = String::from_utf8(Vec::from(key_buffer)) {
            Some(Auth { key, valid_until: None })
        } else {
            None
        }
    }

    #[must_use]
    pub fn from_string(key: &str) -> Option<Auth> {
        if key.len() == AUTH_KEY_LENGTH {
            Some(Auth {
                key: key.to_string(),
                valid_until: None,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Display, PartialEq, Eq, Error)]
#[allow(dead_code)]
pub enum Error {
    #[display(fmt = "Key could not be verified.")]
    KeyVerificationError,
    #[display(fmt = "Key is invalid.")]
    KeyInvalid,
    #[display(fmt = "Key has expired.")]
    KeyExpired,
}

impl From<r2d2_sqlite::rusqlite::Error> for Error {
    fn from(e: r2d2_sqlite::rusqlite::Error) -> Self {
        eprintln!("{}", e);
        Error::KeyVerificationError
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::protocol::clock::{Current, StoppedTime};
    use crate::tracker::key;

    #[test]
    fn auth_key_from_buffer() {
        let auth_key = key::Auth::from_buffer([
            89, 90, 83, 108, 52, 108, 77, 90, 117, 112, 82, 117, 79, 112, 83, 82, 67, 51, 107, 114, 73, 75, 82, 53, 66, 80, 66,
            49, 52, 110, 114, 74,
        ]);

        assert!(auth_key.is_some());
        assert_eq!(auth_key.unwrap().key, "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ");
    }

    #[test]
    fn auth_key_from_string() {
        let key_string = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ";
        let auth_key = key::Auth::from_string(key_string);

        assert!(auth_key.is_some());
        assert_eq!(auth_key.unwrap().key, key_string);
    }

    #[test]
    fn generate_valid_auth_key() {
        let auth_key = key::generate(Duration::new(9999, 0));

        assert!(key::verify(&auth_key).is_ok());
    }

    #[test]
    fn generate_and_check_expired_auth_key() {
        // Set the time to the current time.
        Current::local_set_to_system_time_now();

        // Make key that is valid for 19 seconds.
        let auth_key = key::generate(Duration::from_secs(19));

        // Mock the time has passed 10 sec.
        Current::local_add(&Duration::from_secs(10)).unwrap();

        assert!(key::verify(&auth_key).is_ok());

        // Mock the time has passed another 10 sec.
        Current::local_add(&Duration::from_secs(10)).unwrap();

        assert!(key::verify(&auth_key).is_err());
    }
}
