use std::panic::Location;
use std::str::FromStr;

use ::sqlx::Row;
use async_trait::async_trait;
use bittorrent_primitives::info_hash::InfoHash;

use super::{Mysql, DRIVER};
use crate::databases::error::Error;
use crate::databases::WhitelistStore;

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
        let insert = ::sqlx::query("INSERT INTO whitelist (info_hash) VALUES (?)")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            usize::try_from(insert).map_err(|e| Error::MalformedDatabaseRecord {
                message: format!("rows_affected does not fit in usize: {e}"),
                driver: DRIVER,
            })
        }
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let deleted = ::sqlx::query("DELETE FROM whitelist WHERE info_hash = ?")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        if deleted == 1 {
            Ok(1)
        } else {
            Err(Error::DeleteFailed {
                location: Location::caller(),
                error_code: usize::try_from(deleted).unwrap_or(0),
                driver: DRIVER,
            })
        }
    }
}
