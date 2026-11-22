use std::str::FromStr;

use ::sqlx::Row;
use async_trait::async_trait;
use torrust_info_hash::InfoHash;

use super::{DRIVER, Sqlite};
use crate::databases::WhitelistStore;
use crate::databases::error::Error;

#[async_trait]
impl WhitelistStore for Sqlite {
    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        let rows = ::sqlx::query("SELECT info_hash FROM whitelist")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        rows.into_iter()
            .map(|row| {
                let value: String = row.try_get("info_hash").map_err(|e| (e, DRIVER))?;
                InfoHash::from_str(&value).map_err(|e| Error::MalformedDatabaseRecord {
                    message: format!("{e:?}"),
                    driver: DRIVER,
                })
            })
            .collect()
    }

    async fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error> {
        let maybe_row = ::sqlx::query("SELECT info_hash FROM whitelist WHERE info_hash = ?1")
            .bind(info_hash.to_hex_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        maybe_row
            .map(|row| {
                let value: String = row.try_get("info_hash").map_err(|e| (e, DRIVER))?;
                InfoHash::from_str(&value).map_err(|e| Error::MalformedDatabaseRecord {
                    message: format!("{e:?}"),
                    driver: DRIVER,
                })
            })
            .transpose()
    }

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let insert = ::sqlx::query("INSERT INTO whitelist (info_hash) VALUES (?1)")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        if insert == 0 {
            tracing::warn!(driver = %DRIVER, table = "whitelist", %info_hash, "the insert affected no rows");
            Err(Error::InsertFailed { driver: DRIVER })
        } else {
            usize::try_from(insert).map_err(|e| Error::MalformedDatabaseRecord {
                message: format!("rows_affected does not fit in usize: {e}"),
                driver: DRIVER,
            })
        }
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let deleted = ::sqlx::query("DELETE FROM whitelist WHERE info_hash = ?1")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        if deleted == 1 {
            // should only remove a single record.
            Ok(1)
        } else {
            tracing::warn!(driver = %DRIVER, table = "whitelist", %info_hash, rows_deleted = deleted, "the delete did not remove exactly one row");
            Err(Error::DeleteFailed {
                error_code: usize::try_from(deleted).unwrap_or(0),
                driver: DRIVER,
            })
        }
    }
}
