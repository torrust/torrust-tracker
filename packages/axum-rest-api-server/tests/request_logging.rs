//! The log of a request to the REST API holds no peer key and no admin token.
//!
//! A key is a path segment (`/api/v1/key/<key>`) and the token can be a query parameter
//! (`?token=<token>`), so the URI the server logs for every request, in the `request` event and
//! in the span around it, must not show them. The test binary installs a subscriber that captures
//! every level, which only one binary can do.
use secrecy::ExposeSecret;
use torrust_tracker_axum_rest_api_server::testing::environment::Started;
use torrust_tracker_test_helpers::{configuration, logging};

#[tokio::test]
async fn it_should_not_log_a_key_or_an_admin_token_that_the_uri_of_a_request_holds() {
    // Arrange
    logging::setup_capturing_every_level();
    let env = Started::new(&configuration::ephemeral().into()).await;
    let connection_info = env.get_connection_info();
    let admin_token = connection_info
        .api_token
        .as_ref()
        .expect("the test environment has an admin token")
        .expose_secret()
        .to_string();
    let base = format!("{}api/v1", connection_info.origin);
    let a_key = "TestPeerKeyLoggedByDelete0000001";
    let a_key_of_the_wrong_length = "ShortLoggedKey01";
    let a_key_posted_where_the_seconds_go = "TestPeerKeyPostedByMistake000001";
    let client = reqwest::Client::new();

    // Act
    client.get(format!("{base}/stats?token={admin_token}")).send().await.unwrap();
    client
        .delete(format!("{base}/key/{a_key}?token={admin_token}"))
        .send()
        .await
        .unwrap();
    client
        .delete(format!("{base}/key/{a_key_of_the_wrong_length}"))
        .bearer_auth(&admin_token)
        .send()
        .await
        .unwrap();
    client
        .post(format!("{base}/key/{a_key_posted_where_the_seconds_go}"))
        .bearer_auth(&admin_token)
        .send()
        .await
        .unwrap();
    env.stop().await;

    // Assert
    let logs = logging::captured_logs();
    assert!(!logs.contains(&admin_token), "the log holds the test admin token");
    for key in [a_key, a_key_of_the_wrong_length, a_key_posted_where_the_seconds_go] {
        assert!(
            !logs.contains(key),
            "the log holds a test peer key of {} characters",
            key.len()
        );
    }
    assert!(
        logs.contains("uri=/api/v1/stats?token=***"),
        "the request should be logged with a redacted URI, got none in: {logs}"
    );
}
