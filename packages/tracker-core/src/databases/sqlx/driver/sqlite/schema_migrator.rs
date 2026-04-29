use async_trait::async_trait;

use super::{SqliteSqlx, DRIVER};
use crate::databases::error::Error;
use crate::databases::sqlx::traits::AsyncSchemaMigrator;

#[async_trait]
impl AsyncSchemaMigrator for SqliteSqlx {
    async fn create_database_tables(&self) -> Result<(), Error> {
        let create_whitelist_table = "
        CREATE TABLE IF NOT EXISTS whitelist (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            info_hash TEXT NOT NULL UNIQUE
        );";

        let create_torrents_table = "
        CREATE TABLE IF NOT EXISTS torrents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            info_hash TEXT NOT NULL UNIQUE,
            completed INTEGER DEFAULT 0 NOT NULL
        );";

        let create_torrent_aggregate_metrics_table = "
        CREATE TABLE IF NOT EXISTS torrent_aggregate_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            metric_name TEXT NOT NULL UNIQUE,
            value INTEGER DEFAULT 0 NOT NULL
        );";

        let create_keys_table = "
        CREATE TABLE IF NOT EXISTS keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            valid_until INTEGER
         );";

        ::sqlx::query(create_whitelist_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;
        ::sqlx::query(create_keys_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;
        ::sqlx::query(create_torrents_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;
        ::sqlx::query(create_torrent_aggregate_metrics_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        Ok(())
    }

    async fn drop_database_tables(&self) -> Result<(), Error> {
        let drop_whitelist_table = "
        DROP TABLE whitelist;";

        let drop_torrents_table = "
        DROP TABLE torrents;";

        let drop_keys_table = "
        DROP TABLE keys;";

        ::sqlx::query(drop_whitelist_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;
        ::sqlx::query(drop_torrents_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;
        ::sqlx::query(drop_keys_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        self.schema_ready.store(false, std::sync::atomic::Ordering::Release);

        Ok(())
    }
}
