use std::str::FromStr;

use ::sqlx::Row;
use async_trait::async_trait;
use torrust_info_hash::InfoHash;

use super::{DRIVER, Sqlite};
use crate::databases::WhitelistStore;
use crate::databases::error::Error;

#[async_trait]
impl WhitelistStore for Sqlite {
    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        let rows = ::sqlx::query("SELECT info_hash FROM whitelist")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        rows.into_iter()
            .map(|row| {
                let value: String = row.try_get("info_hash").map_err(|e| (e, DRIVER))?;
                InfoHash::from_str(&value).map_err(|e| Error::MalformedDatabaseRecord {
                    message: format!("{e:?}"),
                    driver: DRIVER,
                })
            })
            .collect()
    }

    async fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error> {
        let maybe_row = ::sqlx::query("SELECT info_hash FROM whitelist WHERE info_hash = ?1")
            .bind(info_hash.to_hex_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        maybe_row
            .map(|row| {
                let value: String = row.try_get("info_hash").map_err(|e| (e, DRIVER))?;
                InfoHash::from_str(&value).map_err(|e| Error::MalformedDatabaseRecord {
                    message: format!("{e:?}"),
                    driver: DRIVER,
                })
            })
            .transpose()
    }

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let insert = ::sqlx::query("INSERT INTO whitelist (info_hash) VALUES (?1)")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        if insert == 0 {
            tracing::warn!(driver = %DRIVER, table = "whitelist", %info_hash, "the insert affected no rows");
            Err(Error::InsertFailed { driver: DRIVER })
        } else {
            usize::try_from(insert).map_err(|e| Error::MalformedDatabaseRecord {
                message: format!("rows_affected does not fit in usize: {e}"),
                driver: DRIVER,
            })
        }
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let deleted = ::sqlx::query("DELETE FROM whitelist WHERE info_hash = ?1")
            .bind(info_hash.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?
            .rows_affected();

        if deleted == 1 {
            // should only remove a single record.
            Ok(1)
        } else {
            tracing::warn!(driver = %DRIVER, table = "whitelist", %info_hash, rows_deleted = deleted, "the delete did not remove exactly one row");
            Err(Error::DeleteFailed {
                error_code: usize::try_from(deleted).unwrap_or(0),
                driver: DRIVER,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Sqlite;
    use crate::databases::WhitelistStore;
    use crate::databases::driver::sqlite::fixture::{execute, migrated_store};
    use crate::databases::error::Error;
    use crate::test_helpers::tests::{EphemeralConfiguration, sample_info_hash};

    /// What is wrong with the database the store under test reads.
    enum Broken {
        /// One stored info-hash that cannot be read: its SQL literal.
        Row { info_hash_sql: &'static str },
        /// The `whitelist` table does not exist, so every query fails.
        WhitelistTableMissing,
    }

    async fn broken_store(broken: Broken) -> (Sqlite, EphemeralConfiguration) {
        let (store, ephemeral) = migrated_store().await;

        let statement = match broken {
            Broken::Row { info_hash_sql } => format!("INSERT INTO whitelist (info_hash) VALUES ({info_hash_sql})"),
            Broken::WhitelistTableMissing => "DROP TABLE whitelist".to_string(),
        };
        execute(&store, &statement).await;

        (store, ephemeral)
    }

    #[rstest]
    #[case::an_info_hash_that_does_not_parse(Broken::Row { info_hash_sql: "'nope'" })]
    #[tokio::test]
    async fn it_should_report_a_corrupted_row_as_a_malformed_record_when_loading_the_whitelist(#[case] broken: Broken) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = store.load_whitelist().await;

        assert!(matches!(result, Err(Error::MalformedDatabaseRecord { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::an_info_hash_that_is_not_text(Broken::Row { info_hash_sql: "X'00'" })]
    #[case::a_missing_whitelist_table(Broken::WhitelistTableMissing)]
    #[tokio::test]
    async fn it_should_report_an_unreadable_row_or_a_failed_query_as_an_invalid_query_when_loading_the_whitelist(
        #[case] broken: Broken,
    ) {
        let (store, _ephemeral) = broken_store(broken).await;

        let result = store.load_whitelist().await;

        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[tokio::test]
    async fn it_should_report_a_failed_query_as_an_invalid_query_when_getting_an_info_hash() {
        // Arrange
        let (store, _ephemeral) = broken_store(Broken::WhitelistTableMissing).await;

        // Act
        let result = store.get_info_hash_from_whitelist(sample_info_hash()).await;

        // Assert
        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[tokio::test]
    async fn it_should_report_a_failed_query_as_an_invalid_query_when_removing_an_info_hash() {
        // Arrange
        let (store, _ephemeral) = broken_store(Broken::WhitelistTableMissing).await;

        // Act
        let result = store.remove_info_hash_from_whitelist(sample_info_hash()).await;

        // Assert
        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }
}
