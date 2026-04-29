use async_trait::async_trait;

use super::{MysqlSqlx, DRIVER};
use crate::authentication::key::AUTH_KEY_LENGTH;
use crate::databases::error::Error;
use crate::databases::sqlx::traits::AsyncSchemaMigrator;

#[async_trait]
impl AsyncSchemaMigrator for MysqlSqlx {
    async fn create_database_tables(&self) -> Result<(), Error> {
        let create_whitelist_table = "
        CREATE TABLE IF NOT EXISTS whitelist (
            id integer PRIMARY KEY AUTO_INCREMENT,
            info_hash VARCHAR(40) NOT NULL UNIQUE
        );";

        let create_torrents_table = "
        CREATE TABLE IF NOT EXISTS torrents (
            id integer PRIMARY KEY AUTO_INCREMENT,
            info_hash VARCHAR(40) NOT NULL UNIQUE,
            completed INTEGER DEFAULT 0 NOT NULL
        );";

        let create_torrent_aggregate_metrics_table = "
        CREATE TABLE IF NOT EXISTS torrent_aggregate_metrics (
            id integer PRIMARY KEY AUTO_INCREMENT,
            metric_name VARCHAR(50) NOT NULL UNIQUE,
            value INTEGER DEFAULT 0 NOT NULL
        );";

        let create_keys_table = format!(
            "
        CREATE TABLE IF NOT EXISTS `keys` (
          `id` INT NOT NULL AUTO_INCREMENT,
          `key` VARCHAR({}) NOT NULL,
          `valid_until` INT(10),
          PRIMARY KEY (`id`),
          UNIQUE (`key`)
        );",
            i8::try_from(AUTH_KEY_LENGTH).expect("authentication key length should fit within a i8!")
        );

        ::sqlx::query(create_torrents_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;
        ::sqlx::query(create_torrent_aggregate_metrics_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;
        ::sqlx::query(&create_keys_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;
        ::sqlx::query(create_whitelist_table)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        Ok(())
    }

    async fn drop_database_tables(&self) -> Result<(), Error> {
        let drop_whitelist_table = "
        DROP TABLE `whitelist`;";

        let drop_torrents_table = "
        DROP TABLE `torrents`;";

        let drop_keys_table = "
            DROP TABLE `keys`;";

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

        Ok(())
    }
}
