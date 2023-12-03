//! UDP tracker job starter.
//!
//! The [`udp_tracker::start_job`](crate::bootstrap::jobs::udp_tracker::start_job)
//! function starts a new UDP tracker server.
//!
//! > **NOTICE**: that the application can launch more than one UDP tracker
//! on different ports. Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! for the configuration options.
use std::sync::Arc;

use log::{error, info, warn};
use tokio::task::JoinHandle;
use torrust_tracker_configuration::UdpTracker;

use crate::core;
use crate::servers::udp::server::Udp;

/// It starts a new UDP server with the provided configuration.
///
/// It spawns a new asynchronous task for the new UDP server.
#[must_use]
pub fn start_job(config: &UdpTracker, tracker: Arc<core::Tracker>) -> JoinHandle<()> {
    let bind_addr = config.bind_address.clone();

    tokio::spawn(async move {
        match Udp::new(&bind_addr).await {
            Ok(udp_server) => {
                info!("Starting UDP server on: udp://{}", bind_addr);
                udp_server.start(tracker).await;
            }
            Err(e) => {
                warn!("Could not start UDP tracker on: udp://{}", bind_addr);
                error!("{}", e);
            }
        }
    })
}
