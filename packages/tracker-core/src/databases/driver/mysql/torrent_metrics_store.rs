use std::str::FromStr;

use ::sqlx::Row;
use ::sqlx::mysql::MySqlRow;
use async_trait::async_trait;
use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsPerInfoHash};

use super::{DRIVER, Mysql};
use crate::databases::TorrentMetricsStore;
use crate::databases::driver::{MAX_INFO_HASHES_PER_QUERY, TORRENTS_DOWNLOADS_TOTAL};
use crate::databases::error::Error;

fn torrent_downloads_from_row(row: &MySqlRow) -> Result<(InfoHash, NumberOfDownloads), Error> {
    let info_hash_value: String = row.try_get("info_hash").map_err(|e| (e, DRIVER))?;
    let completed: i64 = row.try_get("completed").map_err(|e| (e, DRIVER))?;
    let completed = u32::try_from(completed).map_err(|e| Error::MalformedDatabaseRecord {
        message: e.to_string(),
        driver: DRIVER,
    })?;

    InfoHash::from_str(&info_hash_value)
        .map(|info_hash| (info_hash, completed))
        .map_err(|e| Error::MalformedDatabaseRecord {
            message: format!("{e:?}"),
            driver: DRIVER,
        })
}

#[async_trait]
impl TorrentMetricsStore for Mysql {
    async fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsPerInfoHash, Error> {
        let rows = ::sqlx::query("SELECT info_hash, completed FROM torrents")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        rows.iter().map(torrent_downloads_from_row).collect()
    }

    async fn load_torrents_downloads(&self, info_hashes: &[InfoHash]) -> Result<NumberOfDownloadsPerInfoHash, Error> {
        let mut downloads = NumberOfDownloadsPerInfoHash::new();

        for chunk in info_hashes.chunks(MAX_INFO_HASHES_PER_QUERY) {
            let mut query_builder =
                ::sqlx::QueryBuilder::<::sqlx::MySql>::new("SELECT info_hash, completed FROM torrents WHERE info_hash IN (");
            let mut bound_info_hashes = query_builder.separated(", ");
            for info_hash in chunk {
                bound_info_hashes.push_bind(info_hash.to_hex_string());
            }
            bound_info_hashes.push_unseparated(")");

            let rows = query_builder.build().fetch_all(&self.pool).await.map_err(|e| (e, DRIVER))?;

            for row in &rows {
                let (info_hash, completed) = torrent_downloads_from_row(row)?;
                downloads.insert(info_hash, completed);
            }
        }

        Ok(downloads)
    }

    async fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        let maybe_row = ::sqlx::query("SELECT completed FROM torrents WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        maybe_row
            .map(|row| {
                let completed: i64 = row.try_get("completed").map_err(|e| (e, DRIVER))?;
                u32::try_from(completed).map_err(|e| Error::MalformedDatabaseRecord {
                    message: e.to_string(),
                    driver: DRIVER,
                })
            })
            .transpose()
    }

    async fn save_torrent_downloads(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error> {
        // `ON DUPLICATE KEY UPDATE` may legitimately report `rows_affected() == 0`
        // when the row already exists with the same value (no-op update), so we
        // do not treat 0 as a failure here. A real failure surfaces as `Err`
        // from `execute()`.
        ::sqlx::query(
            "INSERT INTO torrents (info_hash, completed) VALUES (?, ?) ON DUPLICATE KEY UPDATE completed = VALUES(completed)",
        )
        .bind(info_hash.to_string())
        .bind(i64::from(completed))
        .execute(&self.pool)
        .await
        .map_err(|e| (e, DRIVER))?;

        Ok(())
    }

    async fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        ::sqlx::query("UPDATE torrents SET completed = completed + 1 WHERE info_hash = ?")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        Ok(())
    }

    async fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.load_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL).await
    }

    async fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.save_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL, downloaded).await
    }

    async fn increase_global_downloads(&self) -> Result<(), Error> {
        let metric_name = TORRENTS_DOWNLOADS_TOTAL;

        ::sqlx::query("UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = ?")
            .bind(metric_name)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        Ok(())
    }
}
