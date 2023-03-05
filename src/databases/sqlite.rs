use std::panic::Location;
use std::str::FromStr;

use async_trait::async_trait;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use super::driver::Driver;
use crate::databases::{Database, Error};
use crate::protocol::clock::DurationSinceUnixEpoch;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::auth::{self, Key};

const DRIVER: Driver = Driver::Sqlite3;

pub struct Sqlite {
    pool: Pool<SqliteConnectionManager>,
}

#[async_trait]
impl Database for Sqlite {
    /// # Errors
    ///
    /// Will return `r2d2::Error` if `db_path` is not able to create `SqLite` database.
    fn new(db_path: &str) -> Result<Sqlite, Error> {
        let cm = SqliteConnectionManager::file(db_path);
        Pool::new(cm).map_or_else(|err| Err((err, Driver::Sqlite3).into()), |pool| Ok(Sqlite { pool }))
    }

    fn create_database_tables(&self) -> Result<(), Error> {
        let create_whitelist_table = "
        CREATE TABLE IF NOT EXISTS whitelist (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            info_hash TEXT NOT NULL UNIQUE
        );"
        .to_string();

        let create_torrents_table = "
        CREATE TABLE IF NOT EXISTS torrents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            info_hash TEXT NOT NULL UNIQUE,
            completed INTEGER DEFAULT 0 NOT NULL
        );"
        .to_string();

        let create_keys_table = "
        CREATE TABLE IF NOT EXISTS keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            valid_until INTEGER NOT NULL
         );"
        .to_string();

        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.execute(&create_whitelist_table, [])?;
        conn.execute(&create_keys_table, [])?;
        conn.execute(&create_torrents_table, [])?;

        Ok(())
    }

    fn drop_database_tables(&self) -> Result<(), Error> {
        let drop_whitelist_table = "
        DROP TABLE whitelist;"
            .to_string();

        let drop_torrents_table = "
        DROP TABLE torrents;"
            .to_string();

        let drop_keys_table = "
        DROP TABLE keys;"
            .to_string();

        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.execute(&drop_whitelist_table, [])
            .and_then(|_| conn.execute(&drop_torrents_table, []))
            .and_then(|_| conn.execute(&drop_keys_table, []))?;

        Ok(())
    }

    async fn load_persistent_torrents(&self) -> Result<Vec<(InfoHash, u32)>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash, completed FROM torrents")?;

        let torrent_iter = stmt.query_map([], |row| {
            let info_hash_string: String = row.get(0)?;
            let info_hash = InfoHash::from_str(&info_hash_string).unwrap();
            let completed: u32 = row.get(1)?;
            Ok((info_hash, completed))
        })?;

        //torrent_iter?;
        //let torrent_iter = torrent_iter.unwrap();

        let torrents: Vec<(InfoHash, u32)> = torrent_iter.filter_map(std::result::Result::ok).collect();

        Ok(torrents)
    }

    async fn load_keys(&self) -> Result<Vec<auth::ExpiringKey>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys")?;

        let keys_iter = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let valid_until: i64 = row.get(1)?;

            Ok(auth::ExpiringKey {
                id: key.parse::<Key>().unwrap(),
                valid_until: DurationSinceUnixEpoch::from_secs(valid_until.unsigned_abs()),
            })
        })?;

        let keys: Vec<auth::ExpiringKey> = keys_iter.filter_map(std::result::Result::ok).collect();

        Ok(keys)
    }

    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist")?;

        let info_hash_iter = stmt.query_map([], |row| {
            let info_hash: String = row.get(0)?;

            Ok(InfoHash::from_str(&info_hash).unwrap())
        })?;

        let info_hashes: Vec<InfoHash> = info_hash_iter.filter_map(std::result::Result::ok).collect();

        Ok(info_hashes)
    }

    async fn save_persistent_torrent(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = conn.execute(
            "INSERT INTO torrents (info_hash, completed) VALUES (?1, ?2) ON CONFLICT(info_hash) DO UPDATE SET completed = ?2",
            [info_hash.to_string(), completed.to_string()],
        )?;

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }

    async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<Option<InfoHash>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist WHERE info_hash = ?")?;

        let mut rows = stmt.query([info_hash])?;

        let query = rows.next()?;

        Ok(query.map(|f| InfoHash::from_str(&f.get_unwrap::<_, String>(0)).unwrap()))
    }

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
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

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
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

    async fn get_key_from_keys(&self, key: &str) -> Result<Option<auth::ExpiringKey>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys WHERE key = ?")?;

        let mut rows = stmt.query([key.to_string()])?;

        let key = rows.next()?;

        Ok(key.map(|f| {
            let expiry: i64 = f.get(1).unwrap();
            let id: String = f.get(0).unwrap();
            auth::ExpiringKey {
                id: id.parse::<Key>().unwrap(),
                valid_until: DurationSinceUnixEpoch::from_secs(expiry.unsigned_abs()),
            }
        }))
    }

    async fn add_key_to_keys(&self, auth_key: &auth::ExpiringKey) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = conn.execute(
            "INSERT INTO keys (key, valid_until) VALUES (?1, ?2)",
            [auth_key.id.to_string(), auth_key.valid_until.as_secs().to_string()],
        )?;

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert)
        }
    }

    async fn remove_key_from_keys(&self, key: &str) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let deleted = conn.execute("DELETE FROM keys WHERE key = ?", [key])?;

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
