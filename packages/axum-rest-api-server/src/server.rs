//! Logic to run the HTTP API server.
//!
//! It contains two main structs: `ApiServer` and `Launcher`,
//! and two main functions: `start` and `start_tls`.
//!
//! The `ApiServer` struct is responsible for:
//! - Starting and stopping the server.
//! - Storing the configuration.
//!
//! `ApiServer` relies on a launcher to start the actual server.
///
/// 1. `ApiServer::start` -> spawns new asynchronous task.
/// 2. `Launcher::start` -> starts the server on the spawned task.
///
/// The `Launcher` struct is responsible for:
///
/// - Knowing how to start the server with graceful shutdown.
///
/// For the time being the `ApiServer` and `Launcher` are only used in tests
/// where we need to start and stop the server multiple times. In production
/// code and the main application uses the `start` and `start_tls` functions
/// to start the servers directly since we do not need to control the server
/// when it's running. In the future we might need to control the server,
/// for example, to restart it to apply new configuration changes, to remotely
/// shutdown the server, etc.
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum_server::Handle;
use axum_server::tls_rustls::RustlsConfig;
use derive_more::Constructor;
use derive_more::derive::Display;
use futures::future::BoxFuture;
use thiserror::Error;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
use torrust_server_lib::logging::STARTED_ON;
use torrust_server_lib::registar::{ServiceHealthCheckJob, ServiceRegistration, ServiceRegistrationForm};
use torrust_server_lib::signals::{Halted, Started};
use torrust_tracker_axum_server::custom_axum_server::{self, TimeoutAcceptor};
use torrust_tracker_axum_server::signals::{GracefulShutdownOutcome, graceful_shutdown, graceful_shutdown_on_cancellation};
use torrust_tracker_configuration::v3_0_0::tracker_api::AccessTokens;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
use tracing::{Level, instrument};

use super::routes::router;
use crate::API_LOG_TARGET;

const API_GRACEFUL_DRAIN_TIMEOUT: Duration = Duration::from_secs(90);

/// Errors that can occur when starting or stopping the API server.
#[derive(Debug, Error)]
pub enum Error {
    #[error("could not bind tracker API listener: {source}")]
    Bind { source: std::io::Error },

    #[error("could not configure tracker API listener: {source}")]
    Listener { source: std::io::Error },

    #[error("tracker API startup notification receiver was dropped")]
    StartupNotificationDropped,

    #[error("tracker API startup notification was not received: {source}")]
    StartupNotification { source: tokio::sync::oneshot::error::RecvError },

    #[error("could not register tracker API service: {source}")]
    Registration {
        source: torrust_server_lib::registar::RegistrationError,
    },

    #[error("could not stop tracker API service: {message}")]
    Stop { message: String },
}

/// An error returned by the token-aware REST API runtime after startup.
#[derive(Debug, Error)]
#[error("tracker API server stopped with an error: {message}")]
pub struct RuntimeError {
    message: String,
}

/// An alias for the `ApiServer` struct with the `Stopped` state.
#[allow(
    clippy::module_name_repetitions,
    reason = "type alias distinguishes the REST API server stopped state"
)]
pub type StoppedApiServer = ApiServer<Stopped>;

/// An alias for the `ApiServer` struct with the `Running` state.
#[allow(
    clippy::module_name_repetitions,
    reason = "type alias distinguishes the REST API server running state"
)]
pub type RunningApiServer = ApiServer<Running>;

/// A struct responsible for starting and stopping an API server with a
/// specific configuration and keeping track of the started server.
///
/// It's a state machine that can be in one of two
/// states: `Stopped` or `Running`.
#[allow(
    clippy::module_name_repetitions,
    reason = "public type identifies the management REST API server controller"
)]
#[derive(Debug, Display)]
pub struct ApiServer<S>
where
    S: std::fmt::Debug + std::fmt::Display,
{
    pub state: S,
}

/// The `Stopped` state of the `ApiServer` struct.
#[derive(Debug, Display)]
#[display("Stopped: {launcher}")]
pub struct Stopped {
    launcher: Launcher,
}

/// The `Running` state of the `ApiServer` struct.
#[derive(Debug, Display)]
#[display("Running (with local address): {local_addr}")]
pub struct Running {
    pub local_addr: SocketAddr,
    pub halt_task: tokio::sync::oneshot::Sender<Halted>,
    pub task: tokio::task::JoinHandle<Launcher>,
}

/// A token-aware REST API runtime and its owned drain controller.
///
/// The caller must retain and join both handles. The existing [`Running`] state
/// remains the compatibility path for consumers using [`Halted`].
pub struct CancellationRunning {
    pub local_addr: SocketAddr,
    pub task: JoinHandle<Result<Launcher, RuntimeError>>,
    pub shutdown_controller: JoinHandle<GracefulShutdownOutcome>,
}

impl Running {
    #[must_use]
    pub const fn new(
        local_addr: SocketAddr,
        halt_task: tokio::sync::oneshot::Sender<Halted>,
        task: tokio::task::JoinHandle<Launcher>,
    ) -> Self {
        Self {
            local_addr,
            halt_task,
            task,
        }
    }
}

impl ApiServer<Stopped> {
    #[must_use]
    pub const fn new(launcher: Launcher) -> Self {
        Self {
            state: Stopped { launcher },
        }
    }

    /// Starts the API server with the given configuration.
    ///
    /// # Errors
    ///
    /// It would return an error if no `SocketAddr` is returned after launching the server.
    ///
    #[instrument(
        skip(self, http_api_container, form, metadata, access_tokens),
        fields(
            service_role = metadata.service_role().as_str(),
            instance_index = metadata.configuration_instance_id().instance_index(),
        ),
        err,
        ret(Display, level = Level::INFO)
    )]
    pub async fn start(
        self,
        http_api_container: Arc<TrackerHttpApiCoreContainer>,
        form: ServiceRegistrationForm<RuntimeServiceMetadata>,
        metadata: RuntimeServiceMetadata,
        access_tokens: Arc<AccessTokens>,
    ) -> Result<ApiServer<Running>, Error> {
        let (tx_start, rx_start) = tokio::sync::oneshot::channel::<Started>();
        let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

        let launcher = self.state.launcher;

        let running = launcher.start(&http_api_container, access_tokens, tx_start, rx_halt)?;
        let task = tokio::spawn(async move {
            running.await;
            launcher
        });

        let started = rx_start.await.map_err(|source| Error::StartupNotification { source })?;
        if let Some(public_url) = metadata.public_url() {
            tracing::info!(target: API_LOG_TARGET, service_binding = %started.service_binding, public_url = %public_url, "Started tracker API");
        } else {
            tracing::info!(target: API_LOG_TARGET, service_binding = %started.service_binding, "Started tracker API");
        }

        if let Err(source) = form
            .register(ServiceRegistration::new(started.service_binding, metadata, Some(check_fn)))
            .await
        {
            let _ = tx_halt.send(Halted::Normal);
            drop(task.await);
            return Err(Error::Registration { source });
        }

        let api_server = ApiServer {
            state: Running::new(started.address, tx_halt, task),
        };

        Ok(api_server)
    }

    /// Starts the REST API using an injected cancellation token.
    ///
    /// This additive path does not subscribe to operating-system signals. Its
    /// caller owns the returned runtime task and drain controller.
    ///
    /// # Errors
    ///
    /// Returns listener or service-registration errors.
    pub async fn start_with_cancellation(
        self,
        http_api_container: Arc<TrackerHttpApiCoreContainer>,
        form: ServiceRegistrationForm<RuntimeServiceMetadata>,
        metadata: RuntimeServiceMetadata,
        access_tokens: Arc<AccessTokens>,
        cancellation_token: CancellationToken,
    ) -> Result<CancellationRunning, Error> {
        let (running, service_binding) = start_token_aware_runtime(
            self.state.launcher,
            &http_api_container,
            access_tokens,
            cancellation_token.clone(),
        )?;

        if let Some(public_url) = metadata.public_url() {
            tracing::info!(target: API_LOG_TARGET, service_binding = %service_binding, public_url = %public_url, "Started tracker API");
        } else {
            tracing::info!(target: API_LOG_TARGET, service_binding = %service_binding, "Started tracker API");
        }

        if let Err(source) = form
            .register(ServiceRegistration::new(service_binding, metadata, Some(check_fn)))
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
    http_api_container: &Arc<TrackerHttpApiCoreContainer>,
    access_tokens: Arc<AccessTokens>,
    cancellation_token: CancellationToken,
) -> Result<(CancellationRunning, ServiceBinding), Error> {
    let socket = std::net::TcpListener::bind(launcher.bind_to).map_err(|source| Error::Bind { source })?;
    socket.set_nonblocking(true).map_err(|source| Error::Listener { source })?;
    let local_addr = socket.local_addr().map_err(|source| Error::Listener { source })?;
    let handle = Handle::new();
    let server_handle = handle.clone();
    let protocol = if launcher.tls.is_some() {
        Protocol::HTTPS
    } else {
        Protocol::HTTP
    };
    let service_binding = ServiceBinding::new(protocol.clone(), local_addr).map_err(|error| Error::Listener {
        source: std::io::Error::other(error),
    })?;
    let router = router(http_api_container, access_tokens, &service_binding);
    let server: BoxFuture<'static, Result<(), RuntimeError>> = if let Some(tls) = launcher.tls.clone() {
        let server =
            custom_axum_server::from_tcp_rustls_with_timeouts(socket, tls).map_err(|source| Error::Listener { source })?;
        Box::pin(async move {
            if let Err(error) = server
                .handle(server_handle)
                .serve(router.into_make_service_with_connect_info::<SocketAddr>())
                .await
            {
                tracing::error!(%error, "Tracker API TLS server stopped with an error");
                return Err(RuntimeError {
                    message: error.to_string(),
                });
            }

            Ok(())
        })
    } else {
        let server = custom_axum_server::from_tcp_with_timeouts(socket).map_err(|source| Error::Listener { source })?;
        Box::pin(async move {
            if let Err(error) = server
                .handle(server_handle)
                .acceptor(TimeoutAcceptor)
                .serve(router.into_make_service_with_connect_info::<SocketAddr>())
                .await
            {
                tracing::error!(%error, "Tracker API server stopped with an error");
                return Err(RuntimeError {
                    message: error.to_string(),
                });
            }

            Ok(())
        })
    };

    tracing::info!(target: API_LOG_TARGET, "Starting on: {protocol}://{local_addr}");
    tracing::info!(target: API_LOG_TARGET, "{STARTED_ON}: {protocol}://{local_addr}");
    let task = tokio::spawn(async move { server.await.map(|()| launcher) });
    let shutdown_controller = tokio::spawn(graceful_shutdown_on_cancellation(
        handle,
        cancellation_token,
        format!("Shutting down tracker API server on socket address: {local_addr}"),
        local_addr,
        API_GRACEFUL_DRAIN_TIMEOUT,
    ));

    Ok((
        CancellationRunning {
            local_addr,
            task,
            shutdown_controller,
        },
        service_binding,
    ))
}

impl ApiServer<Running> {
    /// Stops the API server.
    ///
    /// # Errors
    ///
    /// It would return an error if the channel for the task killer signal was closed.
    #[instrument(skip(self), err, ret(Display, level = Level::INFO))]
    pub async fn stop(self) -> Result<ApiServer<Stopped>, Error> {
        self.state.halt_task.send(Halted::Normal).map_err(|_| Error::Stop {
            message: "task killer channel was closed".to_string(),
        })?;

        let launcher = self.state.task.await.map_err(|error| Error::Stop {
            message: error.to_string(),
        })?;

        Ok(ApiServer {
            state: Stopped { launcher },
        })
    }
}

/// Checks the Health by connecting to the API service endpoint.
///
/// # Errors
///
/// This function will return an error if unable to connect.
/// Or if there request returns an error code.
#[must_use]
#[instrument(skip())]
pub fn check_fn(service_binding: &ServiceBinding) -> ServiceHealthCheckJob {
    let url = format!("http://{}/api/health_check", service_binding.bind_address()); // DevSkim: ignore DS137138

    let info = format!("checking api health check at: {url}");

    let job = tokio::spawn(async move {
        match reqwest::get(url).await {
            Ok(response) => Ok(response.status().to_string()),
            Err(err) => Err(err.to_string()),
        }
    });
    ServiceHealthCheckJob::new(info, job)
}

/// A struct responsible for starting the API server.
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
#[derive(Constructor, Debug)]
pub struct Launcher {
    bind_to: SocketAddr,
    tls: Option<RustlsConfig>,
}

impl std::fmt::Display for Launcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.tls.is_some() {
            write!(f, "(with socket): {}, using TLS", self.bind_to)
        } else {
            write!(f, "(with socket): {}, without TLS", self.bind_to)
        }
    }
}

impl Launcher {
    /// Starts the API server with graceful shutdown.
    ///
    /// If TLS is enabled in the configuration, it will start the server with
    /// TLS. See [`torrust-tracker-configuration`](torrust_tracker_configuration)
    /// for more  information about configuration.
    ///
    /// # Errors
    ///
    /// Returns an error when the listener cannot bind or be configured.
    #[instrument(skip(self, http_api_container, access_tokens, tx_start, rx_halt))]
    pub fn start(
        &self,
        http_api_container: &Arc<TrackerHttpApiCoreContainer>,
        access_tokens: Arc<AccessTokens>,
        tx_start: Sender<Started>,
        rx_halt: Receiver<Halted>,
    ) -> Result<BoxFuture<'static, ()>, Error> {
        let socket = std::net::TcpListener::bind(self.bind_to).map_err(|source| Error::Bind { source })?;
        socket.set_nonblocking(true).map_err(|source| Error::Listener { source })?;
        let address = socket.local_addr().map_err(|source| Error::Listener { source })?;

        let handle = Handle::new();

        tokio::task::spawn(graceful_shutdown(
            handle.clone(),
            rx_halt,
            format!("Shutting down tracker API server on socket address: {address}"),
            address,
        ));

        let tls = self.tls.clone();
        let protocol = if tls.is_some() { Protocol::HTTPS } else { Protocol::HTTP };
        let service_binding = ServiceBinding::new(protocol.clone(), address).map_err(|error| Error::Listener {
            source: std::io::Error::other(error),
        })?;

        let router = router(http_api_container, access_tokens, &service_binding);

        tracing::info!(target: API_LOG_TARGET, "Starting on: {protocol}://{address}");

        let running: BoxFuture<'static, ()> = if let Some(tls) = tls {
            let server =
                custom_axum_server::from_tcp_rustls_with_timeouts(socket, tls).map_err(|source| Error::Listener { source })?;
            Box::pin(async move {
                if let Err(error) = server
                    .handle(handle)
                    // The TimeoutAcceptor is commented because TLS does not work with it.
                    // See: https://github.com/torrust/torrust-index/issues/204#issuecomment-2115529214
                    //.acceptor(TimeoutAcceptor)
                    .serve(router.into_make_service_with_connect_info::<std::net::SocketAddr>())
                    .await
                {
                    tracing::error!(%error, "Tracker API TLS server stopped with an error");
                }
            })
        } else {
            let server = custom_axum_server::from_tcp_with_timeouts(socket).map_err(|source| Error::Listener { source })?;
            Box::pin(async move {
                if let Err(error) = server
                    .handle(handle)
                    .acceptor(TimeoutAcceptor)
                    .serve(router.into_make_service_with_connect_info::<std::net::SocketAddr>())
                    .await
                {
                    tracing::error!(%error, "Tracker API server stopped with an error");
                }
            })
        };

        tracing::info!(target: API_LOG_TARGET, "{STARTED_ON}: {protocol}://{}", address);

        tx_start
            .send(Started {
                service_binding,
                address,
            })
            .map_err(|_| Error::StartupNotificationDropped)?;

        Ok(running)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, TcpListener};
    use std::sync::Arc;
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{Registar, RegistrationError, ServiceRegistration};
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_axum_server::tls::make_rust_tls;
    use torrust_tracker_configuration::v3_0_0::{Configuration, logging};
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use crate::server::{ApiServer, Error, Launcher};

    const TEST_COMPLETION_TIMEOUT: Duration = Duration::from_secs(5);

    fn initialize_global_services(configuration: &Configuration) {
        initialize_static();
        logging::setup(&configuration.logging);
    }

    fn initialize_static() {
        torrust_clock::initialize_static();
        torrust_tracker_udp_core::initialize_static();
    }

    async fn api_container(configuration: &Configuration) -> Arc<TrackerHttpApiCoreContainer> {
        let core_config = Arc::new(configuration.core.clone());
        let http_tracker_config = configuration
            .http_trackers
            .clone()
            .expect("missing HTTP tracker configuration");
        let http_tracker_config = Arc::new(http_tracker_config[0].clone());
        let udp_tracker_configurations = configuration.udp_trackers.clone().expect("missing UDP tracker configuration");
        let udp_tracker_config = Arc::new(udp_tracker_configurations[0].clone());
        let udp_tracker_server_config = configuration.udp_tracker_server.clone();
        let http_api_config = Arc::new(configuration.http_api.clone().expect("missing HTTP API configuration"));

        TrackerHttpApiCoreContainer::initialize(
            &core_config,
            &http_tracker_config,
            ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
            &udp_tracker_config,
            &udp_tracker_server_config,
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
            &http_api_config,
        )
        .await
    }

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_public());
        let http_api_config = Arc::new(cfg.http_api.clone().expect("missing HTTP API configuration"));

        initialize_global_services(&cfg);

        let bind_to = http_api_config.bind_address;

        let tls = if let Some(tls_config) = &http_api_config.tls_config {
            Some(make_rust_tls(tls_config).await.expect("tls config failed"))
        } else {
            None
        };

        let access_tokens = Arc::new(http_api_config.access_tokens.clone());

        let stopped = ApiServer::new(Launcher::new(bind_to, tls));

        let register = &Registar::<RuntimeServiceMetadata>::default();

        let http_api_container = api_container(&cfg).await;

        let started = stopped
            .start(
                http_api_container,
                register.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                access_tokens,
            )
            .await
            .expect("it should start the server");
        let stopped = started.stop().await.expect("it should stop the server");

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }

    #[tokio::test]
    async fn it_should_drain_the_token_aware_rest_api_when_its_cancellation_token_is_cancelled() {
        // Arrange
        let configuration = Arc::new(ephemeral_public());
        initialize_global_services(&configuration);
        let http_api_config = configuration.http_api.as_ref().expect("missing HTTP API configuration");
        let cancellation_token = CancellationToken::new();
        let running = ApiServer::new(Launcher::new(http_api_config.bind_address, None))
            .start_with_cancellation(
                api_container(&configuration).await,
                Registar::default().give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                Arc::new(http_api_config.access_tokens.clone()),
                cancellation_token.clone(),
            )
            .await
            .expect("the token-aware REST API should start");

        // Act
        cancellation_token.cancel();
        let launcher = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, running.task)
            .await
            .expect("the REST API task should stop after token cancellation")
            .expect("the REST API task should not panic")
            .expect("the REST API task should stop without an error");
        let drain_outcome = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, running.shutdown_controller)
            .await
            .expect("the REST API drain controller should complete after token cancellation")
            .expect("the REST API drain controller should not panic");

        // Assert
        assert_eq!(launcher.bind_to, http_api_config.bind_address);
        assert_eq!(drain_outcome, GracefulShutdownOutcome::Drained);
    }

    #[tokio::test]
    async fn it_should_release_the_listener_when_token_aware_startup_registration_fails() {
        // Arrange
        let available_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("select available REST API listener address");
        let bind_to = available_listener
            .local_addr()
            .expect("read available REST API listener address");
        drop(available_listener);
        let mut configuration = ephemeral_public();
        configuration
            .http_api
            .as_mut()
            .expect("test configuration enables REST API")
            .bind_address = bind_to;
        let configuration = Arc::new(configuration);
        initialize_global_services(&configuration);
        let registar = Registar::default();
        registar
            .give_form()
            .register(ServiceRegistration::new(
                ServiceBinding::new(Protocol::HTTP, bind_to).expect("REST API service binding should be valid"),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                None,
            ))
            .await
            .expect("reserve the REST API service registration");
        let http_api_config = configuration.http_api.as_ref().expect("missing HTTP API configuration");

        // Act
        let result = ApiServer::new(Launcher::new(bind_to, None))
            .start_with_cancellation(
                api_container(&configuration).await,
                registar.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                Arc::new(http_api_config.access_tokens.clone()),
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
        assert_eq!(binding.bind_address(), bind_to);
        TcpListener::bind(bind_to).expect("REST API listener should be released after token-aware registration failure");
    }
}
