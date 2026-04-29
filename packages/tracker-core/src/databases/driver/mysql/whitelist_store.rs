use std::str::FromStr;

use bittorrent_primitives::info_hash::InfoHash;
use r2d2_mysql::mysql::params;
use r2d2_mysql::mysql::prelude::Queryable;

use super::{Mysql, DRIVER};
use crate::databases::error::Error;
use crate::databases::WhitelistStore;

impl WhitelistStore for Mysql {
    fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let raw: Vec<String> = conn.query_map("SELECT info_hash FROM whitelist", |info_hash: String| info_hash)?;

        raw.into_iter()
            .map(|s| {
                InfoHash::from_str(&s).map_err(|e| Error::MalformedDatabaseRecord {
                    message: format!("{e:?}"),
                    driver: DRIVER,
                })
            })
            .collect()
    }

    fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let select = conn.exec_first::<String, _, _>(
            "SELECT info_hash FROM whitelist WHERE info_hash = :info_hash",
            params! { "info_hash" => info_hash.to_hex_string() },
        )?;

        let info_hash = select
            .map(|s| {
                InfoHash::from_str(&s).map_err(|e| Error::MalformedDatabaseRecord {
                    message: format!("{e:?}"),
                    driver: DRIVER,
                })
            })
            .transpose()?;

        Ok(info_hash)
    }

    fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hash_str = info_hash.to_string();

        conn.exec_drop(
            "INSERT INTO whitelist (info_hash) VALUES (:info_hash_str)",
            params! { info_hash_str },
        )?;

        Ok(1)
    }

    fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hash = info_hash.to_string();

        conn.exec_drop("DELETE FROM whitelist WHERE info_hash = :info_hash", params! { info_hash })?;

        Ok(1)
    }
}
