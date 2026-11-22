pub mod downloads;

use std::sync::Arc;

use thiserror::Error;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::{metric_collection, metric_name};

use super::repository::Repository;
use super::{TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL, TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL};
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
    if let Some(downloads) = db_downloads_metric_repository.load_global_downloads().await? {
        stats_repository
            .set_counter(
                &metric_name!(TRACKER_CORE_PERSISTENT_TORRENTS_DOWNLOADS_TOTAL),
                &LabelSet::default(),
                u64::from(downloads),
                now,
            )
            .await?;
        stats_repository
            .set_counter(
                &metric_name!(TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL),
                &LabelSet::default(),
                u64::from(downloads),
                now,
            )
            .await?;
    }

    Ok(())
}

#[derive(Error, Debug, Clone)]
#[non_exhaustive]
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_metrics::label::LabelSet;
    use torrust_metrics::metric_name;
    use torrust_tracker_primitives::{Driver, NumberOfDownloads};

    use super::{Error, load_persisted_metrics};
    use crate::databases::error::Error as DatabaseError;
    use crate::databases::{MockTorrentMetricsStore, TorrentMetricsStore};
    use crate::statistics::TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL;
    use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
    use crate::statistics::repository::Repository;

    const NOW: DurationSinceUnixEpoch = DurationSinceUnixEpoch::from_secs(0);

    /// A downloads repository whose store answers the global-downloads query once, with `stored`.
    fn downloads_repository_storing(
        stored: Result<Option<NumberOfDownloads>, DatabaseError>,
    ) -> Arc<DatabaseDownloadsMetricRepository> {
        let mut store = MockTorrentMetricsStore::new();
        store
            .expect_load_global_downloads()
            .times(1)
            .return_once(move || Box::pin(std::future::ready(stored)));
        let store: Arc<dyn TorrentMetricsStore> = Arc::new(store);

        Arc::new(DatabaseDownloadsMetricRepository::new(&store))
    }

    /// A statistics repository with every downloads total registered as a counter.
    fn statistics_repository() -> Arc<Repository> {
        Arc::new(Repository::new(true, true))
    }

    async fn persistent_and_persisted_totals(repository: &Repository) -> (u64, u64) {
        (
            repository.get_torrents_downloads_total().await,
            repository.get_torrents_downloads_persisted_total().await,
        )
    }

    #[tokio::test]
    async fn it_should_set_the_persistent_and_the_persisted_totals_to_the_stored_global_downloads() {
        // Arrange
        let downloads_repository = downloads_repository_storing(Ok(Some(7)));
        let repository = statistics_repository();

        // Act
        let result = load_persisted_metrics(&repository, &downloads_repository, NOW).await;

        // Assert
        assert!(result.is_ok(), "loading a stored total should succeed, got {result:?}");
        assert_eq!(
            persistent_and_persisted_totals(&repository).await,
            (7, 7),
            "both totals should hold the stored global downloads"
        );
        assert_eq!(
            repository.get_torrents_downloads_in_session_total().await,
            0,
            "the in-session total counts only this process's downloads"
        );
    }

    #[tokio::test]
    async fn it_should_set_both_totals_to_the_largest_stored_value() {
        // Arrange
        let downloads_repository = downloads_repository_storing(Ok(Some(u32::MAX)));
        let repository = statistics_repository();

        // Act
        load_persisted_metrics(&repository, &downloads_repository, NOW).await.unwrap();

        // Assert
        let largest = u64::from(u32::MAX);
        assert_eq!(persistent_and_persisted_totals(&repository).await, (largest, largest));
    }

    #[tokio::test]
    async fn it_should_leave_the_totals_at_zero_when_no_global_downloads_are_stored() {
        // Arrange
        let downloads_repository = downloads_repository_storing(Ok(None));
        let repository = statistics_repository();

        // Act
        let result = load_persisted_metrics(&repository, &downloads_repository, NOW).await;

        // Assert
        assert!(result.is_ok(), "an empty store is not an error, got {result:?}");
        assert_eq!(persistent_and_persisted_totals(&repository).await, (0, 0));
    }

    #[tokio::test]
    async fn it_should_return_a_database_error_and_load_nothing_when_the_store_fails() {
        // Arrange
        let downloads_repository = downloads_repository_storing(Err(DatabaseError::MalformedDatabaseRecord {
            message: "corrupt record".to_string(),
            driver: Driver::Sqlite3,
        }));
        let repository = statistics_repository();

        // Act
        let result = load_persisted_metrics(&repository, &downloads_repository, NOW).await;

        // Assert
        assert!(
            matches!(result, Err(Error::DatabaseError { .. })),
            "a store failure should be a database error, got {result:?}"
        );
        assert_eq!(persistent_and_persisted_totals(&repository).await, (0, 0));
    }

    #[tokio::test]
    async fn it_should_return_a_metrics_error_and_keep_the_first_total_when_the_repository_refuses_the_second() {
        // Arrange
        let downloads_repository = downloads_repository_storing(Ok(Some(7)));
        let repository = Arc::new(Repository::new(true, false));
        repository
            .set_gauge(
                &metric_name!(TRACKER_CORE_PERSISTED_TORRENTS_DOWNLOADS_TOTAL),
                &LabelSet::default(),
                1.0,
                NOW,
            )
            .await
            .unwrap();

        // Act
        let result = load_persisted_metrics(&repository, &downloads_repository, NOW).await;

        // Assert
        assert!(
            matches!(result, Err(Error::MetricsError { .. })),
            "a refused write should be a metrics error, got {result:?}"
        );
        assert_eq!(
            repository.get_torrents_downloads_total().await,
            7,
            "the load is not atomic: the persistent total written first stays"
        );
    }
}
