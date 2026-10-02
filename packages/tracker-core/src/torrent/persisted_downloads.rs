//! Lookup of persisted per-torrent download counts.
//!
//! Announce and scrape share this lookup so both report the same `downloaded`
//! count for a torrent whose swarm is absent from memory. See
//! [ADR-20261002173716](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20261002173716_scrape_reports_announce_swarm_stats_without_side_effects.md).
use std::sync::Arc;

use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsPerInfoHash};

use crate::databases;
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;

/// Loads persisted download counts when persistent completed statistics are
/// enabled.
pub struct PersistedDownloads {
    db_downloads_metric_repository: Option<Arc<DatabaseDownloadsMetricRepository>>,
}

impl PersistedDownloads {
    /// Lookup that never reads the database.
    pub const fn disabled() -> Self {
        Self {
            db_downloads_metric_repository: None,
        }
    }

    /// Lookup backed by the persisted downloads repository.
    pub fn enabled(db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>) -> Self {
        Self {
            db_downloads_metric_repository: Some(db_downloads_metric_repository.clone()),
        }
    }

    /// Returns the persisted downloads count of a torrent.
    ///
    /// Returns `None` when the lookup is disabled or the torrent has no
    /// persisted count. Callers use it only for swarms absent from memory,
    /// because an in-memory swarm already carries the count.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn load(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, databases::error::Error> {
        match &self.db_downloads_metric_repository {
            Some(repository) => repository.load_torrent_downloads(info_hash).await,
            None => Ok(None),
        }
    }

    /// Returns the persisted downloads counts of several torrents in one
    /// lookup, avoiding one query per torrent.
    ///
    /// Torrents without a persisted count are absent from the result. No
    /// query runs when the lookup is disabled or `info_hashes` is empty.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn load_many(&self, info_hashes: &[InfoHash]) -> Result<NumberOfDownloadsPerInfoHash, databases::error::Error> {
        match &self.db_downloads_metric_repository {
            Some(repository) if !info_hashes.is_empty() => repository.load_torrents_downloads(info_hashes).await,
            _ => Ok(NumberOfDownloadsPerInfoHash::new()),
        }
    }
}
