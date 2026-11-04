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

    let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

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
    async fn it_should_return_good_health_for_api_service() {
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

            let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

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
    async fn it_should_return_error_when_api_service_was_stopped_after_registration() {
        logging::setup();

        let configuration = Arc::new(configuration::ephemeral());

        let service = torrust_tracker_axum_rest_api_server::testing::environment::Started::new(&configuration).await;

        let binding = service.bind_address();

        let registar = service.registar.clone();

        let _stopped = service.stop().await;

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

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
    async fn it_should_return_good_health_for_http_service() {
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

            let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

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
    async fn it_should_return_good_health_for_https_service_with_a_trusted_test_certificate() {
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

        let service =
            torrust_tracker_axum_http_server::testing::environment::Unstarted::new(&core_config, &Arc::new(http_tracker_config))
                .await
                .start_with_health_check(trusted_test_check_fn)
                .await;

        let registar = service.registar.clone();

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138
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
    async fn it_should_return_error_when_http_service_was_stopped_after_registration() {
        logging::setup();

        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let http_tracker_config = Arc::new(configuration.http_trackers.clone().unwrap()[0].clone());

        let service =
            torrust_tracker_axum_http_server::testing::environment::Started::new(&core_config, &http_tracker_config).await;

        let binding = *service.bind_address();

        let registar = service.registar.clone();

        let _stopped = service.stop().await;

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

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
    async fn it_should_return_good_health_for_udp_service() {
        logging::setup();

        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let udp_tracker_config = Arc::new(configuration.udp_trackers.clone().unwrap()[0].clone());

        let service = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

        let registar = service.registar.clone();

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

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
    async fn it_should_return_error_when_udp_service_was_stopped_after_registration() {
        logging::setup();

        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let udp_tracker_config = Arc::new(configuration.udp_trackers.clone().unwrap()[0].clone());

        let service = torrust_tracker_udp_server::testing::environment::Started::new(&core_config, &udp_tracker_config).await;

        let binding = service.bind_address();

        let registar = service.registar.clone();

        let _stopped = service.stop().await;

        {
            let config = configuration.health_check_api.clone();
            let env = Started::new(&config.into(), registar).await;

            let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

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

// issue: #2697
mod readiness {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{FnSpawnServiceHeathCheck, Registar, ServiceHealthCheckJob, ServiceRegistration};
    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_test_helpers::{configuration, logging};

    use crate::server::client::get;

    // One counter per test: tests run in parallel and a probe is a plain `fn` pointer.
    static PROBES_RUN_WHEN_READY: AtomicUsize = AtomicUsize::new(0);
    static PROBES_RUN_WHEN_NOT_READY: AtomicUsize = AtomicUsize::new(0);

    fn probe_counted_when_ready(_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        PROBES_RUN_WHEN_READY.fetch_add(1, Ordering::SeqCst);
        passing_probe()
    }

    fn probe_counted_when_not_ready(_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        PROBES_RUN_WHEN_NOT_READY.fetch_add(1, Ordering::SeqCst);
        passing_probe()
    }

    fn passing_probe() -> ServiceHealthCheckJob {
        ServiceHealthCheckJob::new("counting probe".to_string(), tokio::spawn(async { Ok("200 OK".to_string()) }))
    }

    async fn registar_with_a_probed_service(probe: FnSpawnServiceHeathCheck) -> Registar<RuntimeServiceMetadata> {
        let registar = Registar::default();
        registar
            .give_form()
            .register(ServiceRegistration::new(
                ServiceBinding::new(Protocol::HTTP, "127.0.0.1:7070".parse().unwrap()).unwrap(), // DevSkim: ignore DS137138
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                Some(probe),
            ))
            .await
            .expect("the probed service should register");
        registar
    }

    #[tokio::test]
    async fn it_should_answer_503_without_probing_any_service_when_the_application_is_not_ready() {
        // Arrange: a probed service is registered, and the application is not ready.
        logging::setup();
        let configuration = configuration::ephemeral_with_no_services();
        let env = Started::new(
            &configuration.health_check_api.into(),
            registar_with_a_probed_service(probe_counted_when_not_ready).await,
        )
        .await;
        env.readiness.mark_not_ready();
        let probes_before = PROBES_RUN_WHEN_NOT_READY.load(Ordering::SeqCst);

        // Act
        let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

        // Assert
        assert_eq!(response.status(), 503);
        assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
        let report: Report = response.json().await.expect("the 503 body should be a report");
        assert_eq!(report.status, Status::Error);
        assert_eq!(report.message, "shutting down");
        assert_eq!(report.details.len(), 0);
        assert_eq!(
            PROBES_RUN_WHEN_NOT_READY.load(Ordering::SeqCst),
            probes_before,
            "a not-ready answer should not run the registered service probes"
        );

        env.stop().await.expect("it should stop the service");
    }

    #[tokio::test]
    async fn it_should_probe_the_services_and_answer_200_while_the_application_is_ready() {
        // Arrange
        logging::setup();
        let configuration = configuration::ephemeral_with_no_services();
        let env = Started::new(
            &configuration.health_check_api.into(),
            registar_with_a_probed_service(probe_counted_when_ready).await,
        )
        .await;
        let probes_before = PROBES_RUN_WHEN_READY.load(Ordering::SeqCst);

        // Act
        let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

        // Assert
        assert_eq!(response.status(), 200);
        let report: Report = response.json().await.expect("the body should be a report");
        assert_eq!(report.status, Status::Ok);
        assert_eq!(report.message, "");
        assert_eq!(PROBES_RUN_WHEN_READY.load(Ordering::SeqCst), probes_before + 1);

        env.stop().await.expect("it should stop the service");
    }

    #[tokio::test]
    async fn it_should_answer_503_on_every_request_after_the_transition_and_never_return_to_ready() {
        // Arrange
        logging::setup();
        let configuration = configuration::ephemeral_with_no_services();
        let env = Started::new(&configuration.health_check_api.into(), Registar::default()).await;
        let url = format!("http://{}/health_check", env.bind_address()); // DevSkim: ignore DS137138
        assert_eq!(get(&url).await.status(), 200);

        // Act
        env.readiness.mark_not_ready();
        let first = get(&url).await.status();
        let second = get(&url).await.status();

        // Assert
        assert_eq!((first.as_u16(), second.as_u16()), (503, 503));

        env.stop().await.expect("it should stop the service");
    }
}
