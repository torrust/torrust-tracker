use ::sqlx::Row;
use async_trait::async_trait;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::{Mysql, DRIVER};
use crate::authentication::{self, Key};
use crate::databases::error::Error;
use crate::databases::AuthKeyStore;

#[async_trait]
impl AuthKeyStore for Mysql {
    async fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        let rows = ::sqlx::query("SELECT `key`, valid_until FROM `keys`")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        rows.into_iter()
            .map(|row| {
                let key_value: String = row.try_get("key").map_err(|e| (e, DRIVER))?;
                let valid_until: Option<i64> = row.try_get("valid_until").map_err(|e| (e, DRIVER))?;

                let parsed_key = key_value.parse::<Key>().map_err(|e| Error::MalformedDatabaseRecord {
                    message: e.to_string(),
                    driver: DRIVER,
                })?;

                Ok(match valid_until {
                    Some(value) => authentication::PeerKey {
                        key: parsed_key,
                        valid_until: Some(DurationSinceUnixEpoch::from_secs(value.unsigned_abs())),
                    },
                    None => authentication::PeerKey {
                        key: parsed_key,
                        valid_until: None,
                    },
                })
            })
            .collect()
    }

    async fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        let maybe_row = ::sqlx::query("SELECT `key`, valid_until FROM `keys` WHERE `key` = ?")
            .bind(key.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        maybe_row
            .map(|row| {
                let key_value: String = row.try_get("key").map_err(|e| (e, DRIVER))?;
                let valid_until: Option<i64> = row.try_get("valid_until").map_err(|e| (e, DRIVER))?;

                let parsed_key = key_value.parse::<Key>().map_err(|e| Error::MalformedDatabaseRecord {
                    message: e.to_string(),
                    driver: DRIVER,
                })?;

                Ok(match valid_until {
                    Some(value) => authentication::PeerKey {
                        key: parsed_key,
                        valid_until: Some(DurationSinceUnixEpoch::from_secs(value.unsigned_abs())),
                    },
                    None => authentication::PeerKey {
                        key: parsed_key,
                        valid_until: None,
                    },
                })
            })
            .transpose()
    }

    async fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        let valid_until = auth_key
            .valid_until
            .map(|value| {
                i64::try_from(value.as_secs()).map_err(|e| Error::MalformedDatabaseRecord {
                    message: e.to_string(),
                    driver: DRIVER,
                })
            })
            .transpose()?;

        let insert = ::sqlx::query("INSERT INTO `keys` (`key`, valid_until) VALUES (?, ?)")
            .bind(auth_key.key.to_string())
            .bind(valid_until)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        if insert == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(usize::try_from(insert).unwrap_or(0))
        }
    }

    async fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        let deleted = ::sqlx::query("DELETE FROM `keys` WHERE `key` = ?")
            .bind(key.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        if deleted == 1 {
            Ok(1)
        } else {
            Err(Error::DeleteFailed {
                location: std::panic::Location::caller(),
                error_code: usize::try_from(deleted).unwrap_or(0),
                driver: DRIVER,
            })
        }
    }
}
