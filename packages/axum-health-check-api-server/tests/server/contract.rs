use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_server_lib::registar::{FnSpawnServiceHeathCheck, Registar, ServiceRegistration};
use torrust_tracker_axum_health_check_api_server::environment::Started;
use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use torrust_tracker_test_helpers::{configuration, logging};

use crate::server::client::get;

/// Registers one service with its health check, as the service does when it starts.
async fn registar_with_one_service(
    service_binding: ServiceBinding,
    role: ServiceRole,
    health_check: FnSpawnServiceHeathCheck,
) -> Registar<RuntimeServiceMetadata> {
    let registar = Registar::default();
    registar
        .give_form()
        .register(ServiceRegistration::new(
            service_binding,
            RuntimeServiceMetadata::new(ConfigurationInstanceId::new(role, 0)),
            Some(health_check),
        ))
        .await
        .expect("the service should register");
    registar
}

/// Starts a health-check API that reports on the services in `registar`.
async fn start_health_check_api(registar: Registar<RuntimeServiceMetadata>) -> Started {
    Started::new(&configuration::ephemeral_with_no_services().health_check_api.into(), registar).await
}

/// Binds a TCP socket without listening on it. While the test holds the socket, connections to its
/// address are refused, and no other test can be given its port.
fn hold_a_tcp_address_that_refuses_connections() -> tokio::net::TcpSocket {
    let socket = tokio::net::TcpSocket::new_v4().expect("a TCP socket should be created");
    socket
        .bind(std::net::SocketAddr::from((std::net::Ipv4Addr::LOCALHOST, 0)))
        .expect("the TCP socket should bind to an ephemeral port");
    socket
}

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

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_configuration::v3_0_0::public_url::HttpUrl;
    use torrust_tracker_primitives::ServiceRole;
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
    async fn it_should_return_error_when_the_registered_api_service_refuses_the_check() {
        logging::setup();

        // Arrange
        let refusing_socket = super::hold_a_tcp_address_that_refuses_connections();
        let binding = refusing_socket.local_addr().unwrap();
        let env = super::start_health_check_api(
            super::registar_with_one_service(
                ServiceBinding::new(Protocol::HTTP, binding).unwrap(),
                ServiceRole::RestApi,
                torrust_tracker_axum_rest_api_server::server::check_fn,
            )
            .await,
        )
        .await;

        // Act
        let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

        // Assert
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

mod http {
    use std::sync::Arc;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::ServiceHealthCheckJob;
    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_configuration::v3_0_0::tls::TlsConfig;
    use torrust_tracker_primitives::ServiceRole;
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
    async fn it_should_return_error_when_the_registered_http_service_refuses_the_check() {
        logging::setup();

        // Arrange
        let refusing_socket = super::hold_a_tcp_address_that_refuses_connections();
        let binding = refusing_socket.local_addr().unwrap();
        let env = super::start_health_check_api(
            super::registar_with_one_service(
                ServiceBinding::new(Protocol::HTTP, binding).unwrap(),
                ServiceRole::HttpTracker,
                torrust_tracker_axum_http_server::server::check_fn,
            )
            .await,
        )
        .await;

        // Act
        let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

        // Assert
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

mod udp {
    use std::net::{SocketAddr, UdpSocket};
    use std::sync::Arc;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{Registar, ServiceRegistration};
    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_test_helpers::{configuration, logging};
    use torrust_tracker_udp_server::server::launcher::Launcher;
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
    async fn it_should_return_error_when_the_registered_udp_service_never_answers_the_check() {
        logging::setup();

        // Arrange
        // While the test holds this socket, the check's datagrams reach it and are never answered.
        let silent_socket = UdpSocket::bind("127.0.0.1:0").expect("a UDP socket should bind to an ephemeral port");
        let binding = silent_socket.local_addr().unwrap();
        let env = super::start_health_check_api(
            super::registar_with_one_service(
                ServiceBinding::new(Protocol::UDP, binding).unwrap(),
                ServiceRole::UdpTracker,
                Launcher::check,
            )
            .await,
        )
        .await;

        // Act
        let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

        // Assert
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

    /// Registers a UDP peer that answers every datagram with bytes that are not a tracker
    /// response, checked by the production UDP tracker probe.
    async fn registar_with_a_junk_answering_udp_service() -> (Registar<RuntimeServiceMetadata>, SocketAddr) {
        // A task of the test's runtime, not a detached thread: the runtime's shutdown at the end of
        // the test drops it, and with it the socket.
        let socket = tokio::net::UdpSocket::bind("127.0.0.1:0")
            .await
            .expect("the junk peer should bind");
        let binding = socket.local_addr().expect("the junk peer should have an address");
        tokio::spawn(async move {
            let mut buffer = [0u8; 2048];
            while let Ok((_, from)) = socket.recv_from(&mut buffer).await {
                drop(socket.send_to(b"junk", from).await);
            }
        });

        let registar = Registar::default();
        registar
            .give_form()
            .register(ServiceRegistration::new(
                ServiceBinding::new(Protocol::UDP, binding).expect("the junk peer address should be a valid UDP binding"),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0)),
                Some(Launcher::check),
            ))
            .await
            .expect("the junk-answering udp service should register");

        (registar, binding)
    }

    // issue: #2867
    #[tokio::test]
    async fn it_should_return_error_when_the_registered_udp_service_answers_the_check_with_junk() {
        // Arrange
        logging::setup();
        let configuration = configuration::ephemeral_with_no_services();
        let (registar, binding) = registar_with_a_junk_answering_udp_service().await;
        let env = Started::new(&configuration.health_check_api.into(), registar).await;

        // Act
        let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

        // Assert
        assert_eq!(response.status(), 200);
        let report: Report = response
            .json()
            .await
            .expect("it should be able to get the report from the json");
        assert_eq!(report.status, Status::Error);
        assert_eq!(report.message, "health check failed".to_string());
        let details = report.details.first().expect("it should have some details");
        assert_eq!(details.binding, binding);
        assert!(
            details
                .result
                .as_ref()
                .is_err_and(|error| error.contains("Unrecognized UDP tracker response")),
            "the junk-answering udp service should be a failed check naming the cause, got: {:?}",
            details.result
        );

        env.stop().await.expect("it should stop the service");
    }
}

// issue: #2697
mod readiness {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{FnSpawnServiceHeathCheck, Registar, ServiceHealthCheckJob};
    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_primitives::{RuntimeServiceMetadata, ServiceRole};
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
        super::registar_with_one_service(
            ServiceBinding::new(Protocol::HTTP, "127.0.0.1:7070".parse().unwrap()).unwrap(), // DevSkim: ignore DS137138
            ServiceRole::RestApi,
            probe,
        )
        .await
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

// issue: #2867
mod failed_probe_job {
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{FnSpawnServiceHeathCheck, Registar, ServiceHealthCheckJob, ServiceRegistration};
    use torrust_tracker_axum_health_check_api_server::environment::Started;
    use torrust_tracker_axum_health_check_api_server::resources::{Report, Status};
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_test_helpers::{configuration, logging};

    use crate::server::client::get;

    fn panicking_probe(_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        ServiceHealthCheckJob::new(
            "panicking probe".to_string(),
            tokio::spawn(async { panic!("the probe job panicked on purpose") }),
        )
    }

    fn passing_probe(_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        ServiceHealthCheckJob::new("passing probe".to_string(), tokio::spawn(async { Ok("200 OK".to_string()) }))
    }

    async fn register(
        registar: &Registar<RuntimeServiceMetadata>,
        address: &str,
        role: ServiceRole,
        probe: FnSpawnServiceHeathCheck,
    ) {
        registar
            .give_form()
            .register(ServiceRegistration::new(
                ServiceBinding::new(Protocol::HTTP, address.parse().unwrap()).unwrap(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(role, 0)),
                Some(probe),
            ))
            .await
            .expect("the probed service should register");
    }

    #[tokio::test]
    async fn it_should_report_a_probe_job_that_panicked_as_a_failed_check_and_keep_the_other_results() {
        // Arrange: one service whose probe job panics, next to one whose probe passes.
        logging::setup();
        let configuration = configuration::ephemeral_with_no_services();
        let registar = Registar::default();
        register(&registar, "127.0.0.1:7070", ServiceRole::HttpTracker, panicking_probe).await; // DevSkim: ignore DS137138
        register(&registar, "127.0.0.1:1212", ServiceRole::RestApi, passing_probe).await; // DevSkim: ignore DS137138
        let env = Started::new(&configuration.health_check_api.into(), registar).await;

        // Act
        let response = get(&format!("http://{}/health_check", env.bind_address())).await; // DevSkim: ignore DS137138

        // Assert
        assert_eq!(response.status(), 200);
        let report: Report = response.json().await.expect("the body should be a report");
        assert_eq!(report.status, Status::Error);
        assert_eq!(report.message, "health check failed");
        let result_of = |info: &str| {
            report
                .details
                .iter()
                .find(|detail| detail.info == info)
                .unwrap_or_else(|| panic!("the report should have a detail for the {info}"))
                .result
                .clone()
        };
        let panicked = result_of("panicking probe");
        assert!(
            panicked.is_err(),
            "the panicking probe should be a failed check, got: {panicked:?}"
        );
        assert_eq!(result_of("passing probe"), Ok("200 OK".to_string()));

        env.stop().await.expect("it should stop the service");
    }
}
