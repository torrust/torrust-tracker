use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_client::http::client::Client;
use torrust_tracker_test_helpers::{configuration, logging};

#[tokio::test]
async fn should_accept_ipv6_connections_with_ipv6_v6only_enabled() {
    logging::setup();

    let cfg = configuration::ephemeral();
    let core_config = Arc::new(cfg.core.clone());
    let mut http_tracker_config = cfg.http_trackers.unwrap()[0].clone();
    http_tracker_config.bind_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);
    http_tracker_config.ipv6_v6only = true;
    let http_tracker_config = Arc::new(http_tracker_config);
    let env = Started::new(&core_config, &http_tracker_config).await;

    let client = Client::bind(env.base_url(), Duration::from_secs(5), IpAddr::V6(Ipv6Addr::UNSPECIFIED)).unwrap();

    let response = client.health_check().await.unwrap();

    assert_eq!(response.status(), 200);

    env.stop().await;
}
