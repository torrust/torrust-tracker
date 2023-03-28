//! API resources for the [`auth_key`](crate::servers::apis::v1::context::auth_key) API context.
use std::convert::From;

use serde::{Deserialize, Serialize};

use crate::shared::clock::convert_from_iso_8601_to_timestamp;
use crate::tracker::auth::{self, Key};

/// A resource that represents an authentication key.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AuthKey {
    /// The authentication key.
    pub key: String,
    /// The timestamp when the key will expire.
    #[deprecated(since = "3.0.0", note = "please use `expiry_time` instead")]
    pub valid_until: u64, // todo: remove when the torrust-index-backend starts using the `expiry_time` attribute.
    /// The ISO 8601 timestamp when the key will expire.
    pub expiry_time: String,
}

impl From<AuthKey> for auth::ExpiringKey {
    fn from(auth_key_resource: AuthKey) -> Self {
        auth::ExpiringKey {
            key: auth_key_resource.key.parse::<Key>().unwrap(),
            valid_until: convert_from_iso_8601_to_timestamp(&auth_key_resource.expiry_time),
        }
    }
}

impl From<auth::ExpiringKey> for AuthKey {
    fn from(auth_key: auth::ExpiringKey) -> Self {
        AuthKey {
            key: auth_key.key.to_string(),
            valid_until: auth_key.valid_until.as_secs(),
            expiry_time: auth_key.expiry_time().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::AuthKey;
    use crate::shared::clock::{Current, TimeNow};
    use crate::tracker::auth::{self, Key};

    struct TestTime {
        pub timestamp: u64,
        pub iso_8601_v1: String,
        pub iso_8601_v2: String,
    }

    fn one_hour_after_unix_epoch() -> TestTime {
        let timestamp = 60_u64;
        let iso_8601_v1 = "1970-01-01T00:01:00.000Z".to_string();
        let iso_8601_v2 = "1970-01-01 00:01:00 UTC".to_string();
        TestTime {
            timestamp,
            iso_8601_v1,
            iso_8601_v2,
        }
    }

    #[test]
    fn it_should_be_convertible_into_an_auth_key() {
        let auth_key_resource = AuthKey {
            key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
            valid_until: one_hour_after_unix_epoch().timestamp,
            expiry_time: one_hour_after_unix_epoch().iso_8601_v1,
        };

        assert_eq!(
            auth::ExpiringKey::from(auth_key_resource),
            auth::ExpiringKey {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".parse::<Key>().unwrap(), // cspell:disable-line
                valid_until: Current::add(&Duration::new(one_hour_after_unix_epoch().timestamp, 0)).unwrap()
            }
        );
    }

    #[test]
    fn it_should_be_convertible_from_an_auth_key() {
        let auth_key = auth::ExpiringKey {
            key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".parse::<Key>().unwrap(), // cspell:disable-line
            valid_until: Current::add(&Duration::new(one_hour_after_unix_epoch().timestamp, 0)).unwrap(),
        };

        assert_eq!(
            AuthKey::from(auth_key),
            AuthKey {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
                valid_until: one_hour_after_unix_epoch().timestamp,
                expiry_time: one_hour_after_unix_epoch().iso_8601_v2,
            }
        );
    }

    #[test]
    fn it_should_be_convertible_into_json() {
        assert_eq!(
            serde_json::to_string(&AuthKey {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
                valid_until: one_hour_after_unix_epoch().timestamp,
                expiry_time: one_hour_after_unix_epoch().iso_8601_v1,
            })
            .unwrap(),
            "{\"key\":\"IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM\",\"valid_until\":60,\"expiry_time\":\"1970-01-01T00:01:00.000Z\"}" // cspell:disable-line
        );
    }
}
