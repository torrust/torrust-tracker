use std::sync::Arc;

use log::info;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::api::server;
use crate::config::Configuration;
use crate::tracker;

#[derive(Debug)]
pub struct ApiServerJobStarted();

pub async fn start_job(config: &Configuration, tracker: Arc<tracker::Tracker>) -> JoinHandle<()> {
    let bind_addr = config
        .http_api
        .bind_address
        .parse::<std::net::SocketAddr>()
        .expect("Tracker API bind_address invalid.");

    info!("Starting Torrust API server on: {}", bind_addr);

    let (tx, rx) = oneshot::channel::<ApiServerJobStarted>();

    // Run the API server
    let join_handle = tokio::spawn(async move {
        let handel = server::start(bind_addr, &tracker);

        assert!(tx.send(ApiServerJobStarted()).is_ok(), "the start job dropped");

        handel.await;
    });

    // Wait until the API server job is running
    match rx.await {
        Ok(_msg) => info!("Torrust API server started"),
        Err(_) => panic!("the api server dropped"),
    }

    join_handle
}
