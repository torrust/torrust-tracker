//! Typed UDP server lifecycle states.
//!
//! # Test ownership
//!
//! The public `server` module owns registration-error preservation,
//! listener-release coverage and token-aware stop coverage. Bind failures are
//! not covered by coverage-only tests here.
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use derive_more::derive::Display;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::{ServiceRegistration, ServiceRegistrationForm};
use torrust_tracker_primitives::RuntimeServiceMetadata;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_core::{ConnectionIdValidationPolicy, UDP_TRACKER_LOG_TARGET};
use tracing::instrument;

use super::spawner::Spawner;
use super::{Server, UdpError};
use crate::container::UdpTrackerServerContainer;
use crate::server::bound_socket::BoundSocket;
use crate::server::launcher::{Launcher, OwnedReceiveLoop, StartedReceiveLoop};

/// A UDP server instance controller with no UDP instance running.
#[allow(
    clippy::module_name_repetitions,
    reason = "type alias distinguishes the UDP server stopped state"
)]
pub type StoppedUdpServer = Server<Stopped>;

/// A stopped UDP server state.
#[derive(Debug, Display)]
#[display("Stopped: {spawner}")]
pub struct Stopped {
    pub spawner: Spawner,
}

/// A token-aware UDP runtime: the receive-loop task, owned by the caller.
///
/// The task returns `Ok(())` after cancellation and an error when the receive
/// loop stops for any other reason.
pub struct CancellationRunning {
    /// The address where the server is bound.
    pub local_addr: SocketAddr,
    pub task: JoinHandle<Result<(), std::io::Error>>,
}

impl Server<Stopped> {
    /// Creates a new `UdpServer` instance in `stopped`state.
    #[must_use]
    pub const fn new(spawner: Spawner) -> Self {
        Self {
            state: Stopped { spawner },
        }
    }

    /// Starts the UDP server using an injected cancellation token.
    ///
    /// This additive path does not subscribe to operating-system signals and
    /// has no launcher task. It registers the service and, if registration
    /// fails, cancels `cancellation_token`, stops the receive loop, and
    /// releases the socket before returning. Otherwise the caller owns the
    /// returned receive-loop task. After `cancellation_token` is cancelled the
    /// receive loop gives accepted requests at most `request_drain_deadline` to finish.
    ///
    /// # Errors
    ///
    /// Returns bind or service-registration errors.
    #[instrument(
        skip(self, udp_tracker_core_container, udp_tracker_server_container, form, metadata, cancellation_token),
        fields(
            service_role = metadata.service_role().as_str(),
            instance_index = metadata.configuration_instance_id().instance_index(),
        ),
        err
    )]
    #[allow(
        clippy::too_many_arguments,
        reason = "mirrors the legacy start inputs plus the injected cancellation token"
    )]
    pub async fn start_with_cancellation(
        self,
        udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
        udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
        form: ServiceRegistrationForm<RuntimeServiceMetadata>,
        metadata: RuntimeServiceMetadata,
        cookie_lifetime: Duration,
        connection_id_validation: ConnectionIdValidationPolicy,
        request_drain_deadline: Duration,
        cancellation_token: CancellationToken,
    ) -> Result<CancellationRunning, UdpError> {
        let bound_socket = BoundSocket::bind(
            self.state.spawner.bind_to,
            udp_tracker_core_container.udp_tracker_config.network.ipv6_v6only,
        )
        .map_err(|source| UdpError::Bind { source: *source })?;

        let StartedReceiveLoop {
            service_binding,
            address: local_addr,
            task,
        } = Launcher::start_receive_loop(
            udp_tracker_core_container,
            udp_tracker_server_container,
            bound_socket,
            cookie_lifetime,
            connection_id_validation,
            cancellation_token.clone(),
            request_drain_deadline,
        );
        let receive_loop = OwnedReceiveLoop::new(task);

        if let Some(public_url) = metadata.public_url() {
            tracing::info!(target: UDP_TRACKER_LOG_TARGET, service_binding = %service_binding, public_url = %public_url, "Started UDP tracker");
        } else {
            tracing::info!(target: UDP_TRACKER_LOG_TARGET, service_binding = %service_binding, "Started UDP tracker");
        }

        if let Err(error) = form
            .register(ServiceRegistration::new(service_binding, metadata, Some(Launcher::check)))
            .await
        {
            cancellation_token.cancel();
            let task = receive_loop.into_task();
            task.abort();
            drop(task.await);
            return Err(UdpError::Registration { source: error });
        }

        Ok(CancellationRunning {
            local_addr,
            task: receive_loop.into_task(),
        })
    }
}
