use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::routing::get;
use axum_server::Handle;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_tracker_axum_server::signals::{GracefulShutdownOutcome, graceful_shutdown_on_cancellation};

#[tokio::main]
async fn main() {
    run_drained_scenario().await;
    run_timed_out_scenario().await;
}

async fn run_drained_scenario() {
    let mut server = start_server_with_releasable_request().await;
    let cancellation_token = CancellationToken::new();
    let shutdown = tokio::spawn(graceful_shutdown_on_cancellation(
        server.handle,
        cancellation_token.clone(),
        String::from("draining released request"),
        server.address,
        Duration::from_secs(2),
    ));

    cancellation_token.cancel();
    wait_until_new_connections_are_refused(server.address).await;
    server.release_request.notify_one();
    let mut response = Vec::new();
    server
        .connection
        .read_to_end(&mut response)
        .await
        .expect("client should receive the released response");

    let outcome = shutdown.await.expect("shutdown task should not panic");
    assert_eq!(outcome, GracefulShutdownOutcome::Drained);
    assert!(response.starts_with(b"HTTP/1.1 200 OK"));
    wait_for_server(server.task).await;
}

async fn run_timed_out_scenario() {
    let server = start_server_with_in_flight_request().await;
    let cancellation_token = CancellationToken::new();
    let shutdown = tokio::spawn(graceful_shutdown_on_cancellation(
        server.handle,
        cancellation_token.clone(),
        String::from("draining held request"),
        server.address,
        Duration::from_millis(100),
    ));

    cancellation_token.cancel();

    let outcome = shutdown.await.expect("shutdown task should not panic");
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

async fn start_server_with_releasable_request() -> StartedServer {
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
    F: Fn(Arc<Notify>) -> Fut + Clone + Send + Sync + 'static,
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
    let task = tokio::spawn(async move {
        axum_server::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
            .handle(server_handle)
            .serve(router.into_make_service())
            .await
            .expect("server should stop without an error");
    });
    let address = handle.listening().await.expect("server should start listening");
    let connection = TcpStream::connect(address).await.expect("client should connect to server");
    connection
        .try_write(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .expect("client should send the request");
    request_started.notified().await;

    StartedServer {
        handle,
        address,
        connection,
        task,
        release_request,
    }
}

async fn wait_for_server(task: JoinHandle<()>) {
    tokio::time::timeout(Duration::from_secs(1), task)
        .await
        .expect("server should stop after the client connection closes")
        .expect("server task should not panic");
}

async fn wait_until_new_connections_are_refused(address: SocketAddr) {
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            match TcpStream::connect(address).await {
                Ok(connection) => drop(connection),
                Err(_) => return,
            }

            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("server should refuse new connections after cancellation");
}
