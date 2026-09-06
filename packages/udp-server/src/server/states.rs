use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use derive_more::Constructor;
use derive_more::derive::Display;
use tokio::task::JoinHandle;
use torrust_server_lib::registar::{ServiceRegistration, ServiceRegistrationForm};
use torrust_server_lib::signals::{Halted, Started};
use torrust_tracker_primitives::RuntimeServiceMetadata;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_core::{ConnectionIdValidationPolicy, UDP_TRACKER_LOG_TARGET};
use tracing::{Level, instrument};

use super::spawner::{LaunchRequest, Spawner};
use super::{Server, UdpError};
use crate::container::UdpTrackerServerContainer;
use crate::server::bound_socket::BoundSocket;
use crate::server::launcher::Launcher;

/// A UDP server instance controller with no UDP instance running.
#[allow(clippy::module_name_repetitions)]
pub type StoppedUdpServer = Server<Stopped>;

/// A UDP server instance controller with a running UDP instance.
#[allow(clippy::module_name_repetitions)]
pub type RunningUdpServer = Server<Running>;

/// A stopped UDP server state.
#[derive(Debug, Display)]
#[display("Stopped: {spawner}")]
pub struct Stopped {
    pub spawner: Spawner,
}

/// A running UDP server state.
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(clippy::redundant_field_names)]
#[derive(Debug, Display, Constructor)]
#[display("Running (with local address): {local_addr}")]
pub struct Running {
    /// The address where the server is bound.
    pub local_addr: SocketAddr,
    pub halt_task: tokio::sync::oneshot::Sender<Halted>,
    pub task: JoinHandle<Result<Spawner, std::io::Error>>,
}

impl Server<Stopped> {
    /// Creates a new `UdpServer` instance in `stopped`state.
    #[must_use]
    pub const fn new(spawner: Spawner) -> Self {
        Self {
            state: Stopped { spawner },
        }
    }

    /// It starts the server and returns a `UdpServer` controller in `running`
    /// state.
    ///
    /// # Errors
    ///
    /// Will return `Err` if UDP can't bind to given bind address.
    ///
    #[instrument(
        skip(self, udp_tracker_core_container, udp_tracker_server_container, form, metadata),
        fields(
            service_role = metadata.service_role().as_str(),
            instance_index = metadata.configuration_instance_id().instance_index(),
        ),
        err,
        ret(Display, level = Level::INFO)
    )]
    pub async fn start(
        self,
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        form: ServiceRegistrationForm<RuntimeServiceMetadata>,
        metadata: RuntimeServiceMetadata,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
    ) -> Result<Server<Running>, UdpError> {
        let (tx_start, rx_start) = tokio::sync::oneshot::channel::<Started>();
        let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

        assert!(!tx_halt.is_closed(), "Halt channel for UDP tracker should be open");

        // May need to wrap in a task to about a tokio bug.
        let bound_socket = BoundSocket::bind(
            self.state.spawner.bind_to,
            udp_tracker_core_container.udp_tracker_config.network.ipv6_v6only,
        )
        .map_err(|source| UdpError::Bind { source: *source })?;
        let mut task = self.state.spawner.spawn_launcher(LaunchRequest {
            udp_tracker_core_container,
            udp_tracker_server_container,
            cookie_lifetime,
            connection_id_validation,
            bound_socket,
            tx_start,
            rx_halt,
        });

        let started = await_startup_notification(rx_start, &mut task).await?;

        let service_binding = started.service_binding;
        let local_addr = started.address;

        if let Some(public_url) = metadata.public_url() {
            tracing::info!(target: UDP_TRACKER_LOG_TARGET, service_binding = %service_binding, public_url = %public_url, "Started UDP tracker");
        } else {
            tracing::info!(target: UDP_TRACKER_LOG_TARGET, service_binding = %service_binding, "Started UDP tracker");
        }

        if let Err(error) = form
            .register(ServiceRegistration::new(service_binding, metadata, Some(Launcher::check)))
            .await
        {
            let _ = tx_halt.send(Halted::Normal);
            drop(task.await);
            return Err(UdpError::Registration { source: error });
        }

        let running_udp_server: Server<Running> = Server {
            state: Running {
                local_addr,
                halt_task: tx_halt,
                task,
            },
        };

        let local_addr = format!("udp://{local_addr}");
        tracing::trace!(target: UDP_TRACKER_LOG_TARGET, local_addr, "UdpServer<Stopped>::start (running)");

        Ok(running_udp_server)
    }
}

async fn await_startup_notification(
    rx_start: tokio::sync::oneshot::Receiver<Started>,
    task: &mut JoinHandle<Result<Spawner, std::io::Error>>,
) -> Result<Started, UdpError> {
    match rx_start.await {
        Ok(started) => Ok(started),
        Err(notification_error) => match task.await {
            Ok(Err(source)) => Err(UdpError::Launcher { source }),
            Ok(Ok(_)) => Err(UdpError::StartupNotification {
                source: notification_error,
            }),
            Err(error) => Err(UdpError::FailedToStartOrStopServer(error.to_string())),
        },
    }
}

impl Server<Running> {
    /// It stops the server and returns a `UdpServer` controller in `stopped`
    /// state.
    ///     
    /// # Errors
    ///
    /// Will return `Err` if the oneshot channel to send the stop signal
    /// has already been called once.
    ///
    /// # Panics
    ///
    /// It panics if unable to shutdown service.
    #[instrument(skip(self), err, ret(Display, level = Level::INFO))]
    pub async fn stop(self) -> Result<Server<Stopped>, UdpError> {
        self.state
            .halt_task
            .send(Halted::Normal)
            .map_err(|e| UdpError::FailedToStartOrStopServer(e.to_string()))?;

        let launcher = self
            .state
            .task
            .await
            .map_err(|error| UdpError::FailedToStartOrStopServer(error.to_string()))?
            .map_err(|source| UdpError::Launcher { source })?;

        let stopped_api_server: Server<Stopped> = Server {
            state: Stopped { spawner: launcher },
        };

        Ok(stopped_api_server)
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::oneshot;

    use super::{UdpError, await_startup_notification};
    use crate::server::spawner::Spawner;

    #[tokio::test]
    async fn it_should_preserve_a_broken_pipe_launcher_error_when_startup_notification_fails() {
        // Arrange
        let (tx_start, rx_start) = oneshot::channel();
        drop(tx_start);
        let mut task = tokio::spawn(async {
            Err::<Spawner, _>(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "UDP startup receiver was dropped",
            ))
        });

        // Act
        let result = await_startup_notification(rx_start, &mut task).await;

        // Assert
        let UdpError::Launcher { source } = result.expect_err("launcher startup failure should be returned") else {
            panic!("launcher error should not be collapsed into a startup-notification error");
        };
        assert_eq!(source.kind(), std::io::ErrorKind::BrokenPipe);
    }
}
