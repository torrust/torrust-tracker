//! The repository that persists the whitelist.
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;

use crate::databases::{self, WhitelistStore};

/// The persisted list of allowed torrents.
///
/// This repository handles adding, removing, and loading torrents
/// from a persistent database like `SQLite` or `MySQL`.
pub struct DatabaseWhitelist {
    /// A whitelist store implementation (e.g., `SQLite3` or `MySQL`).
    database: Arc<dyn WhitelistStore>,
}

fn block_on_current_or_new_runtime<F>(future: F) -> F::Output
where
    F: std::future::Future + Send,
    F::Output: Send,
{
    if tokio::runtime::Handle::try_current().is_ok() {
        std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .expect("failed to build Tokio runtime")
                        .block_on(future)
                })
                .join()
                .expect("failed to join blocking runtime thread")
        })
    } else {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build Tokio runtime")
            .block_on(future)
    }
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
    pub(crate) fn add(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        let is_whitelisted = block_on_current_or_new_runtime(self.database.is_info_hash_whitelisted(*info_hash))?;

        if is_whitelisted {
            return Ok(());
        }

        block_on_current_or_new_runtime(self.database.add_info_hash_to_whitelist(*info_hash))?;

        Ok(())
    }

    /// Removes a torrent from the whitelist if it exists.
    ///
    /// # Errors
    /// Returns a `database::Error` if unable to remove the `info_hash`.
    pub(crate) fn remove(&self, info_hash: &InfoHash) -> Result<(), databases::error::Error> {
        let is_whitelisted = block_on_current_or_new_runtime(self.database.is_info_hash_whitelisted(*info_hash))?;

        if !is_whitelisted {
            return Ok(());
        }

        block_on_current_or_new_runtime(self.database.remove_info_hash_from_whitelist(*info_hash))?;

        Ok(())
    }

    /// Loads the entire whitelist from the database.
    ///
    /// # Errors
    /// Returns a `database::Error` if unable to load whitelisted `info_hash`
    /// values.
    pub(crate) fn load_from_database(&self) -> Result<Vec<InfoHash>, databases::error::Error> {
        block_on_current_or_new_runtime(self.database.load_whitelist())
    }
}

#[cfg(test)]
mod tests {
    mod the_persisted_whitelist_repository {

        use crate::databases::setup::initialize_database;
        use crate::test_helpers::tests::{ephemeral_configuration_for_listed_tracker, sample_info_hash};
        use crate::whitelist::repository::persisted::DatabaseWhitelist;

        fn initialize_database_whitelist() -> DatabaseWhitelist {
            let configuration = ephemeral_configuration_for_listed_tracker();
            let stores = initialize_database(&configuration);
            DatabaseWhitelist::new(stores.whitelist_store)
        }

        #[tokio::test]
        async fn should_add_a_new_infohash_to_the_list() {
            let whitelist = initialize_database_whitelist();

            let infohash = sample_info_hash();

            let _result = whitelist.add(&infohash);

            assert_eq!(whitelist.load_from_database().unwrap(), vec!(infohash));
        }

        #[tokio::test]
        async fn should_remove_a_infohash_from_the_list() {
            let whitelist = initialize_database_whitelist();

            let infohash = sample_info_hash();

            let _result = whitelist.add(&infohash);

            let _result = whitelist.remove(&infohash);

            assert_eq!(whitelist.load_from_database().unwrap(), vec!());
        }

        #[tokio::test]
        async fn should_load_all_infohashes_from_the_database() {
            let whitelist = initialize_database_whitelist();

            let infohash = sample_info_hash();

            let _result = whitelist.add(&infohash);

            let result = whitelist.load_from_database().unwrap();

            assert_eq!(result, vec!(infohash));
        }

        #[tokio::test]
        async fn should_not_add_the_same_infohash_to_the_list_twice() {
            let whitelist = initialize_database_whitelist();

            let infohash = sample_info_hash();

            let _result = whitelist.add(&infohash);
            let _result = whitelist.add(&infohash);

            assert_eq!(whitelist.load_from_database().unwrap(), vec!(infohash));
        }

        #[tokio::test]
        async fn should_not_fail_removing_an_infohash_that_is_not_in_the_list() {
            let whitelist = initialize_database_whitelist();

            let infohash = sample_info_hash();

            let result = whitelist.remove(&infohash);

            assert!(result.is_ok());
        }
    }
}
