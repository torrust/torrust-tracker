//! Lookup of persisted per-torrent download counts.
//!
//! Announce and scrape share this lookup so both report the same `downloaded`
//! count for a torrent whose swarm is absent from memory. See
//! [ADR-20261002173716](../../../../docs/adrs/20261002173716_scrape_reports_announce_swarm_stats_without_side_effects.md).
use std::sync::Arc;

use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::NumberOfDownloads;

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
}
