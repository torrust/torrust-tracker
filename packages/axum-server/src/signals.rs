use std::time::Duration;

use tokio::time::sleep;
use torrust_server_lib::signals::{shutdown_signal_with_message, Halted};
use tracing::instrument;

#[instrument(skip(handle, rx_halt, message))]
pub async fn graceful_shutdown(handle: axum_server::Handle, rx_halt: tokio::sync::oneshot::Receiver<Halted>, message: String) {
    shutdown_signal_with_message(rx_halt, message).await;

    let duration = Duration::from_secs(90);

    tracing::info!(
        "Http server received shutdown signal, shutting down server listening on: {:?}",
        handle.listening().await
    );

    handle.graceful_shutdown(Some(Duration::from_secs(90)));

    tracing::info!("!! Shuting down in {} seconds !!", duration.as_secs());

    loop {
        sleep(Duration::from_secs(1)).await;

        tracing::info!("Remaining alive connections: {}", handle.connection_count());
    }
}
