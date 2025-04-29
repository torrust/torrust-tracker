use std::time::Duration;

use tokio::time::{sleep, Instant};
use torrust_server_lib::signals::{shutdown_signal_with_message, Halted};
use tracing::instrument;

#[instrument(skip(handle, rx_halt, message))]
pub async fn graceful_shutdown(handle: axum_server::Handle, rx_halt: tokio::sync::oneshot::Receiver<Halted>, message: String) {
    shutdown_signal_with_message(rx_halt, message).await;

    let grace_period = Duration::from_secs(90);
    let max_wait = Duration::from_secs(95);
    let start = Instant::now();

    handle.graceful_shutdown(Some(grace_period));

    tracing::info!("!! Shutting down http server in {} seconds !!", grace_period.as_secs());

    loop {
        if handle.connection_count() == 0 {
            tracing::info!("All connections closed, shutting down server");
            break;
        }

        if start.elapsed() >= max_wait {
            tracing::warn!(
                "Shutdown timeout of {} seconds reached. Forcing shutdown with {} active connections.",
                max_wait.as_secs(),
                handle.connection_count()
            );
            break;
        }

        tracing::info!(
            "Remaining alive connections: {} ({}s elapsed)",
            handle.connection_count(),
            start.elapsed().as_secs()
        );

        sleep(Duration::from_secs(1)).await;
    }
}
