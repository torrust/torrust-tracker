pub mod downloads;

use std::sync::Arc;

use thiserror::Error;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::{metric_collection, metric_name};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use super::repository::Repository;
use super::TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL;
use crate::databases;
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;

/// Loads persisted metrics from the database and sets them in the stats repository.
///
/// # Errors
///
/// This function will return an error if the database query fails or if the
/// metric collection fails to set the initial metric values.
pub async fn load_persisted_metrics(
    stats_repository: &Arc<Repository>,
    db_downloads_metric_repository: &Arc<DatabaseDownloadsMetricRepository>,
    now: DurationSinceUnixEpoch,
) -> Result<(), Error> {
    if let Some(downloads) = db_downloads_metric_repository.load_global_downloads()? {
        stats_repository
            .set_counter(
                &metric_name!(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL),
                &LabelSet::default(),
                u64::from(downloads),
                now,
            )
            .await?;
    }

    Ok(())
}

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Database error: {err}")]
    DatabaseError { err: databases::error::Error },

    #[error("Metrics error: {err}")]
    MetricsError { err: metric_collection::Error },
}

impl From<databases::error::Error> for Error {
    fn from(err: databases::error::Error) -> Self {
        Self::DatabaseError { err }
    }
}

impl From<metric_collection::Error> for Error {
    fn from(err: metric_collection::Error) -> Self {
        Self::MetricsError { err }
    }
}
