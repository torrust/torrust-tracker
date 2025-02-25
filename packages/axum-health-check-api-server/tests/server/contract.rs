use torrust_axum_health_check_api_server::environment::Started;
use torrust_axum_health_check_api_server::resources::{Report, Status};
use torrust_server_lib::registar::Registar;
use torrust_tracker_test_helpers::{configuration, logging};

use crate::server::client::get;

#[tokio::test]
async fn health_check_endpoint_should_return_status_ok_when_there_is_no_services_registered() {
    logging::setup();

    let configuration = configuration::ephemeral_with_no_services();

    let env = Started::new(&configuration.health_check_api.into(), Registar::default()).await;

    let response = get(&format!("http://{}/health_check", env.state.binding)).await; // DevSkim: ignore DS137138

    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

    let report = response
        .json::<Report>()
        .await
        .expect("it should be able to get the report as json");

    assert_eq!(report.status, Status::None);

    env.stop().await.expect("it should stop the service");
}
