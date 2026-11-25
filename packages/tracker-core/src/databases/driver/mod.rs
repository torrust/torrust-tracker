//! Database driver factory.

use torrust_clock::DurationSinceUnixEpoch;
use torrust_tracker_primitives::Driver;

use super::error::Error;
use crate::authentication::{Key, PeerKey};

/// Metric name in DB for the total number of downloads across all torrents.
pub(super) const TORRENTS_DOWNLOADS_TOTAL: &str = "torrents_downloads_total";

/// Maximum number of info-hashes bound in one `IN (...)` torrent lookup.
///
/// A full UDP scrape (about 74 info-hashes, BEP 15) fits in one query. The
/// value is deliberately independent of protocol request limits, which exist
/// for other reasons and may grow.
pub(super) const MAX_INFO_HASHES_PER_QUERY: usize = 100;

pub mod mysql;
pub mod postgres;
pub mod sqlite;

/// Converts a signed seconds value loaded from the database into a
/// [`DurationSinceUnixEpoch`].
///
/// Negative values indicate a corrupted record (timestamps before the Unix
/// epoch are not representable) and are rejected as
/// [`Error::MalformedDatabaseRecord`] for the given `driver`.
pub(super) fn parse_valid_until(value: i64, driver: &Driver) -> Result<DurationSinceUnixEpoch, Error> {
    let secs = u64::try_from(value).map_err(|_| Error::MalformedDatabaseRecord {
        message: format!("negative valid_until timestamp: {value}"),
        driver: driver.clone(),
    })?;
    Ok(DurationSinceUnixEpoch::from_secs(secs))
}

/// Builds a [`PeerKey`] from the two columns of a stored authentication key.
///
/// `key_value` is the stored key text and `valid_until` the stored expiry in
/// seconds since the Unix epoch, or `None` for a permanent key. A key that does
/// not parse and an expiry that [`parse_valid_until`] rejects are corrupted
/// records, reported as [`Error::MalformedDatabaseRecord`] for the given `driver`.
pub(super) fn peer_key_from_row(key_value: &str, valid_until: Option<i64>, driver: &Driver) -> Result<PeerKey, Error> {
    let key = key_value.parse::<Key>().map_err(|e| Error::MalformedDatabaseRecord {
        message: e.to_string(),
        driver: driver.clone(),
    })?;

    Ok(PeerKey {
        key,
        valid_until: valid_until.map(|value| parse_valid_until(value, driver)).transpose()?,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use crate::databases::traits::Database;

    pub async fn run_tests(driver: &Arc<Box<dyn Database>>) {
        database_setup(driver).await;

        handling_torrent_persistence::it_should_save_and_load_persistent_torrents(driver).await;
        handling_torrent_persistence::it_should_load_all_persistent_torrents(driver).await;
        handling_torrent_persistence::it_should_increase_the_number_of_downloads_for_a_given_torrent(driver).await;
        handling_torrent_persistence::it_should_load_the_persisted_downloads_of_the_requested_torrents_only(driver).await;
        handling_torrent_persistence::it_should_load_the_persisted_downloads_of_more_torrents_than_fit_in_one_query(driver).await;
        handling_torrent_persistence::it_should_save_and_load_the_global_number_of_downloads(driver).await;
        handling_torrent_persistence::it_should_load_the_global_number_of_downloads(driver).await;
        handling_torrent_persistence::it_should_increase_the_global_number_of_downloads(driver).await;

        handling_authentication_keys::it_should_load_the_keys(driver).await;
        handling_authentication_keys::it_should_save_and_load_permanent_authentication_keys(driver).await;
        handling_authentication_keys::it_should_remove_a_permanent_authentication_key(driver).await;
        handling_authentication_keys::it_should_save_and_load_expiring_authentication_keys(driver).await;
        handling_authentication_keys::it_should_remove_an_expiring_authentication_key(driver).await;
        handling_authentication_keys::it_should_fail_without_a_source_path_trying_to_add_the_same_key_twice(driver).await;
        handling_authentication_keys::it_should_fail_without_a_source_path_trying_to_remove_a_missing_key(driver).await;

        handling_the_whitelist::it_should_load_the_whitelist(driver).await;
        handling_the_whitelist::it_should_add_and_get_infohashes(driver).await;
        handling_the_whitelist::it_should_remove_an_infohash_from_the_whitelist(driver).await;
        handling_the_whitelist::it_should_not_fail_adding_the_same_infohash_concurrently(driver).await;
        handling_the_whitelist::it_should_not_fail_removing_the_same_infohash_concurrently(driver).await;
        handling_the_whitelist::it_should_not_fail_adding_the_same_infohash_twice(driver).await;
        handling_the_whitelist::it_should_not_fail_removing_a_missing_infohash(driver).await;
        handling_the_whitelist::it_should_not_fail_removing_the_same_infohash_twice(driver).await;
    }

    async fn database_setup(driver: &Arc<Box<dyn Database>>) {
        create_database_tables(driver).await.expect("database tables creation failed");
        driver
            .drop_database_tables()
            .await
            .expect("old database tables deletion failed");
        create_database_tables(driver)
            .await
            .expect("database tables creation from empty schema failed");
    }

    async fn create_database_tables(driver: &Arc<Box<dyn Database>>) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..20 {
            if driver.create_database_tables().await.is_ok() {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
        Err("Database is not ready after retries.".into())
    }

    mod handling_torrent_persistence {

        use std::sync::Arc;

        use torrust_info_hash::InfoHash;
        use torrust_tracker_primitives::NumberOfDownloadsPerInfoHash;

        use crate::databases::driver::MAX_INFO_HASHES_PER_QUERY;
        use crate::databases::traits::Database;
        use crate::test_helpers::tests::sample_info_hash;

        fn numbered_info_hash(number: u32) -> InfoHash {
            let mut bytes = [0xAB; 20];
            bytes[..4].copy_from_slice(&number.to_be_bytes());
            InfoHash(bytes)
        }

        // Metrics per torrent

        pub async fn it_should_save_and_load_persistent_torrents(driver: &Arc<Box<dyn Database>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).await.unwrap();

            let number_of_downloads = driver.load_torrent_downloads(&infohash).await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_load_all_persistent_torrents(driver: &Arc<Box<dyn Database>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).await.unwrap();

            let torrents = driver.load_all_torrents_downloads().await.unwrap();

            assert_eq!(torrents.len(), 1);
            assert_eq!(torrents.get(&infohash), Some(number_of_downloads).as_ref());
        }

        pub async fn it_should_increase_the_number_of_downloads_for_a_given_torrent(driver: &Arc<Box<dyn Database>>) {
            let infohash = sample_info_hash();

            let number_of_downloads = 1;

            driver.save_torrent_downloads(&infohash, number_of_downloads).await.unwrap();

            driver.increase_downloads_for_torrent(&infohash).await.unwrap();

            let number_of_downloads = driver.load_torrent_downloads(&infohash).await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 2);
        }

        // adr: docs/adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md
        pub async fn it_should_load_the_persisted_downloads_of_the_requested_torrents_only(driver: &Arc<Box<dyn Database>>) {
            // Arrange
            let persisted = numbered_info_hash(1);
            let not_requested = numbered_info_hash(2);
            let not_persisted = numbered_info_hash(3);
            driver.save_torrent_downloads(&persisted, 5).await.unwrap();
            driver.save_torrent_downloads(&not_requested, 6).await.unwrap();

            // Act
            let downloads = driver.load_torrents_downloads(&[persisted, not_persisted]).await.unwrap();

            // Assert
            assert_eq!(
                downloads,
                NumberOfDownloadsPerInfoHash::from([(persisted, 5)]),
                "only requested torrents with a persisted row should be returned"
            );
        }

        pub async fn it_should_load_the_persisted_downloads_of_more_torrents_than_fit_in_one_query(
            driver: &Arc<Box<dyn Database>>,
        ) {
            // Arrange
            let torrents = u32::try_from(MAX_INFO_HASHES_PER_QUERY).unwrap() + 1;
            let mut expected = NumberOfDownloadsPerInfoHash::new();
            for number in 100..100 + torrents {
                driver
                    .save_torrent_downloads(&numbered_info_hash(number), number)
                    .await
                    .unwrap();
                expected.insert(numbered_info_hash(number), number);
            }
            let info_hashes: Vec<InfoHash> = expected.keys().copied().collect();

            // Act
            let downloads = driver.load_torrents_downloads(&info_hashes).await.unwrap();

            // Assert
            assert_eq!(
                downloads, expected,
                "all {torrents} torrents should be loaded across chunks of {MAX_INFO_HASHES_PER_QUERY} info-hashes"
            );
        }

        // Aggregate metrics for all torrents

        pub async fn it_should_save_and_load_the_global_number_of_downloads(driver: &Arc<Box<dyn Database>>) {
            let number_of_downloads = 1;

            driver.save_global_downloads(number_of_downloads).await.unwrap();

            let number_of_downloads = driver.load_global_downloads().await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_load_the_global_number_of_downloads(driver: &Arc<Box<dyn Database>>) {
            let number_of_downloads = 1;

            driver.save_global_downloads(number_of_downloads).await.unwrap();

            let number_of_downloads = driver.load_global_downloads().await.unwrap().unwrap();

            assert_eq!(number_of_downloads, 1);
        }

        pub async fn it_should_increase_the_global_number_of_downloads(driver: &Arc<Box<dyn Database>>) {
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

        use torrust_tracker_test_helpers::assertions::assert_names_no_source_path;

        use crate::authentication::key::{generate_expiring_key, generate_permanent_key};
        use crate::databases::error::Error;
        use crate::databases::traits::Database;
        use crate::test_helpers::tests::assert_holds_no_secret;

        pub async fn it_should_load_the_keys(driver: &Arc<Box<dyn Database>>) {
            let permanent_peer_key = generate_permanent_key();
            driver.add_key_to_keys(&permanent_peer_key).await.unwrap();

            let expiring_peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&expiring_peer_key).await.unwrap();

            let keys = driver.load_keys().await.unwrap();

            assert!(keys.contains(&permanent_peer_key));
            assert!(keys.contains(&expiring_peer_key));
        }

        pub async fn it_should_save_and_load_permanent_authentication_keys(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_permanent_key();
            driver.add_key_to_keys(&peer_key).await.unwrap();

            let stored_peer_key = driver.get_key_from_keys(&peer_key.key()).await.unwrap().unwrap();

            assert_eq!(stored_peer_key, peer_key);
        }

        pub async fn it_should_save_and_load_expiring_authentication_keys(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&peer_key).await.unwrap();

            let stored_peer_key = driver.get_key_from_keys(&peer_key.key()).await.unwrap().unwrap();

            assert_eq!(stored_peer_key, peer_key);
            assert_eq!(stored_peer_key.expiry_time(), peer_key.expiry_time());
        }

        pub async fn it_should_remove_a_permanent_authentication_key(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_permanent_key();
            driver.add_key_to_keys(&peer_key).await.unwrap();

            driver.remove_key_from_keys(&peer_key.key()).await.unwrap();

            assert!(driver.get_key_from_keys(&peer_key.key()).await.unwrap().is_none());
        }

        pub async fn it_should_remove_an_expiring_authentication_key(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_expiring_key(Duration::from_secs(120));
            driver.add_key_to_keys(&peer_key).await.unwrap();

            driver.remove_key_from_keys(&peer_key.key()).await.unwrap();

            assert!(driver.get_key_from_keys(&peer_key.key()).await.unwrap().is_none());
        }

        pub async fn it_should_fail_without_a_source_path_trying_to_add_the_same_key_twice(driver: &Arc<Box<dyn Database>>) {
            let peer_key = generate_permanent_key();
            driver.add_key_to_keys(&peer_key).await.unwrap();

            let error = driver.add_key_to_keys(&peer_key).await.unwrap_err();

            assert_names_no_source_path(&error.to_string());
            // The server text of a duplicate entry can hold the key (`MySQL` prints it).
            assert_holds_no_secret(&error, peer_key.key.value());
            assert!(matches!(error, Error::KeyAlreadyExists { .. }), "{error:?}");
        }

        pub async fn it_should_fail_without_a_source_path_trying_to_remove_a_missing_key(driver: &Arc<Box<dyn Database>>) {
            let missing_key = generate_permanent_key().key();

            let error = driver.remove_key_from_keys(&missing_key).await.unwrap_err();

            assert_names_no_source_path(&error.to_string());
            assert!(error.to_string().starts_with("Failed to remove record from "), "{error}");
        }
    }

    mod handling_the_whitelist {

        use std::sync::Arc;

        use crate::databases::traits::Database;
        use crate::test_helpers::tests::random_info_hash;

        /// Rounds of the concurrent cases. There is no read-then-write window at the store level, so a
        /// single round already fails on a store that rejects a duplicate write; repeating the cases
        /// also catches nondeterministic lock errors on `MySQL` and `PostgreSQL`.
        const CONCURRENT_ROUNDS: usize = 50;

        pub async fn it_should_load_the_whitelist(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();
            driver.add_info_hash_to_whitelist(infohash).await.unwrap();

            let whitelist = driver.load_whitelist().await.unwrap();

            assert!(whitelist.contains(&infohash));
        }

        pub async fn it_should_add_and_get_infohashes(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();

            driver.add_info_hash_to_whitelist(infohash).await.unwrap();

            let stored_infohash = driver.get_info_hash_from_whitelist(infohash).await.unwrap().unwrap();

            assert_eq!(stored_infohash, infohash);
        }

        pub async fn it_should_remove_an_infohash_from_the_whitelist(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();
            driver.add_info_hash_to_whitelist(infohash).await.unwrap();

            driver.remove_info_hash_from_whitelist(infohash).await.unwrap();

            assert!(driver.get_info_hash_from_whitelist(infohash).await.unwrap().is_none());
        }

        /// Concurrent identical adds must both succeed and leave the torrent listed.
        pub async fn it_should_not_fail_adding_the_same_infohash_concurrently(driver: &Arc<Box<dyn Database>>) {
            for _ in 0..CONCURRENT_ROUNDS {
                let infohash = random_info_hash();

                let (first, second) = tokio::join!(
                    driver.add_info_hash_to_whitelist(infohash),
                    driver.add_info_hash_to_whitelist(infohash)
                );

                first.unwrap();
                second.unwrap();
                assert!(driver.get_info_hash_from_whitelist(infohash).await.unwrap().is_some());
            }
        }

        /// Concurrent identical removes must both succeed and change exactly one row in total.
        pub async fn it_should_not_fail_removing_the_same_infohash_concurrently(driver: &Arc<Box<dyn Database>>) {
            for _ in 0..CONCURRENT_ROUNDS {
                let infohash = random_info_hash();
                driver.add_info_hash_to_whitelist(infohash).await.unwrap();

                let (first, second) = tokio::join!(
                    driver.remove_info_hash_from_whitelist(infohash),
                    driver.remove_info_hash_from_whitelist(infohash)
                );

                assert_eq!(first.unwrap() + second.unwrap(), 1);
                assert!(driver.get_info_hash_from_whitelist(infohash).await.unwrap().is_none());
            }
        }

        pub async fn it_should_not_fail_adding_the_same_infohash_twice(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();

            driver.add_info_hash_to_whitelist(infohash).await.unwrap();
            driver.add_info_hash_to_whitelist(infohash).await.unwrap();

            assert!(driver.get_info_hash_from_whitelist(infohash).await.unwrap().is_some());
        }

        pub async fn it_should_not_fail_removing_a_missing_infohash(driver: &Arc<Box<dyn Database>>) {
            let missing_infohash = random_info_hash();

            let removed = driver.remove_info_hash_from_whitelist(missing_infohash).await.unwrap();

            assert_eq!(removed, 0);
        }

        pub async fn it_should_not_fail_removing_the_same_infohash_twice(driver: &Arc<Box<dyn Database>>) {
            let infohash = random_info_hash();
            driver.add_info_hash_to_whitelist(infohash).await.unwrap();

            let first = driver.remove_info_hash_from_whitelist(infohash).await.unwrap();
            let second = driver.remove_info_hash_from_whitelist(infohash).await.unwrap();

            assert_eq!((first, second), (1, 0));
            assert!(driver.get_info_hash_from_whitelist(infohash).await.unwrap().is_none());
        }
    }
}

#[cfg(test)]
mod stored_key_tests {
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use rstest::rstest;
    use torrust_clock::DurationSinceUnixEpoch;
    use torrust_tracker_primitives::Driver;

    use super::{parse_valid_until, peer_key_from_row};
    use crate::authentication::{Key, PeerKey};
    use crate::databases::error::Error;

    const A_KEY: &str = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ";

    /// Unwraps the `MalformedDatabaseRecord` of an error, failing on any other variant.
    fn malformed_record(error: Error) -> (String, Driver) {
        match error {
            Error::MalformedDatabaseRecord { message, driver } => (message, driver),
            other => panic!("expected a malformed database record, got {other:?}"),
        }
    }

    #[rstest]
    #[case::zero(0, 0)]
    #[case::one_second(1, 1)]
    #[case::the_largest_stored_value(i64::MAX, i64::MAX.unsigned_abs())]
    fn it_should_parse_a_stored_expiry_that_is_not_negative(#[case] stored: i64, #[case] expected_seconds: u64) {
        let parsed = parse_valid_until(stored, &Driver::Sqlite3).unwrap();

        assert_eq!(parsed, DurationSinceUnixEpoch::from_secs(expected_seconds));
    }

    #[rstest]
    #[case::minus_one(-1, Driver::MySQL)]
    #[case::the_smallest_stored_value(i64::MIN, Driver::PostgreSQL)]
    fn it_should_reject_a_negative_stored_expiry_as_a_malformed_record_of_the_given_driver(
        #[case] stored: i64,
        #[case] driver: Driver,
    ) {
        let (message, reported_driver) = malformed_record(parse_valid_until(stored, &driver).unwrap_err());

        assert_eq!(reported_driver, driver);
        assert!(
            message.contains(&stored.to_string()),
            "the message should name the stored value {stored}, got: {message}"
        );
    }

    #[quickcheck]
    fn it_should_accept_a_stored_expiry_exactly_when_it_is_not_negative(stored: i64) -> TestResult {
        TestResult::from_bool(match parse_valid_until(stored, &Driver::Sqlite3) {
            Ok(parsed) => stored >= 0 && i64::try_from(parsed.as_secs()) == Ok(stored),
            Err(Error::MalformedDatabaseRecord { .. }) => stored < 0,
            Err(_) => false,
        })
    }

    #[rstest]
    #[case::a_permanent_key(None)]
    #[case::an_expiring_key(Some(1_700_000_000))]
    fn it_should_build_a_peer_key_from_the_stored_columns(#[case] stored_expiry: Option<i64>) {
        let peer_key = peer_key_from_row(A_KEY, stored_expiry, &Driver::MySQL).unwrap();

        assert_eq!(
            peer_key,
            PeerKey {
                key: A_KEY.parse::<Key>().unwrap(),
                valid_until: stored_expiry.map(|seconds| DurationSinceUnixEpoch::from_secs(seconds.unsigned_abs())),
            }
        );
    }

    #[rstest]
    #[case::a_key_that_is_too_short("short", None)]
    #[case::a_key_with_a_symbol("YZSl4lMZupRuOpSRC3krIKR5BPB14nr!", None)]
    #[case::a_valid_key_with_a_negative_expiry(A_KEY, Some(-1))]
    fn it_should_reject_a_corrupted_stored_key_as_a_malformed_record_of_the_given_driver(
        #[case] stored_key: &str,
        #[case] stored_expiry: Option<i64>,
    ) {
        let (_message, reported_driver) =
            malformed_record(peer_key_from_row(stored_key, stored_expiry, &Driver::PostgreSQL).unwrap_err());

        assert_eq!(reported_driver, Driver::PostgreSQL);
    }
}
