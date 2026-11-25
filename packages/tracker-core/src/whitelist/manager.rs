//! Whitelist manager.
//!
//! This module provides the `WhitelistManager` struct, which is responsible for
//! managing the whitelist of torrents.
use std::sync::Arc;

use torrust_info_hash::InfoHash;

use super::repository::in_memory::InMemoryWhitelist;
use super::repository::persisted::DatabaseWhitelist;
use crate::databases;
/// Manages the whitelist of allowed torrents.
///
/// This structure handles both the in-memory and persistent representations of
/// the whitelist. It is primarily relevant for private trackers that restrict
/// access to specific torrents.
pub struct WhitelistManager {
    /// The in-memory list of allowed torrents.
    in_memory_whitelist: Arc<InMemoryWhitelist>,

    /// The persisted list of allowed torrents.
    database_whitelist: Arc<DatabaseWhitelist>,
}

impl WhitelistManager {
    /// Creates a new `WhitelistManager` instance.
    ///
    /// # Arguments
    ///
    /// - `database_whitelist`: Persistent database-backed whitelist repository.
    /// - `in_memory_whitelist`: In-memory whitelist repository for fast runtime
    ///   access.
    ///
    /// # Returns
    ///
    /// A new `WhitelistManager` instance.
    #[must_use]
    pub const fn new(database_whitelist: Arc<DatabaseWhitelist>, in_memory_whitelist: Arc<InMemoryWhitelist>) -> Self {
        Self {
            in_memory_whitelist,
            database_whitelist,
        }
    }

    /// Adds a torrent to the whitelist.
    ///
    /// This operation is relevant for private trackers to control which
    /// torrents are allowed.
    ///
    /// # Errors
    /// Returns a `database::Error` if the operation fails in the database.
    pub async fn add_torrent_to_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        self.database_whitelist.add(info_hash).await?;
        self.in_memory_whitelist.add(info_hash).await;
        Ok(())
    }

    /// Removes a torrent from the whitelist.
    ///
    /// This operation is relevant for private trackers to revoke access to
    /// specific torrents.
    ///
    /// # Errors
    /// Returns a `database::Error` if the operation fails in the database.
    pub async fn remove_torrent_from_whitelist(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        self.database_whitelist.remove(info_hash).await?;
        self.in_memory_whitelist.remove(info_hash).await;
        Ok(())
    }

    /// Loads the whitelist from the database into memory.
    ///
    /// This is useful when restarting the tracker to ensure the in-memory
    /// whitelist is synchronized with the database.
    ///
    /// # Errors
    /// Returns a `database::Error` if the operation fails to load from the database.
    pub async fn load_whitelist_from_database(&self) -> Result<(), databases::error::Error> {
        let whitelisted_torrents_from_database = self.database_whitelist.load_from_database().await?;

        self.in_memory_whitelist.clear().await;

        for info_hash in whitelisted_torrents_from_database {
            let _: bool = self.in_memory_whitelist.add(&info_hash).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use torrust_tracker_configuration::v3_0_0::core::Core;

    use crate::databases::setup::initialize_database;
    use crate::whitelist::manager::WhitelistManager;
    use crate::whitelist::repository::in_memory::InMemoryWhitelist;
    use crate::whitelist::repository::persisted::DatabaseWhitelist;

    struct WhitelistManagerDeps {
        pub database_whitelist: Arc<DatabaseWhitelist>,
        pub in_memory_whitelist: Arc<InMemoryWhitelist>,
    }

    async fn initialize_whitelist_manager_and_deps(config: &Core) -> (Arc<WhitelistManager>, Arc<WhitelistManagerDeps>) {
        let stores = initialize_database(config).await;
        let database_whitelist = Arc::new(DatabaseWhitelist::new(stores.whitelist_store.clone()));
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());

        let whitelist_manager = Arc::new(WhitelistManager::new(database_whitelist.clone(), in_memory_whitelist.clone()));

        (
            whitelist_manager,
            Arc::new(WhitelistManagerDeps {
                database_whitelist,
                in_memory_whitelist,
            }),
        )
    }

    mod configured_as_whitelisted {

        mod handling_the_torrent_whitelist {
            use crate::test_helpers::tests::{ephemeral_configuration_for_listed_tracker, sample_info_hash};
            use crate::whitelist::manager::tests::initialize_whitelist_manager_and_deps;

            #[tokio::test]
            async fn it_should_add_a_torrent_to_the_whitelist() {
                let ephemeral = ephemeral_configuration_for_listed_tracker();
                let (whitelist_manager, services) = initialize_whitelist_manager_and_deps(&ephemeral.config).await;

                let info_hash = sample_info_hash();

                whitelist_manager.add_torrent_to_whitelist(&info_hash).await.unwrap();

                assert!(services.in_memory_whitelist.contains(&info_hash).await);
                assert!(
                    services
                        .database_whitelist
                        .load_from_database()
                        .await
                        .unwrap()
                        .contains(&info_hash)
                );
            }

            #[tokio::test]
            async fn it_should_remove_a_torrent_from_the_whitelist() {
                let ephemeral = ephemeral_configuration_for_listed_tracker();
                let (whitelist_manager, services) = initialize_whitelist_manager_and_deps(&ephemeral.config).await;

                let info_hash = sample_info_hash();

                whitelist_manager.add_torrent_to_whitelist(&info_hash).await.unwrap();

                whitelist_manager.remove_torrent_from_whitelist(&info_hash).await.unwrap();

                assert!(!services.in_memory_whitelist.contains(&info_hash).await);
                assert!(
                    !services
                        .database_whitelist
                        .load_from_database()
                        .await
                        .unwrap()
                        .contains(&info_hash)
                );
            }

            mod persistence {
                use crate::test_helpers::tests::{ephemeral_configuration_for_listed_tracker, sample_info_hash};
                use crate::whitelist::manager::tests::initialize_whitelist_manager_and_deps;

                #[tokio::test]
                async fn it_should_load_the_whitelist_from_the_database() {
                    let ephemeral = ephemeral_configuration_for_listed_tracker();
                    let (whitelist_manager, services) = initialize_whitelist_manager_and_deps(&ephemeral.config).await;

                    let info_hash = sample_info_hash();

                    services.database_whitelist.add(&info_hash).await.unwrap();

                    whitelist_manager.load_whitelist_from_database().await.unwrap();

                    assert!(services.in_memory_whitelist.contains(&info_hash).await);
                }
            }
        }
    }

    mod with_a_failing_or_changing_database {
        use std::sync::Arc;

        use torrust_tracker_primitives::Driver;

        use crate::databases::error::Error;
        use crate::databases::{MockWhitelistStore, WhitelistStore};
        use crate::test_helpers::tests::{sample_info_hash_one, sample_info_hash_two};
        use crate::whitelist::manager::WhitelistManager;
        use crate::whitelist::repository::in_memory::InMemoryWhitelist;
        use crate::whitelist::repository::persisted::DatabaseWhitelist;

        fn manager_over(store: MockWhitelistStore) -> (WhitelistManager, Arc<InMemoryWhitelist>) {
            let store: Arc<dyn WhitelistStore> = Arc::new(store);
            let database_whitelist = Arc::new(DatabaseWhitelist::new(store));
            let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());

            (
                WhitelistManager::new(database_whitelist, in_memory_whitelist.clone()),
                in_memory_whitelist,
            )
        }

        fn failed_insert() -> Error {
            Error::InsertFailed { driver: Driver::Sqlite3 }
        }

        #[tokio::test]
        async fn it_should_not_add_the_torrent_to_memory_when_persisting_it_fails() {
            // Arrange
            let info_hash = sample_info_hash_one();
            let mut store = MockWhitelistStore::new();
            store
                .expect_is_info_hash_whitelisted()
                .times(1)
                .returning(|_| Box::pin(async { Ok(false) }));
            store
                .expect_add_info_hash_to_whitelist()
                .times(1)
                .returning(|_| Box::pin(async { Err(failed_insert()) }));
            let (manager, in_memory) = manager_over(store);

            // Act
            let result = manager.add_torrent_to_whitelist(&info_hash).await;

            // Assert
            assert!(
                matches!(result, Err(Error::InsertFailed { .. })),
                "a failed persistence should be returned, got {result:?}"
            );
            assert!(
                !in_memory.contains(&info_hash).await,
                "the torrent should not be in memory when it could not be persisted"
            );
        }

        #[tokio::test]
        async fn it_should_keep_the_torrent_in_memory_when_removing_it_from_the_database_fails() {
            // Arrange
            let info_hash = sample_info_hash_one();
            let mut store = MockWhitelistStore::new();
            store
                .expect_is_info_hash_whitelisted()
                .times(1)
                .returning(|_| Box::pin(async { Ok(true) }));
            store.expect_remove_info_hash_from_whitelist().times(1).returning(|_| {
                Box::pin(async {
                    Err(Error::DeleteFailed {
                        error_code: 1,
                        driver: Driver::Sqlite3,
                    })
                })
            });
            let (manager, in_memory) = manager_over(store);
            in_memory.add(&info_hash).await;

            // Act
            let result = manager.remove_torrent_from_whitelist(&info_hash).await;

            // Assert
            assert!(
                matches!(result, Err(Error::DeleteFailed { .. })),
                "a failed persistence should be returned, got {result:?}"
            );
            assert!(
                in_memory.contains(&info_hash).await,
                "the torrent should stay in memory when the database could not remove it"
            );
        }

        #[tokio::test]
        async fn it_should_keep_the_in_memory_whitelist_when_loading_it_from_the_database_fails() {
            // Arrange
            let info_hash = sample_info_hash_one();
            let mut store = MockWhitelistStore::new();
            store.expect_load_whitelist().times(1).returning(|| {
                Box::pin(async {
                    Err(Error::MalformedDatabaseRecord {
                        message: "corrupt info-hash".to_string(),
                        driver: Driver::Sqlite3,
                    })
                })
            });
            let (manager, in_memory) = manager_over(store);
            in_memory.add(&info_hash).await;

            // Act
            let result = manager.load_whitelist_from_database().await;

            // Assert
            assert!(
                matches!(result, Err(Error::MalformedDatabaseRecord { .. })),
                "a failed load should be returned, got {result:?}"
            );
            assert!(
                in_memory.contains(&info_hash).await,
                "the in-memory whitelist should stay when the database could not be loaded"
            );
        }

        #[tokio::test]
        async fn it_should_replace_the_in_memory_whitelist_with_the_one_in_the_database() {
            // Arrange
            let stale = sample_info_hash_one();
            let stored = sample_info_hash_two();
            let mut store = MockWhitelistStore::new();
            store
                .expect_load_whitelist()
                .times(1)
                .returning(move || Box::pin(std::future::ready(Ok(vec![stored]))));
            let (manager, in_memory) = manager_over(store);
            in_memory.add(&stale).await;

            // Act
            let result = manager.load_whitelist_from_database().await;

            // Assert
            assert!(
                result.is_ok(),
                "loading the database whitelist should succeed, got {result:?}"
            );
            assert_eq!(
                (in_memory.contains(&stale).await, in_memory.contains(&stored).await),
                (false, true),
                "the in-memory whitelist should be exactly the database whitelist"
            );
        }
    }
}
