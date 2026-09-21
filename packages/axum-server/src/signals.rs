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

    tracing::info!("!! {} in {:?} !!", message, drain_timeout);

    let drain_deadline = Instant::now() + drain_timeout;
    let drain_timer = sleep_until(drain_deadline);
    tokio::pin!(drain_timer);

    loop {
        if handle.connection_count() == 0 {
            tracing::info!("All connections closed, shutting down server in address {}", address);
            return GracefulShutdownOutcome::Drained;
        }

        tracing::info!(
            "Remaining alive connections: {} ({:?} remaining)",
            handle.connection_count(),
            drain_deadline.saturating_duration_since(Instant::now())
        );

        tokio::select! {
            () = &mut drain_timer => {
                tracing::warn!(
                    "Shutdown timeout of {:?} reached in address {} with {} active connections.",
                    drain_timeout,
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
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tokio::sync::Notify;
    use tokio::task::JoinHandle;
    use tokio_util::sync::CancellationToken;

    use super::{GracefulShutdownOutcome, graceful_shutdown_on_cancellation};

    #[tokio::test(start_paused = true)]
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
    async fn it_should_return_drained_when_an_in_flight_connection_closes_after_cancellation() {
        // Arrange
        let mut server = start_server_with_releasable_in_flight_request().await;
        let cancellation_token = CancellationToken::new();
        let drain_timeout = Duration::from_secs(2);
        let shutdown = tokio::spawn(graceful_shutdown_on_cancellation(
            server.handle,
            cancellation_token.clone(),
            String::from("shutting down test server"),
            server.address,
            drain_timeout,
        ));

        // Act
        cancellation_token.cancel();
        server.release_request.notify_one();
        tokio::task::yield_now().await;
        let mut response = Vec::new();
        server
            .connection
            .read_to_end(&mut response)
            .await
            .expect("client should receive the released response");
        let outcome = tokio::time::timeout(drain_timeout, shutdown)
            .await
            .expect("shutdown should complete after the connection closes")
            .expect("shutdown task should not panic");

        // Assert
        assert_eq!(outcome, GracefulShutdownOutcome::Drained);
        assert!(response.starts_with(b"HTTP/1.1 200 OK"));
        wait_for_server(server.task).await;
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_return_timed_out_when_a_connection_remains_active_after_cancellation() {
        // Arrange
        let server = start_server_with_in_flight_request().await;
        let cancellation_token = CancellationToken::new();
        let drain_timeout = Duration::from_millis(500);
        let shutdown = tokio::spawn(graceful_shutdown_on_cancellation(
            server.handle,
            cancellation_token.clone(),
            String::from("shutting down test server"),
            server.address,
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

        drop(server.connection);
        wait_for_server(server.task).await;
    }

    struct StartedServer {
        handle: Handle<SocketAddr>,
        address: SocketAddr,
        connection: TcpStream,
        task: JoinHandle<()>,
        release_request: Arc<Notify>,
    }

    async fn start_server_with_releasable_in_flight_request() -> StartedServer {
        let release_request = Arc::new(Notify::new());
        start_server(release_request.clone(), move |release_request| async move {
            release_request.notified().await;
        })
        .await
    }

    async fn start_server_with_in_flight_request() -> StartedServer {
        start_server(Arc::new(Notify::new()), |_| async {
            std::future::pending::<()>().await;
        })
        .await
    }

    async fn start_server<F, Fut>(release_request: Arc<Notify>, handler: F) -> StartedServer
    where
        F: FnOnce(Arc<Notify>) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let handle = Handle::new();
        let server_handle = handle.clone();
        let request_started = Arc::new(Notify::new());
        let handler_request_started = request_started.clone();
        let handler_release_request = release_request.clone();
        let router = Router::new().route(
            "/",
            get(move || {
                let request_started = handler_request_started.clone();
                let release_request = handler_release_request.clone();
                let handler = handler.clone();

                async move {
                    request_started.notify_one();
                    handler(release_request).await;
                }
            }),
        );
        let server = tokio::spawn(async move {
            axum_server::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
                .handle(server_handle)
                .serve(router.into_make_service())
                .await
                .expect("server should stop without an error");
        });
        let address = handle.listening().await.expect("server should start listening");
        let mut connection = TcpStream::connect(address).await.expect("client should connect to server");
        connection
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
            .await
            .expect("client should send the request");
        request_started.notified().await;

        StartedServer {
            handle,
            address,
            connection,
            task: server,
            release_request,
        }
    }

    async fn wait_for_server(task: JoinHandle<()>) {
        tokio::time::timeout(Duration::from_secs(1), task)
            .await
            .expect("server should stop after the client connection closes")
            .expect("server task should not panic");
    }
}
