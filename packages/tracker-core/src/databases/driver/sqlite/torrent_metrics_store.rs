use std::str::FromStr;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{Sqlite, DRIVER};
use crate::databases::driver::TORRENTS_DOWNLOADS_TOTAL;
use crate::databases::error::Error;
use crate::databases::TorrentMetricsStore;

impl TorrentMetricsStore for Sqlite {
    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash, completed FROM torrents")?;

        let raw: Vec<(String, u32)> = stmt
            .query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?)))?
            .filter_map(std::result::Result::ok)
            .collect();

        raw.into_iter()
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
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT completed FROM torrents WHERE info_hash = ?")?;

        let mut rows = stmt.query([info_hash.to_hex_string()])?;

        let persistent_torrent = rows.next()?;

        Ok(persistent_torrent.map(|f| {
            let completed: i64 = f.get(0).unwrap();
            u32::try_from(completed).unwrap()
        }))
    }

    fn save_torrent_downloads(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = conn.execute(
            "INSERT INTO torrents (info_hash, completed) VALUES (?1, ?2) ON CONFLICT(info_hash) DO UPDATE SET completed = ?2",
            [info_hash.to_string(), completed.to_string()],
        )?;

        if insert == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }

    fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let _ = conn.execute(
            "UPDATE torrents SET completed = completed + 1 WHERE info_hash = ?",
            [info_hash.to_string()],
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
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let metric_name = TORRENTS_DOWNLOADS_TOTAL;

        let _ = conn.execute(
            "UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = ?",
            [metric_name],
        )?;

        Ok(())
    }
}
