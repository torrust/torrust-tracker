use std::time::Duration;

use r2d2_mysql::mysql::params;
use r2d2_mysql::mysql::prelude::Queryable;

use super::{Mysql, DRIVER};
use crate::authentication::{self, Key};
use crate::databases::error::Error;
use crate::databases::AuthKeyStore;

impl AuthKeyStore for Mysql {
    fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let keys = conn.query_map(
            "SELECT `key`, valid_until FROM `keys`",
            |(key, valid_until): (String, Option<i64>)| match valid_until {
                Some(valid_until) => authentication::PeerKey {
                    key: key.parse::<Key>().unwrap(),
                    valid_until: Some(Duration::from_secs(valid_until.unsigned_abs())),
                },
                None => authentication::PeerKey {
                    key: key.parse::<Key>().unwrap(),
                    valid_until: None,
                },
            },
        )?;

        Ok(keys)
    }

    fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let query = conn.exec_first::<(String, Option<i64>), _, _>(
            "SELECT `key`, valid_until FROM `keys` WHERE `key` = :key",
            params! { "key" => key.to_string() },
        );

        let key = query?;

        Ok(key.map(|(key, opt_valid_until)| match opt_valid_until {
            Some(valid_until) => authentication::PeerKey {
                key: key.parse::<Key>().unwrap(),
                valid_until: Some(Duration::from_secs(valid_until.unsigned_abs())),
            },
            None => authentication::PeerKey {
                key: key.parse::<Key>().unwrap(),
                valid_until: None,
            },
        }))
    }

    fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        match auth_key.valid_until {
            Some(valid_until) => conn.exec_drop(
                "INSERT INTO `keys` (`key`, valid_until) VALUES (:key, :valid_until)",
                params! { "key" => auth_key.key.to_string(), "valid_until" => valid_until.as_secs().to_string() },
            )?,
            None => conn.exec_drop(
                "INSERT INTO `keys` (`key`) VALUES (:key)",
                params! { "key" => auth_key.key.to_string() },
            )?,
        }

        Ok(1)
    }

    fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.exec_drop("DELETE FROM `keys` WHERE `key` = :key", params! { "key" => key.to_string() })?;

        Ok(1)
    }
}
