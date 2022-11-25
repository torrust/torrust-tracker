use std::sync::Arc;

use log::info;
use tokio::sync::oneshot::{self, Receiver};
use tokio::task::JoinHandle;

use crate::api::server;
use crate::tracker::TorrentTracker;
use crate::Configuration;

#[derive(Debug)]
pub struct ApiReady();

pub fn start_job(config: &Configuration, tracker: Arc<TorrentTracker>) -> (JoinHandle<()>, Receiver<ApiReady>) {
    let bind_addr = config
        .http_api
        .bind_address
        .parse::<std::net::SocketAddr>()
        .expect("Tracker API bind_address invalid.");

    let (tx, rx) = oneshot::channel::<ApiReady>();

    info!("Starting Torrust API server on: {}", bind_addr);

    let join_handle = tokio::spawn(async move {
        server::start(bind_addr, tracker, tx).await;
    });

    (join_handle, rx)
}
