//! Module to handle the HTTP server instances.
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum_server::Handle;
use axum_server::tls_rustls::RustlsConfig;
use derive_more::Constructor;
use futures::future::BoxFuture;
use socket2::{Domain, Socket, Type};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_server_lib::logging::STARTED_ON;
use torrust_server_lib::registar::{
    FnSpawnServiceHeathCheck, ServiceHealthCheckJob, ServiceRegistration, ServiceRegistrationForm,
};
use torrust_tracker_axum_server::custom_axum_server;
use torrust_tracker_axum_server::signals::{GracefulShutdownOutcome, graceful_shutdown_on_cancellation};
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::RuntimeServiceMetadata;

use super::v1::routes::router;
use crate::HTTP_TRACKER_LOG_TARGET;

/// Error that can occur when starting the HTTP server: the listener cannot be
/// bound or configured, or the service cannot be registered.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("could not bind HTTP tracker listener: {source}")]
    Bind { source: std::io::Error },

    #[error("could not configure HTTP tracker listener: {source}")]
    Listener { source: std::io::Error },

    #[error("could not register HTTP tracker service: {source}")]
    Registration {
        source: torrust_server_lib::registar::RegistrationError,
    },
}

// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
#[derive(Constructor, Debug)]
pub struct Launcher {
    pub bind_to: SocketAddr,
    pub tls: Option<RustlsConfig>,
    pub ipv6_v6only: bool,
    /// How long the server waits for open connections to finish after its cancellation token is
    /// cancelled, before it closes them.
    pub drain_timeout: Duration,
}

impl Launcher {
    /// Creates a [`std::net::TcpListener`] with `IPV6_V6ONLY` set according to
    /// the `ipv6_v6only` parameter.
    ///
    /// When `ipv6_v6only` is `true`, IPv6 sockets are restricted to IPv6 only,
    /// allowing a separate IPv4 socket to bind on the same port
    /// (e.g. `0.0.0.0:7070` and `[::]:7070`).
    ///
    /// When `ipv6_v6only` is `false` (the default), the socket option is
    /// **not** explicitly set — the OS default applies:
    ///
    /// | Platform | Default `IPV6_V6ONLY` | Behaviour with `false` |
    /// |---|---|---|
    /// | Linux | `0` (dual-stack) | Dual-stack — single `[::]` socket accepts IPv4 + IPv6 |
    /// | Windows, macOS, FreeBSD, Solaris | `1` (IPv6-only) | IPv6-only — must also bind `0.0.0.0:<port>` for IPv4 |
    /// | OpenBSD | `1` (forced) | IPv6-only — `IPV6_V6ONLY` cannot be disabled |
    ///
    /// We intentionally do **not** call `set_only_v6(false)` because on OpenBSD
    /// that syscall would return `EINVAL` and cause a runtime panic.
    /// # Errors
    ///
    /// Will return an error if the socket cannot be created, configured, or bound.
    fn create_tcp_listener(addr: SocketAddr, ipv6_v6only: bool) -> Result<std::net::TcpListener, std::io::Error> {
        let domain = if addr.is_ipv6() { Domain::IPV6 } else { Domain::IPV4 };
        let socket = Socket::new(domain, Type::STREAM, Some(socket2::Protocol::TCP))?;

        if addr.is_ipv6() && ipv6_v6only {
            socket.set_only_v6(true)?;
        }

        socket.set_nonblocking(true)?;
        socket.bind(&addr.into())?;
        socket.listen(1024)?;

        Ok(std::net::TcpListener::from(socket))
    }

    fn server_future(
        &self,
        socket: std::net::TcpListener,
        handle: Handle<SocketAddr>,
        http_tracker_container: &Arc<HttpTrackerCoreContainer>,
        service_binding: &ServiceBinding,
    ) -> Result<BoxFuture<'static, ()>, Error> {
        let app = router(http_tracker_container, service_binding);

        if let Some(tls) = self.tls.clone() {
            // Build listeners only through `custom_axum_server`: it applies the connection timeouts that close
            // silent connections, which protects against idle-connection floods (#2776, ADR in `docs/adrs/`).
            let server =
                custom_axum_server::from_tcp_rustls_with_timeouts(socket, tls).map_err(|source| Error::Listener { source })?;

            Ok(Box::pin(async move {
                if let Err(error) = server
                    .handle(handle)
                    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                    .await
                {
                    tracing::error!(%error, "HTTP TLS server stopped with an error");
                }
            }))
        } else {
            // Build listeners only through `custom_axum_server`: it applies the connection timeouts that close
            // silent connections, which protects against idle-connection floods (#2776, ADR in `docs/adrs/`).
            let server = custom_axum_server::from_tcp_with_timeouts(socket).map_err(|source| Error::Listener { source })?;

            Ok(Box::pin(async move {
                if let Err(error) = server
                    .handle(handle)
                    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                    .await
                {
                    tracing::error!(%error, "HTTP server stopped with an error");
                }
            }))
        }
    }
}

/// A HTTP server instance controller with no HTTP instance running.
#[allow(
    clippy::module_name_repetitions,
    reason = "type alias distinguishes the HTTP server stopped state"
)]
pub type StoppedHttpServer = HttpServer<Stopped>;

/// A HTTP server instance controller.
///
/// It's responsible for:
///
/// - Keeping the initial configuration of the server.
/// - Starting and stopping the server.
/// - Keeping the state of the server: `running` or `stopped`.
///
/// It's an state machine. Configurations cannot be changed. This struct
/// represents concrete configuration and state. It allows to start and stop the
/// server but always keeping the same configuration.
///
/// > **NOTICE**: if the configurations changes after running the server it will
/// > reset to the initial value after stopping the server. This struct is not
/// > intended to persist configurations between runs.
#[allow(
    clippy::module_name_repetitions,
    reason = "public type identifies the HTTP server controller among workspace server types"
)]
pub struct HttpServer<S> {
    /// The state of the server: `running` or `stopped`.
    pub state: S,
}

/// A stopped HTTP server state.
pub struct Stopped {
    launcher: Launcher,
}

/// A token-aware HTTP tracker runtime and its owned drain controller.
///
/// The caller must retain and join both handles.
pub struct CancellationRunning {
    pub binding: SocketAddr,
    pub task: JoinHandle<Launcher>,
    pub shutdown_controller: JoinHandle<GracefulShutdownOutcome>,
}

impl HttpServer<Stopped> {
    /// It creates a new `HttpServer` controller in `stopped` state.
    #[must_use]
    pub const fn new(launcher: Launcher) -> Self {
        Self {
            state: Stopped { launcher },
        }
    }

    /// Starts the HTTP tracker using an injected cancellation token.
    ///
    /// This path does not subscribe to operating-system signals. Its
    /// caller owns the returned runtime task and drain controller.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Bind`] or [`Error::Listener`] if the listener cannot be
    /// set up, or [`Error::Registration`] if the service cannot be registered.
    pub async fn start_with_cancellation(
        self,
        http_tracker_container: Arc<HttpTrackerCoreContainer>,
        form: ServiceRegistrationForm<RuntimeServiceMetadata>,
        metadata: RuntimeServiceMetadata,
        cancellation_token: CancellationToken,
    ) -> Result<CancellationRunning, Error> {
        self.start_with_cancellation_and_health_check(http_tracker_container, form, metadata, cancellation_token, check_fn)
            .await
    }

    /// Starts the HTTP tracker using an injected cancellation token and
    /// registers the supplied health-check callback.
    ///
    /// The application uses [`check_fn`] through [`Self::start_with_cancellation`];
    /// this seam lets integration tests use a client that trusts their test
    /// certificate. It does not subscribe to operating-system signals; the caller
    /// owns the returned runtime task and drain controller.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Bind`] or [`Error::Listener`] if the listener cannot be
    /// set up, or [`Error::Registration`] if the service cannot be registered.
    pub async fn start_with_cancellation_and_health_check(
        self,
        http_tracker_container: Arc<HttpTrackerCoreContainer>,
        form: ServiceRegistrationForm<RuntimeServiceMetadata>,
        metadata: RuntimeServiceMetadata,
        cancellation_token: CancellationToken,
        health_check: FnSpawnServiceHeathCheck,
    ) -> Result<CancellationRunning, Error> {
        let (running, service_binding) =
            start_token_aware_runtime(self.state.launcher, &http_tracker_container, cancellation_token.clone())?;

        if let Some(public_url) = metadata.public_url() {
            tracing::info!(service_binding = %service_binding, public_url = %public_url, "Started HTTP tracker");
        } else {
            tracing::info!(service_binding = %service_binding, "Started HTTP tracker");
        }

        if let Err(source) = form
            .register(ServiceRegistration::new(service_binding, metadata, Some(health_check)))
            .await
        {
            let CancellationRunning {
                task,
                shutdown_controller,
                ..
            } = running;
            cancellation_token.cancel();
            task.abort();
            shutdown_controller.abort();
            drop(task.await);
            drop(shutdown_controller.await);
            return Err(Error::Registration { source });
        }

        Ok(running)
    }
}

fn start_token_aware_runtime(
    launcher: Launcher,
    http_tracker_container: &Arc<HttpTrackerCoreContainer>,
    cancellation_token: CancellationToken,
) -> Result<(CancellationRunning, ServiceBinding), Error> {
    let socket =
        Launcher::create_tcp_listener(launcher.bind_to, launcher.ipv6_v6only).map_err(|source| Error::Bind { source })?;
    let binding = socket.local_addr().map_err(|source| Error::Listener { source })?;
    let handle = Handle::new();
    let protocol = if launcher.tls.is_some() {
        Protocol::HTTPS
    } else {
        Protocol::HTTP
    };
    let service_binding = ServiceBinding::new(protocol.clone(), binding).map_err(|error| Error::Listener {
        source: std::io::Error::other(error),
    })?;
    let server = launcher.server_future(socket, handle.clone(), http_tracker_container, &service_binding)?;
    let drain_timeout = launcher.drain_timeout;

    tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Starting on: {protocol}://{binding}");
    tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "{STARTED_ON}: {protocol}://{binding}");
    let task = tokio::spawn(async move {
        server.await;
        launcher
    });
    let shutdown_controller = tokio::spawn(graceful_shutdown_on_cancellation(
        handle,
        cancellation_token,
        format!("Shutting down HTTP server on socket address: {binding}"),
        binding,
        drain_timeout,
    ));

    Ok((
        CancellationRunning {
            binding,
            task,
            shutdown_controller,
        },
        service_binding,
    ))
}

/// Checks the Health by connecting to the HTTP tracker endpoint.
///
/// # Errors
///
/// This function will return an error if unable to connect.
/// Or if the request returns a status that is not a success (2xx).
#[must_use]
pub fn check_fn(service_binding: &ServiceBinding) -> ServiceHealthCheckJob {
    check_fn_with_client(service_binding, reqwest::Client::new())
}

/// Checks a tracker health endpoint using the supplied HTTP client.
///
/// This preserves normal production certificate validation when called from
/// [`check_fn`] and allows integration tests to trust a known test certificate.
#[must_use]
pub fn check_fn_with_client(service_binding: &ServiceBinding, client: reqwest::Client) -> ServiceHealthCheckJob {
    let url = health_check_url(service_binding);

    let info = format!("checking http tracker health check at: {url}");

    let job = tokio::spawn(async move {
        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => Ok(response.status().to_string()),
            Ok(response) => Err(response.status().to_string()),
            Err(err) => Err(err.to_string()),
        }
    });

    ServiceHealthCheckJob::new(info, job)
}

fn health_check_url(service_binding: &ServiceBinding) -> String {
    service_binding
        .url()
        .join("health_check")
        .expect("Service binding URL can always resolve a health check path")
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, TcpListener};
    use std::sync::Arc;
    use std::time::Duration;

    use rustls::pki_types::pem::PemObject;
    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{
        Registar, RegisteredService, RegistrationError, ServiceHealthCheckJob, ServiceRegistration, ServiceRegistrationForm,
    };
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_axum_server::tls::make_rust_tls;
    use torrust_tracker_configuration::v3_0_0::shutdown::Shutdown;
    use torrust_tracker_configuration::v3_0_0::{Configuration, logging};
    use torrust_tracker_core::container::TrackerCoreContainer;
    use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
    use torrust_tracker_http_core::event::bus::EventBus;
    use torrust_tracker_http_core::event::sender::Broadcaster;
    use torrust_tracker_http_core::services::announce::AnnounceService;
    use torrust_tracker_http_core::services::scrape::ScrapeService;
    use torrust_tracker_http_core::statistics::event::listener::{DEFAULT_DRAIN_TIMEOUT, run_event_listener};
    use torrust_tracker_http_core::statistics::repository::Repository;
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use crate::server::{Error, HttpServer, Launcher, check_fn_with_client, health_check_url};

    #[test]
    fn it_should_build_a_health_check_url_using_the_service_binding_protocol() {
        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 7070));

        for (protocol, expected_url) in [
            (Protocol::HTTP, "http://127.0.0.1:7070/health_check"),
            (Protocol::HTTPS, "https://127.0.0.1:7070/health_check"),
        ] {
            let service_binding = ServiceBinding::new(protocol, address).expect("service binding should be valid");

            assert_eq!(health_check_url(&service_binding), expected_url);
        }
    }

    #[test]
    fn it_should_return_a_typed_bind_error_when_the_listener_address_is_already_in_use() {
        // Arrange
        let occupied_listener = std::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("occupy a local TCP port");
        let occupied_address = occupied_listener.local_addr().expect("read occupied listener address");

        // Act
        let result = Launcher::create_tcp_listener(occupied_address, false).map_err(|source| Error::Bind { source });

        // Assert
        assert!(matches!(result, Err(Error::Bind { .. })));
    }

    pub async fn initialize_container(configuration: &Configuration) -> HttpTrackerCoreContainer {
        let cancellation_token = CancellationToken::new();

        let core_config = Arc::new(configuration.core.clone());

        let http_trackers = configuration
            .http_trackers
            .clone()
            .expect("missing HTTP trackers configuration");

        let http_tracker_config = &http_trackers[0];

        let http_tracker_config = Arc::new(http_tracker_config.clone());
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);

        // HTTP core stats
        let http_core_broadcaster = Broadcaster::default();
        let http_stats_repository = Arc::new(Repository::new());
        let http_stats_event_bus = Arc::new(EventBus::new(
            configuration.core.tracker_usage_statistics.into(),
            http_core_broadcaster.clone(),
        ));

        let http_stats_event_sender = http_stats_event_bus.sender();

        if configuration.core.tracker_usage_statistics {
            let _unused = run_event_listener(
                http_stats_event_bus.receiver(),
                cancellation_token,
                &http_stats_repository,
                [(configuration_instance_id, true)].into(),
                DEFAULT_DRAIN_TIMEOUT,
            );
        }

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            configuration.core.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                &core_config,
                &swarm_coordination_registry_container,
                core_config.database.as_ref(),
            )
            .await
            .expect("HTTP server test initialization requires persistence"),
        );

        let announce_service = Arc::new(AnnounceService::new_with_http_tracker_config(
            tracker_core_container.core_config.clone(),
            tracker_core_container.announce_handler.clone(),
            tracker_core_container.authentication_service.clone(),
            tracker_core_container.whitelist_authorization.clone(),
            http_stats_event_sender.clone(),
            &http_tracker_config,
            configuration_instance_id,
        ));

        let scrape_service = Arc::new(ScrapeService::new_with_http_tracker_config(
            tracker_core_container.core_config.clone(),
            tracker_core_container.scrape_handler.clone(),
            tracker_core_container.authentication_service.clone(),
            http_stats_event_sender.clone(),
            &http_tracker_config,
            configuration_instance_id,
        ));

        HttpTrackerCoreContainer {
            tracker_core_container,
            http_tracker_config,
            event_bus: http_stats_event_bus,
            stats_event_sender: http_stats_event_sender,
            stats_repository: http_stats_repository,
            announce_service,
            scrape_service,
        }
    }

    fn initialize_global_services(configuration: &Configuration) {
        initialize_static();
        logging::setup(&configuration.logging);
    }

    fn initialize_static() {
        torrust_clock::initialize_static();
    }

    /// A server-start scenario whose HTTP binding is available to the OS but already registered.
    struct ServerStartWithDuplicateRegistration {
        bind_to: SocketAddr,
        configuration: Arc<Configuration>,
        configuration_instance_id: ConfigurationInstanceId,
        registar: Registar<RuntimeServiceMetadata>,
    }

    impl ServerStartWithDuplicateRegistration {
        async fn new() -> Self {
            let bind_to = Self::available_http_bind_address();
            let configuration = Self::configuration_bound_to(bind_to);
            let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
            let registar = Registar::default();

            Self::pre_register_http_binding(&registar, bind_to, configuration_instance_id).await;

            Self {
                bind_to,
                configuration,
                configuration_instance_id,
                registar,
            }
        }

        fn available_http_bind_address() -> SocketAddr {
            // Registration must fail after the server has bound this address, so this listener is
            // released before start. Do not add retries or waits: they would hide this OS-level handoff
            // risk instead of preserving the cleanup contract under test.
            let available_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("select available HTTP listener address");
            let bind_to = available_listener.local_addr().expect("read available listener address");
            drop(available_listener);

            bind_to
        }

        fn configuration_bound_to(bind_to: SocketAddr) -> Arc<Configuration> {
            let mut configuration = ephemeral_public();
            configuration.http_trackers.as_mut().expect("test configuration enables HTTP")[0].bind_address = bind_to;

            Arc::new(configuration)
        }

        async fn pre_register_http_binding(
            registar: &Registar<RuntimeServiceMetadata>,
            bind_to: SocketAddr,
            configuration_instance_id: ConfigurationInstanceId,
        ) {
            let service_binding = ServiceBinding::new(Protocol::HTTP, bind_to).expect("HTTP service binding should be valid");

            registar
                .give_form()
                .register(ServiceRegistration::new(
                    service_binding,
                    RuntimeServiceMetadata::new(configuration_instance_id),
                    None,
                ))
                .await
                .expect("reserve the HTTP service registration");
        }

        async fn container(&self) -> Arc<HttpTrackerCoreContainer> {
            initialize_global_services(&self.configuration);
            Arc::new(initialize_container(&self.configuration).await)
        }

        fn launcher(&self) -> Launcher {
            let http_tracker_config = &self
                .configuration
                .http_trackers
                .as_ref()
                .expect("test configuration enables HTTP")[0];

            Launcher::new(
                self.bind_to,
                None,
                http_tracker_config.network.ipv6_v6only,
                Shutdown::default().http_drain(),
            )
        }

        fn registration_form(&self) -> ServiceRegistrationForm<RuntimeServiceMetadata> {
            self.registar.give_form()
        }

        fn metadata(&self) -> RuntimeServiceMetadata {
            RuntimeServiceMetadata::new(self.configuration_instance_id)
        }
    }

    /// A server-start scenario whose HTTP binding is available to both the OS and the registry.
    struct ServerStartWithAvailableHttpBinding {
        bind_to: SocketAddr,
        configuration: Arc<Configuration>,
        configuration_instance_id: ConfigurationInstanceId,
        registar: Registar<RuntimeServiceMetadata>,
    }

    impl ServerStartWithAvailableHttpBinding {
        fn new() -> Self {
            let configuration = Arc::new(ephemeral_public());
            let bind_to = configuration
                .http_trackers
                .as_ref()
                .expect("missing HTTP trackers configuration")[0]
                .bind_address;

            Self {
                bind_to,
                configuration,
                configuration_instance_id: ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
                registar: Registar::default(),
            }
        }

        async fn container(&self) -> Arc<HttpTrackerCoreContainer> {
            initialize_global_services(&self.configuration);
            Arc::new(initialize_container(&self.configuration).await)
        }

        async fn launcher(&self) -> Launcher {
            let http_tracker_config = &self
                .configuration
                .http_trackers
                .as_ref()
                .expect("missing HTTP trackers configuration")[0];
            let tls = if let Some(tls_config) = &http_tracker_config.tls_config {
                Some(make_rust_tls(tls_config).await.expect("tls config failed"))
            } else {
                None
            };

            Launcher::new(
                self.bind_to,
                tls,
                http_tracker_config.network.ipv6_v6only,
                Shutdown::default().http_drain(),
            )
        }

        fn registration_form(&self) -> ServiceRegistrationForm<RuntimeServiceMetadata> {
            self.registar.give_form()
        }

        fn metadata(&self) -> RuntimeServiceMetadata {
            RuntimeServiceMetadata::new(self.configuration_instance_id)
        }
    }

    #[tokio::test]
    async fn it_should_drain_the_token_aware_http_server_when_its_cancellation_token_is_cancelled() {
        // Arrange
        let scenario = ServerStartWithAvailableHttpBinding::new();
        let http_tracker_container = scenario.container().await;
        let cancellation_token = CancellationToken::new();
        let running = HttpServer::new(scenario.launcher().await)
            .start_with_cancellation(
                http_tracker_container,
                scenario.registration_form(),
                scenario.metadata(),
                cancellation_token.clone(),
            )
            .await
            .expect("the token-aware HTTP server should start");

        // Act
        cancellation_token.cancel();
        let launcher = running.task.await.expect("the HTTP server task should not panic");
        let drain_outcome = running
            .shutdown_controller
            .await
            .expect("the HTTP drain controller should not panic");

        // Assert
        assert_eq!(launcher.bind_to, scenario.bind_to);
        assert_eq!(drain_outcome, GracefulShutdownOutcome::Drained);
    }

    /// A client that connects and sends nothing must be disconnected. An idle connection would
    /// hold a task and a file descriptor for as long as the client keeps it open, which is how a
    /// flood of silent connections exhausts the process (issue #2776, EPIC #2411 case A2).
    ///
    /// The listener uses the production first-request timeout (5 s), so the bound is an absolute,
    /// generous deadline instead of an open-ended wait.
    const SILENT_CONNECTION_DEADLINE: Duration = Duration::from_secs(8);

    async fn assert_closed_when_silent(mut stream: impl tokio::io::AsyncRead + Unpin, what: &str) {
        use tokio::io::AsyncReadExt;

        let mut buffer = [0_u8; 16];
        let closed = tokio::time::timeout(SILENT_CONNECTION_DEADLINE, stream.read(&mut buffer)).await;

        assert!(
            matches!(closed, Ok(Ok(0) | Err(_))),
            "the HTTP tracker should close {what}: {closed:?}"
        );
    }

    #[tokio::test]
    async fn it_should_close_a_plaintext_connection_that_sends_nothing() {
        // Arrange
        let scenario = ServerStartWithAvailableHttpBinding::new();
        let cancellation_token = CancellationToken::new();
        let running = HttpServer::new(scenario.launcher().await)
            .start_with_cancellation(
                scenario.container().await,
                scenario.registration_form(),
                scenario.metadata(),
                cancellation_token.clone(),
            )
            .await
            .expect("the token-aware HTTP server should start");
        // Cancels the server even if an assertion below panics, so no task outlives the test.
        let _cancel_on_drop = cancellation_token.clone().drop_guard();
        let silent_connection = tokio::net::TcpStream::connect(running.binding)
            .await
            .expect("open the silent connection");

        // Act and assert
        assert_closed_when_silent(silent_connection, "a plaintext connection that sends nothing").await;
    }

    #[tokio::test]
    async fn it_should_close_a_tls_connection_that_completes_the_handshake_and_sends_nothing() {
        // Arrange
        rustls::crypto::ring::default_provider().install_default().ok();
        let scenario = ServerStartWithAvailableHttpBinding::new();
        let cancellation_token = CancellationToken::new();
        let mut launcher = scenario.launcher().await;
        launcher.tls = Some(
            axum_server::tls_rustls::RustlsConfig::from_pem(TEST_CERTIFICATE.to_vec(), TEST_KEY.to_vec())
                .await
                .expect("the test certificate and key should load"),
        );
        let running = HttpServer::new(launcher)
            .start_with_cancellation(
                scenario.container().await,
                scenario.registration_form(),
                scenario.metadata(),
                cancellation_token.clone(),
            )
            .await
            .expect("the token-aware HTTP server should start with TLS");
        // Cancels the server even if an assertion below panics, so no task outlives the test.
        let _cancel_on_drop = cancellation_token.clone().drop_guard();
        let tcp = tokio::net::TcpStream::connect(running.binding)
            .await
            .expect("open the connection");
        let mut roots = rustls::RootCertStore::empty();
        roots
            .add(rustls::pki_types::CertificateDer::from_pem_slice(TEST_CERTIFICATE).expect("the test certificate should parse"))
            .expect("the test certificate should be a valid root");
        let client_config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let server_name = rustls::pki_types::ServerName::try_from("127.0.0.1").expect("an IP address is a valid server name");
        let silent_connection = tokio_rustls::TlsConnector::from(Arc::new(client_config))
            .connect(server_name, tcp)
            .await
            .expect("the TLS handshake should complete");

        // Act and assert
        assert_closed_when_silent(silent_connection, "a TLS connection that sends nothing after the handshake").await;
    }

    // Embedded at compile time: the container test stage runs the tests from a different directory than the build stage.
    const TEST_CERTIFICATE: &[u8] = include_bytes!("../tests/fixtures/https-health-check-cert.pem");
    const TEST_KEY: &[u8] = include_bytes!("../tests/fixtures/https-health-check-key.pem");

    #[tokio::test]
    async fn it_should_time_out_the_drain_after_the_configured_drain_timeout_when_a_connection_stays_open() {
        // Arrange
        // The launcher is configured with a 300 ms drain timeout, far below the 15 s default. A client
        // opens a connection and sends only part of a request header, so the server keeps waiting for
        // it. A second request that completes proves the server has accepted the first connection.
        let configured_drain_timeout = Duration::from_millis(300);
        let test_deadline = Duration::from_secs(5);
        let scenario = ServerStartWithAvailableHttpBinding::new();
        let http_tracker_container = scenario.container().await;
        let cancellation_token = CancellationToken::new();
        let mut launcher = scenario.launcher().await;
        launcher.drain_timeout = configured_drain_timeout;
        let running = HttpServer::new(launcher)
            .start_with_cancellation(
                http_tracker_container,
                scenario.registration_form(),
                scenario.metadata(),
                cancellation_token.clone(),
            )
            .await
            .expect("the token-aware HTTP server should start");
        let mut held_connection = std::net::TcpStream::connect(running.binding).expect("open the connection to hold");
        std::io::Write::write_all(&mut held_connection, b"GET /health_check HTTP/1.1\r\nHost: tracker\r\n")
            .expect("send a partial request header");
        tokio::time::timeout(
            test_deadline,
            reqwest::get(format!("http://{}/health_check", running.binding)),
        )
        .await
        .expect("the server should answer within the test deadline")
        .expect("the server should answer once it has accepted the held connection");

        // Act
        cancellation_token.cancel();
        let drain_outcome = tokio::time::timeout(test_deadline, running.shutdown_controller)
            .await
            .expect("the drain should end by the configured timeout, not the default one")
            .expect("the HTTP drain controller should not panic");

        // Assert
        assert_eq!(
            drain_outcome,
            GracefulShutdownOutcome::TimedOut,
            "a connection held open past the {configured_drain_timeout:?} drain timeout should end the drain as timed out"
        );
        drop(held_connection);
    }

    const INJECTED_HEALTH_CHECK_RESULT: &str = "injected health check ran";

    fn injected_health_check(_service_binding: &ServiceBinding) -> ServiceHealthCheckJob {
        ServiceHealthCheckJob::new(
            "injected health check".to_owned(),
            tokio::spawn(async { Ok(INJECTED_HEALTH_CHECK_RESULT.to_owned()) }),
        )
    }

    #[tokio::test]
    async fn it_should_register_the_injected_health_check_when_starting_the_token_aware_http_server() {
        // Arrange
        let scenario = ServerStartWithAvailableHttpBinding::new();
        let http_tracker_container = scenario.container().await;
        let cancellation_token = CancellationToken::new();

        // Act
        let running = HttpServer::new(scenario.launcher().await)
            .start_with_cancellation_and_health_check(
                http_tracker_container,
                scenario.registration_form(),
                scenario.metadata(),
                cancellation_token.clone(),
                injected_health_check,
            )
            .await
            .expect("the token-aware HTTP server should start");

        // Assert
        let registered_services = scenario.registar.services().await;
        let health_check = registered_services
            .first()
            .and_then(RegisteredService::spawn_check)
            .expect("the started HTTP server should register a health check");
        assert_eq!(
            health_check.job.await.expect("the health-check job should not panic"),
            Ok(INJECTED_HEALTH_CHECK_RESULT.to_owned()),
            "the registry should run the injected callback, not the default HTTP check"
        );

        cancellation_token.cancel();
        running.task.await.expect("the HTTP server task should not panic");
        running
            .shutdown_controller
            .await
            .expect("the HTTP drain controller should not panic");
    }

    #[tokio::test]
    async fn it_should_release_the_listener_when_token_aware_startup_registration_fails() {
        // Arrange
        let scenario = ServerStartWithDuplicateRegistration::new().await;
        let http_tracker_container = scenario.container().await;

        // Act
        let result = HttpServer::new(scenario.launcher())
            .start_with_cancellation(
                http_tracker_container,
                scenario.registration_form(),
                scenario.metadata(),
                CancellationToken::new(),
            )
            .await;

        // Assert
        let binding = match result {
            Err(Error::Registration {
                source: RegistrationError::DuplicateBinding(binding),
            }) => binding,
            Err(error) => panic!("token-aware starter should retain the registration failure source: {error}"),
            Ok(_) => panic!("duplicate registration should fail"),
        };
        assert_eq!(binding.bind_address(), scenario.bind_to);
        TcpListener::bind(scenario.bind_to).expect("HTTP listener should be released after token-aware registration failure");
    }

    /// Serves a canned HTTP response to one connection on a local port, then stops and returns its address.
    fn serve_status(status_line: &'static str) -> SocketAddr {
        let listener = std::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("bind a local test server");
        let address = listener.local_addr().expect("read the test server address");
        std::thread::spawn(move || {
            let (mut stream, _peer) = listener.accept().expect("accept a connection");
            let mut buffer = [0_u8; 1024];
            drop(std::io::Read::read(&mut stream, &mut buffer));
            let response = format!("HTTP/1.1 {status_line}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
            drop(std::io::Write::write_all(&mut stream, response.as_bytes()));
        });
        address
    }

    async fn run_health_check(status_line: &'static str) -> Result<String, String> {
        let address = serve_status(status_line);
        let service_binding = ServiceBinding::new(Protocol::HTTP, address).expect("service binding should be valid");

        let client = reqwest::Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("client should build");

        tokio::time::timeout(Duration::from_secs(10), check_fn_with_client(&service_binding, client).job)
            .await
            .expect("the health check job should finish before the test deadline")
            .expect("the health check job should not panic")
    }

    #[tokio::test]
    async fn it_should_report_ok_when_the_health_endpoint_answers_a_success_status() {
        assert_eq!(run_health_check("200 OK").await, Ok("200 OK".to_string()));
    }

    #[tokio::test]
    async fn it_should_report_an_error_with_the_status_when_the_health_endpoint_answers_a_failing_status() {
        assert_eq!(
            run_health_check("500 Internal Server Error").await,
            Err("500 Internal Server Error".to_string())
        );
    }
}
