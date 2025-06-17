//! Module to handle the HTTP server instances.
use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
use derive_more::Constructor;
use futures::future::BoxFuture;
use tokio::sync::oneshot::{Receiver, Sender};
use torrust_axum_server::custom_axum_server::{self, TimeoutAcceptor};
use torrust_axum_server::signals::graceful_shutdown;
use torrust_server_lib::logging::STARTED_ON;
use torrust_server_lib::registar::{ServiceHealthCheckJob, ServiceRegistration, ServiceRegistrationForm};
use torrust_server_lib::signals::{Halted, Started};
use torrust_tracker_primitives::service_binding::{Protocol, ServiceBinding};
use tracing::instrument;

use super::v1::routes::router;
use crate::HTTP_TRACKER_LOG_TARGET;

const TYPE_STRING: &str = "http_tracker";
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

#[derive(Constructor, Debug)]
pub struct Launcher {
    pub bind_to: SocketAddr,
    pub tls: Option<RustlsConfig>,
}

impl Launcher {
    #[instrument(skip(self, http_tracker_container, tx_start, rx_halt))]
    fn start(
        &self,
        http_tracker_container: &Arc<HttpTrackerCoreContainer>,
        tx_start: Sender<Started>,
        rx_halt: Receiver<Halted>,
    ) -> BoxFuture<'static, ()> {
        let socket = std::net::TcpListener::bind(self.bind_to).expect("Could not bind tcp_listener to address.");
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
                    .handle(handle)
                    // The TimeoutAcceptor is commented because TSL does not work with it.
                    // See: https://github.com/torrust/torrust-index/issues/204#issuecomment-2115529214
                    //.acceptor(TimeoutAcceptor)
                    .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
                    .await
                    .expect("Axum server crashed."),
                None => custom_axum_server::from_tcp_with_timeouts(socket)
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
    pub async fn start(
        self,
        http_tracker_container: Arc<HttpTrackerCoreContainer>,
        form: ServiceRegistrationForm,
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

        let listen_url = started.service_binding;
        let binding = started.address;

        form.send(ServiceRegistration::new(listen_url, check_fn))
            .expect("it should be able to send service registration");

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
    let url = format!("http://{}/health_check", service_binding.bind_address()); // DevSkim: ignore DS137138

    let info = format!("checking http tracker health check at: {url}");

    let job = tokio::spawn(async move {
        match reqwest::get(url).await {
            Ok(response) => Ok(response.status().to_string()),
            Err(err) => Err(err.to_string()),
        }
    });

    ServiceHealthCheckJob::new(service_binding.clone(), info, TYPE_STRING.to_string(), job)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
    use bittorrent_http_tracker_core::event::bus::EventBus;
    use bittorrent_http_tracker_core::event::sender::Broadcaster;
    use bittorrent_http_tracker_core::services::announce::AnnounceService;
    use bittorrent_http_tracker_core::services::scrape::ScrapeService;
    use bittorrent_http_tracker_core::statistics::event::listener::run_event_listener;
    use bittorrent_http_tracker_core::statistics::repository::Repository;
    use bittorrent_tracker_core::container::TrackerCoreContainer;
    use tokio_util::sync::CancellationToken;
    use torrust_axum_server::tsl::make_rust_tls;
    use torrust_server_lib::registar::Registar;
    use torrust_tracker_configuration::{logging, Configuration};
    use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use crate::server::{HttpServer, Launcher};

    pub fn initialize_container(configuration: &Configuration) -> HttpTrackerCoreContainer {
        let cancellation_token = CancellationToken::new();

        let core_config = Arc::new(configuration.core.clone());

        let http_trackers = configuration
            .http_trackers
            .clone()
            .expect("missing HTTP trackers configuration");

        let http_tracker_config = &http_trackers[0];

        let http_tracker_config = Arc::new(http_tracker_config.clone());

        // HTTP core stats
        let http_core_broadcaster = Broadcaster::default();
        let http_stats_repository = Arc::new(Repository::new());
        let http_stats_event_bus = Arc::new(EventBus::new(
            configuration.core.tracker_usage_statistics.into(),
            http_core_broadcaster.clone(),
        ));

        let http_stats_event_sender = http_stats_event_bus.sender();

        if configuration.core.tracker_usage_statistics {
            let _unused = run_event_listener(http_stats_event_bus.receiver(), cancellation_token, &http_stats_repository);
        }

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            configuration.core.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize_from(
            &core_config,
            &swarm_coordination_registry_container,
        ));

        let announce_service = Arc::new(AnnounceService::new(
            tracker_core_container.core_config.clone(),
            tracker_core_container.announce_handler.clone(),
            tracker_core_container.authentication_service.clone(),
            tracker_core_container.whitelist_authorization.clone(),
            http_stats_event_sender.clone(),
        ));

        let scrape_service = Arc::new(ScrapeService::new(
            tracker_core_container.core_config.clone(),
            tracker_core_container.scrape_handler.clone(),
            tracker_core_container.authentication_service.clone(),
            http_stats_event_sender.clone(),
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
        torrust_tracker_clock::initialize_static();
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

        let http_tracker_container = Arc::new(initialize_container(&configuration));

        let bind_to = http_tracker_config.bind_address;

        let tls = make_rust_tls(&http_tracker_config.tsl_config)
            .await
            .map(|tls| tls.expect("tls config failed"));

        let register = &Registar::default();
        let stopped = HttpServer::new(Launcher::new(bind_to, tls));

        let started = stopped
            .start(http_tracker_container, register.give_form())
            .await
            .expect("it should start the server");
        let stopped = started.stop().await.expect("it should stop the server");

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }
}
