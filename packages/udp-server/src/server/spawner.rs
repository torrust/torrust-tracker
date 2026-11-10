//! A thin wrapper for tokio spawn to launch the UDP server launcher as a new task.
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use derive_more::Constructor;
use derive_more::derive::Display;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
#[allow(
    deprecated,
    reason = "legacy shutdown path on the deprecated `Halted` channel; SI-19 removes it"
)]
use torrust_server_lib::signals::{Halted, Started};
use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;

use super::launcher::Launcher;
use crate::container::UdpTrackerServerContainer;
use crate::server::bound_socket::BoundSocket;

#[allow(
    deprecated,
    reason = "legacy shutdown path on the deprecated `Halted` channel; SI-19 removes it"
)]
#[deprecated(
    since = "0.1.0",
    note = "use `Server<Stopped>::start_with_cancellation`, which spawns the launcher with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)"
)]
pub struct LaunchRequest {
    pub udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
    pub udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
    pub cookie_lifetime: Duration,
    pub connection_id_validation: ConnectionIdValidationPolicy,
    pub bound_socket: BoundSocket,
    pub tx_start: oneshot::Sender<Started>,
    #[deprecated(
        since = "0.1.0",
        note = "use a `CancellationToken` instead of a `Halted` receiver; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)"
    )]
    pub rx_halt: oneshot::Receiver<Halted>,
}

// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
#[derive(Constructor, Copy, Clone, Debug, Display)]
#[display("(with socket): {bind_to}")]
pub struct Spawner {
    pub bind_to: SocketAddr,
}

impl Spawner {
    /// It spawns a new task to run the UDP server instance.
    ///
    #[must_use]
    #[deprecated(
        since = "0.1.0",
        note = "use `Server<Stopped>::start_with_cancellation`, which spawns the launcher with a `CancellationToken`; removal is planned for the next breaking release of this package (SI-19, EPIC #1488); executables, not libraries, own OS-signal subscriptions (see docs/features/shutdown-process/legacy-api-migration.md)"
    )]
    #[allow(
        deprecated,
        reason = "deprecated legacy shutdown API kept for compatibility until SI-19 removes it"
    )]
    pub fn spawn_launcher(&self, request: LaunchRequest) -> JoinHandle<Result<Self, std::io::Error>> {
        let spawner = Self::new(self.bind_to);

        tokio::spawn(async move {
            Launcher::run_with_graceful_shutdown(
                request.udp_tracker_core_container,
                request.udp_tracker_server_container,
                request.bound_socket,
                request.cookie_lifetime,
                request.connection_id_validation,
                request.tx_start,
                request.rx_halt,
            )
            .await
            .map(|()| spawner)
        })
    }
}
