use ::sqlx::Row;
use async_trait::async_trait;

use super::{DRIVER, Sqlite};
use crate::authentication::{self, Key};
use crate::databases::AuthKeyStore;
use crate::databases::driver::peer_key_from_row;
use crate::databases::error::Error;

#[async_trait]
impl AuthKeyStore for Sqlite {
    async fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        let rows = ::sqlx::query("SELECT key, valid_until FROM keys")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        rows.into_iter()
            .map(|row| {
                let key_value: String = row.try_get("key").map_err(|e| (e, DRIVER))?;
                let valid_until: Option<i64> = row.try_get("valid_until").map_err(|e| (e, DRIVER))?;

                peer_key_from_row(&key_value, valid_until, &DRIVER)
            })
            .collect()
    }

    async fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        let maybe_row = ::sqlx::query("SELECT key, valid_until FROM keys WHERE key = ?1")
            .bind(key.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::from_statement_that_binds_a_secret(e, DRIVER))?;

        maybe_row
            .map(|row| {
                let key_value: String = row.try_get("key").map_err(|e| (e, DRIVER))?;
                let valid_until: Option<i64> = row.try_get("valid_until").map_err(|e| (e, DRIVER))?;

                peer_key_from_row(&key_value, valid_until, &DRIVER)
            })
            .transpose()
    }

    async fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        let valid_until = auth_key
            .valid_until
            .map(|value| {
                i64::try_from(value.as_secs()).map_err(|e| Error::MalformedDatabaseRecord {
                    message: e.to_string(),
                    driver: DRIVER,
                })
            })
            .transpose()?;

        let insert = ::sqlx::query("INSERT INTO keys (key, valid_until) VALUES (?1, ?2)")
            .bind(auth_key.key.to_string())
            .bind(valid_until)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::from_statement_that_binds_a_secret(e, DRIVER))?
            .rows_affected();

        if insert == 0 {
            tracing::warn!(driver = %DRIVER, table = "keys", "the insert affected no rows");
            Err(Error::InsertFailed { driver: DRIVER })
        } else {
            usize::try_from(insert).map_err(|e| Error::MalformedDatabaseRecord {
                message: format!("rows_affected does not fit in usize: {e}"),
                driver: DRIVER,
            })
        }
    }

    async fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        let deleted = ::sqlx::query("DELETE FROM keys WHERE key = ?1")
            .bind(key.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| Error::from_statement_that_binds_a_secret(e, DRIVER))?
            .rows_affected();

        if deleted == 1 {
            // should only remove a single record.
            Ok(1)
        } else {
            tracing::warn!(driver = %DRIVER, table = "keys", rows_deleted = deleted, "the delete did not remove exactly one row");
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
    use torrust_clock::DurationSinceUnixEpoch;

    use super::Sqlite;
    use crate::authentication::{Key, PeerKey};
    use crate::databases::AuthKeyStore;
    use crate::databases::driver::sqlite::fixture::{execute, migrated_store};
    use crate::databases::error::Error;
    use crate::test_helpers::tests::EphemeralConfiguration;

    const A_KEY: &str = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ";
    const A_KEY_SQL: &str = "'YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ'";

    /// What is wrong with the database the store under test reads.
    enum Broken {
        /// One stored row that cannot be read: the SQL literals of its key and of its expiry.
        Row {
            key_sql: &'static str,
            valid_until_sql: &'static str,
        },
        /// The `keys` table does not exist, so every query fails.
        KeysTableMissing,
    }

    fn a_key() -> Key {
        A_KEY.parse::<Key>().unwrap()
    }

    async fn broken_key_store(broken: Broken) -> (Sqlite, EphemeralConfiguration) {
        let (store, ephemeral) = migrated_store().await;

        let statement = match broken {
            Broken::Row {
                key_sql,
                valid_until_sql,
            } => {
                format!("INSERT INTO keys (key, valid_until) VALUES ({key_sql}, {valid_until_sql})")
            }
            Broken::KeysTableMissing => "DROP TABLE keys".to_string(),
        };
        execute(&store, &statement).await;

        (store, ephemeral)
    }

    #[rstest]
    #[case::a_key_that_does_not_parse(Broken::Row { key_sql: "'short'", valid_until_sql: "NULL" })]
    #[case::a_negative_valid_until(Broken::Row { key_sql: A_KEY_SQL, valid_until_sql: "-1" })]
    #[tokio::test]
    async fn it_should_report_a_corrupted_row_as_a_malformed_record_when_loading_the_keys(#[case] broken: Broken) {
        let (store, _ephemeral) = broken_key_store(broken).await;

        let result = store.load_keys().await;

        assert!(matches!(result, Err(Error::MalformedDatabaseRecord { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_key_that_is_not_text(Broken::Row { key_sql: "X'00'", valid_until_sql: "NULL" })]
    #[case::a_valid_until_that_is_not_an_integer(Broken::Row { key_sql: A_KEY_SQL, valid_until_sql: "'soon'" })]
    #[case::a_missing_keys_table(Broken::KeysTableMissing)]
    #[tokio::test]
    async fn it_should_report_an_unreadable_row_or_a_failed_query_as_an_invalid_query_when_loading_the_keys(
        #[case] broken: Broken,
    ) {
        let (store, _ephemeral) = broken_key_store(broken).await;

        let result = store.load_keys().await;

        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_negative_valid_until(Broken::Row { key_sql: A_KEY_SQL, valid_until_sql: "-1" })]
    #[tokio::test]
    async fn it_should_report_a_corrupted_row_as_a_malformed_record_when_getting_a_key(#[case] broken: Broken) {
        let (store, _ephemeral) = broken_key_store(broken).await;

        let result = store.get_key_from_keys(&a_key()).await;

        assert!(matches!(result, Err(Error::MalformedDatabaseRecord { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_valid_until_that_is_not_an_integer(Broken::Row { key_sql: A_KEY_SQL, valid_until_sql: "'soon'" })]
    #[case::a_missing_keys_table(Broken::KeysTableMissing)]
    #[tokio::test]
    async fn it_should_report_an_unreadable_row_or_a_failed_query_as_an_invalid_query_when_getting_a_key(#[case] broken: Broken) {
        let (store, _ephemeral) = broken_key_store(broken).await;

        let result = store.get_key_from_keys(&a_key()).await;

        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[tokio::test]
    async fn it_should_refuse_to_add_a_key_that_expires_after_the_largest_stored_timestamp() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;
        let key = PeerKey {
            key: a_key(),
            valid_until: Some(DurationSinceUnixEpoch::from_secs(u64::MAX)),
        };

        // Act
        let result = store.add_key_to_keys(&key).await;

        // Assert
        assert!(matches!(result, Err(Error::MalformedDatabaseRecord { .. })), "got {result:?}");
        assert!(
            store.load_keys().await.unwrap().is_empty(),
            "a refused key must not be stored"
        );
    }

    #[tokio::test]
    async fn it_should_refuse_to_add_a_key_that_is_already_stored_and_keep_the_stored_one() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;
        let stored = PeerKey {
            key: a_key(),
            valid_until: None,
        };
        store.add_key_to_keys(&stored).await.unwrap();
        let duplicate = PeerKey {
            key: a_key(),
            valid_until: Some(DurationSinceUnixEpoch::from_secs(1_700_000_000)),
        };

        // Act
        let result = store.add_key_to_keys(&duplicate).await;

        // Assert
        assert!(matches!(result, Err(Error::KeyAlreadyExists { .. })), "got {result:?}");
        assert_eq!(
            store.load_keys().await.unwrap(),
            vec![stored],
            "the first key must stay, still permanent"
        );
    }

    #[tokio::test]
    async fn it_should_report_a_failed_query_as_an_invalid_query_when_removing_a_key() {
        // Arrange
        let (store, _ephemeral) = broken_key_store(Broken::KeysTableMissing).await;

        // Act
        let result = store.remove_key_from_keys(&a_key()).await;

        // Assert
        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }
}
