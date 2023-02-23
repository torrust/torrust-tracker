use std::sync::Arc;

use log::{error, info, warn};
use tokio::task::JoinHandle;
use torrust_tracker_configuration::UdpTracker;

use crate::tracker;
use crate::udp::server::Udp;

#[must_use]
pub fn start_job(config: &UdpTracker, tracker: Arc<tracker::Tracker>) -> JoinHandle<()> {
    let bind_addr = config.bind_address.clone();

    tokio::spawn(async move {
        match Udp::new(tracker, &bind_addr).await {
            Ok(udp_server) => {
                info!("Starting UDP server on: udp://{}", bind_addr);
                udp_server.start().await;
            }
            Err(e) => {
                warn!("Could not start UDP tracker on: udp://{}", bind_addr);
                error!("{}", e);
            }
        }
    })
}
