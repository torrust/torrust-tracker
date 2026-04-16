//! The `SQLite3` database driver.
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

use async_trait::async_trait;
use bittorrent_primitives::info_hash::InfoHash;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Row, SqlitePool};
use tokio::sync::Mutex;
use torrust_tracker_primitives::{DurationSinceUnixEpoch, NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{Driver, TORRENTS_DOWNLOADS_TOTAL};
use crate::authentication::{self, Key};
use crate::databases::error::Error;
use crate::databases::{AuthKeyStore, SchemaMigrator, TorrentMetricsStore, WhitelistStore};

const DRIVER: Driver = Driver::Sqlite3;
static MIGRATOR: Migrator = sqlx::migrate!("migrations/sqlite");

/// `SQLite` driver implementation backed by `sqlx`.
pub(crate) struct Sqlite {
    pool: SqlitePool,
    schema_ready: AtomicBool,
    schema_lock: Mutex<()>,
}

impl Sqlite {
    /// Instantiates a new `SQLite3` database driver.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the database URL cannot be parsed.
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let options = SqliteConnectOptions::from_str(db_path)
            .map_err(|err| Error::connection_error(DRIVER, err))?
            .create_if_missing(true)
            .disable_statement_logging();

        let pool = SqlitePoolOptions::new().connect_lazy_with(options);

        Ok(Self {
            pool,
            schema_ready: AtomicBool::new(false),
            schema_lock: Mutex::new(()),
        })
    }

    async fn ensure_schema(&self) -> Result<(), Error> {
        if self.schema_ready.load(Ordering::Acquire) {
            return Ok(());
        }

        let _guard = self.schema_lock.lock().await;

        if self.schema_ready.load(Ordering::Acquire) {
            return Ok(());
        }

        self.run_migrations().await
    }

    async fn run_migrations(&self) -> Result<(), Error> {
        MIGRATOR
            .run(&self.pool)
            .await
            .map_err(|err| Error::migration_error(DRIVER, err))?;

        self.schema_ready.store(true, Ordering::Release);

        Ok(())
    }

    fn decode_counter_i64(&self, value: i64) -> Result<NumberOfDownloads, Error> {
        u64::try_from(value).map_err(|err| Error::invalid_query(DRIVER, err))
    }

    fn encode_counter(&self, value: NumberOfDownloads) -> Result<i64, Error> {
        i64::try_from(value).map_err(|err| Error::invalid_query(DRIVER, err))
    }

    fn decode_info_hash(&self, value: String) -> Result<InfoHash, Error> {
        InfoHash::from_str(&value).map_err(|err| {
            Error::invalid_query(
                DRIVER,
                std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{err:?}")),
            )
        })
    }

    fn decode_key(&self, value: String) -> Result<Key, Error> {
        value.parse::<Key>().map_err(|err| Error::invalid_query(DRIVER, err))
    }

    fn decode_valid_until(&self, value: Option<i64>) -> Result<Option<DurationSinceUnixEpoch>, Error> {
        value
            .map(|seconds| {
                u64::try_from(seconds)
                    .map(DurationSinceUnixEpoch::from_secs)
                    .map_err(|err| Error::invalid_query(DRIVER, err))
            })
            .transpose()
    }
}

#[async_trait]
impl SchemaMigrator for Sqlite {
    async fn create_database_tables(&self) -> Result<(), Error> {
        self.run_migrations().await
    }

    async fn drop_database_tables(&self) -> Result<(), Error> {
        sqlx::query("DROP TABLE IF EXISTS whitelist")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;
        sqlx::query("DROP TABLE IF EXISTS torrent_aggregate_metrics")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;
        sqlx::query("DROP TABLE IF EXISTS torrents")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;
        sqlx::query("DROP TABLE IF EXISTS keys")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;
        sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        Ok(())
    }
}

#[async_trait]
impl TorrentMetricsStore for Sqlite {
    async fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        self.ensure_schema().await?;

        let rows = sqlx::query("SELECT info_hash, completed FROM torrents")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        let mut torrents = NumberOfDownloadsBTreeMap::new();

        for row in rows {
            let info_hash_string: String = row.try_get("info_hash").map_err(|err| (err, DRIVER))?;
            let completed: i64 = row.try_get("completed").map_err(|err| (err, DRIVER))?;

            torrents.insert(self.decode_info_hash(info_hash_string)?, self.decode_counter_i64(completed)?);
        }

        Ok(torrents)
    }

    async fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        self.ensure_schema().await?;

        let row = sqlx::query("SELECT completed FROM torrents WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        row.map(|row| {
            let completed: i64 = row.try_get("completed").map_err(|err| (err, DRIVER))?;
            self.decode_counter_i64(completed)
        })
        .transpose()
    }

    async fn save_torrent_downloads(&self, info_hash: &InfoHash, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.ensure_schema().await?;

        let encoded_downloaded = self.encode_counter(downloaded)?;

        let insert = sqlx::query(
            "INSERT INTO torrents (info_hash, completed) VALUES (?, ?) ON CONFLICT(info_hash) DO UPDATE SET completed = excluded.completed",
        )
        .bind(info_hash.to_hex_string())
        .bind(encoded_downloaded)
        .execute(&self.pool)
        .await
        .map_err(|err| (err, DRIVER))?;

        if insert.rows_affected() == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }

    async fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        self.ensure_schema().await?;

        sqlx::query("UPDATE torrents SET completed = completed + 1 WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        Ok(())
    }

    async fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.ensure_schema().await?;

        let row = sqlx::query("SELECT value FROM torrent_aggregate_metrics WHERE metric_name = ?")
            .bind(TORRENTS_DOWNLOADS_TOTAL)
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        row.map(|row| {
            let value: i64 = row.try_get("value").map_err(|err| (err, DRIVER))?;
            self.decode_counter_i64(value)
        })
        .transpose()
    }

    async fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.ensure_schema().await?;

        let encoded_downloaded = self.encode_counter(downloaded)?;

        let insert = sqlx::query(
            "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (?, ?) ON CONFLICT(metric_name) DO UPDATE SET value = excluded.value",
        )
        .bind(TORRENTS_DOWNLOADS_TOTAL)
        .bind(encoded_downloaded)
        .execute(&self.pool)
        .await
        .map_err(|err| (err, DRIVER))?;

        if insert.rows_affected() == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }

    async fn increase_global_downloads(&self) -> Result<(), Error> {
        self.ensure_schema().await?;

        sqlx::query("UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = ?")
            .bind(TORRENTS_DOWNLOADS_TOTAL)
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        Ok(())
    }
}

#[async_trait]
impl WhitelistStore for Sqlite {
    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        self.ensure_schema().await?;

        let rows = sqlx::query("SELECT info_hash FROM whitelist")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        rows.into_iter()
            .map(|row| {
                let info_hash: String = row.try_get("info_hash").map_err(|err| (err, DRIVER))?;
                self.decode_info_hash(info_hash)
            })
            .collect()
    }

    async fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error> {
        self.ensure_schema().await?;

        let row = sqlx::query("SELECT info_hash FROM whitelist WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        row.map(|row| {
            let value: String = row.try_get("info_hash").map_err(|err| (err, DRIVER))?;
            self.decode_info_hash(value)
        })
        .transpose()
    }

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        self.ensure_schema().await?;

        let insert = sqlx::query("INSERT INTO whitelist (info_hash) VALUES (?)")
            .bind(info_hash.to_hex_string())
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        if insert.rows_affected() == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert.rows_affected() as usize)
        }
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        self.ensure_schema().await?;

        let deleted = sqlx::query("DELETE FROM whitelist WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        let deleted = deleted.rows_affected() as usize;

        if deleted == 1 {
            Ok(deleted)
        } else {
            Err(Error::DeleteFailed {
                location: std::panic::Location::caller(),
                error_code: deleted,
                driver: DRIVER,
            })
        }
    }
}

#[async_trait]
impl AuthKeyStore for Sqlite {
    async fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        self.ensure_schema().await?;

        let rows = sqlx::query("SELECT key, valid_until FROM keys")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        rows.into_iter()
            .map(|row| {
                let key: String = row.try_get("key").map_err(|err| (err, DRIVER))?;
                let valid_until: Option<i64> = row.try_get("valid_until").map_err(|err| (err, DRIVER))?;

                Ok(authentication::PeerKey {
                    key: self.decode_key(key)?,
                    valid_until: self.decode_valid_until(valid_until)?,
                })
            })
            .collect()
    }

    async fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        self.ensure_schema().await?;

        let row = sqlx::query("SELECT key, valid_until FROM keys WHERE key = ?")
            .bind(key.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        row.map(|row| {
            let key: String = row.try_get("key").map_err(|err| (err, DRIVER))?;
            let valid_until: Option<i64> = row.try_get("valid_until").map_err(|err| (err, DRIVER))?;

            Ok(authentication::PeerKey {
                key: self.decode_key(key)?,
                valid_until: self.decode_valid_until(valid_until)?,
            })
        })
        .transpose()
    }

    async fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        self.ensure_schema().await?;

        let valid_until = auth_key
            .valid_until
            .map(|valid_until| valid_until.as_secs())
            .map(i64::try_from)
            .transpose()
            .map_err(|err| Error::invalid_query(DRIVER, err))?;

        let insert = sqlx::query("INSERT INTO keys (key, valid_until) VALUES (?, ?)")
            .bind(auth_key.key.to_string())
            .bind(valid_until)
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        if insert.rows_affected() == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert.rows_affected() as usize)
        }
    }

    async fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        self.ensure_schema().await?;

        let deleted = sqlx::query("DELETE FROM keys WHERE key = ?")
            .bind(key.to_string())
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        let deleted = deleted.rows_affected() as usize;

        if deleted == 1 {
            Ok(deleted)
        } else {
            Err(Error::DeleteFailed {
                location: std::panic::Location::caller(),
                error_code: deleted,
                driver: DRIVER,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use torrust_tracker_configuration::Core;
    use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;

    use crate::databases::driver::build;
    use crate::databases::driver::tests::run_tests;
    use crate::databases::driver::Driver;

    fn ephemeral_configuration() -> Core {
        let mut config = Core::default();
        let temp_file = ephemeral_sqlite_database();
        temp_file.to_str().unwrap().clone_into(&mut config.database.path);
        config
    }

    #[tokio::test]
    async fn run_sqlite_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let config = ephemeral_configuration();
        let driver = build(&Driver::Sqlite3, &config.database.path)?;

        run_tests(&driver).await;

        Ok(())
    }
}
