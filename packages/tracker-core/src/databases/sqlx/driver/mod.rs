#![allow(dead_code)]

pub mod sqlite;

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use crate::databases::sqlx::traits::AsyncDatabase;

    pub async fn run_tests(driver: &Arc<Box<dyn AsyncDatabase>>) {
        database_setup(driver).await;

        handling_torrent_persistence::it_should_save_and_load_persistent_torrents(driver).await;
        handling_torrent_persistence::it_should_load_all_persistent_torrents(driver).await;
        handling_torrent_persistence::it_should_increase_the_number_of_downloads_for_a_given_torrent(driver).await;
        handling_torrent_persistence::it_should_save_and_load_the_global_number_of_downloads(driver).await;
        handling_torrent_persistence::it_should_load_the_global_number_of_downloads(driver).await;
        handling_torrent_persistence::it_should_increase_the_global_number_of_downloads(driver).await;

        handling_authentication_keys::it_should_load_the_keys(driver).await;
        handling_authentication_keys::it_should_save_and_load_permanent_authentication_keys(driver).await;
        handling_authentication_keys::it_should_remove_a_permanent_authentication_key(driver).await;
        handling_authentication_keys::it_should_save_and_load_expiring_authentication_keys(driver).await;
        handling_authentication_keys::it_should_remove_an_expiring_authentication_key(driver).await;

        handling_the_whitelist::it_should_load_the_whitelist(driver).await;
        handling_the_whitelist::it_should_add_and_get_infohashes(driver).await;
        handling_the_whitelist::it_should_remove_an_infohash_from_the_whitelist(driver).await;
        handling_the_whitelist::it_should_fail_trying_to_add_the_same_infohash_twice(driver).await;
    }

    async fn database_setup(driver: &Arc<Box<dyn AsyncDatabase>>) {
        create_database_tables(driver).await.expect("database tables creation failed");
        driver
            .drop_database_tables()
            .await
            .expect("old database tables deletion failed");
        create_database_tables(driver)
            .await
            .expect("database tables creation from empty schema failed");
    }

    async fn create_database_tables(driver: &Arc<Box<dyn AsyncDatabase>>) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..5 {
            if driver.create_database_tables().await.is_ok() {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        Err("Database is not ready after retries.".into())
    }

    mod handling_torrent_persistence {
        use std::sync::Arc;

        use crate::databases::sqlx::traits::AsyncDatabase;
        use crate::test_helpers::tests::sample_info_hash;

        pub async fn it_should_save_and_load_persistent_torrents(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).await.unwrap();

            let number_of_downloads = driver.load_torrent_downloads(&infohash).await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_load_all_persistent_torrents(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).await.unwrap();

            let torrents = driver.load_all_torrents_downloads().await.unwrap();

            assert_eq!(torrents.len(), 1);
            assert_eq!(torrents.get(&infohash), Some(number_of_downloads).as_ref());
        }

        pub async fn it_should_increase_the_number_of_downloads_for_a_given_torrent(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).await.unwrap();

            driver.increase_downloads_for_torrent(&infohash).await.unwrap();

            let number_of_downloads = driver.load_torrent_downloads(&infohash).await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 2);
        }

        pub async fn it_should_save_and_load_the_global_number_of_downloads(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let number_of_downloads = 1;

            driver.save_global_downloads(number_of_downloads).await.unwrap();

            let number_of_downloads = driver.load_global_downloads().await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_load_the_global_number_of_downloads(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let number_of_downloads = 1;

            driver.save_global_downloads(number_of_downloads).await.unwrap();

            let number_of_downloads = driver.load_global_downloads().await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_increase_the_global_number_of_downloads(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let number_of_downloads = 1;

            driver.save_global_downloads(number_of_downloads).await.unwrap();

            driver.increase_global_downloads().await.unwrap();

            let number_of_downloads = driver.load_global_downloads().await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 2);
        }
    }

    mod handling_authentication_keys {
        use std::sync::Arc;
        use std::time::Duration;

        use crate::authentication::key::{generate_expiring_key, generate_permanent_key};
        use crate::databases::sqlx::traits::AsyncDatabase;

        pub async fn it_should_load_the_keys(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let permanent_peer_key = generate_permanent_key();
            driver.add_key_to_keys(&permanent_peer_key).await.unwrap();

            let expiring_peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&expiring_peer_key).await.unwrap();

            let keys = driver.load_keys().await.unwrap();

            assert!(keys.contains(&permanent_peer_key));
            assert!(keys.contains(&expiring_peer_key));
        }

        pub async fn it_should_save_and_load_permanent_authentication_keys(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let peer_key = generate_permanent_key();
            driver.add_key_to_keys(&peer_key).await.unwrap();

            let stored_peer_key = driver.get_key_from_keys(&peer_key.key()).await.unwrap().unwrap();

            assert_eq!(stored_peer_key, peer_key);
        }

        pub async fn it_should_save_and_load_expiring_authentication_keys(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&peer_key).await.unwrap();

            let stored_peer_key = driver.get_key_from_keys(&peer_key.key()).await.unwrap().unwrap();

            assert_eq!(stored_peer_key, peer_key);
            assert_eq!(stored_peer_key.expiry_time(), peer_key.expiry_time());
        }

        pub async fn it_should_remove_a_permanent_authentication_key(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let peer_key = generate_permanent_key();
            driver.add_key_to_keys(&peer_key).await.unwrap();

            driver.remove_key_from_keys(&peer_key.key()).await.unwrap();

            assert!(driver.get_key_from_keys(&peer_key.key()).await.unwrap().is_none());
        }

        pub async fn it_should_remove_an_expiring_authentication_key(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&peer_key).await.unwrap();

            driver.remove_key_from_keys(&peer_key.key()).await.unwrap();

            assert!(driver.get_key_from_keys(&peer_key.key()).await.unwrap().is_none());
        }
    }

    mod handling_the_whitelist {
        use std::sync::Arc;

        use crate::databases::sqlx::traits::AsyncDatabase;
        use crate::test_helpers::tests::random_info_hash;

        pub async fn it_should_load_the_whitelist(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let infohash = random_info_hash();
            driver.add_info_hash_to_whitelist(infohash).await.unwrap();

            let whitelist = driver.load_whitelist().await.unwrap();

            assert!(whitelist.contains(&infohash));
        }

        pub async fn it_should_add_and_get_infohashes(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let infohash = random_info_hash();

            driver.add_info_hash_to_whitelist(infohash).await.unwrap();

            let stored_infohash = driver.get_info_hash_from_whitelist(infohash).await.unwrap().unwrap();

            assert_eq!(stored_infohash, infohash);
        }

        pub async fn it_should_remove_an_infohash_from_the_whitelist(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let infohash = random_info_hash();
            driver.add_info_hash_to_whitelist(infohash).await.unwrap();

            driver.remove_info_hash_from_whitelist(infohash).await.unwrap();

            assert!(driver.get_info_hash_from_whitelist(infohash).await.unwrap().is_none());
        }

        pub async fn it_should_fail_trying_to_add_the_same_infohash_twice(driver: &Arc<Box<dyn AsyncDatabase>>) {
            let infohash = random_info_hash();

            driver.add_info_hash_to_whitelist(infohash).await.unwrap();
            let result = driver.add_info_hash_to_whitelist(infohash).await;

            assert!(result.is_err());
        }
    }
}
