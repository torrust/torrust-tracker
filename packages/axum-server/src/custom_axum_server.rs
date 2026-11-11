//! Constructors for the Axum servers with the tracker's connection timeouts.
//!
//! A client that opens a connection and never sends a request is disconnected
//! after [`Timeouts::first_request`], for plaintext and TLS listeners. You can
//! test it with:
//!
//! ```text
//! telnet 127.0.0.1 1212
//! Trying 127.0.0.1...
//! Connected to 127.0.0.1.
//! Escape character is '^]'.
//! Connection closed by foreign host.
//! ```
//!
//! The server closes the connection without sending a `408` response.
//!
//! The timeouts come from `axum-server` 0.9 (`Server::first_request_timeout`, a
//! default timer and the TLS handshake timeout) and from Hyper (header read
//! timeout and HTTP/2 keep-alive). They replace the `TimeoutAcceptor` copied
//! from [programatik29's gist](https://gist.github.com/programatik29/36d371c657392fd7f322e7342957b6d1),
//! which did not work with TLS and did not cover the health check API. See
//! issue #2776.
//!
//! Every Axum listener of the tracker must be built here. `clippy.toml` forbids calling the
//! `axum-server` constructors elsewhere, because a listener built without these timeouts keeps idle
//! connections open and lets silent connections exhaust the process's file descriptors. See the ADR
//! `docs/adrs/20261111095222_close_idle_connections_without_a_408_response.md`.
//!
//! If you want to know more about Axum and timeouts see <https://github.com/josecelano/axum-server-timeout>.
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;

use axum_server::Server;
use axum_server::tls_rustls::{RustlsAcceptor, RustlsConfig};
use hyper_util::rt::TokioTimer;

type RustlsServerResult = Result<Server<SocketAddr, RustlsAcceptor>, std::io::Error>;
type ServerResult = Result<Server<SocketAddr>, std::io::Error>;

/// Connection timeouts applied to every Axum listener.
///
/// The defaults are the values the tracker has always used for plaintext
/// listeners (5 s), plus the `axum-server` default TLS handshake bound (10 s).
/// The fields are public so tests can inject short values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timeouts {
    /// Time a new connection has to send a complete first request.
    pub first_request: Duration,
    /// Hyper HTTP/1 header read timeout.
    pub http1_header_read: Duration,
    /// Hyper HTTP/2 keep-alive timeout (wait for a ping acknowledgement).
    pub http2_keep_alive_timeout: Duration,
    /// Hyper HTTP/2 keep-alive interval (time between pings).
    pub http2_keep_alive_interval: Duration,
    /// Time a TLS client has to complete the handshake.
    pub tls_handshake: Duration,
}

impl Default for Timeouts {
    fn default() -> Self {
        Self {
            first_request: Duration::from_secs(5),
            http1_header_read: Duration::from_secs(5),
            http2_keep_alive_timeout: Duration::from_secs(5),
            http2_keep_alive_interval: Duration::from_secs(5),
            tls_handshake: Duration::from_secs(10),
        }
    }
}

/// Creates an Axum server from a TCP listener with the default timeouts ([`Timeouts::default`]).
///
/// The name says the server has timeouts, not that the caller supplies them; use
/// [`from_tcp_with`] to choose the values.
///
/// # Errors
///
/// Returns an error if the server cannot be created from the TCP socket.
pub fn from_tcp_with_timeouts(socket: TcpListener) -> ServerResult {
    from_tcp_with(socket, Timeouts::default())
}

/// Creates an Axum server from a TCP listener with the given timeouts.
///
/// # Errors
///
/// Returns an error if the server cannot be created from the TCP socket.
#[allow(
    clippy::disallowed_methods,
    reason = "this is the single place that builds listeners; every other crate must call it so the connection timeouts apply (#2776)"
)]
pub fn from_tcp_with(socket: TcpListener, timeouts: Timeouts) -> ServerResult {
    axum_server::from_tcp(socket).map(|server| add_timeouts(server, &timeouts))
}

/// Creates an Axum server from a TCP listener with TLS and the default timeouts ([`Timeouts::default`]).
///
/// Use [`from_tcp_rustls_with`] to choose the values.
///
/// # Errors
///
/// Returns an error if the server cannot be created from the TCP socket or if TLS configuration fails.
pub fn from_tcp_rustls_with_timeouts(socket: TcpListener, tls: RustlsConfig) -> RustlsServerResult {
    from_tcp_rustls_with(socket, tls, Timeouts::default())
}

/// Creates an Axum server from a TCP listener with TLS and the given timeouts.
///
/// # Errors
///
/// Returns an error if the server cannot be created from the TCP socket or if TLS configuration fails.
#[allow(
    clippy::disallowed_methods,
    reason = "this is the single place that builds listeners; every other crate must call it so the connection timeouts apply (#2776)"
)]
pub fn from_tcp_rustls_with(socket: TcpListener, tls: RustlsConfig, timeouts: Timeouts) -> RustlsServerResult {
    let acceptor = RustlsAcceptor::new(tls).handshake_timeout(timeouts.tls_handshake);

    axum_server::from_tcp(socket).map(|server| add_timeouts(server.acceptor(acceptor), &timeouts))
}

fn add_timeouts<Addr: axum_server::Address, Acc>(mut server: Server<Addr, Acc>, timeouts: &Timeouts) -> Server<Addr, Acc> {
    server.http_builder().http1().timer(TokioTimer::new());
    server.http_builder().http2().timer(TokioTimer::new());

    server.http_builder().http1().header_read_timeout(timeouts.http1_header_read);
    server
        .http_builder()
        .http2()
        .keep_alive_timeout(timeouts.http2_keep_alive_timeout)
        .keep_alive_interval(timeouts.http2_keep_alive_interval);

    // Closes connections that never send a complete first request, for plaintext and TLS (after the handshake).
    server.first_request_timeout(timeouts.first_request)
}
