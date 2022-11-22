use serde::{Deserialize, Serialize};

use crate::key::AuthKey;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AuthKeyResource {
    pub key: String,
    pub valid_until: Option<u64>,
}

impl AuthKeyResource {
    pub fn from_auth_key(auth_key: &AuthKey) -> Self {
        Self {
            key: auth_key.key.clone(),
            valid_until: auth_key.valid_until.map(|duration| duration.as_secs()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::AuthKeyResource;
    use crate::key::AuthKey;
    use crate::protocol::clock::{DefaultClock, TimeNow};

    #[test]
    fn it_should_be_instantiated_from_an_auth_key() {
        let expire_time = DefaultClock::add(&Duration::new(60, 0)).unwrap();

        let auth_key_resource = AuthKey {
            key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
            valid_until: Some(expire_time),
        };

        assert_eq!(
            AuthKeyResource::from_auth_key(&auth_key_resource),
            AuthKeyResource {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
                valid_until: Some(expire_time.as_secs())
            }
        )
    }

    #[test]
    fn it_should_be_converted_to_json() {
        assert_eq!(
            serde_json::to_string(&AuthKeyResource {
                key: "IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM".to_string(), // cspell:disable-line
                valid_until: Some(60)
            })
            .unwrap(),
            "{\"key\":\"IaWDneuFNZi8IB4MPA3qW1CD0M30EZSM\",\"valid_until\":60}" // cspell:disable-line
        );
    }
}
