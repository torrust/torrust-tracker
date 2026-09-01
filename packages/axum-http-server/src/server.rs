//! Module to handle the HTTP server instances.
use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::Handle;
use axum_server::tls_rustls::RustlsConfig;
use derive_more::Constructor;
use futures::future::BoxFuture;
use socket2::{Domain, Socket, Type};
use tokio::sync::oneshot::{Receiver, Sender};
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_server_lib::logging::STARTED_ON;
use torrust_server_lib::registar::{
    FnSpawnServiceHeathCheck, ServiceHealthCheckJob, ServiceRegistration, ServiceRegistrationForm,
};
use torrust_server_lib::signals::{Halted, Started};
use torrust_tracker_axum_server::custom_axum_server::{self, TimeoutAcceptor};
use torrust_tracker_axum_server::signals::graceful_shutdown;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use tracing::instrument;

use super::v1::routes::router;
use crate::HTTP_TRACKER_LOG_TARGET;

/// Error that can occur when starting or stopping the HTTP server.
///
/// Some errors triggered while starting the server are:
///
/// - The spawned server cannot send its `SocketAddr` back to the main thread.
/// - The launcher cannot receive the `SocketAddr` from the spawned server.
///
/// Some errors triggered while stopping the server are:
///
/// - The channel to send the shutdown signal to the server is closed.
/// - The task to shutdown the server on the spawned server failed to execute to
///   completion.
#[derive(Debug)]
pub enum Error {
    Error(String),
}

// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(clippy::redundant_field_names)]
#[derive(Constructor, Debug)]
pub struct Launcher {
    pub bind_to: SocketAddr,
    pub tls: Option<RustlsConfig>,
    pub ipv6_v6only: bool,
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
    fn create_tcp_listener(addr: SocketAddr, ipv6_v6only: bool) -> Result<std::net::TcpListener, Box<dyn std::error::Error>> {
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

    #[instrument(skip(self, http_tracker_container, tx_start, rx_halt))]
    fn start(
        &self,
        http_tracker_container: &Arc<HttpTrackerCoreContainer>,
        tx_start: Sender<Started>,
        rx_halt: Receiver<Halted>,
    ) -> BoxFuture<'static, ()> {
        let socket = Self::create_tcp_listener(self.bind_to, self.ipv6_v6only).expect("Could not create TCP listener.");
        let address = socket.local_addr().expect("Could not get local_addr from tcp_listener.");

        let handle = Handle::new();

        tokio::task::spawn(graceful_shutdown(
            handle.clone(),
            rx_halt,
            format!("Shutting down HTTP server on socket address: {address}"),
            address,
        ));

        let tls = self.tls.clone();
        let protocol = if tls.is_some() { Protocol::HTTPS } else { Protocol::HTTP };
        let service_binding = ServiceBinding::new(protocol.clone(), address).expect("Service binding creation failed");

        tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "Starting on: {protocol}://{address}");

        let app = router(http_tracker_container, &service_binding);

        let running = Box::pin(async {
            match tls {
                Some(tls) => custom_axum_server::from_tcp_rustls_with_timeouts(socket, tls)
                    .expect("Failed to create server from TCP socket with TLS")
                    .handle(handle)
                    // The TimeoutAcceptor is commented because TLS does not work with it.
                    // See: https://github.com/torrust/torrust-index/issues/204#issuecomment-2115529214
                    //.acceptor(TimeoutAcceptor)
                    .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
                    .await
                    .expect("Axum server crashed."),
                None => custom_axum_server::from_tcp_with_timeouts(socket)
                    .expect("Failed to create server from TCP socket")
                    .handle(handle)
                    .acceptor(TimeoutAcceptor)
                    .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
                    .await
                    .expect("Axum server crashed."),
            }
        });

        tracing::info!(target: HTTP_TRACKER_LOG_TARGET, "{STARTED_ON}: {protocol}://{}", address);

        tx_start
            .send(Started {
                service_binding,
                address,
            })
            .expect("the HTTP(s) Tracker service should not be dropped");

        running
    }
}

/// A HTTP server instance controller with no HTTP instance running.
#[allow(clippy::module_name_repetitions)]
pub type StoppedHttpServer = HttpServer<Stopped>;

/// A HTTP server instance controller with a running HTTP instance.
#[allow(clippy::module_name_repetitions)]
pub type RunningHttpServer = HttpServer<Running>;

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
#[allow(clippy::module_name_repetitions)]
pub struct HttpServer<S> {
    /// The state of the server: `running` or `stopped`.
    pub state: S,
}

/// A stopped HTTP server state.
pub struct Stopped {
    launcher: Launcher,
}

/// A running HTTP server state.
pub struct Running {
    /// The address where the server is bound.
    pub binding: SocketAddr,
    pub halt_task: tokio::sync::oneshot::Sender<Halted>,
    pub task: tokio::task::JoinHandle<Launcher>,
}

impl HttpServer<Stopped> {
    /// It creates a new `HttpServer` controller in `stopped` state.
    #[must_use]
    pub fn new(launcher: Launcher) -> Self {
        Self {
            state: Stopped { launcher },
        }
    }

    /// It starts the server and returns a `HttpServer` controller in `running`
    /// state.
    ///
    /// # Errors
    ///
    /// It would return an error if no `SocketAddr` is returned after launching the server.
    ///
    /// # Panics
    ///
    /// It would panic spawned HTTP server launcher cannot send the bound `SocketAddr`
    /// back to the main thread.
    #[instrument(
        skip(self, http_tracker_container, form, metadata),
        fields(
            service_role = metadata.service_role().as_str(),
            instance_index = metadata.configuration_instance_id().instance_index(),
        )
    )]
    pub async fn start(
        self,
        http_tracker_container: Arc<HttpTrackerCoreContainer>,
        form: ServiceRegistrationForm<RuntimeServiceMetadata>,
        metadata: RuntimeServiceMetadata,
    ) -> Result<HttpServer<Running>, Error> {
        self.start_with_health_check(http_tracker_container, form, metadata, check_fn)
            .await
    }

    /// Starts the server and registers the supplied health-check callback.
    ///
    /// The application uses [`check_fn`]. This explicit callback seam lets
    /// integration tests use a client that trusts their test certificate
    /// without altering production certificate validation.
    ///
    /// # Errors
    ///
    /// Returns an error if no `SocketAddr` is returned after launching the
    /// server.
    ///
    /// # Panics
    ///
    /// Panics if the spawned HTTP server launcher cannot send its bound
    /// `SocketAddr` back to the main thread, or if service registration fails.
    pub async fn start_with_health_check(
        self,
        http_tracker_container: Arc<HttpTrackerCoreContainer>,
        form: ServiceRegistrationForm<RuntimeServiceMetadata>,
        metadata: RuntimeServiceMetadata,
        health_check: FnSpawnServiceHeathCheck,
    ) -> Result<HttpServer<Running>, Error> {
        let (tx_start, rx_start) = tokio::sync::oneshot::channel::<Started>();
        let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

        let launcher = self.state.launcher;

        let task = tokio::spawn(async move {
            let server = launcher.start(&http_tracker_container, tx_start, rx_halt);

            server.await;

            launcher
        });

        let started = rx_start.await.expect("it should be able to start the service");

        let service_binding = started.service_binding;
        let binding = started.address;

        if let Some(public_url) = metadata.public_url() {
            tracing::info!(service_binding = %service_binding, public_url = %public_url, "Started HTTP tracker");
        } else {
            tracing::info!(service_binding = %service_binding, "Started HTTP tracker");
        }

        form.register(ServiceRegistration::new(service_binding, metadata, Some(health_check)))
            .await
            .expect("it should be able to register the started service");

        Ok(HttpServer {
            state: Running {
                binding,
                halt_task: tx_halt,
                task,
            },
        })
    }
}

impl HttpServer<Running> {
    /// It stops the server and returns a `HttpServer` controller in `stopped`
    /// state.
    ///
    /// # Errors
    ///
    /// It would return an error if the channel for the task killer signal was closed.
    pub async fn stop(self) -> Result<HttpServer<Stopped>, Error> {
        self.state
            .halt_task
            .send(Halted::Normal)
            .map_err(|_| Error::Error("Task killer channel was closed.".to_string()))?;

        let launcher = self.state.task.await.map_err(|e| Error::Error(e.to_string()))?;

        Ok(HttpServer {
            state: Stopped { launcher },
        })
    }
}

/// Checks the Health by connecting to the HTTP tracker endpoint.
///
/// # Errors
///
/// This function will return an error if unable to connect.
/// Or if the request returns an error.
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
            Ok(response) => Ok(response.status().to_string()),
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
    use std::net::{Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::Registar;
    use torrust_tracker_axum_server::tls::make_rust_tls;
    use torrust_tracker_configuration::v3_0_0::{Configuration, logging};
    use torrust_tracker_core::container::TrackerCoreContainer;
    use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
    use torrust_tracker_http_core::event::bus::EventBus;
    use torrust_tracker_http_core::event::sender::Broadcaster;
    use torrust_tracker_http_core::services::announce::AnnounceService;
    use torrust_tracker_http_core::services::scrape::ScrapeService;
    use torrust_tracker_http_core::statistics::event::listener::run_event_listener;
    use torrust_tracker_http_core::statistics::repository::Repository;
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use crate::server::{HttpServer, Launcher, health_check_url};

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

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let configuration = Arc::new(ephemeral_public());

        let http_trackers = configuration
            .http_trackers
            .clone()
            .expect("missing HTTP trackers configuration");

        let http_tracker_config = &http_trackers[0];

        initialize_global_services(&configuration);

        let http_tracker_container = Arc::new(initialize_container(&configuration).await);

        let bind_to = http_tracker_config.bind_address;

        let tls = if let Some(tls_config) = &http_tracker_config.tls_config {
            Some(make_rust_tls(tls_config).await.expect("tls config failed"))
        } else {
            None
        };

        let register = &Registar::default();
        let stopped = HttpServer::new(Launcher::new(bind_to, tls, http_tracker_config.network.ipv6_v6only));

        let started = stopped
            .start(
                http_tracker_container,
                register.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0)),
            )
            .await
            .expect("it should start the server");
        let stopped = started.stop().await.expect("it should stop the server");

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }
}
