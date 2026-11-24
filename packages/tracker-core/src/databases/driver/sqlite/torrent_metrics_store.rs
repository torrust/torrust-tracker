use std::str::FromStr;

use ::sqlx::Row;
use ::sqlx::sqlite::SqliteRow;
use async_trait::async_trait;
use torrust_info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsPerInfoHash};

use super::{DRIVER, Sqlite};
use crate::databases::TorrentMetricsStore;
use crate::databases::driver::{MAX_INFO_HASHES_PER_QUERY, TORRENTS_DOWNLOADS_TOTAL};
use crate::databases::error::Error;

fn torrent_downloads_from_row(row: &SqliteRow) -> Result<(InfoHash, NumberOfDownloads), Error> {
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
impl TorrentMetricsStore for Sqlite {
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
                ::sqlx::QueryBuilder::<::sqlx::Sqlite>::new("SELECT info_hash, completed FROM torrents WHERE info_hash IN (");
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Sqlite;
    use crate::databases::TorrentMetricsStore;
    use crate::databases::driver::sqlite::fixture::{execute, migrated_store};
    use crate::databases::error::Error;
    use crate::test_helpers::tests::{EphemeralConfiguration, sample_info_hash};

    const AN_INFO_HASH_SQL: &str = "'3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0'";

    /// What is wrong with the database the store under test reads.
    enum Broken {
        /// One stored torrent that cannot be read: the SQL literals of its info-hash and of its count.
        Row {
            info_hash_sql: &'static str,
            completed_sql: &'static str,
        },
        /// The `torrents` table does not exist, so every torrent query fails.
        TorrentsTableMissing,
        /// The `torrent_aggregate_metrics` table does not exist, so every global-total query fails.
        AggregateMetricsTableMissing,
    }

    /// A write of the store, to run against a database that cannot take it.
    enum Write {
        SaveTorrent,
        IncreaseTorrent,
        IncreaseGlobal,
    }

    impl Write {
        async fn perform(&self, store: &Sqlite) -> Result<(), Error> {
            match self {
                Self::SaveTorrent => store.save_torrent_downloads(&sample_info_hash(), 1).await,
                Self::IncreaseTorrent => store.increase_downloads_for_torrent(&sample_info_hash()).await,
                Self::IncreaseGlobal => store.increase_global_downloads().await,
            }
        }
    }

    async fn broken_store(broken: Broken) -> (Sqlite, EphemeralConfiguration) {
        let (store, ephemeral) = migrated_store().await;

        let statement = match broken {
            Broken::Row {
                info_hash_sql,
                completed_sql,
            } => format!("INSERT INTO torrents (info_hash, completed) VALUES ({info_hash_sql}, {completed_sql})"),
            Broken::TorrentsTableMissing => "DROP TABLE torrents".to_string(),
            Broken::AggregateMetricsTableMissing => "DROP TABLE torrent_aggregate_metrics".to_string(),
        };
        execute(&store, &statement).await;

        (store, ephemeral)
    }

    #[rstest]
    #[case::a_completed_below_zero(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "-1" })]
    #[case::a_completed_above_u32(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "4294967296" })]
    #[case::an_info_hash_that_does_not_parse(Broken::Row { info_hash_sql: "'nope'", completed_sql: "1" })]
    #[tokio::test]
    async fn it_should_report_a_corrupted_row_as_a_malformed_record_when_loading_all_the_torrents(#[case] broken: Broken) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = store.load_all_torrents_downloads().await;

        assert!(matches!(result, Err(Error::MalformedDatabaseRecord { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_completed_that_is_not_an_integer(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "'many'" })]
    #[case::an_info_hash_that_is_not_text(Broken::Row { info_hash_sql: "X'00'", completed_sql: "1" })]
    #[case::a_missing_torrents_table(Broken::TorrentsTableMissing)]
    #[tokio::test]
    async fn it_should_report_an_unreadable_row_or_a_failed_query_as_an_invalid_query_when_loading_all_the_torrents(
        #[case] broken: Broken,
    ) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = store.load_all_torrents_downloads().await;

        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_completed_below_zero(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "-1" })]
    #[case::a_completed_above_u32(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "4294967296" })]
    #[tokio::test]
    async fn it_should_report_a_corrupted_row_as_a_malformed_record_when_loading_the_requested_torrents(#[case] broken: Broken) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = store.load_torrents_downloads(&[sample_info_hash()]).await;

        assert!(matches!(result, Err(Error::MalformedDatabaseRecord { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_completed_that_is_not_an_integer(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "'many'" })]
    #[case::a_missing_torrents_table(Broken::TorrentsTableMissing)]
    #[tokio::test]
    async fn it_should_report_an_unreadable_row_or_a_failed_query_as_an_invalid_query_when_loading_the_requested_torrents(
        #[case] broken: Broken,
    ) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = store.load_torrents_downloads(&[sample_info_hash()]).await;

        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_completed_below_zero(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "-1" })]
    #[case::a_completed_above_u32(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "4294967296" })]
    #[tokio::test]
    async fn it_should_report_a_corrupted_row_as_a_malformed_record_when_loading_one_torrent(#[case] broken: Broken) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = store.load_torrent_downloads(&sample_info_hash()).await;

        assert!(matches!(result, Err(Error::MalformedDatabaseRecord { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_completed_that_is_not_an_integer(Broken::Row { info_hash_sql: AN_INFO_HASH_SQL, completed_sql: "'many'" })]
    #[case::a_missing_torrents_table(Broken::TorrentsTableMissing)]
    #[tokio::test]
    async fn it_should_report_an_unreadable_row_or_a_failed_query_as_an_invalid_query_when_loading_one_torrent(
        #[case] broken: Broken,
    ) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = store.load_torrent_downloads(&sample_info_hash()).await;

        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[tokio::test]
    async fn it_should_overwrite_the_stored_downloads_when_a_torrent_is_saved_again() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;
        store.save_torrent_downloads(&sample_info_hash(), 1).await.unwrap();

        // Act
        store.save_torrent_downloads(&sample_info_hash(), 7).await.unwrap();

        // Assert
        assert_eq!(store.load_torrent_downloads(&sample_info_hash()).await.unwrap(), Some(7));
    }

    #[tokio::test]
    async fn it_should_store_and_load_back_the_largest_number_of_downloads() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;

        // Act
        store.save_torrent_downloads(&sample_info_hash(), u32::MAX).await.unwrap();

        // Assert
        assert_eq!(
            store.load_torrent_downloads(&sample_info_hash()).await.unwrap(),
            Some(u32::MAX)
        );
    }

    #[tokio::test]
    async fn it_should_not_create_a_torrent_when_increasing_the_downloads_of_one_that_is_not_stored() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;

        // Act
        let result = store.increase_downloads_for_torrent(&sample_info_hash()).await;

        // Assert
        assert!(result.is_ok(), "got {result:?}");
        assert_eq!(store.load_torrent_downloads(&sample_info_hash()).await.unwrap(), None);
    }

    #[tokio::test]
    async fn it_should_not_create_the_global_downloads_when_increasing_them_before_they_are_stored() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;

        // Act
        let result = store.increase_global_downloads().await;

        // Assert
        assert!(result.is_ok(), "got {result:?}");
        assert_eq!(store.load_global_downloads().await.unwrap(), None);
    }

    #[rstest]
    #[case::saving_a_torrent(Write::SaveTorrent, Broken::TorrentsTableMissing)]
    #[case::increasing_a_torrent(Write::IncreaseTorrent, Broken::TorrentsTableMissing)]
    #[case::increasing_the_global_downloads(Write::IncreaseGlobal, Broken::AggregateMetricsTableMissing)]
    #[tokio::test]
    async fn it_should_report_a_failed_write_as_an_invalid_query(#[case] write: Write, #[case] broken: Broken) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = write.perform(&store).await;

        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }
}
