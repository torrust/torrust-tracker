use std::panic::Location;
use std::str::FromStr;

use bittorrent_primitives::info_hash::InfoHash;

use super::{Sqlite, DRIVER};
use crate::databases::error::Error;
use crate::databases::WhitelistStore;

impl WhitelistStore for Sqlite {
    fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist")?;

        let raw: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .filter_map(std::result::Result::ok)
            .collect();

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
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist WHERE info_hash = ?")?;

        let mut rows = stmt.query([info_hash.to_hex_string()])?;

        let query = rows.next()?;

        let info_hash = query
            .map(|f| -> Result<InfoHash, Error> {
                let s: String = f.get(0).map_err(Error::from)?;
                InfoHash::from_str(&s).map_err(|e| Error::MalformedDatabaseRecord {
                    message: format!("{e:?}"),
                    driver: DRIVER,
                })
            })
            .transpose()?;

        Ok(info_hash)
    }

    fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = conn.execute("INSERT INTO whitelist (info_hash) VALUES (?)", [info_hash.to_string()])?;

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert)
        }
    }

    fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let deleted = conn.execute("DELETE FROM whitelist WHERE info_hash = ?", [info_hash.to_string()])?;

        if deleted == 1 {
            // should only remove a single record.
            Ok(deleted)
        } else {
            Err(Error::DeleteFailed {
                location: Location::caller(),
                error_code: deleted,
                driver: DRIVER,
            })
        }
    }
}
