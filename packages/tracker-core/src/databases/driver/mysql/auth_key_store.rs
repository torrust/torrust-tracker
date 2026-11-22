use ::sqlx::Row;
use async_trait::async_trait;

use super::{DRIVER, Mysql};
use crate::authentication::{self, Key};
use crate::databases::AuthKeyStore;
use crate::databases::driver::peer_key_from_row;
use crate::databases::error::Error;

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

                peer_key_from_row(&key_value, valid_until, &DRIVER)
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

                peer_key_from_row(&key_value, valid_until, &DRIVER)
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
            tracing::warn!(driver = %DRIVER, table = "keys", "the insert affected no rows");
            Err(Error::InsertFailed { driver: DRIVER })
        } else {
            usize::try_from(insert).map_err(|e| Error::MalformedDatabaseRecord {
                message: format!("rows_affected does not fit in usize: {e}"),
                driver: DRIVER,
            })
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
            tracing::warn!(driver = %DRIVER, table = "keys", rows_deleted = deleted, "the delete did not remove exactly one row");
            Err(Error::DeleteFailed {
                error_code: usize::try_from(deleted).unwrap_or(0),
                driver: DRIVER,
            })
        }
    }
}
