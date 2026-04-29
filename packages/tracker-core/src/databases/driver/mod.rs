//! Database driver factory.
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::error::Error;

/// Metric name in DB for the total number of downloads across all torrents.
pub(super) const TORRENTS_DOWNLOADS_TOTAL: &str = "torrents_downloads_total";

/// The database management system used by the tracker.
///
/// Refer to:
///
/// - [Torrust Tracker Configuration](https://docs.rs/torrust-tracker-configuration).
/// - [Torrust Tracker](https://docs.rs/torrust-tracker).
///
/// For more information about persistence.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, derive_more::Display, Clone)]
pub enum Driver {
    /// The Sqlite3 database driver.
    Sqlite3,
    /// The `MySQL` database driver.
    MySQL,
}

impl Driver {
    /// Returns the stable lowercase identifier used by CLI and reports.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sqlite3 => "sqlite3",
            Self::MySQL => "mysql",
        }
    }
}

impl FromStr for Driver {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "sqlite3" => Ok(Self::Sqlite3),
            "mysql" => Ok(Self::MySQL),
            _ => Err("driver must be one of: sqlite3, mysql".to_string()),
        }
    }
}

pub mod mysql;
pub mod sqlite;

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use crate::databases::traits::Database;

    pub async fn run_tests(driver: &Arc<Box<dyn Database>>) {
        // Since the interface is very simple and there are no conflicts between
        // tests, we share the same database. If we want to isolate the tests in
        // the future, we can create a new database for each test.

        database_setup(driver).await;

        // Persistent torrents (stats)

        // Torrent metrics
        handling_torrent_persistence::it_should_save_and_load_persistent_torrents(driver);
        handling_torrent_persistence::it_should_load_all_persistent_torrents(driver);
        handling_torrent_persistence::it_should_increase_the_number_of_downloads_for_a_given_torrent(driver);
        // Aggregate metrics for all torrents
        handling_torrent_persistence::it_should_save_and_load_the_global_number_of_downloads(driver);
        handling_torrent_persistence::it_should_load_the_global_number_of_downloads(driver);
        handling_torrent_persistence::it_should_increase_the_global_number_of_downloads(driver);

        // Authentication keys (for private trackers)

        handling_authentication_keys::it_should_load_the_keys(driver);

        // Permanent keys
        handling_authentication_keys::it_should_save_and_load_permanent_authentication_keys(driver);
        handling_authentication_keys::it_should_remove_a_permanent_authentication_key(driver);

        // Expiring keys
        handling_authentication_keys::it_should_save_and_load_expiring_authentication_keys(driver);
        handling_authentication_keys::it_should_remove_an_expiring_authentication_key(driver);

        // Whitelist (for listed trackers)

        handling_the_whitelist::it_should_load_the_whitelist(driver);
        handling_the_whitelist::it_should_add_and_get_infohashes(driver);
        handling_the_whitelist::it_should_remove_an_infohash_from_the_whitelist(driver);
        handling_the_whitelist::it_should_fail_trying_to_add_the_same_infohash_twice(driver);
    }

    /// It initializes the database schema.
    ///
    /// Since the drop SQL queries don't check if the tables already exist,
    /// we have to create them first, and then drop them.
    ///
    /// The method to drop tables does not use "DROP TABLE IF EXISTS". We can
    /// change this function when we update the `Database::drop_database_tables`
    /// method to use "DROP TABLE IF EXISTS".
    async fn database_setup(driver: &Arc<Box<dyn Database>>) {
        create_database_tables(driver).await.expect("database tables creation failed");
        driver.drop_database_tables().expect("old database tables deletion failed");
        create_database_tables(driver)
            .await
            .expect("database tables creation from empty schema failed");
    }

    async fn create_database_tables(driver: &Arc<Box<dyn Database>>) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..5 {
            if driver.create_database_tables().is_ok() {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        Err("Database is not ready after retries.".into())
    }

    mod handling_torrent_persistence {

        use std::sync::Arc;

        use crate::databases::traits::Database;
        use crate::test_helpers::tests::sample_info_hash;

        // Metrics per torrent

        pub fn it_should_save_and_load_persistent_torrents(driver: &Arc<Box<dyn Database>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).unwrap();

            let number_of_downloads = driver.load_torrent_downloads(&infohash).unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub fn it_should_load_all_persistent_torrents(driver: &Arc<Box<dyn Database>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).unwrap();

            let torrents = driver.load_all_torrents_downloads().unwrap();

            assert_eq!(torrents.len(), 1);
            assert_eq!(torrents.get(&infohash), Some(number_of_downloads).as_ref());
        }

        pub fn it_should_increase_the_number_of_downloads_for_a_given_torrent(driver: &Arc<Box<dyn Database>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).unwrap();

            driver.increase_downloads_for_torrent(&infohash).unwrap();

            let number_of_downloads = driver.load_torrent_downloads(&infohash).unwrap().unwrap();

            assert_eq!(number_of_downloads, 2);
        }

        // Aggregate metrics for all torrents

        pub fn it_should_save_and_load_the_global_number_of_downloads(driver: &Arc<Box<dyn Database>>) {
            let number_of_downloads = 1;

            driver.save_global_downloads(number_of_downloads).unwrap();

            let number_of_downloads = driver.load_global_downloads().unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub fn it_should_load_the_global_number_of_downloads(driver: &Arc<Box<dyn Database>>) {
            let number_of_downloads = 1;

            driver.save_global_downloads(number_of_downloads).unwrap();

            let number_of_downloads = driver.load_global_downloads().unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub fn it_should_increase_the_global_number_of_downloads(driver: &Arc<Box<dyn Database>>) {
            let number_of_downloads = 1;

            driver.save_global_downloads(number_of_downloads).unwrap();

            driver.increase_global_downloads().unwrap();

            let number_of_downloads = driver.load_global_downloads().unwrap().unwrap();

            assert_eq!(number_of_downloads, 2);
        }
    }

    mod handling_authentication_keys {

        use std::sync::Arc;
        use std::time::Duration;

        use crate::authentication::key::{generate_expiring_key, generate_permanent_key};
        use crate::databases::traits::Database;

        pub fn it_should_load_the_keys(driver: &Arc<Box<dyn Database>>) {
            let permanent_peer_key = generate_permanent_key();
            driver.add_key_to_keys(&permanent_peer_key).unwrap();

            let expiring_peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&expiring_peer_key).unwrap();

            let keys = driver.load_keys().unwrap();

            assert!(keys.contains(&permanent_peer_key));
            assert!(keys.contains(&expiring_peer_key));
        }

        pub fn it_should_save_and_load_permanent_authentication_keys(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_permanent_key();
            driver.add_key_to_keys(&peer_key).unwrap();

            let stored_peer_key = driver.get_key_from_keys(&peer_key.key()).unwrap().unwrap();

            assert_eq!(stored_peer_key, peer_key);
        }

        pub fn it_should_save_and_load_expiring_authentication_keys(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&peer_key).unwrap();

            let stored_peer_key = driver.get_key_from_keys(&peer_key.key()).unwrap().unwrap();

            assert_eq!(stored_peer_key, peer_key);
            assert_eq!(stored_peer_key.expiry_time(), peer_key.expiry_time());
        }

        pub fn it_should_remove_a_permanent_authentication_key(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_permanent_key();
            driver.add_key_to_keys(&peer_key).unwrap();

            driver.remove_key_from_keys(&peer_key.key()).unwrap();

            assert!(driver.get_key_from_keys(&peer_key.key()).unwrap().is_none());
        }

        pub fn it_should_remove_an_expiring_authentication_key(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&peer_key).unwrap();

            driver.remove_key_from_keys(&peer_key.key()).unwrap();

            assert!(driver.get_key_from_keys(&peer_key.key()).unwrap().is_none());
        }
    }

    mod handling_the_whitelist {

        use std::sync::Arc;

        use crate::databases::traits::Database;
        use crate::test_helpers::tests::random_info_hash;

        pub fn it_should_load_the_whitelist(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();
            driver.add_info_hash_to_whitelist(infohash).unwrap();

            let whitelist = driver.load_whitelist().unwrap();

            assert!(whitelist.contains(&infohash));
        }

        pub fn it_should_add_and_get_infohashes(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();

            driver.add_info_hash_to_whitelist(infohash).unwrap();

            let stored_infohash = driver.get_info_hash_from_whitelist(infohash).unwrap().unwrap();

            assert_eq!(stored_infohash, infohash);
        }

        pub fn it_should_remove_an_infohash_from_the_whitelist(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();
            driver.add_info_hash_to_whitelist(infohash).unwrap();

            driver.remove_info_hash_from_whitelist(infohash).unwrap();

            assert!(driver.get_info_hash_from_whitelist(infohash).unwrap().is_none());
        }

        pub fn it_should_fail_trying_to_add_the_same_infohash_twice(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();

            driver.add_info_hash_to_whitelist(infohash).unwrap();
            let result = driver.add_info_hash_to_whitelist(infohash);

            assert!(result.is_err());
        }
    }
}
