use std::convert::From;

use serde::{Deserialize, Serialize};

use crate::protocol::clock::DurationSinceUnixEpoch;
use crate::tracker::key::Auth;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AuthKey {
    pub key: String,
    pub valid_until: Option<u64>,
}

impl From<AuthKey> for Auth {
    fn from(auth_key_resource: AuthKey) -> Self {
        Auth {
            key: auth_key_resource.key,
            valid_until: auth_key_resource
                .valid_until
                .map(|valid_until| DurationSinceUnixEpoch::new(valid_until, 0)),
        }
    }
}

impl From<Auth> for AuthKey {
    fn from(auth_key: Auth) -> Self {
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
    use crate::tracker::key::Auth;

    #[test]
    fn it_should_be_convertible_into_an_auth_key() {
        let duration_in_secs = 60;

        let auth_key_resource = AuthKey {
            key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
            valid_until: Some(duration_in_secs),
        };

        assert_eq!(
            Auth::from(auth_key_resource),
            Auth {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
                valid_until: Some(Current::add(&Duration::new(duration_in_secs, 0)).unwrap())
            }
        );
    }

    #[test]
    fn it_should_be_convertible_from_an_auth_key() {
        let duration_in_secs = 60;

        let auth_key = Auth {
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
