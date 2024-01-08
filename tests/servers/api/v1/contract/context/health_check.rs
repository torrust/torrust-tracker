use torrust_tracker::servers::apis::v1::context::health_check::resources::{Report, Status};
use torrust_tracker_test_helpers::configuration;

use crate::servers::api::test_environment::running_test_environment;
use crate::servers::api::v1::client::get;

#[tokio::test]
async fn health_check_endpoint_should_return_status_ok_if_api_is_running() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let url = format!("http://{}/api/health_check", test_env.get_connection_info().bind_address);

    let response = get(&url, None).await;

    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.json::<Report>().await.unwrap(), Report { status: Status::Ok });

    test_env.stop().await;
}
