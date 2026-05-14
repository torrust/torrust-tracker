use std::str::FromStr;

use ::sqlx::Row;
use async_trait::async_trait;
use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{DRIVER, Sqlite};
use crate::databases::TorrentMetricsStore;
use crate::databases::driver::TORRENTS_DOWNLOADS_TOTAL;
use crate::databases::error::Error;

#[async_trait]
impl TorrentMetricsStore for Sqlite {
    async fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
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
        let maybe_row = ::sqlx::query("SELECT completed FROM torrents WHERE info_hash = ?1")
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
        // `ON CONFLICT ... DO UPDATE` may legitimately report `rows_affected() == 0`
        // when the row already exists with the same value (no-op update), so we
        // do not treat 0 as a failure here. A real failure surfaces as `Err`
        // from `execute()`.
        ::sqlx::query(
            "INSERT INTO torrents (info_hash, completed) VALUES (?1, ?2) ON CONFLICT(info_hash) DO UPDATE SET completed = ?2",
        )
        .bind(info_hash.to_string())
        .bind(i64::from(completed))
        .execute(&self.pool)
        .await
        .map_err(|e| (e, DRIVER))?;

        Ok(())
    }

    async fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        ::sqlx::query("UPDATE torrents SET completed = completed + 1 WHERE info_hash = ?1")
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

        ::sqlx::query("UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = ?1")
            .bind(metric_name)
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        Ok(())
    }
}
