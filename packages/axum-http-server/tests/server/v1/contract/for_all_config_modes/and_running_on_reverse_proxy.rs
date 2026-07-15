use std::sync::Arc;
use std::time::Duration;

use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_client::http::client::Client;
use torrust_tracker_http_protocol::v1::requests::announce::AnnounceBuilder;
use torrust_tracker_test_helpers::{configuration, logging};

use crate::server::asserts::assert_could_not_find_remote_address_on_x_forwarded_for_header_error_response;

#[tokio::test]
async fn should_fail_when_the_http_request_does_not_include_the_xff_http_request_header() {
    logging::setup();

    // If the tracker is running behind a reverse proxy, the peer IP is the
    // right most IP in the `X-Forwarded-For` HTTP header, which is the IP of the proxy's client.

    let cfg = configuration::ephemeral_with_reverse_proxy();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let params = AnnounceBuilder::default().query().to_string();

    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .get(&format!("announce?{params}"))
        .await
        .unwrap();

    assert_could_not_find_remote_address_on_x_forwarded_for_header_error_response(response).await;

    env.stop().await;
}

#[tokio::test]
async fn should_fail_when_the_xff_http_request_header_contains_an_invalid_ip() {
    logging::setup();

    let cfg = configuration::ephemeral_with_reverse_proxy();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let params = AnnounceBuilder::default().query().to_string();

    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .get_with_header(&format!("announce?{params}"), "X-Forwarded-For", "INVALID IP")
        .await
        .unwrap();

    assert_could_not_find_remote_address_on_x_forwarded_for_header_error_response(response).await;

    env.stop().await;
}
