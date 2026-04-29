use r2d2_mysql::mysql::prelude::Queryable;

use super::{Mysql, DRIVER};
use crate::authentication::key::AUTH_KEY_LENGTH;
use crate::databases::error::Error;
use crate::databases::SchemaMigrator;

impl SchemaMigrator for Mysql {
    fn create_database_tables(&self) -> Result<(), Error> {
        let create_whitelist_table = "
        CREATE TABLE IF NOT EXISTS whitelist (
            id integer PRIMARY KEY AUTO_INCREMENT,
            info_hash VARCHAR(40) NOT NULL UNIQUE
        );"
        .to_string();

        let create_torrents_table = "
        CREATE TABLE IF NOT EXISTS torrents (
            id integer PRIMARY KEY AUTO_INCREMENT,
            info_hash VARCHAR(40) NOT NULL UNIQUE,
            completed INTEGER DEFAULT 0 NOT NULL
        );"
        .to_string();

        let create_torrent_aggregate_metrics_table = "
        CREATE TABLE IF NOT EXISTS torrent_aggregate_metrics (
            id integer PRIMARY KEY AUTO_INCREMENT,
            metric_name VARCHAR(50) NOT NULL UNIQUE,
            value INTEGER DEFAULT 0 NOT NULL
        );"
        .to_string();

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

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.query_drop(&create_torrents_table)
            .expect("Could not create torrents table.");
        conn.query_drop(&create_torrent_aggregate_metrics_table)
            .expect("Could not create create_torrent_aggregate_metrics_table table.");
        conn.query_drop(&create_keys_table).expect("Could not create keys table.");
        conn.query_drop(&create_whitelist_table)
            .expect("Could not create whitelist table.");

        Ok(())
    }

    fn drop_database_tables(&self) -> Result<(), Error> {
        let drop_whitelist_table = "
        DROP TABLE `whitelist`;"
            .to_string();

        let drop_torrents_table = "
        DROP TABLE `torrents`;"
            .to_string();

        let drop_keys_table = "
            DROP TABLE `keys`;"
            .to_string();

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.query_drop(&drop_whitelist_table)
            .expect("Could not drop `whitelist` table.");
        conn.query_drop(&drop_torrents_table)
            .expect("Could not drop `torrents` table.");
        conn.query_drop(&drop_keys_table).expect("Could not drop `keys` table.");

        Ok(())
    }
}
