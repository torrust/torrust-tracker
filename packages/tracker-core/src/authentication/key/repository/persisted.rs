//! The database repository for the authentication keys.
use std::sync::Arc;

use crate::authentication::key::{Key, PeerKey};
use crate::databases::{self, AuthKeyStore};

/// A repository for storing authentication keys in a persistent database.
///
/// This repository provides methods to add, remove, and load authentication
/// keys from the underlying database. It wraps an instance of a type
/// implementing the [`AuthKeyStore`] trait.
pub struct DatabaseKeyRepository {
    database: Arc<dyn AuthKeyStore>,
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

impl DatabaseKeyRepository {
    /// Creates a new `DatabaseKeyRepository` instance.
    ///
    /// # Arguments
    ///
    /// * `database` - A shared reference to an auth-key store implementation.
    ///
    /// # Returns
    ///
    /// A new instance of `DatabaseKeyRepository`
    #[must_use]
    pub fn new(database: &Arc<dyn AuthKeyStore>) -> Self {
        Self {
            database: database.clone(),
        }
    }

    /// Adds a new authentication key to the database.
    ///
    /// # Arguments
    ///
    /// * `peer_key` - A reference to the [`PeerKey`] to be persisted.
    ///
    /// # Errors
    ///
    /// Returns a [`databases::error::Error`] if the key cannot be added.
    pub(crate) fn add(&self, peer_key: &PeerKey) -> Result<(), databases::error::Error> {
        block_on_current_or_new_runtime(self.database.add_key_to_keys(peer_key))?;
        Ok(())
    }

    /// Removes an authentication key from the database.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the [`Key`] corresponding to the key to remove.
    ///
    /// # Errors
    ///
    /// Returns a [`databases::error::Error`] if the key cannot be removed.
    pub(crate) fn remove(&self, key: &Key) -> Result<(), databases::error::Error> {
        block_on_current_or_new_runtime(self.database.remove_key_from_keys(key))?;
        Ok(())
    }

    /// Loads all authentication keys from the database.
    ///
    /// # Errors
    ///
    /// Returns a [`databases::error::Error`] if the keys cannot be loaded.
    ///
    /// # Returns
    ///
    /// A vector containing all persisted [`PeerKey`] entries.
    pub(crate) fn load_keys(&self) -> Result<Vec<PeerKey>, databases::error::Error> {
        let keys = block_on_current_or_new_runtime(self.database.load_keys())?;
        Ok(keys)
    }
}

#[cfg(test)]
mod tests {

    mod the_persisted_key_repository_should {

        use std::time::Duration;

        use torrust_tracker_configuration::Core;
        use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;

        use crate::authentication::key::repository::persisted::DatabaseKeyRepository;
        use crate::authentication::{Key, PeerKey};
        use crate::databases::setup::initialize_database;

        fn ephemeral_configuration() -> Core {
            let mut config = Core::default();
            let temp_file = ephemeral_sqlite_database();
            temp_file.to_str().unwrap().clone_into(&mut config.database.path);
            config
        }

        #[tokio::test]
        async fn persist_a_new_peer_key() {
            let configuration = ephemeral_configuration();

            let stores = initialize_database(&configuration);

            let repository = DatabaseKeyRepository::new(&stores.auth_key_store);

            let peer_key = PeerKey {
                key: Key::new("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap(),
                valid_until: Some(Duration::new(9999, 0)),
            };

            let result = repository.add(&peer_key);
            assert!(result.is_ok());

            let keys = repository.load_keys().unwrap();
            assert_eq!(keys, vec!(peer_key));
        }

        #[tokio::test]
        async fn remove_a_persisted_peer_key() {
            let configuration = ephemeral_configuration();

            let stores = initialize_database(&configuration);

            let repository = DatabaseKeyRepository::new(&stores.auth_key_store);

            let peer_key = PeerKey {
                key: Key::new("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap(),
                valid_until: Some(Duration::new(9999, 0)),
            };

            let _unused = repository.add(&peer_key);

            let result = repository.remove(&peer_key.key);
            assert!(result.is_ok());

            let keys = repository.load_keys().unwrap();
            assert!(keys.is_empty());
        }

        #[tokio::test]
        async fn load_all_persisted_peer_keys() {
            let configuration = ephemeral_configuration();

            let stores = initialize_database(&configuration);

            let repository = DatabaseKeyRepository::new(&stores.auth_key_store);

            let peer_key = PeerKey {
                key: Key::new("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap(),
                valid_until: Some(Duration::new(9999, 0)),
            };

            let _unused = repository.add(&peer_key);

            let keys = repository.load_keys().unwrap();

            assert_eq!(keys, vec!(peer_key));
        }
    }
}
