mod and_running_on_reverse_proxy;
mod receiving_an_announce_request;
mod receiving_an_scrape_request;

use std::sync::Arc;
use std::time::Duration;

use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_axum_http_server::v1::handlers::health_check::{Report, Status};
use torrust_tracker_client::http::client::Client;
use torrust_tracker_test_helpers::{configuration, logging};

#[tokio::test]
async fn health_check_endpoint_should_return_ok_if_the_http_tracker_is_running() {
    logging::setup();

    let cfg = configuration::ephemeral_with_reverse_proxy();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .health_check()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.json::<Report>().await.unwrap(), Report { status: Status::Ok });

    env.stop().await;
}
