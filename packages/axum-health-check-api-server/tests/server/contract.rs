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

mod api {
    use std::sync::Arc;

    use torrust_axum_health_check_api_server::environment::Started;
    use torrust_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_test_helpers::{configuration, logging};

    use crate::server::client::get;

    #[tokio::test]
    pub(crate) async fn it_should_return_good_health_for_api_service() {
        logging::setup();

        let configuration = Arc::new(configuration::ephemeral());

        let service = torrust_axum_rest_tracker_api_server::environment::Started::new(&configuration).await;

        let registar = service.registar.clone();

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.state.binding)).await; // DevSkim: ignore DS137138

            assert_eq!(response.status(), 200);
            assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

            let report: Report = response
                .json()
                .await
                .expect("it should be able to get the report from the json");

            assert_eq!(report.status, Status::Ok);
            assert_eq!(report.message, String::new());

            let details = report.details.first().expect("it should have some details");

            assert_eq!(details.binding, service.bind_address());

            assert_eq!(details.result, Ok("200 OK".to_string()));

            assert_eq!(
                details.info,
                format!(
                    "checking api health check at: http://{}/api/health_check", // DevSkim: ignore DS137138
                    service.bind_address()
                )
            );

            env.stop().await.expect("it should stop the service");
        }

        service.stop().await;
    }

    #[tokio::test]
    pub(crate) async fn it_should_return_error_when_api_service_was_stopped_after_registration() {
        logging::setup();

        let configuration = Arc::new(configuration::ephemeral());

        let service = torrust_axum_rest_tracker_api_server::environment::Started::new(&configuration).await;

        let binding = service.bind_address();

        let registar = service.registar.clone();

        service.server.stop().await.expect("it should stop udp server");

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.state.binding)).await; // DevSkim: ignore DS137138

            assert_eq!(response.status(), 200);
            assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

            let report: Report = response
                .json()
                .await
                .expect("it should be able to get the report from the json");

            assert_eq!(report.status, Status::Error);
            assert_eq!(report.message, "health check failed".to_string());

            let details = report.details.first().expect("it should have some details");

            assert_eq!(details.binding, binding);
            assert!(
                details.result.as_ref().is_err_and(|e| e.contains("error sending request")),
                "Expected to contain, \"error sending request\", but have message \"{:?}\".",
                details.result
            );
            assert_eq!(
                details.info,
                format!("checking api health check at: http://{binding}/api/health_check") // DevSkim: ignore DS137138
            );

            env.stop().await.expect("it should stop the service");
        }
    }
}

mod http {
    use std::sync::Arc;

    use torrust_axum_health_check_api_server::environment::Started;
    use torrust_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_test_helpers::{configuration, logging};

    use crate::server::client::get;

    #[tokio::test]
    pub(crate) async fn it_should_return_good_health_for_http_service() {
        logging::setup();

        let configuration = Arc::new(configuration::ephemeral());

        let service = torrust_axum_http_tracker_server::environment::Started::new(&configuration).await;

        let registar = service.registar.clone();

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.state.binding)).await; // DevSkim: ignore DS137138

            assert_eq!(response.status(), 200);
            assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

            let report: Report = response
                .json()
                .await
                .expect("it should be able to get the report from the json");

            assert_eq!(report.status, Status::Ok);
            assert_eq!(report.message, String::new());

            let details = report.details.first().expect("it should have some details");

            assert_eq!(details.binding, *service.bind_address());
            assert_eq!(details.result, Ok("200 OK".to_string()));

            assert_eq!(
                details.info,
                format!(
                    "checking http tracker health check at: http://{}/health_check", // DevSkim: ignore DS137138
                    service.bind_address()
                )
            );

            env.stop().await.expect("it should stop the service");
        }

        service.stop().await;
    }

    #[tokio::test]
    pub(crate) async fn it_should_return_error_when_http_service_was_stopped_after_registration() {
        logging::setup();

        let configuration = Arc::new(configuration::ephemeral());

        let service = torrust_axum_http_tracker_server::environment::Started::new(&configuration).await;

        let binding = *service.bind_address();

        let registar = service.registar.clone();

        service.server.stop().await.expect("it should stop udp server");

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.state.binding)).await; // DevSkim: ignore DS137138

            assert_eq!(response.status(), 200);
            assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

            let report: Report = response
                .json()
                .await
                .expect("it should be able to get the report from the json");

            assert_eq!(report.status, Status::Error);
            assert_eq!(report.message, "health check failed".to_string());

            let details = report.details.first().expect("it should have some details");

            assert_eq!(details.binding, binding);
            assert!(
                details.result.as_ref().is_err_and(|e| e.contains("error sending request")),
                "Expected to contain, \"error sending request\", but have message \"{:?}\".",
                details.result
            );
            assert_eq!(
                details.info,
                format!("checking http tracker health check at: http://{binding}/health_check") // DevSkim: ignore DS137138
            );

            env.stop().await.expect("it should stop the service");
        }
    }
}

mod udp {
    use std::sync::Arc;

    use torrust_axum_health_check_api_server::environment::Started;
    use torrust_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_test_helpers::{configuration, logging};

    use crate::server::client::get;

    #[tokio::test]
    pub(crate) async fn it_should_return_good_health_for_udp_service() {
        logging::setup();

        let configuration = Arc::new(configuration::ephemeral());

        let service = torrust_udp_tracker_server::environment::Started::new(&configuration).await;

        let registar = service.registar.clone();

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.state.binding)).await; // DevSkim: ignore DS137138

            assert_eq!(response.status(), 200);
            assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

            let report: Report = response
                .json()
                .await
                .expect("it should be able to get the report from the json");

            assert_eq!(report.status, Status::Ok);
            assert_eq!(report.message, String::new());

            let details = report.details.first().expect("it should have some details");

            assert_eq!(details.binding, service.bind_address());
            assert_eq!(details.result, Ok("Connected".to_string()));

            assert_eq!(
                details.info,
                format!("checking the udp tracker health check at: {}", service.bind_address())
            );

            env.stop().await.expect("it should stop the service");
        }

        service.stop().await;
    }

    #[tokio::test]
    pub(crate) async fn it_should_return_error_when_udp_service_was_stopped_after_registration() {
        logging::setup();

        let configuration = Arc::new(configuration::ephemeral());

        let service = torrust_udp_tracker_server::environment::Started::new(&configuration).await;

        let binding = service.bind_address();

        let registar = service.registar.clone();

        service.server.stop().await.expect("it should stop udp server");

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.state.binding)).await; // DevSkim: ignore DS137138

            assert_eq!(response.status(), 200);
            assert_eq!(response.headers().get("content-type").unwrap(), "application/json");

            let report: Report = response
                .json()
                .await
                .expect("it should be able to get the report from the json");

            assert_eq!(report.status, Status::Error);
            assert_eq!(report.message, "health check failed".to_string());

            let details = report.details.first().expect("it should have some details");

            assert_eq!(details.binding, binding);
            assert_eq!(details.result, Err("Timed Out".to_string()));
            assert_eq!(details.info, format!("checking the udp tracker health check at: {binding}"));

            env.stop().await.expect("it should stop the service");
        }
    }
}
