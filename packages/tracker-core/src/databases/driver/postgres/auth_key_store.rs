use ::sqlx::Row;
use async_trait::async_trait;
use torrust_tracker_clock::DurationSinceUnixEpoch;

use super::{DRIVER, Postgres};
use crate::authentication::{self, Key};
use crate::databases::AuthKeyStore;
use crate::databases::error::Error;

#[async_trait]
impl AuthKeyStore for Postgres {
    async fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        let rows = ::sqlx::query("SELECT key, valid_until FROM keys")
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

                Ok(authentication::PeerKey {
                    key: parsed_key,
                    valid_until: valid_until.map(parse_valid_until).transpose()?,
                })
            })
            .collect()
    }

    async fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        let maybe_row = ::sqlx::query("SELECT key, valid_until FROM keys WHERE key = $1")
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

                Ok(authentication::PeerKey {
                    key: parsed_key,
                    valid_until: valid_until.map(parse_valid_until).transpose()?,
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

        let insert = ::sqlx::query("INSERT INTO keys (key, valid_until) VALUES ($1, $2)")
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
            usize::try_from(insert).map_err(|e| Error::MalformedDatabaseRecord {
                message: format!("rows_affected does not fit in usize: {e}"),
                driver: DRIVER,
            })
        }
    }

    async fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        let deleted = ::sqlx::query("DELETE FROM keys WHERE key = $1")
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

/// Convert a signed seconds value loaded from the database into a
/// [`DurationSinceUnixEpoch`].
///
/// Negative values indicate a corrupted record (timestamps before the Unix
/// epoch are not representable) and are rejected as
/// [`Error::MalformedDatabaseRecord`].
fn parse_valid_until(value: i64) -> Result<DurationSinceUnixEpoch, Error> {
    let secs = u64::try_from(value).map_err(|_| Error::MalformedDatabaseRecord {
        message: format!("negative valid_until timestamp: {value}"),
        driver: DRIVER,
    })?;
    Ok(DurationSinceUnixEpoch::from_secs(secs))
}
