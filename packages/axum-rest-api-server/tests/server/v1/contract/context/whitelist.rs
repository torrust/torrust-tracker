use std::str::FromStr;

use torrust_info_hash::InfoHash;
use torrust_tracker_axum_rest_api_server::testing::environment::Started;
use torrust_tracker_rest_api_client::v1::client::{ApiHttpClient, headers_with_request_id};
use torrust_tracker_test_helpers::logging::logs_contains_a_line_with;
use torrust_tracker_test_helpers::{configuration, logging};
use uuid::Uuid;

use crate::server::connection_info::{connection_with_invalid_token, connection_with_no_token};
use crate::server::force_database_error;
use crate::server::v1::asserts::{
    assert_disabled_by_configuration, assert_failed_to_reload_whitelist, assert_failed_to_remove_torrent_from_whitelist,
    assert_failed_to_whitelist_torrent, assert_invalid_infohash_param, assert_not_found, assert_ok, assert_token_not_valid,
    assert_unauthorized,
};
use crate::server::v1::contract::fixtures::{invalid_infohashes_returning_bad_request, invalid_infohashes_returning_not_found};

#[tokio::test]
async fn should_reject_whitelist_requests_when_listed_mode_is_disabled_without_database_access() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral().into()).await;
    force_database_error(
        &env.container
            .tracker_core_container
            .persistence
            .as_ref()
            .expect("whitelist test requires persistence")
            .database_stores
            .schema_migrator,
    )
    .await;

    let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d"; // DevSkim: ignore DS173237
    let response = ApiHttpClient::new(env.get_connection_info())
        .unwrap()
        .whitelist_a_torrent(info_hash, None)
        .await
        .unwrap();

    assert_disabled_by_configuration(response, "listed").await;

    env.stop().await;
}

#[tokio::test]
async fn should_allow_whitelisting_a_torrent() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let request_id = Uuid::new_v4();
    let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237

    let response = ApiHttpClient::new(env.get_connection_info())
        .unwrap()
        .whitelist_a_torrent(&info_hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_ok(response).await;
    assert!(
        env.container
            .tracker_core_container
            .in_memory_whitelist
            .contains(&InfoHash::from_str(&info_hash).unwrap())
            .await
    );

    env.stop().await;
}

#[tokio::test]
async fn should_allow_whitelisting_a_torrent_that_has_been_already_whitelisted() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237

    let api_client = ApiHttpClient::new(env.get_connection_info()).unwrap();

    let request_id = Uuid::new_v4();

    let response = api_client
        .whitelist_a_torrent(&info_hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();
    assert_ok(response).await;

    let request_id = Uuid::new_v4();

    let response = api_client
        .whitelist_a_torrent(&info_hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();
    assert_ok(response).await;

    env.stop().await;
}

#[tokio::test]
async fn should_not_allow_whitelisting_a_torrent_for_unauthenticated_users() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(connection_with_invalid_token(env.get_connection_info().origin))
        .unwrap()
        .whitelist_a_torrent(&info_hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_token_not_valid(response).await;

    assert!(
        logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
        "Expected logs to contain: ERROR ... API ... request_id={request_id}"
    );

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(connection_with_no_token(env.get_connection_info().origin))
        .unwrap()
        .whitelist_a_torrent(&info_hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_unauthorized(response).await;

    assert!(
        logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
        "Expected logs to contain: ERROR ... API ... request_id={request_id}"
    );

    env.stop().await;
}

#[tokio::test]
async fn should_fail_when_the_torrent_cannot_be_whitelisted() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237

    force_database_error(
        &env.container
            .tracker_core_container
            .persistence
            .as_ref()
            .expect("whitelist test requires persistence")
            .database_stores
            .schema_migrator,
    )
    .await;

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(env.get_connection_info())
        .unwrap()
        .whitelist_a_torrent(&info_hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_failed_to_whitelist_torrent(response).await;

    assert!(
        logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
        "Expected logs to contain: ERROR ... API ... request_id={request_id}"
    );

    env.stop().await;
}

#[tokio::test]
async fn should_fail_whitelisting_a_torrent_when_the_provided_infohash_is_invalid() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let request_id = Uuid::new_v4();

    for invalid_infohash in &invalid_infohashes_returning_bad_request() {
        let response = ApiHttpClient::new(env.get_connection_info())
            .unwrap()
            .whitelist_a_torrent(invalid_infohash, Some(headers_with_request_id(request_id)))
            .await
            .unwrap();

        assert_invalid_infohash_param(response, invalid_infohash).await;
    }

    let request_id = Uuid::new_v4();

    for invalid_infohash in &invalid_infohashes_returning_not_found() {
        let response = ApiHttpClient::new(env.get_connection_info())
            .unwrap()
            .whitelist_a_torrent(invalid_infohash, Some(headers_with_request_id(request_id)))
            .await
            .unwrap();

        assert_not_found(response).await;
    }

    env.stop().await;
}

#[tokio::test]
async fn should_allow_removing_a_torrent_from_the_whitelist() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&hash).unwrap();

    env.container
        .tracker_core_container
        .persistence
        .as_ref()
        .expect("whitelist test requires persistence")
        .whitelist_manager
        .add_torrent_to_whitelist(&info_hash)
        .await
        .unwrap();

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(env.get_connection_info())
        .unwrap()
        .remove_torrent_from_whitelist(&hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_ok(response).await;
    assert!(
        !env.container
            .tracker_core_container
            .in_memory_whitelist
            .contains(&info_hash)
            .await
    );

    env.stop().await;
}

#[tokio::test]
async fn should_not_fail_trying_to_remove_a_non_whitelisted_torrent_from_the_whitelist() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let non_whitelisted_torrent_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(env.get_connection_info())
        .unwrap()
        .remove_torrent_from_whitelist(&non_whitelisted_torrent_hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_ok(response).await;

    env.stop().await;
}

#[tokio::test]
async fn should_fail_removing_a_torrent_from_the_whitelist_when_the_provided_infohash_is_invalid() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    for invalid_infohash in &invalid_infohashes_returning_bad_request() {
        let request_id = Uuid::new_v4();

        let response = ApiHttpClient::new(env.get_connection_info())
            .unwrap()
            .remove_torrent_from_whitelist(invalid_infohash, Some(headers_with_request_id(request_id)))
            .await
            .unwrap();

        assert_invalid_infohash_param(response, invalid_infohash).await;
    }

    for invalid_infohash in &invalid_infohashes_returning_not_found() {
        let request_id = Uuid::new_v4();

        let response = ApiHttpClient::new(env.get_connection_info())
            .unwrap()
            .remove_torrent_from_whitelist(invalid_infohash, Some(headers_with_request_id(request_id)))
            .await
            .unwrap();

        assert_not_found(response).await;
    }

    env.stop().await;
}

#[tokio::test]
async fn should_fail_when_the_torrent_cannot_be_removed_from_the_whitelist() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&hash).unwrap();
    env.container
        .tracker_core_container
        .persistence
        .as_ref()
        .expect("whitelist test requires persistence")
        .whitelist_manager
        .add_torrent_to_whitelist(&info_hash)
        .await
        .unwrap();

    force_database_error(
        &env.container
            .tracker_core_container
            .persistence
            .as_ref()
            .expect("whitelist test requires persistence")
            .database_stores
            .schema_migrator,
    )
    .await;

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(env.get_connection_info())
        .unwrap()
        .remove_torrent_from_whitelist(&hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_failed_to_remove_torrent_from_whitelist(response).await;

    assert!(
        logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
        "Expected logs to contain: ERROR ... API ... request_id={request_id}"
    );

    env.stop().await;
}

#[tokio::test]
async fn should_not_allow_removing_a_torrent_from_the_whitelist_for_unauthenticated_users() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&hash).unwrap();

    env.container
        .tracker_core_container
        .persistence
        .as_ref()
        .expect("whitelist test requires persistence")
        .whitelist_manager
        .add_torrent_to_whitelist(&info_hash)
        .await
        .unwrap();

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(connection_with_invalid_token(env.get_connection_info().origin))
        .unwrap()
        .remove_torrent_from_whitelist(&hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_token_not_valid(response).await;

    assert!(
        logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
        "Expected logs to contain: ERROR ... API ... request_id={request_id}"
    );

    env.container
        .tracker_core_container
        .persistence
        .as_ref()
        .expect("whitelist test requires persistence")
        .whitelist_manager
        .add_torrent_to_whitelist(&info_hash)
        .await
        .unwrap();

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(connection_with_no_token(env.get_connection_info().origin))
        .unwrap()
        .remove_torrent_from_whitelist(&hash, Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_unauthorized(response).await;

    assert!(
        logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
        "Expected logs to contain: ERROR ... API ... request_id={request_id}"
    );

    env.stop().await;
}

#[tokio::test]
async fn should_allow_reload_the_whitelist_from_the_database() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&hash).unwrap();

    env.container
        .tracker_core_container
        .persistence
        .as_ref()
        .expect("whitelist test requires persistence")
        .whitelist_manager
        .add_torrent_to_whitelist(&info_hash)
        .await
        .unwrap();

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(env.get_connection_info())
        .unwrap()
        .reload_whitelist(Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_ok(response).await;
    /* todo: this assert fails because the whitelist has not been reloaded yet.
       We could add a new endpoint GET /api/whitelist/:info_hash to check if a torrent
       is whitelisted and use that endpoint to check if the torrent is still there after reloading.
    assert!(
        !(env
            .tracker
            .is_info_hash_whitelisted(&InfoHash::from_str(&info_hash).unwrap())
            .await)
    );
    */

    env.stop().await;
}

#[tokio::test]
async fn should_fail_when_the_whitelist_cannot_be_reloaded_from_the_database() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral_listed().into()).await;

    let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned(); // DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&hash).unwrap();
    env.container
        .tracker_core_container
        .persistence
        .as_ref()
        .expect("whitelist test requires persistence")
        .whitelist_manager
        .add_torrent_to_whitelist(&info_hash)
        .await
        .unwrap();

    force_database_error(
        &env.container
            .tracker_core_container
            .persistence
            .as_ref()
            .expect("whitelist test requires persistence")
            .database_stores
            .schema_migrator,
    )
    .await;

    let request_id = Uuid::new_v4();

    let response = ApiHttpClient::new(env.get_connection_info())
        .unwrap()
        .reload_whitelist(Some(headers_with_request_id(request_id)))
        .await
        .unwrap();

    assert_failed_to_reload_whitelist(response).await;

    assert!(
        logs_contains_a_line_with(&["ERROR", "API", &format!("{request_id}")]),
        "Expected logs to contain: ERROR ... API ... request_id={request_id}"
    );

    env.stop().await;
}
