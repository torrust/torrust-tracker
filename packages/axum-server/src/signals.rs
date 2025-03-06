use std::time::Duration;

use tokio::time::sleep;
use torrust_server_lib::signals::{shutdown_signal_with_message, Halted};
use tracing::instrument;

#[instrument(skip(handle, rx_halt, message))]
pub async fn graceful_shutdown(handle: axum_server::Handle, rx_halt: tokio::sync::oneshot::Receiver<Halted>, message: String) {
    shutdown_signal_with_message(rx_halt, message).await;

    tracing::debug!("Sending graceful shutdown signal");
    handle.graceful_shutdown(Some(Duration::from_secs(90)));

    println!("!! shuting down in 90 seconds !!");

    loop {
        sleep(Duration::from_secs(1)).await;

        tracing::info!("remaining alive connections: {}", handle.connection_count());
    }
}
