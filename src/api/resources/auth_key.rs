use std::convert::From;

use serde::{Deserialize, Serialize};

use crate::protocol::clock::DurationSinceUnixEpoch;
use crate::tracker::auth;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AuthKey {
    pub key: String,
    pub valid_until: Option<u64>,
}

impl From<AuthKey> for auth::Key {
    fn from(auth_key_resource: AuthKey) -> Self {
        auth::Key {
            key: auth_key_resource.key,
            valid_until: auth_key_resource
                .valid_until
                .map(|valid_until| DurationSinceUnixEpoch::new(valid_until, 0)),
        }
    }
}

impl From<auth::Key> for AuthKey {
    fn from(auth_key: auth::Key) -> Self {
        AuthKey {
            key: auth_key.key,
            valid_until: auth_key.valid_until.map(|valid_until| valid_until.as_secs()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::AuthKey;
    use crate::protocol::clock::{Current, TimeNow};
    use crate::tracker::auth;

    #[test]
    fn it_should_be_convertible_into_an_auth_key() {
        let duration_in_secs = 60;

        let auth_key_resource = AuthKey {
            key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
            valid_until: Some(duration_in_secs),
        };

        assert_eq!(
            auth::Key::from(auth_key_resource),
            auth::Key {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
                valid_until: Some(Current::add(&Duration::new(duration_in_secs, 0)).unwrap())
            }
        );
    }

    #[test]
    fn it_should_be_convertible_from_an_auth_key() {
        let duration_in_secs = 60;

        let auth_key = auth::Key {
            key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
            valid_until: Some(Current::add(&Duration::new(duration_in_secs, 0)).unwrap()),
        };

        assert_eq!(
            AuthKey::from(auth_key),
            AuthKey {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
                valid_until: Some(duration_in_secs)
            }
        );
    }

    #[test]
    fn it_should_be_convertible_into_json() {
        assert_eq!(
            serde_json::to_string(&AuthKey {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
                valid_until: Some(60)
            })
            .unwrap(),
            "{\"key\":\"IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM\",\"valid_until\":60}" // cspell:disable-line
        );
    }
}
