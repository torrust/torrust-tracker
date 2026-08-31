use torrust_server_lib::registar::Registar;
use torrust_tracker_axum_health_check_api_server::environment::Started;
use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
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
    use std::net::{Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_configuration::v3_0_0::public_url::HttpUrl;
    use torrust_tracker_test_helpers::{configuration, logging};
    use url::Url;

    use crate::server::client::get;

    #[tokio::test]
    pub(crate) async fn it_should_return_good_health_for_api_service() {
        logging::setup();

        let mut configuration = configuration::ephemeral();
        let http_api_config = configuration.http_api.as_mut().expect("missing HTTP API configuration");
        http_api_config.bind_address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0));
        http_api_config.public_url = Some(HttpUrl::parse("https://tracker.example.test/api").expect("valid public URL"));
        let configured_bind_address = configuration
            .http_api
            .as_ref()
            .expect("missing HTTP API configuration")
            .bind_address;
        assert!(configured_bind_address.ip().is_unspecified());
        assert_eq!(configured_bind_address.port(), 0);
        let configuration = Arc::new(configuration);

        let service = torrust_tracker_axum_rest_api_server::testing::environment::Started::new(&configuration).await;

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

            assert_eq!(
                details.service_binding,
                Url::parse(&format!("http://{}", service.bind_address())).unwrap()
            );
            assert_eq!(details.binding, service.bind_address());
            assert_eq!(details.service_type, "tracker_rest_api");
            assert_eq!(details.public_url.as_deref(), Some("https://tracker.example.test/api"));
            assert_eq!(details.binding.ip(), configured_bind_address.ip());
            assert_ne!(details.binding.port(), configured_bind_address.port());

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

        let service = torrust_tracker_axum_rest_api_server::testing::environment::Started::new(&configuration).await;

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

            assert_eq!(details.service_binding, Url::parse(&format!("http://{binding}")).unwrap());
            assert_eq!(details.binding, binding);
            assert_eq!(details.service_type, "tracker_rest_api");
            assert_eq!(details.public_url, None);
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

    use torrust_net_primitives::service_binding::ServiceBinding;
    use torrust_server_lib::registar::ServiceHealthCheckJob;
    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_configuration::v3_0_0::tls::TlsConfig;
    use torrust_tracker_test_helpers::{configuration, logging};
    use url::Url;

    use crate::server::client::{get, install_rustls_crypto_provider};

    fn trusted_test_check_fn(service_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        let certificate = reqwest::Certificate::from_pem(include_bytes!("../fixtures/https-health-check-cert.pem"))
            .expect("test certificate should parse");
        let client = reqwest::Client::builder()
            .add_root_certificate(certificate)
            .build()
            .expect("trusted test client should build");

        torrust_tracker_axum_http_server::server::check_fn_with_client(service_binding, client)
    }

    #[tokio::test]
    pub(crate) async fn it_should_return_good_health_for_http_service() {
        logging::setup();

        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let http_tracker_config = Arc::new(configuration.http_trackers.clone().unwrap()[0].clone());

        let service =
            torrust_tracker_axum_http_server::testing::environment::Started::new(&core_config, &http_tracker_config).await;

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

            assert_eq!(
                details.service_binding,
                Url::parse(&format!("http://{}", service.bind_address())).unwrap()
            );
            assert_eq!(details.binding, *service.bind_address());
            assert_eq!(details.service_type, "http_tracker");
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
    pub(crate) async fn it_should_return_good_health_for_https_service_with_a_trusted_test_certificate() {
        logging::setup();
        install_rustls_crypto_provider();

        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let mut http_tracker_config = configuration
            .http_trackers
            .clone()
            .expect("missing HTTP tracker configuration")[0]
            .clone();
        http_tracker_config.tls_config = Some(TlsConfig {
            ssl_cert_path: "tests/fixtures/https-health-check-cert.pem".into(),
            ssl_key_path: "tests/fixtures/https-health-check-key.pem".into(),
        });

        let service = torrust_tracker_axum_http_server::testing::environment::Environment::<
            torrust_tracker_axum_http_server::server::Stopped,
        >::new(&core_config, &Arc::new(http_tracker_config))
        .await
        .start_with_health_check(trusted_test_check_fn)
        .await;

        let registar = service.registar.clone();

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.state.binding)).await; // DevSkim: ignore DS137138
            let report: Report = response.json().await.expect("health report should deserialize");
            let details = report
                .details
                .first()
                .expect("health report should include the HTTPS tracker");

            assert_eq!(report.status, Status::Ok);
            assert_eq!(
                details.service_binding,
                Url::parse(&format!("https://{}", service.bind_address())).unwrap()
            );
            assert_eq!(details.binding, *service.bind_address());
            assert_eq!(details.service_type, "http_tracker");
            assert_eq!(details.result, Ok("200 OK".to_string()));
            assert_eq!(
                details.info,
                format!(
                    "checking http tracker health check at: https://{}/health_check",
                    service.bind_address()
                )
            );

            env.stop().await.expect("health-check API should stop");
        }

        service.stop().await;
    }

    #[tokio::test]
    pub(crate) async fn it_should_return_error_when_http_service_was_stopped_after_registration() {
        logging::setup();

        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let http_tracker_config = Arc::new(configuration.http_trackers.clone().unwrap()[0].clone());

        let service =
            torrust_tracker_axum_http_server::testing::environment::Started::new(&core_config, &http_tracker_config).await;

        let binding = *service.bind_address();

        let registar = service.registar.clone();

        service.server.stop().await.expect("it should stop udp server");

        // Give the OS a moment to fully release the TCP port after the server stops.
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

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

            assert_eq!(details.service_binding, Url::parse(&format!("http://{binding}")).unwrap());
            assert_eq!(details.binding, binding);
            assert_eq!(details.service_type, "http_tracker");
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

    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_test_helpers::{configuration, logging};
    use url::Url;

    use crate::server::client::get;

    #[tokio::test]
    pub(crate) async fn it_should_return_good_health_for_udp_service() {
        logging::setup();

        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let udp_tracker_config = Arc::new(configuration.udp_trackers.clone().unwrap()[0].clone());

        let service = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

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

            assert_eq!(
                details.service_binding,
                Url::parse(&format!("udp://{}", service.bind_address())).unwrap()
            );
            assert_eq!(details.binding, service.bind_address());
            assert_eq!(details.service_type, "udp_tracker");
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

        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let udp_tracker_config = Arc::new(configuration.udp_trackers.clone().unwrap()[0].clone());

        let service = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

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

            assert_eq!(details.service_binding, Url::parse(&format!("udp://{binding}")).unwrap());
            assert_eq!(details.binding, binding);
            assert_eq!(details.service_type, "udp_tracker");
            assert_eq!(details.result, Err("Timed Out".to_string()));
            assert_eq!(details.info, format!("checking the udp tracker health check at: {binding}"));

            env.stop().await.expect("it should stop the service");
        }
    }
}
