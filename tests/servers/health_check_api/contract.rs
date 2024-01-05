use torrust_tracker::servers::health_check_api::resources::{Report, Status};
use torrust_tracker::servers::registar::Registar;
use torrust_tracker_test_helpers::configuration;

use crate::servers::health_check_api::client::get;
use crate::servers::health_check_api::test_environment;

#[tokio::test]
async fn health_check_endpoint_should_return_status_ok_when_no_service_is_running() {
    let configuration = configuration::ephemeral_with_no_services();

    let registar = &Registar::default();

    let (bound_addr, test_env) = test_environment::start(&configuration.health_check_api, registar.entries()).await;

    let url = format!("http://{bound_addr}/health_check");

    let response = get(&url).await;

    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.json::<Report>().await.unwrap().status, Status::Ok);

    test_env.abort();
}
