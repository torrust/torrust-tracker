use std::sync::Arc;
use log::{error, info, warn};
use tokio::task::JoinHandle;
use crate::{UdpServer, UdpTrackerConfig};
use crate::tracker::tracker::TorrentTracker;

pub fn start_job(config: &UdpTrackerConfig, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    let bind_addr = config.bind_address.clone();

    tokio::spawn(async move {
        match UdpServer::new(tracker, &bind_addr).await {
            Ok(udp_server) => {
                info!("Starting UDP server on: {}", bind_addr);
                udp_server.start().await;
            }
            Err(e) => {
                warn!("Could not start UDP tracker on: {}", bind_addr);
                error!("{}", e);
            }
        }
    })
}
