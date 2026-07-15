use std::sync::Arc;

use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_test_helpers::{configuration, logging};

mod configured_as_private;
mod configured_as_private_and_whitelisted;
mod configured_as_whitelisted;
mod for_all_config_modes;
mod using_ipv6_v6only;

#[tokio::test]
async fn environment_should_be_started_and_stopped() {
    logging::setup();

    let cfg = configuration::ephemeral();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    env.stop().await;
}
