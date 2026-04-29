use std::str::FromStr;

use bittorrent_primitives::info_hash::InfoHash;
use r2d2_mysql::mysql::params;
use r2d2_mysql::mysql::prelude::Queryable;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{Mysql, DRIVER};
use crate::databases::driver::TORRENTS_DOWNLOADS_TOTAL;
use crate::databases::error::Error;
use crate::databases::TorrentMetricsStore;

impl TorrentMetricsStore for Mysql {
    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let raw_rows: Vec<(String, u32)> = conn.query_map(
            "SELECT info_hash, completed FROM torrents",
            |(info_hash_string, completed): (String, u32)| (info_hash_string, completed),
        )?;

        raw_rows
            .into_iter()
            .map(|(s, completed)| {
                InfoHash::from_str(&s)
                    .map(|info_hash| (info_hash, completed))
                    .map_err(|e| Error::MalformedDatabaseRecord {
                        message: format!("{e:?}"),
                        driver: DRIVER,
                    })
            })
            .collect::<Result<Vec<_>, Error>>()
            .map(|v| v.iter().copied().collect())
    }

    fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let query = conn.exec_first::<u32, _, _>(
            "SELECT completed FROM torrents WHERE info_hash = :info_hash",
            params! { "info_hash" => info_hash.to_hex_string() },
        );

        let persistent_torrent = query?;

        Ok(persistent_torrent)
    }

    fn save_torrent_downloads(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error> {
        const COMMAND : &str = "INSERT INTO torrents (info_hash, completed) VALUES (:info_hash_str, :completed) ON DUPLICATE KEY UPDATE completed = VALUES(completed)";

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hash_str = info_hash.to_string();

        Ok(conn.exec_drop(COMMAND, params! { info_hash_str, completed })?)
    }

    fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hash_str = info_hash.to_string();

        conn.exec_drop(
            "UPDATE torrents SET completed = completed + 1 WHERE info_hash = :info_hash_str",
            params! { info_hash_str },
        )?;

        Ok(())
    }

    fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.load_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL)
    }

    fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.save_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL, downloaded)
    }

    fn increase_global_downloads(&self) -> Result<(), Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let metric_name = TORRENTS_DOWNLOADS_TOTAL;

        conn.exec_drop(
            "UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = :metric_name",
            params! { metric_name },
        )?;

        Ok(())
    }
}
