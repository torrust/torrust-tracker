use std::str::FromStr;

use ::sqlx::Row;
use async_trait::async_trait;
use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{MysqlSqlx, DRIVER};
use crate::databases::driver::TORRENTS_DOWNLOADS_TOTAL;
use crate::databases::error::Error;
use crate::databases::sqlx::traits::AsyncTorrentMetricsStore;

#[async_trait]
impl AsyncTorrentMetricsStore for MysqlSqlx {
    async fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        self.ensure_schema().await?;

        let rows = ::sqlx::query("SELECT info_hash, completed FROM torrents")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        rows.into_iter()
            .map(|row| {
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
            })
            .collect::<Result<Vec<_>, Error>>()
            .map(|v| v.iter().copied().collect())
    }

    async fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        self.ensure_schema().await?;

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
        self.ensure_schema().await?;

        let insert = ::sqlx::query(
            "INSERT INTO torrents (info_hash, completed) VALUES (?, ?) ON DUPLICATE KEY UPDATE completed = VALUES(completed)",
        )
        .bind(info_hash.to_string())
        .bind(i64::from(completed))
        .execute(&self.pool)
        .await
        .map_err(|e| (e, DRIVER))?
        .rows_affected();

        if insert == 0 {
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

        ::sqlx::query("UPDATE torrents SET completed = completed + 1 WHERE info_hash = ?")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        Ok(())
    }

    async fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.ensure_schema().await?;
        self.load_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL).await
    }

    async fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.ensure_schema().await?;
        self.save_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL, downloaded).await
    }

    async fn increase_global_downloads(&self) -> Result<(), Error> {
        self.ensure_schema().await?;

        let metric_name = TORRENTS_DOWNLOADS_TOTAL;

        ::sqlx::query("UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = ?")
            .bind(metric_name)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        Ok(())
    }
}
