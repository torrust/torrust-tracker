//! Database driver factory.
use std::sync::Arc;

use mysql::Mysql;
use postgres::Postgres;
use serde::{Deserialize, Serialize};
use sqlite::Sqlite;

use super::error::Error;
use super::{AuthKeyStore, Persistence, SchemaMigrator, TorrentMetricsStore, WhitelistStore};

/// Metric name in DB for the total number of downloads across all torrents.
pub const TORRENTS_DOWNLOADS_TOTAL: &str = "torrents_downloads_total";

/// The database management system used by the tracker.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, derive_more::Display, Clone)]
pub enum Driver {
    /// The Sqlite3 database driver.
    Sqlite3,
    /// The `MySQL` database driver.
    MySQL,
    /// The `PostgreSQL` database driver.
    PostgreSQL,
}

pub mod mysql;
pub mod postgres;
pub mod sqlite;

/// Builds a new persistence backend.
///
/// # Errors
///
/// Will return [`Error`] if unable to build the backend.
pub(crate) fn build(driver: &Driver, db_path: &str) -> Result<Persistence, Error> {
    match driver {
        Driver::Sqlite3 => {
            let backend = Arc::new(Sqlite::new(db_path)?);
            Ok(Persistence::new(
                backend.clone() as Arc<dyn SchemaMigrator>,
                backend.clone() as Arc<dyn TorrentMetricsStore>,
                backend.clone() as Arc<dyn WhitelistStore>,
                backend as Arc<dyn AuthKeyStore>,
            ))
        }
        Driver::MySQL => {
            let backend = Arc::new(Mysql::new(db_path)?);
            Ok(Persistence::new(
                backend.clone() as Arc<dyn SchemaMigrator>,
                backend.clone() as Arc<dyn TorrentMetricsStore>,
                backend.clone() as Arc<dyn WhitelistStore>,
                backend as Arc<dyn AuthKeyStore>,
            ))
        }
        Driver::PostgreSQL => {
            let backend = Arc::new(Postgres::new(db_path)?);
            Ok(Persistence::new(
                backend.clone() as Arc<dyn SchemaMigrator>,
                backend.clone() as Arc<dyn TorrentMetricsStore>,
                backend.clone() as Arc<dyn WhitelistStore>,
                backend as Arc<dyn AuthKeyStore>,
            ))
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::Persistence;

    pub async fn run_tests(driver: &Persistence) {
        database_setup(driver).await;

        handling_torrent_persistence::it_should_save_and_load_persistent_torrents(driver).await;
        handling_torrent_persistence::it_should_load_all_persistent_torrents(driver).await;
        handling_torrent_persistence::it_should_increase_the_number_of_downloads_for_a_given_torrent(driver).await;
        handling_torrent_persistence::it_should_save_and_load_the_global_number_of_downloads(driver).await;
        handling_torrent_persistence::it_should_load_the_global_number_of_downloads(driver).await;
        handling_torrent_persistence::it_should_increase_the_global_number_of_downloads(driver).await;
        handling_torrent_persistence::it_should_support_large_download_counters(driver).await;

        handling_authentication_keys::it_should_load_the_keys(driver).await;
        handling_authentication_keys::it_should_save_and_load_permanent_authentication_keys(driver).await;
        handling_authentication_keys::it_should_save_and_load_expiring_authentication_keys(driver).await;
        handling_authentication_keys::it_should_remove_a_permanent_authentication_key(driver).await;
        handling_authentication_keys::it_should_remove_an_expiring_authentication_key(driver).await;

        handling_the_whitelist::it_should_load_the_whitelist(driver).await;
        handling_the_whitelist::it_should_add_and_get_infohashes(driver).await;
        handling_the_whitelist::it_should_remove_an_infohash_from_the_whitelist(driver).await;
        handling_the_whitelist::it_should_fail_trying_to_add_the_same_infohash_twice(driver).await;
    }

    async fn database_setup(driver: &Persistence) {
        create_database_tables(driver).await.expect("database tables creation failed");
        driver
            .schema_migrator()
            .drop_database_tables()
            .await
            .expect("old database tables deletion failed");
        create_database_tables(driver)
            .await
            .expect("database tables creation from empty schema failed");
    }

    async fn create_database_tables(driver: &Persistence) -> Result<(), Box<dyn std::error::Error>> {
        use std::time::Duration;

        let mut last_error = None;

        for _ in 0..5 {
            match driver.schema_migrator().create_database_tables().await {
                Ok(()) => return Ok(()),
                Err(err) => last_error = Some(err),
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        match last_error {
            Some(err) => Err(format!("Database is not ready after retries: {err}").into()),
            None => Err("Database is not ready after retries.".into()),
        }
    }

    mod handling_torrent_persistence {
        use crate::databases::driver::tests::Persistence;
        use crate::test_helpers::tests::sample_info_hash;

        pub async fn it_should_save_and_load_persistent_torrents(driver: &Persistence) {
            let infohash = sample_info_hash();

            driver
                .torrent_metrics_store()
                .save_torrent_downloads(&infohash, 1)
                .await
                .unwrap();

            let number_of_downloads = driver
                .torrent_metrics_store()
                .load_torrent_downloads(&infohash)
                .await
                .unwrap()
                .unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_load_all_persistent_torrents(driver: &Persistence) {
            let infohash = sample_info_hash();

            driver
                .torrent_metrics_store()
                .save_torrent_downloads(&infohash, 1)
                .await
                .unwrap();

            let torrents = driver.torrent_metrics_store().load_all_torrents_downloads().await.unwrap();

            assert_eq!(torrents.len(), 1);
            assert_eq!(torrents.get(&infohash), Some(1_u64).as_ref());
        }

        pub async fn it_should_increase_the_number_of_downloads_for_a_given_torrent(driver: &Persistence) {
            let infohash = sample_info_hash();

            driver
                .torrent_metrics_store()
                .save_torrent_downloads(&infohash, 1)
                .await
                .unwrap();

            driver
                .torrent_metrics_store()
                .increase_downloads_for_torrent(&infohash)
                .await
                .unwrap();

            let number_of_downloads = driver
                .torrent_metrics_store()
                .load_torrent_downloads(&infohash)
                .await
                .unwrap()
                .unwrap();

            assert_eq!(number_of_downloads, 2);
        }

        pub async fn it_should_save_and_load_the_global_number_of_downloads(driver: &Persistence) {
            driver.torrent_metrics_store().save_global_downloads(1).await.unwrap();

            let number_of_downloads = driver
                .torrent_metrics_store()
                .load_global_downloads()
                .await
                .unwrap()
                .unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_load_the_global_number_of_downloads(driver: &Persistence) {
            driver.torrent_metrics_store().save_global_downloads(1).await.unwrap();

            let number_of_downloads = driver
                .torrent_metrics_store()
                .load_global_downloads()
                .await
                .unwrap()
                .unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_increase_the_global_number_of_downloads(driver: &Persistence) {
            driver.torrent_metrics_store().save_global_downloads(1).await.unwrap();

            driver.torrent_metrics_store().increase_global_downloads().await.unwrap();

            let number_of_downloads = driver
                .torrent_metrics_store()
                .load_global_downloads()
                .await
                .unwrap()
                .unwrap();

            assert_eq!(number_of_downloads, 2);
        }

        pub async fn it_should_support_large_download_counters(driver: &Persistence) {
            let infohash = sample_info_hash();
            let large_value = u64::from(u32::MAX);

            driver
                .torrent_metrics_store()
                .save_torrent_downloads(&infohash, large_value)
                .await
                .unwrap();
            driver
                .torrent_metrics_store()
                .save_global_downloads(large_value)
                .await
                .unwrap();

            assert_eq!(
                driver
                    .torrent_metrics_store()
                    .load_torrent_downloads(&infohash)
                    .await
                    .unwrap(),
                Some(large_value)
            );
            assert_eq!(
                driver
                    .torrent_metrics_store()
                    .load_global_downloads()
                    .await
                    .unwrap(),
                Some(large_value)
            );
        }
    }

    mod handling_authentication_keys {
        use std::time::Duration;

        use crate::authentication::key::{generate_expiring_key, generate_permanent_key};
        use crate::databases::driver::tests::Persistence;

        pub async fn it_should_load_the_keys(driver: &Persistence) {
            let permanent_peer_key = generate_permanent_key();
            driver.auth_key_store().add_key_to_keys(&permanent_peer_key).await.unwrap();

            let expiring_peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.auth_key_store().add_key_to_keys(&expiring_peer_key).await.unwrap();

            let keys = driver.auth_key_store().load_keys().await.unwrap();

            assert!(keys.contains(&permanent_peer_key));
            assert!(keys.contains(&expiring_peer_key));
        }

        pub async fn it_should_save_and_load_permanent_authentication_keys(driver: &Persistence) {
            let peer_key = generate_permanent_key();
            driver.auth_key_store().add_key_to_keys(&peer_key).await.unwrap();

            let stored_peer_key = driver
                .auth_key_store()
                .get_key_from_keys(&peer_key.key())
                .await
                .unwrap()
                .unwrap();

            assert_eq!(stored_peer_key, peer_key);
        }

        pub async fn it_should_save_and_load_expiring_authentication_keys(driver: &Persistence) {
            let peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.auth_key_store().add_key_to_keys(&peer_key).await.unwrap();

            let stored_peer_key = driver
                .auth_key_store()
                .get_key_from_keys(&peer_key.key())
                .await
                .unwrap()
                .unwrap();

            assert_eq!(stored_peer_key, peer_key);
            assert_eq!(stored_peer_key.expiry_time(), peer_key.expiry_time());
        }

        pub async fn it_should_remove_a_permanent_authentication_key(driver: &Persistence) {
            let peer_key = generate_permanent_key();
            driver.auth_key_store().add_key_to_keys(&peer_key).await.unwrap();

            driver
                .auth_key_store()
                .remove_key_from_keys(&peer_key.key())
                .await
                .unwrap();

            assert!(driver
                .auth_key_store()
                .get_key_from_keys(&peer_key.key())
                .await
                .unwrap()
                .is_none());
        }

        pub async fn it_should_remove_an_expiring_authentication_key(driver: &Persistence) {
            let peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.auth_key_store().add_key_to_keys(&peer_key).await.unwrap();

            driver
                .auth_key_store()
                .remove_key_from_keys(&peer_key.key())
                .await
                .unwrap();

            assert!(driver
                .auth_key_store()
                .get_key_from_keys(&peer_key.key())
                .await
                .unwrap()
                .is_none());
        }
    }

    mod handling_the_whitelist {
        use crate::databases::driver::tests::Persistence;
        use crate::test_helpers::tests::random_info_hash;

        pub async fn it_should_load_the_whitelist(driver: &Persistence) {
            let infohash = random_info_hash();
            driver
                .whitelist_store()
                .add_info_hash_to_whitelist(infohash)
                .await
                .unwrap();

            let whitelist = driver.whitelist_store().load_whitelist().await.unwrap();

            assert!(whitelist.contains(&infohash));
        }

        pub async fn it_should_add_and_get_infohashes(driver: &Persistence) {
            let infohash = random_info_hash();

            driver
                .whitelist_store()
                .add_info_hash_to_whitelist(infohash)
                .await
                .unwrap();

            let stored_infohash = driver
                .whitelist_store()
                .get_info_hash_from_whitelist(infohash)
                .await
                .unwrap()
                .unwrap();

            assert_eq!(stored_infohash, infohash);
        }

        pub async fn it_should_remove_an_infohash_from_the_whitelist(driver: &Persistence) {
            let infohash = random_info_hash();
            driver
                .whitelist_store()
                .add_info_hash_to_whitelist(infohash)
                .await
                .unwrap();

            driver
                .whitelist_store()
                .remove_info_hash_from_whitelist(infohash)
                .await
                .unwrap();

            assert!(driver
                .whitelist_store()
                .get_info_hash_from_whitelist(infohash)
                .await
                .unwrap()
                .is_none());
        }

        pub async fn it_should_fail_trying_to_add_the_same_infohash_twice(driver: &Persistence) {
            let infohash = random_info_hash();

            driver
                .whitelist_store()
                .add_info_hash_to_whitelist(infohash)
                .await
                .unwrap();
            let result = driver
                .whitelist_store()
                .add_info_hash_to_whitelist(infohash)
                .await;

            assert!(result.is_err());
        }
    }
}
