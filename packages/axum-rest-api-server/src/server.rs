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
#[allow(
    deprecated,
    reason = "legacy shutdown path on the deprecated `Halted` channel; SI-19 removes it"
)]
use torrust_server_lib::signals::{Halted, Started};
use torrust_tracker_axum_server::custom_axum_server::{self, TimeoutAcceptor};
#[allow(
    deprecated,
    reason = "deprecated legacy shutdown API kept for compatibility until SI-19 removes it"
)]
use torrust_tracker_axum_server::signals::{GracefulShutdownOutcome, graceful_shutdown, graceful_shutdown_on_cancellation};
use torrust_tracker_configuration::v3_0_0::tracker_api::AccessTokens;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
use tracing::{Level, instrument};

use super::routes::router;
use crate::API_LOG_TARGET;

/// Errors that can occur when starting or stopping the API server.
#[derive(Debug, Error)]
#[non_exhaustive]
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
#[allow(
    deprecated,
    reason = "deprecated legacy shutdown API kept for compatibility until SI-19 removes it"
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
#[allow(
    deprecated,
    reason = "legacy shutdown path on the deprecated `Halted` channel; SI-19 removes it"
)]
#[deprecated(
    since = "0.1.0",
    note = "use `CancellationRunning` from `ApiServer::start_with_cancellation` and cancel the injected `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488)"
)]
pub struct Running {
    pub local_addr: SocketAddr,
    #[deprecated(
        since = "0.1.0",
        note = "cancel the injected `CancellationToken` instead of sending `Halted`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488)"
    )]
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

#[allow(
    deprecated,
    reason = "deprecated legacy shutdown API kept for compatibility until SI-19 removes it"
)]
impl Running {
    #[must_use]
    #[allow(
        deprecated,
        reason = "legacy shutdown path on the deprecated `Halted` channel; SI-19 removes it"
    )]
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
    #[allow(
        deprecated,
        reason = "legacy shutdown path on the deprecated `Halted` channel; SI-19 removes it"
    )]
    #[deprecated(
        since = "0.1.0",
        note = "use `ApiServer::start_with_cancellation` with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488)"
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
    let drain_timeout = launcher.drain_timeout;
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
        drain_timeout,
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

#[allow(
    deprecated,
    reason = "deprecated legacy shutdown API kept for compatibility until SI-19 removes it"
)]
impl ApiServer<Running> {
    /// Stops the API server.
    ///
    /// # Errors
    ///
    /// It would return an error if the channel for the task killer signal was closed.
    #[instrument(skip(self), err, ret(Display, level = Level::INFO))]
    #[allow(
        deprecated,
        reason = "legacy shutdown path on the deprecated `Halted` channel; SI-19 removes it"
    )]
    #[deprecated(
        since = "0.1.0",
        note = "cancel the `CancellationToken` passed to `start_with_cancellation` and join `CancellationRunning::task` and `shutdown_controller`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488)"
    )]
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
/// Or if the request returns a status that is not a success (2xx).
#[must_use]
#[instrument(skip())]
pub fn check_fn(service_binding: &ServiceBinding) -> ServiceHealthCheckJob {
    check_fn_with_client(service_binding, reqwest::Client::new())
}

/// Checks the API health endpoint using the supplied HTTP client.
///
/// This preserves normal production certificate validation when called from
/// [`check_fn`] and allows tests to trust a known test certificate.
#[must_use]
pub fn check_fn_with_client(service_binding: &ServiceBinding, client: reqwest::Client) -> ServiceHealthCheckJob {
    let url = health_check_url(service_binding);

    let info = format!("checking api health check at: {url}");

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
        .join("api/health_check")
        .expect("Service binding URL can always resolve a health check path")
        .to_string()
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
    /// How long the server waits for open connections to finish after its cancellation token is
    /// cancelled, before it closes them.
    drain_timeout: Duration,
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
    #[allow(
        deprecated,
        reason = "legacy shutdown path on the deprecated `Halted` channel; SI-19 removes it"
    )]
    #[deprecated(
        since = "0.1.0",
        note = "use the token-aware path through `ApiServer::start_with_cancellation`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488)"
    )]
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
    use std::net::{Ipv4Addr, SocketAddr, TcpListener};
    use std::sync::{Arc, Once};
    use std::time::Duration;

    use axum_server::tls_rustls::RustlsConfig;
    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{Registar, RegistrationError, ServiceRegistration};
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_configuration::v3_0_0::{Configuration, logging};
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use crate::server::{ApiServer, CancellationRunning, Error, Launcher, check_fn, check_fn_with_client, health_check_url};
    use crate::test_support::{TEST_DEADLINE, TestApiContainer, api_container, within_deadline};

    fn initialize_global_services(configuration: &Configuration) {
        initialize_static();
        logging::setup(&configuration.logging);
    }

    fn initialize_static() {
        torrust_clock::initialize_static();
    }

    /// Starts the plain token-aware REST API on the configured address and registers it in the given registar.
    ///
    /// The returned container owns the database directory and must live as long as the server runs.
    async fn start_plain(
        configuration: &Configuration,
        registar: &Registar<RuntimeServiceMetadata>,
        cancellation_token: &CancellationToken,
    ) -> (TestApiContainer, CancellationRunning) {
        let http_api_config = configuration.http_api.as_ref().expect("missing HTTP API configuration");
        let api = within_deadline(api_container(configuration)).await;
        let running = within_deadline(
            ApiServer::new(Launcher::new(
                http_api_config.bind_address,
                None,
                configuration.application.shutdown.http_drain(),
            ))
            .start_with_cancellation(
                api.container.clone(),
                registar.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                Arc::new(http_api_config.access_tokens.clone()),
                cancellation_token.clone(),
            ),
        )
        .await
        .expect("the token-aware REST API should start");

        (api, running)
    }

    /// Cancels the token and joins the runtime task and the drain controller, each within the test deadline.
    async fn stop(running: CancellationRunning, cancellation_token: &CancellationToken) {
        cancellation_token.cancel();
        tokio::time::timeout(TEST_DEADLINE, running.task)
            .await
            .expect("the REST API task should stop after token cancellation")
            .expect("the REST API task should not panic")
            .expect("the REST API task should stop without an error");
        tokio::time::timeout(TEST_DEADLINE, running.shutdown_controller)
            .await
            .expect("the REST API drain controller should complete after token cancellation")
            .expect("the REST API drain controller should not panic");
    }

    #[tokio::test]
    #[allow(
        deprecated,
        reason = "deprecated legacy shutdown API kept for compatibility until SI-19 removes it"
    )]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_public());
        let http_api_config = Arc::new(cfg.http_api.clone().expect("missing HTTP API configuration"));

        initialize_global_services(&cfg);

        let bind_to = http_api_config.bind_address;

        let access_tokens = Arc::new(http_api_config.access_tokens.clone());

        let stopped = ApiServer::new(Launcher::new(bind_to, None, cfg.application.shutdown.http_drain()));

        let register = &Registar::<RuntimeServiceMetadata>::default();

        let api = within_deadline(api_container(&cfg)).await;

        let started = within_deadline(stopped.start(
            api.container.clone(),
            register.give_form(),
            RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
            access_tokens,
        ))
        .await
        .expect("it should start the server");
        let stopped = within_deadline(started.stop()).await.expect("it should stop the server");

        assert_eq!(stopped.state.launcher.bind_to, bind_to);
    }

    #[tokio::test]
    async fn it_should_drain_the_token_aware_rest_api_when_its_cancellation_token_is_cancelled() {
        // Arrange
        let configuration = Arc::new(ephemeral_public());
        initialize_global_services(&configuration);
        let http_api_config = configuration.http_api.as_ref().expect("missing HTTP API configuration");
        let cancellation_token = CancellationToken::new();
        let (_api, running) = start_plain(&configuration, &Registar::default(), &cancellation_token).await;

        // Act
        cancellation_token.cancel();
        let launcher = tokio::time::timeout(TEST_DEADLINE, running.task)
            .await
            .expect("the REST API task should stop after token cancellation")
            .expect("the REST API task should not panic")
            .expect("the REST API task should stop without an error");
        let drain_outcome = tokio::time::timeout(TEST_DEADLINE, running.shutdown_controller)
            .await
            .expect("the REST API drain controller should complete after token cancellation")
            .expect("the REST API drain controller should not panic");

        // Assert
        assert_eq!(launcher.bind_to, http_api_config.bind_address);
        assert_eq!(drain_outcome, GracefulShutdownOutcome::Drained);
    }

    #[tokio::test]
    async fn it_should_time_out_the_drain_after_the_configured_drain_timeout_when_a_connection_stays_open() {
        // Arrange
        // The launcher is configured with a 300 ms drain timeout, far below the 15 s default. A client
        // opens a connection and sends only part of a request header, so the server keeps waiting for
        // it. A second request that completes proves the server has accepted the first connection.
        let configured_drain_timeout = Duration::from_millis(300);
        let configuration = Arc::new(ephemeral_public());
        initialize_global_services(&configuration);
        let http_api_config = configuration.http_api.as_ref().expect("missing HTTP API configuration");
        let cancellation_token = CancellationToken::new();
        let api = within_deadline(api_container(&configuration)).await;
        let running = within_deadline(
            ApiServer::new(Launcher::new(http_api_config.bind_address, None, configured_drain_timeout)).start_with_cancellation(
                api.container.clone(),
                Registar::default().give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                Arc::new(http_api_config.access_tokens.clone()),
                cancellation_token.clone(),
            ),
        )
        .await
        .expect("the token-aware REST API should start");
        let mut held_connection = std::net::TcpStream::connect(running.local_addr).expect("open the connection to hold");
        std::io::Write::write_all(&mut held_connection, b"GET /api/health_check HTTP/1.1\r\nHost: tracker\r\n")
            .expect("send a partial request header");
        tokio::time::timeout(
            TEST_DEADLINE,
            reqwest::get(format!("http://{}/api/health_check", running.local_addr)),
        )
        .await
        .expect("the server should answer within the test deadline")
        .expect("the server should answer once it has accepted the held connection");

        // Act
        cancellation_token.cancel();
        let drain_outcome = tokio::time::timeout(TEST_DEADLINE, running.shutdown_controller)
            .await
            .expect("the drain should end by the configured timeout, not the default one")
            .expect("the REST API drain controller should not panic");

        // Assert
        assert_eq!(
            drain_outcome,
            GracefulShutdownOutcome::TimedOut,
            "a connection held open past the {configured_drain_timeout:?} drain timeout should end the drain as timed out"
        );
        drop(held_connection);
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

        let cancellation_token = CancellationToken::new();

        // Act
        let api = within_deadline(api_container(&configuration)).await;
        let result = within_deadline(
            ApiServer::new(Launcher::new(bind_to, None, configuration.application.shutdown.http_drain()))
                .start_with_cancellation(
                    api.container.clone(),
                    registar.give_form(),
                    RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                    Arc::new(http_api_config.access_tokens.clone()),
                    cancellation_token.clone(),
                ),
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
        assert!(
            cancellation_token.is_cancelled(),
            "a failed registration should cancel the caller's token so the caller's other tasks stop too"
        );
        TcpListener::bind(bind_to).expect("REST API listener should be released after token-aware registration failure");
    }

    // Embedded at compile time: the container test stage runs the tests from a different directory than the build stage.
    const TEST_CERTIFICATE: &[u8] = include_bytes!("../tests/fixtures/https-health-check-cert.pem");
    static RUSTLS_CRYPTO_PROVIDER: Once = Once::new();

    /// Selects `ring` as the process-level provider: with more than one provider enabled in the
    /// workspace, Rustls cannot choose one itself.
    fn install_rustls_crypto_provider() {
        RUSTLS_CRYPTO_PROVIDER.call_once(|| {
            rustls::crypto::ring::default_provider()
                .install_default()
                .expect("ring should be the Rustls crypto provider for the tests");
        });
    }

    const TEST_KEY: &[u8] = include_bytes!("../tests/fixtures/https-health-check-key.pem");

    #[test]
    fn it_should_build_a_health_check_url_using_the_service_binding_protocol() {
        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 1212));

        for (protocol, expected_url) in [
            (Protocol::HTTP, "http://127.0.0.1:1212/api/health_check"), // DevSkim: ignore DS137138
            (Protocol::HTTPS, "https://127.0.0.1:1212/api/health_check"),
        ] {
            let service_binding = ServiceBinding::new(protocol, address).expect("service binding should be valid");

            assert_eq!(health_check_url(&service_binding), expected_url);
        }
    }

    #[tokio::test]
    async fn it_should_report_a_tls_rest_api_as_healthy_when_the_check_trusts_the_test_certificate() {
        // Arrange
        let configuration = Arc::new(ephemeral_public());
        initialize_global_services(&configuration);
        install_rustls_crypto_provider();
        let http_api_config = configuration.http_api.as_ref().expect("missing HTTP API configuration");
        let tls = RustlsConfig::from_pem(TEST_CERTIFICATE.to_vec(), TEST_KEY.to_vec())
            .await
            .expect("the test certificate and key should load");
        let registar = Registar::<RuntimeServiceMetadata>::default();
        let cancellation_token = CancellationToken::new();
        let api = within_deadline(api_container(&configuration)).await;
        let running = within_deadline(
            ApiServer::new(Launcher::new(
                http_api_config.bind_address,
                Some(tls),
                configuration.application.shutdown.http_drain(),
            ))
            .start_with_cancellation(
                api.container.clone(),
                registar.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                Arc::new(http_api_config.access_tokens.clone()),
                cancellation_token.clone(),
            ),
        )
        .await
        .expect("the token-aware REST API should start with TLS");
        let services = within_deadline(registar.services()).await;
        let service_binding = services
            .first()
            .expect("the REST API should register one service")
            .service_binding();
        let certificate = reqwest::Certificate::from_pem(TEST_CERTIFICATE).expect("the test certificate should parse");
        let client = reqwest::Client::builder()
            .add_root_certificate(certificate)
            .build()
            .expect("the client trusting the test certificate should build");

        // Act
        let result = within_deadline(check_fn_with_client(service_binding, client).job)
            .await
            .expect("the health check job should not panic");

        // Assert
        assert_eq!(result, Ok("200 OK".to_string()));

        stop(running, &cancellation_token).await;
    }

    #[tokio::test]
    async fn it_should_fail_to_start_with_a_bind_error_when_the_address_is_already_in_use() {
        // Arrange
        let configuration = Arc::new(ephemeral_public());
        initialize_global_services(&configuration);
        let http_api_config = configuration.http_api.as_ref().expect("missing HTTP API configuration");
        let occupying_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("select an address to occupy");
        let occupied_address = occupying_listener.local_addr().expect("read the occupied address");
        let api = within_deadline(api_container(&configuration)).await;

        // Act
        let result = within_deadline(
            ApiServer::new(Launcher::new(
                occupied_address,
                None,
                configuration.application.shutdown.http_drain(),
            ))
            .start_with_cancellation(
                api.container.clone(),
                Registar::default().give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                Arc::new(http_api_config.access_tokens.clone()),
                CancellationToken::new(),
            ),
        )
        .await;

        // Assert
        let kind = match result {
            Err(Error::Bind { source }) => source.kind(),
            Err(error) => panic!("an occupied address should be a bind error: {error}"),
            Ok(_) => panic!("an occupied address should not start the server"),
        };
        assert_eq!(kind, std::io::ErrorKind::AddrInUse);
        drop(occupying_listener);
    }

    #[tokio::test]
    async fn it_should_register_the_plain_rest_api_with_an_http_binding_on_the_address_it_serves() {
        // Arrange
        let configuration = ephemeral_public();
        initialize_global_services(&configuration);
        let registar = Registar::<RuntimeServiceMetadata>::default();
        let cancellation_token = CancellationToken::new();

        // Act
        let (_api, running) = start_plain(&configuration, &registar, &cancellation_token).await;
        let services = within_deadline(registar.services()).await;

        // Assert
        let binding = services
            .first()
            .expect("the REST API should register one service")
            .service_binding();
        assert_eq!(
            (services.len(), binding.protocol(), binding.bind_address()),
            (1, Protocol::HTTP, running.local_addr)
        );

        stop(running, &cancellation_token).await;
    }

    #[tokio::test]
    async fn it_should_report_a_plain_rest_api_as_healthy_with_the_default_health_check() {
        // Arrange
        let configuration = ephemeral_public();
        initialize_global_services(&configuration);
        let registar = Registar::<RuntimeServiceMetadata>::default();
        let cancellation_token = CancellationToken::new();
        let (_api, running) = start_plain(&configuration, &registar, &cancellation_token).await;
        let services = within_deadline(registar.services()).await;
        let service_binding = services
            .first()
            .expect("the REST API should register one service")
            .service_binding();
        let expected_info = format!("checking api health check at: http://{}/api/health_check", running.local_addr); // DevSkim: ignore DS137138

        // Act
        let job = check_fn(service_binding);
        let info = job.info.clone();
        let result = within_deadline(job.job).await.expect("the health check job should not panic");

        // Assert
        assert_eq!((info, result), (expected_info, Ok("200 OK".to_string())));

        stop(running, &cancellation_token).await;
    }

    #[tokio::test]
    async fn it_should_report_an_error_with_the_checked_url_when_nothing_listens_on_the_checked_address() {
        // Arrange
        let free_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("select a free address");
        let dead_address = free_listener.local_addr().expect("read the free address");
        drop(free_listener);
        let service_binding = ServiceBinding::new(Protocol::HTTP, dead_address).expect("service binding should be valid");

        // Act
        let result = within_deadline(check_fn(&service_binding).job)
            .await
            .expect("the health check job should not panic");

        // Assert
        let message = result.expect_err("a dead address should fail the health check");
        assert!(
            message.contains(&format!("{dead_address}/api/health_check")),
            "the error should name the checked URL, got: {message}"
        );
    }

    #[tokio::test]
    async fn it_should_display_the_launcher_with_its_socket_and_whether_it_uses_tls() {
        // Arrange
        install_rustls_crypto_provider();
        let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 1212));
        let tls = RustlsConfig::from_pem(TEST_CERTIFICATE.to_vec(), TEST_KEY.to_vec())
            .await
            .expect("the test certificate and key should load");
        let with_tls = Launcher::new(address, Some(tls), Duration::from_secs(1));
        let without_tls = Launcher::new(address, None, Duration::from_secs(1));

        // Act
        let displayed = (with_tls.to_string(), without_tls.to_string());

        // Assert
        assert_eq!(
            displayed,
            (
                "(with socket): 127.0.0.1:1212, using TLS".to_string(),
                "(with socket): 127.0.0.1:1212, without TLS".to_string()
            )
        );
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
