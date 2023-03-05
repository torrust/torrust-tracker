use std::convert::From;

use serde::{Deserialize, Serialize};

use crate::protocol::clock::DurationSinceUnixEpoch;
use crate::tracker::auth::{self, KeyId};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AuthKey {
    pub key: String, // todo: rename to `id` (API breaking change!)
    pub valid_until: Option<u64>, // todo: `auth::ExpiringKey` has now always a value (API breaking change!)
}

impl From<AuthKey> for auth::ExpiringKey {
    fn from(auth_key_resource: AuthKey) -> Self {
        let valid_until = match auth_key_resource.valid_until {
            Some(valid_until) => DurationSinceUnixEpoch::from_secs(valid_until),
            None => DurationSinceUnixEpoch::from_secs(0),
        };

        auth::ExpiringKey {
            id: auth_key_resource.key.parse::<KeyId>().unwrap(),
            valid_until,
        }
    }
}

impl From<auth::ExpiringKey> for AuthKey {
    fn from(auth_key: auth::ExpiringKey) -> Self {
        AuthKey {
            key: auth_key.id.to_string(),
            valid_until: Some(auth_key.valid_until.as_secs()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::AuthKey;
    use crate::protocol::clock::{Current, TimeNow};
    use crate::tracker::auth::{self, KeyId};

    #[test]
    fn it_should_be_convertible_into_an_auth_key() {
        let duration_in_secs = 60;

        let auth_key_resource = AuthKey {
            key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
            valid_until: Some(duration_in_secs),
        };

        assert_eq!(
            auth::ExpiringKey::from(auth_key_resource),
            auth::ExpiringKey {
                id: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".parse::<KeyId>().unwrap(), // cspell:disable-line
                valid_until: Current::add(&Duration::new(duration_in_secs, 0)).unwrap()
            }
        );
    }

    #[test]
    fn it_should_be_convertible_from_an_auth_key() {
        let duration_in_secs = 60;

        let auth_key = auth::ExpiringKey {
            id: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".parse::<KeyId>().unwrap(), // cspell:disable-line
            valid_until: Current::add(&Duration::new(duration_in_secs, 0)).unwrap(),
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
