use torrust_tracker_axum_rest_api_server::testing::environment::Started;
use torrust_tracker_rest_api_client::v1::client::get;
use torrust_tracker_rest_api_protocol::v1::context::health_check::resources::report::{Report, Status};
use torrust_tracker_test_helpers::{configuration, logging};
use url::Url;

#[tokio::test]
async fn health_check_endpoint_should_return_status_ok_if_api_is_running() {
    logging::setup();

    let env = Started::new(&configuration::ephemeral().into()).await;

    let url = Url::parse(&format!("{}api/health_check", env.get_connection_info().origin)).unwrap();

    let response = get(url, None, None).await.unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.json::<Report>().await.unwrap(), Report { status: Status::Ok });

    env.stop().await;
}
