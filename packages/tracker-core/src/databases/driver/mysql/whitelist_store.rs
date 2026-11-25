use std::str::FromStr;

use ::sqlx::Row;
use async_trait::async_trait;
use torrust_info_hash::InfoHash;

use super::{DRIVER, Mysql};
use crate::databases::WhitelistStore;
use crate::databases::error::Error;

#[async_trait]
impl WhitelistStore for Mysql {
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
        let maybe_row = ::sqlx::query("SELECT info_hash FROM whitelist WHERE info_hash = ?")
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
        // `ON DUPLICATE KEY UPDATE info_hash = info_hash` is a no-op on a duplicate (not `INSERT IGNORE`,
        // which would also hide unrelated errors). The driver connects with the
        // found-rows flag, so a duplicate still reports 1 row.
        let inserted =
            ::sqlx::query("INSERT INTO whitelist (info_hash) VALUES (?) ON DUPLICATE KEY UPDATE info_hash = info_hash")
                .bind(info_hash.to_string())
                .execute(&self.pool)
                .await
                .map_err(|e| (e, DRIVER))?
                .rows_affected();

        usize::try_from(inserted).map_err(|e| Error::MalformedDatabaseRecord {
            message: format!("rows_affected does not fit in usize: {e}"),
            driver: DRIVER,
        })
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        // Removing a torrent that is not listed is not an error: the delete affects no rows.
        let deleted = ::sqlx::query("DELETE FROM whitelist WHERE info_hash = ?")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        usize::try_from(deleted).map_err(|e| Error::MalformedDatabaseRecord {
            message: format!("rows_affected does not fit in usize: {e}"),
            driver: DRIVER,
        })
    }
}
