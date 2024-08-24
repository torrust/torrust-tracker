use torrust_tracker::servers::apis::v1::context::health_check::resources::{Report, Status};
use torrust_tracker_test_helpers::configuration;
use tracing::level_filters::LevelFilter;

use crate::common::logging::{tracing_stderr_init, INIT};
use crate::servers::api::v1::client::get;
use crate::servers::api::Started;

#[tokio::test]
async fn health_check_endpoint_should_return_status_ok_if_api_is_running() {
    INIT.call_once(|| {
        tracing_stderr_init(LevelFilter::ERROR);
    });

    let env = Started::new(&configuration::ephemeral().into()).await;

    let url = format!("http://{}/api/health_check", env.get_connection_info().bind_address);

    let response = get(&url, None).await;

    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.json::<Report>().await.unwrap(), Report { status: Status::Ok });

    env.stop().await;
}
