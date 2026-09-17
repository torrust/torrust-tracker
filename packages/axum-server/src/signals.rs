use std::net::SocketAddr;
use std::time::Duration;

use tokio::time::{Instant, sleep};
use torrust_server_lib::signals::{Halted, shutdown_signal_with_message};
use tracing::instrument;

#[instrument(skip(handle, rx_halt, message))]
pub async fn graceful_shutdown(
    handle: axum_server::Handle<SocketAddr>,
    rx_halt: tokio::sync::oneshot::Receiver<Halted>,
    message: String,
    address: SocketAddr,
) {
    shutdown_signal_with_message(rx_halt, message.clone()).await;

    let grace_period = Duration::from_secs(90);
    let max_wait = Duration::from_secs(95);
    let start = Instant::now();

    handle.graceful_shutdown(Some(grace_period));

    tracing::info!("!! {} in {} seconds !!", message, grace_period.as_secs());

    loop {
        if handle.connection_count() == 0 {
            tracing::info!("All connections closed, shutting down server in address {}", address);
            break;
        }

        if start.elapsed() >= max_wait {
            tracing::warn!(
                "Shutdown timeout of {} seconds reached. Forcing shutdown in address {} with {} active connections.",
                max_wait.as_secs(),
                address,
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

#[cfg(test)]
mod tests {
    use tokio_util::sync::CancellationToken;
    use torrust_server_lib::signals::cancellation_signal;

    #[tokio::test]
    async fn it_should_compile_and_resolve_the_server_lib_cancellation_signal_when_token_is_cancelled() {
        // Arrange
        let cancellation_token = CancellationToken::new();
        let wait_task = tokio::spawn(cancellation_signal(cancellation_token.clone()));

        // Act
        cancellation_token.cancel();

        // Assert
        wait_task.await.expect("cancellation signal should resolve");
    }
}
