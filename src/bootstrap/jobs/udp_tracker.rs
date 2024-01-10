//! UDP tracker job starter.
//!
//! The [`udp_tracker::start_job`](crate::bootstrap::jobs::udp_tracker::start_job)
//! function starts a new UDP tracker server.
//!
//! > **NOTICE**: that the application can launch more than one UDP tracker
//! on different ports. Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! for the configuration options.
use std::sync::Arc;

use log::debug;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::UdpTracker;

use crate::core;
use crate::servers::udp::server::{Launcher, UdpServer};

/// It starts a new UDP server with the provided configuration.
///
/// It spawns a new asynchronous task for the new UDP server.
///
/// # Panics
///
/// It will panic if the API binding address is not a valid socket.
/// It will panic if it is unable to start the UDP service.
/// It will panic if the task did not finish successfully.
#[must_use]
pub async fn start_job(config: &UdpTracker, tracker: Arc<core::Tracker>) -> JoinHandle<()> {
    let bind_to = config
        .bind_address
        .parse::<std::net::SocketAddr>()
        .expect("it should have a valid udp tracker bind address");

    let server = UdpServer::new(Launcher::new(bind_to))
        .start(tracker)
        .await
        .expect("it should be able to start the udp tracker");

    tokio::spawn(async move {
        debug!(target: "UDP Tracker", "Wait for launcher (UDP service) to finish ...");

        assert!(
            !server.state.halt_task.is_closed(),
            "Halt channel for UDP tracker should be open"
        );

        server
            .state
            .task
            .await
            .expect("it should be able to join to the udp tracker task");
    })
}
