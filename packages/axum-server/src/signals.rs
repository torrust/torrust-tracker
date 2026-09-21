use std::net::SocketAddr;
use std::time::Duration;

use tokio::time::{Instant, sleep, sleep_until};
use tokio_util::sync::CancellationToken;
use torrust_server_lib::signals::{Halted, shutdown_signal_with_message};
use tracing::instrument;

#[derive(Debug, PartialEq, Eq)]
pub enum GracefulShutdownOutcome {
    Drained,
    TimedOut,
}

#[instrument(skip(handle, cancellation_token, message))]
pub async fn graceful_shutdown_on_cancellation(
    handle: axum_server::Handle<SocketAddr>,
    cancellation_token: CancellationToken,
    message: String,
    address: SocketAddr,
    drain_timeout: Duration,
) -> GracefulShutdownOutcome {
    cancellation_token.cancelled().await;

    handle.graceful_shutdown(None);

    tracing::info!("!! {} in {} seconds !!", message, drain_timeout.as_secs());

    let drain_deadline = Instant::now() + drain_timeout;
    let drain_timer = sleep_until(drain_deadline);
    tokio::pin!(drain_timer);

    loop {
        if handle.connection_count() == 0 {
            tracing::info!("All connections closed, shutting down server in address {}", address);
            return GracefulShutdownOutcome::Drained;
        }

        tracing::info!(
            "Remaining alive connections: {} ({}s remaining)",
            handle.connection_count(),
            (drain_deadline - Instant::now()).as_secs()
        );

        tokio::select! {
            () = &mut drain_timer => {
                tracing::warn!(
                    "Shutdown timeout of {} seconds reached in address {} with {} active connections.",
                    drain_timeout.as_secs(),
                    address,
                    handle.connection_count()
                );
                return GracefulShutdownOutcome::TimedOut;
            }
            () = sleep(Duration::from_secs(1)) => (),
        }
    }
}

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
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;
    use std::time::Duration;

    use axum::Router;
    use axum::routing::get;
    use axum_server::Handle;
    use tokio::net::TcpStream;
    use tokio::sync::Notify;
    use tokio::task::JoinHandle;
    use tokio_util::sync::CancellationToken;
    use torrust_server_lib::signals::cancellation_signal;

    use super::{GracefulShutdownOutcome, graceful_shutdown_on_cancellation};

    #[tokio::test]
    async fn it_should_return_drained_when_the_token_is_cancelled_with_no_active_connections() {
        // Arrange
        let cancellation_token = CancellationToken::new();
        let shutdown = tokio::spawn(graceful_shutdown_on_cancellation(
            Handle::new(),
            cancellation_token.clone(),
            String::from("shutting down test server"),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
            Duration::from_secs(1),
        ));
        tokio::task::yield_now().await;
        assert!(!shutdown.is_finished(), "shutdown should wait for cancellation");

        // Act
        cancellation_token.cancel();
        let outcome = tokio::time::timeout(Duration::from_secs(1), shutdown)
            .await
            .expect("shutdown should complete after cancellation")
            .expect("shutdown task should not panic");

        // Assert
        assert_eq!(outcome, GracefulShutdownOutcome::Drained);
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_return_timed_out_when_a_connection_remains_active_after_cancellation() {
        // Arrange
        let (handle, connection, server, address) = start_server_with_in_flight_request().await;
        let cancellation_token = CancellationToken::new();
        let drain_timeout = Duration::from_millis(500);
        let shutdown = tokio::spawn(graceful_shutdown_on_cancellation(
            handle,
            cancellation_token.clone(),
            String::from("shutting down test server"),
            address,
            drain_timeout,
        ));

        // Act
        cancellation_token.cancel();
        tokio::task::yield_now().await;
        tokio::time::advance(drain_timeout).await;
        tokio::task::yield_now().await;
        assert!(
            shutdown.is_finished(),
            "drain timeout should wake the helper before the one-second status tick"
        );
        let outcome = shutdown.await.expect("shutdown task should not panic");

        // Assert
        assert_eq!(outcome, GracefulShutdownOutcome::TimedOut);

        drop(connection);
        tokio::time::timeout(Duration::from_secs(1), server)
            .await
            .expect("server should stop after its drain timeout")
            .expect("server task should not panic");
    }

    async fn start_server_with_in_flight_request() -> (Handle<SocketAddr>, TcpStream, JoinHandle<()>, SocketAddr) {
        let handle = Handle::new();
        let server_handle = handle.clone();
        let request_started = Arc::new(Notify::new());
        let handler_request_started = request_started.clone();
        let router = Router::new().route(
            "/",
            get(move || {
                let request_started = handler_request_started.clone();

                async move {
                    request_started.notify_one();
                    std::future::pending::<()>().await;
                }
            }),
        );
        let server = tokio::spawn(async move {
            axum_server::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
                .handle(server_handle.clone())
                .serve(router.into_make_service())
                .await
                .expect("server should stop without an error");
        });
        let address = handle.listening().await.expect("server should start listening");
        let connection = TcpStream::connect(address).await.expect("client should connect to server");
        connection
            .try_write(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .expect("client should send the request");
        request_started.notified().await;

        (handle, connection, server, address)
    }

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
