//! Idle-connection regression tests for the shared Axum listener constructors
//! (issue #2776).
//!
//! Every test injects short timeouts and an absolute test deadline, so a
//! regression fails in a bounded time and never waits on the 5 s production
//! values.
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Once};
use std::time::Duration;

use axum::Router;
use axum::routing::get;
use axum_server::Handle;
use axum_server::tls_rustls::RustlsConfig;
use rustls::RootCertStore;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, ServerName};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::Instant;
use tokio_rustls::TlsConnector;
use torrust_tracker_axum_server::custom_axum_server::{self, Timeouts};

const CERTIFICATE: &[u8] = include_bytes!("fixtures/https-health-check-cert.pem");
const KEY: &[u8] = include_bytes!("fixtures/https-health-check-key.pem");

/// Timeouts injected into the listener under test.
const SHORT: Duration = Duration::from_millis(300);

/// Absolute bound: a connection the server still keeps open after this long is
/// reported as not closed.
const TEST_DEADLINE: Duration = Duration::from_secs(3);

const fn short_timeouts() -> Timeouts {
    Timeouts {
        first_request: SHORT,
        http1_header_read: SHORT,
        http2_keep_alive_timeout: SHORT,
        http2_keep_alive_interval: SHORT,
        tls_handshake: SHORT,
    }
}

fn install_crypto_provider() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("ring should be the Rustls crypto provider for the tests");
    });
}

trait Io: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> Io for T {}

type Stream = Box<dyn Io>;

#[derive(Clone, Copy, Debug)]
enum Transport {
    Plain,
    Tls,
}

struct TestServer {
    addr: SocketAddr,
    transport: Transport,
    handle: Handle<SocketAddr>,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.handle.shutdown();
    }
}

fn router() -> Router {
    Router::new().route("/health_check", get(|| async { "OK" }))
}

fn bind() -> TcpListener {
    let socket = TcpListener::bind("127.0.0.1:0").expect("the test socket should bind");
    socket.set_nonblocking(true).expect("the socket should be non-blocking");
    socket
}

async fn start(transport: Transport, timeouts: Timeouts) -> TestServer {
    install_crypto_provider();

    let socket = bind();
    let addr = socket.local_addr().expect("the socket should have an address");
    let handle = Handle::new();

    match transport {
        Transport::Plain => {
            let server = custom_axum_server::from_tcp_with(socket, timeouts)
                .expect("the plain server should be created")
                .handle(handle.clone());
            tokio::spawn(server.serve(router().into_make_service()));
        }
        Transport::Tls => {
            let tls = RustlsConfig::from_pem(CERTIFICATE.to_vec(), KEY.to_vec())
                .await
                .expect("the test certificate should load");
            let server = custom_axum_server::from_tcp_rustls_with(socket, tls, timeouts)
                .expect("the TLS server should be created")
                .handle(handle.clone());
            tokio::spawn(server.serve(router().into_make_service()));
        }
    }

    TestServer { addr, transport, handle }
}

async fn tls_connect(tcp: TcpStream, alpn: &[&[u8]]) -> std::io::Result<Stream> {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from_pem_slice(CERTIFICATE).expect("the test certificate should parse"))
        .expect("the test certificate should be a valid root");
    let mut config = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    config.alpn_protocols = alpn.iter().map(|p| p.to_vec()).collect();

    let name = ServerName::try_from("127.0.0.1").expect("an IP address is a valid server name");
    let stream = TlsConnector::from(Arc::new(config)).connect(name, tcp).await?;

    Ok(Box::new(stream))
}

impl TestServer {
    /// Connects and completes the TLS handshake when the server uses TLS.
    async fn connect(&self, alpn: &[&[u8]]) -> Stream {
        let tcp = TcpStream::connect(self.addr).await.expect("the server should accept");

        match self.transport {
            Transport::Plain => Box::new(tcp),
            Transport::Tls => tls_connect(tcp, alpn).await.expect("the TLS handshake should complete"),
        }
    }
}

/// Reads until the server closes the connection. Returns the elapsed time, or
/// `None` if the connection is still open at the test deadline.
async fn closed_within_deadline(stream: &mut (impl AsyncRead + Unpin)) -> Option<Duration> {
    let start = Instant::now();
    let mut buffer = [0_u8; 1024];

    tokio::time::timeout(TEST_DEADLINE, async {
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) | Err(_) => return,
                Ok(_) => {}
            }
        }
    })
    .await
    .ok()
    .map(|()| start.elapsed())
}

async fn read_until_contains(stream: &mut (impl AsyncRead + Unpin), needle: &[u8]) {
    let mut received = Vec::new();
    let mut buffer = [0_u8; 1024];

    tokio::time::timeout(TEST_DEADLINE, async {
        while !received.windows(needle.len()).any(|w| w == needle) {
            let n = stream.read(&mut buffer).await.expect("the response should be readable");
            assert!(n > 0, "the server closed before answering");
            received.extend_from_slice(&buffer[..n]);
        }
    })
    .await
    .expect("the response should arrive before the test deadline");
}

const HTTP1_REQUEST: &[u8] = b"GET /health_check HTTP/1.1\r\nHost: localhost\r\n\r\n";

/// HTTP/2 client preface, an empty SETTINGS frame, and a `GET /` request on stream 1 (HPACK static table entries).
fn http2_request() -> Vec<u8> {
    let mut bytes = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n".to_vec();
    bytes.extend_from_slice(&[0, 0, 0, 4, 0, 0, 0, 0, 0]); // SETTINGS
    let headers: Vec<u8> = [
        &[0x82_u8, 0x84, 0x86][..], // :method GET, :path /, :scheme http
        &[0x01, 9][..],             // :authority, literal value of length 9
        b"localhost",
    ]
    .concat();
    let len = u32::try_from(headers.len()).expect("a short header block");
    bytes.extend_from_slice(&len.to_be_bytes()[1..]); // 24-bit length
    bytes.extend_from_slice(&[1, 0x05, 0, 0, 0, 1]); // HEADERS, END_STREAM | END_HEADERS, stream 1
    bytes.extend_from_slice(&headers);
    bytes
}

// A client that connects and sends nothing must be closed (AC1, AC2).

#[tokio::test]
async fn it_should_close_a_plaintext_connection_that_sends_nothing() {
    let server = start(Transport::Plain, short_timeouts()).await;
    let mut stream = server.connect(&[]).await;

    let closed_after = closed_within_deadline(&mut stream).await;

    assert!(closed_after.is_some(), "a silent plaintext connection should be closed");
}

#[tokio::test]
async fn it_should_close_a_tls_connection_that_completes_the_handshake_and_sends_nothing() {
    let server = start(Transport::Tls, short_timeouts()).await;
    let mut stream = server.connect(&[]).await;

    let closed_after = closed_within_deadline(&mut stream).await;

    assert!(closed_after.is_some(), "a silent TLS connection should be closed");
}

// A TLS client that never completes the handshake must be closed (AC3).

#[tokio::test]
async fn it_should_close_a_tls_connection_that_never_sends_handshake_bytes() {
    let server = start(Transport::Tls, short_timeouts()).await;
    let mut tcp = TcpStream::connect(server.addr).await.expect("the server should accept");

    let closed_after = closed_within_deadline(&mut tcp).await;

    assert!(closed_after.is_some(), "a stalled TLS handshake should be closed");
}

#[tokio::test]
async fn it_should_close_a_tls_connection_that_sends_one_handshake_byte_and_stalls() {
    let server = start(Transport::Tls, short_timeouts()).await;
    let mut tcp = TcpStream::connect(server.addr).await.expect("the server should accept");
    tcp.write_all(&[0x16]).await.expect("one byte should be written");

    let closed_after = closed_within_deadline(&mut tcp).await;

    assert!(closed_after.is_some(), "a half-sent TLS handshake should be closed");
}

// A persistent connection that is idle after a response must be closed (AC4).

#[tokio::test]
async fn it_should_close_an_idle_http1_connection_after_a_response_over_plaintext() {
    let server = start(Transport::Plain, short_timeouts()).await;
    let mut stream = server.connect(&[]).await;
    stream.write_all(HTTP1_REQUEST).await.expect("the request should be written");
    read_until_contains(&mut stream, b"OK").await;

    let closed_after = closed_within_deadline(&mut stream).await;

    assert!(
        closed_after.is_some(),
        "an idle HTTP/1 connection should be closed after its response"
    );
}

#[tokio::test]
async fn it_should_close_an_idle_http1_connection_after_a_response_over_tls() {
    let server = start(Transport::Tls, short_timeouts()).await;
    let mut stream = server.connect(&[b"http/1.1"]).await;
    stream.write_all(HTTP1_REQUEST).await.expect("the request should be written");
    read_until_contains(&mut stream, b"OK").await;

    let closed_after = closed_within_deadline(&mut stream).await;

    assert!(
        closed_after.is_some(),
        "an idle HTTP/1 TLS connection should be closed after its response"
    );
}

#[tokio::test]
async fn it_should_close_an_idle_http2_connection_after_a_response_over_plaintext() {
    let server = start(Transport::Plain, short_timeouts()).await;
    let mut stream = server.connect(&[]).await;
    stream
        .write_all(&http2_request())
        .await
        .expect("the request should be written");

    // The client never acknowledges the server's keep-alive pings.
    let closed_after = closed_within_deadline(&mut stream).await;

    assert!(
        closed_after.is_some(),
        "an idle HTTP/2 connection should be closed after its response"
    );
}

#[tokio::test]
async fn it_should_close_an_idle_http2_connection_after_a_response_over_tls() {
    let server = start(Transport::Tls, short_timeouts()).await;
    let mut stream = server.connect(&[b"h2"]).await;
    stream
        .write_all(&http2_request())
        .await
        .expect("the request should be written");

    let closed_after = closed_within_deadline(&mut stream).await;

    assert!(
        closed_after.is_some(),
        "an idle HTTP/2 TLS connection should be closed after its response"
    );
}

// The timeouts must not break a client that behaves.

#[tokio::test]
async fn it_should_answer_a_request_sent_within_the_first_request_timeout() {
    let server = start(Transport::Plain, short_timeouts()).await;
    let mut stream = server.connect(&[]).await;

    stream.write_all(HTTP1_REQUEST).await.expect("the request should be written");

    read_until_contains(&mut stream, b"200 OK").await;
}
