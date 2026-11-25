//! The repository that persists the whitelist.
use std::sync::Arc;

use torrust_info_hash::InfoHash;

use crate::databases::{self, WhitelistStore};

/// The persisted list of allowed torrents.
///
/// This repository handles adding, removing, and loading torrents
/// from a persistent database like `SQLite` or `MySQL`.
pub struct DatabaseWhitelist {
    /// A whitelist store implementation (e.g., `SQLite3` or `MySQL`).
    database: Arc<dyn WhitelistStore>,
}

impl DatabaseWhitelist {
    /// Creates a new `DatabaseWhitelist`.
    #[must_use]
    pub fn new(database: Arc<dyn WhitelistStore>) -> Self {
        Self { database }
    }

    /// Adds a torrent to the whitelist if not already present.
    ///
    /// # Errors
    /// Returns a `database::Error` if unable to add the `info_hash` to the
    /// whitelist.
    pub(crate) async fn add(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        // The store write is idempotent and atomic: no read before it, which would race.
        self.database.add_info_hash_to_whitelist(*info_hash).await?;

        Ok(())
    }

    /// Removes a torrent from the whitelist if it exists.
    ///
    /// # Errors
    /// Returns a `database::Error` if unable to remove the `info_hash`.
    pub(crate) async fn remove(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        // The store write is idempotent and atomic: no read before it, which would race.
        self.database.remove_info_hash_from_whitelist(*info_hash).await?;

        Ok(())
    }

    /// Loads the entire whitelist from the database.
    ///
    /// # Errors
    /// Returns a `database::Error` if unable to load whitelisted `info_hash`
    /// values.
    pub(crate) async fn load_from_database(&self) -> Result<Vec<InfoHash>, databases::error::Error> {
        self.database.load_whitelist().await
    }
}

#[cfg(test)]
mod tests {
    mod the_persisted_whitelist_repository {

        use std::sync::Arc;

        use torrust_tracker_configuration::v3_0_0::core::Core;

        use crate::databases::setup::initialize_database;
        use crate::test_helpers::tests::{ephemeral_configuration_for_listed_tracker, sample_info_hash};
        use crate::whitelist::repository::persisted::DatabaseWhitelist;

        async fn initialize_database_whitelist(configuration: &Core) -> DatabaseWhitelist {
            let stores = initialize_database(configuration).await;
            DatabaseWhitelist::new(stores.whitelist_store)
        }

        #[tokio::test]
        async fn should_add_a_new_infohash_to_the_list() {
            let ephemeral = ephemeral_configuration_for_listed_tracker();
            let whitelist = initialize_database_whitelist(&ephemeral.config).await;

            let infohash = sample_info_hash();

            let _result = whitelist.add(&infohash).await;

            assert_eq!(whitelist.load_from_database().await.unwrap(), vec![infohash]);
        }

        #[tokio::test]
        async fn should_remove_a_infohash_from_the_list() {
            let ephemeral = ephemeral_configuration_for_listed_tracker();
            let whitelist = initialize_database_whitelist(&ephemeral.config).await;

            let infohash = sample_info_hash();

            let _result = whitelist.add(&infohash).await;

            let _result = whitelist.remove(&infohash).await;

            assert_eq!(whitelist.load_from_database().await.unwrap(), vec![]);
        }

        #[tokio::test]
        async fn should_load_all_infohashes_from_the_database() {
            let ephemeral = ephemeral_configuration_for_listed_tracker();
            let whitelist = initialize_database_whitelist(&ephemeral.config).await;

            let infohash = sample_info_hash();

            let _result = whitelist.add(&infohash).await;

            let result = whitelist.load_from_database().await.unwrap();

            assert_eq!(result, vec![infohash]);
        }

        #[tokio::test]
        async fn should_not_add_the_same_infohash_to_the_list_twice() {
            let ephemeral = ephemeral_configuration_for_listed_tracker();
            let whitelist = initialize_database_whitelist(&ephemeral.config).await;

            let infohash = sample_info_hash();

            let _result = whitelist.add(&infohash).await;
            let _result = whitelist.add(&infohash).await;

            assert_eq!(whitelist.load_from_database().await.unwrap(), vec![infohash]);
        }

        #[tokio::test]
        async fn should_not_fail_removing_an_infohash_that_is_not_in_the_list() {
            let ephemeral = ephemeral_configuration_for_listed_tracker();
            let whitelist = initialize_database_whitelist(&ephemeral.config).await;

            let infohash = sample_info_hash();

            let result = whitelist.remove(&infohash).await;

            assert!(result.is_ok());
        }

        // Concurrent identical calls must all succeed: both operations are idempotent.
        //
        // The race window is the gap between the read and the write, so these tests need a
        // multi-threaded runtime and many rounds to fail reliably on the racy code.
        const CONCURRENT_ROUNDS: usize = 200;

        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn should_not_fail_on_concurrent_identical_adds() {
            let ephemeral = ephemeral_configuration_for_listed_tracker();
            let whitelist = Arc::new(initialize_database_whitelist(&ephemeral.config).await);
            let infohash = sample_info_hash();

            for round in 0..CONCURRENT_ROUNDS {
                let (first, second) = (whitelist.clone(), whitelist.clone());
                let (first, second) = tokio::join!(
                    tokio::spawn(async move { first.add(&infohash).await }),
                    tokio::spawn(async move { second.add(&infohash).await })
                );
                for result in [first.unwrap(), second.unwrap()] {
                    assert!(result.is_ok(), "concurrent add failed in round {round}: {result:?}");
                }
                assert!(whitelist.load_from_database().await.unwrap().contains(&infohash));

                whitelist.remove(&infohash).await.unwrap();
            }
        }

        #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
        async fn should_not_fail_on_concurrent_identical_removes() {
            let ephemeral = ephemeral_configuration_for_listed_tracker();
            let whitelist = Arc::new(initialize_database_whitelist(&ephemeral.config).await);
            let infohash = sample_info_hash();

            for round in 0..CONCURRENT_ROUNDS {
                whitelist.add(&infohash).await.unwrap();

                let (first, second) = (whitelist.clone(), whitelist.clone());
                let (first, second) = tokio::join!(
                    tokio::spawn(async move { first.remove(&infohash).await }),
                    tokio::spawn(async move { second.remove(&infohash).await })
                );
                for result in [first.unwrap(), second.unwrap()] {
                    assert!(result.is_ok(), "concurrent remove failed in round {round}: {result:?}");
                }
                assert!(!whitelist.load_from_database().await.unwrap().contains(&infohash));
            }
        }
    }

    mod the_persisted_whitelist_repository_with_a_failing_store {
        use std::sync::Arc;

        use torrust_tracker_primitives::Driver;

        use crate::databases::error::Error;
        use crate::databases::{MockWhitelistStore, WhitelistStore};
        use crate::test_helpers::tests::sample_info_hash;
        use crate::whitelist::repository::persisted::DatabaseWhitelist;

        fn failed_insert() -> Error {
            Error::InsertFailed { driver: Driver::Sqlite3 }
        }

        fn failed_delete() -> Error {
            Error::DeleteFailed {
                error_code: 1,
                driver: Driver::Sqlite3,
            }
        }

        fn whitelist_over(store: MockWhitelistStore) -> DatabaseWhitelist {
            let store: Arc<dyn WhitelistStore> = Arc::new(store);
            DatabaseWhitelist::new(store)
        }

        #[tokio::test]
        async fn it_should_not_write_when_the_check_before_adding_fails() {
            // Arrange
            let mut store = MockWhitelistStore::new();
            store
                .expect_is_info_hash_whitelisted()
                .times(1)
                .returning(|_| Box::pin(async { Err(failed_insert()) }));
            store.expect_add_info_hash_to_whitelist().never();
            let whitelist = whitelist_over(store);

            // Act
            let result = whitelist.add(&sample_info_hash()).await;

            // Assert
            assert!(
                matches!(result, Err(Error::InsertFailed { .. })),
                "a failed check should be returned, got {result:?}"
            );
        }

        #[tokio::test]
        async fn it_should_return_the_error_when_the_write_of_an_add_fails() {
            // Arrange
            let mut store = MockWhitelistStore::new();
            store
                .expect_is_info_hash_whitelisted()
                .times(1)
                .returning(|_| Box::pin(async { Ok(false) }));
            store
                .expect_add_info_hash_to_whitelist()
                .times(1)
                .returning(|_| Box::pin(async { Err(failed_insert()) }));
            let whitelist = whitelist_over(store);

            // Act
            let result = whitelist.add(&sample_info_hash()).await;

            // Assert
            assert!(
                matches!(result, Err(Error::InsertFailed { .. })),
                "a failed write should be returned, got {result:?}"
            );
        }

        #[tokio::test]
        async fn it_should_not_delete_when_the_check_before_removing_fails() {
            // Arrange
            let mut store = MockWhitelistStore::new();
            store
                .expect_is_info_hash_whitelisted()
                .times(1)
                .returning(|_| Box::pin(async { Err(failed_delete()) }));
            store.expect_remove_info_hash_from_whitelist().never();
            let whitelist = whitelist_over(store);

            // Act
            let result = whitelist.remove(&sample_info_hash()).await;

            // Assert
            assert!(
                matches!(result, Err(Error::DeleteFailed { .. })),
                "a failed check should be returned, got {result:?}"
            );
        }

        #[tokio::test]
        async fn it_should_return_the_error_when_the_delete_of_a_remove_fails() {
            // Arrange
            let mut store = MockWhitelistStore::new();
            store
                .expect_is_info_hash_whitelisted()
                .times(1)
                .returning(|_| Box::pin(async { Ok(true) }));
            store
                .expect_remove_info_hash_from_whitelist()
                .times(1)
                .returning(|_| Box::pin(async { Err(failed_delete()) }));
            let whitelist = whitelist_over(store);

            // Act
            let result = whitelist.remove(&sample_info_hash()).await;

            // Assert
            assert!(
                matches!(result, Err(Error::DeleteFailed { .. })),
                "a failed delete should be returned, got {result:?}"
            );
        }

        #[tokio::test]
        async fn it_should_return_the_error_when_loading_the_whitelist_fails() {
            // Arrange
            let mut store = MockWhitelistStore::new();
            store.expect_load_whitelist().times(1).returning(|| {
                Box::pin(async {
                    Err(Error::MalformedDatabaseRecord {
                        message: "corrupt info-hash".to_string(),
                        driver: Driver::Sqlite3,
                    })
                })
            });
            let whitelist = whitelist_over(store);

            // Act
            let result = whitelist.load_from_database().await;

            // Assert
            assert!(
                matches!(result, Err(Error::MalformedDatabaseRecord { .. })),
                "a failed load should be returned, got {result:?}"
            );
        }
    }
}
