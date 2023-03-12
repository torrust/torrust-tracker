use std::time::Duration;

use torrust_tracker::tracker::auth::Key;
use torrust_tracker_test_helpers::configuration;

use crate::api::connection_info::{connection_with_invalid_token, connection_with_no_token};
use crate::api::force_database_error;
use crate::api::test_environment::running_test_environment;
use crate::api::v1::asserts::{
    assert_auth_key_utf8, assert_failed_to_delete_key, assert_failed_to_generate_key, assert_failed_to_reload_keys,
    assert_invalid_auth_key_param, assert_invalid_key_duration_param, assert_ok, assert_token_not_valid, assert_unauthorized,
};
use crate::api::v1::client::Client;

#[tokio::test]
async fn should_allow_generating_a_new_auth_key() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let seconds_valid = 60;

    let response = Client::new(test_env.get_connection_info())
        .generate_auth_key(seconds_valid)
        .await;

    let auth_key_resource = assert_auth_key_utf8(response).await;

    // Verify the key with the tracker
    assert!(test_env
        .tracker
        .verify_auth_key(&auth_key_resource.key.parse::<Key>().unwrap())
        .await
        .is_ok());

    test_env.stop().await;
}

#[tokio::test]
async fn should_not_allow_generating_a_new_auth_key_for_unauthenticated_users() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let seconds_valid = 60;

    let response = Client::new(connection_with_invalid_token(
        test_env.get_connection_info().bind_address.as_str(),
    ))
    .generate_auth_key(seconds_valid)
    .await;

    assert_token_not_valid(response).await;

    let response = Client::new(connection_with_no_token(test_env.get_connection_info().bind_address.as_str()))
        .generate_auth_key(seconds_valid)
        .await;

    assert_unauthorized(response).await;

    test_env.stop().await;
}

#[tokio::test]
async fn should_fail_generating_a_new_auth_key_when_the_key_duration_is_invalid() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let invalid_key_durations = [
        // "", it returns 404
        // " ", it returns 404
        "-1", "text",
    ];

    for invalid_key_duration in invalid_key_durations {
        let response = Client::new(test_env.get_connection_info())
            .post(&format!("key/{invalid_key_duration}"))
            .await;

        assert_invalid_key_duration_param(response, invalid_key_duration).await;
    }

    test_env.stop().await;
}

#[tokio::test]
async fn should_fail_when_the_auth_key_cannot_be_generated() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    force_database_error(&test_env.tracker);

    let seconds_valid = 60;
    let response = Client::new(test_env.get_connection_info())
        .generate_auth_key(seconds_valid)
        .await;

    assert_failed_to_generate_key(response).await;

    test_env.stop().await;
}

#[tokio::test]
async fn should_allow_deleting_an_auth_key() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let seconds_valid = 60;
    let auth_key = test_env
        .tracker
        .generate_auth_key(Duration::from_secs(seconds_valid))
        .await
        .unwrap();

    let response = Client::new(test_env.get_connection_info())
        .delete_auth_key(&auth_key.key.to_string())
        .await;

    assert_ok(response).await;

    test_env.stop().await;
}

#[tokio::test]
async fn should_fail_deleting_an_auth_key_when_the_key_id_is_invalid() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let invalid_auth_keys = [
        // "", it returns a 404
        // " ", it returns a 404
        "0",
        "-1",
        "INVALID AUTH KEY ID",
        "IrweYtVuQPGbG9Jzx1DihcPmJGGpVy8",   // 32 char key cspell:disable-line
        "IrweYtVuQPGbG9Jzx1DihcPmJGGpVy8zs", // 34 char key cspell:disable-line
    ];

    for invalid_auth_key in &invalid_auth_keys {
        let response = Client::new(test_env.get_connection_info())
            .delete_auth_key(invalid_auth_key)
            .await;

        assert_invalid_auth_key_param(response, invalid_auth_key).await;
    }

    test_env.stop().await;
}

#[tokio::test]
async fn should_fail_when_the_auth_key_cannot_be_deleted() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let seconds_valid = 60;
    let auth_key = test_env
        .tracker
        .generate_auth_key(Duration::from_secs(seconds_valid))
        .await
        .unwrap();

    force_database_error(&test_env.tracker);

    let response = Client::new(test_env.get_connection_info())
        .delete_auth_key(&auth_key.key.to_string())
        .await;

    assert_failed_to_delete_key(response).await;

    test_env.stop().await;
}

#[tokio::test]
async fn should_not_allow_deleting_an_auth_key_for_unauthenticated_users() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let seconds_valid = 60;

    // Generate new auth key
    let auth_key = test_env
        .tracker
        .generate_auth_key(Duration::from_secs(seconds_valid))
        .await
        .unwrap();

    let response = Client::new(connection_with_invalid_token(
        test_env.get_connection_info().bind_address.as_str(),
    ))
    .delete_auth_key(&auth_key.key.to_string())
    .await;

    assert_token_not_valid(response).await;

    // Generate new auth key
    let auth_key = test_env
        .tracker
        .generate_auth_key(Duration::from_secs(seconds_valid))
        .await
        .unwrap();

    let response = Client::new(connection_with_no_token(test_env.get_connection_info().bind_address.as_str()))
        .delete_auth_key(&auth_key.key.to_string())
        .await;

    assert_unauthorized(response).await;

    test_env.stop().await;
}

#[tokio::test]
async fn should_allow_reloading_keys() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let seconds_valid = 60;
    test_env
        .tracker
        .generate_auth_key(Duration::from_secs(seconds_valid))
        .await
        .unwrap();

    let response = Client::new(test_env.get_connection_info()).reload_keys().await;

    assert_ok(response).await;

    test_env.stop().await;
}

#[tokio::test]
async fn should_fail_when_keys_cannot_be_reloaded() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let seconds_valid = 60;
    test_env
        .tracker
        .generate_auth_key(Duration::from_secs(seconds_valid))
        .await
        .unwrap();

    force_database_error(&test_env.tracker);

    let response = Client::new(test_env.get_connection_info()).reload_keys().await;

    assert_failed_to_reload_keys(response).await;

    test_env.stop().await;
}

#[tokio::test]
async fn should_not_allow_reloading_keys_for_unauthenticated_users() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let seconds_valid = 60;
    test_env
        .tracker
        .generate_auth_key(Duration::from_secs(seconds_valid))
        .await
        .unwrap();

    let response = Client::new(connection_with_invalid_token(
        test_env.get_connection_info().bind_address.as_str(),
    ))
    .reload_keys()
    .await;

    assert_token_not_valid(response).await;

    let response = Client::new(connection_with_no_token(test_env.get_connection_info().bind_address.as_str()))
        .reload_keys()
        .await;

    assert_unauthorized(response).await;

    test_env.stop().await;
}
