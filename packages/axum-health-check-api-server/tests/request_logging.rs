//! The log of a request to the health-check API holds no peer key and no admin token.
//!
//! The health-check API serves no secret, but it logs the URI of every request it receives, and a
//! client can send a key or a token to it by mistake. The test binary installs a subscriber that
//! captures every level, which only one binary can do.
use torrust_server_lib::registar::Registar;
use torrust_tracker_axum_health_check_api_server::environment::Started;
use torrust_tracker_test_helpers::{configuration, logging};

#[tokio::test]
async fn it_should_not_log_a_key_or_an_admin_token_that_the_uri_of_a_request_holds() {
    // Arrange
    logging::setup_capturing_every_level();
    let configuration = configuration::ephemeral_with_no_services();
    let env = Started::new(&configuration.health_check_api.into(), Registar::default()).await;
    let address = env.bind_address();
    let admin_token = "TestAdminTokenLoggedByHealth001";
    let a_key = "TestPeerKeyLoggedByHealth0000001";

    // Act
    reqwest::get(format!("http://{address}/health_check?token={admin_token}"))
        .await
        .unwrap(); // DevSkim: ignore DS137138
    reqwest::get(format!("http://{address}/announce/{a_key}")).await.unwrap(); // DevSkim: ignore DS137138
    env.stop().await.unwrap();

    // Assert
    let logs = logging::captured_logs();
    assert!(!logs.contains(admin_token), "the log holds the test admin token");
    assert!(!logs.contains(a_key), "the log holds the test peer key");
    assert!(
        logs.contains("uri=/health_check?token=***"),
        "the request should be logged with a redacted URI, got none in: {logs}"
    );
}
