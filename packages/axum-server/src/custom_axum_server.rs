//! Wrapper for Axum server to add timeouts.
//!
//! Copyright (c) Eray Karatay ([@programatik29](https://github.com/programatik29)).
//!
//! See: <https://gist.github.com/programatik29/36d371c657392fd7f322e7342957b6d1>.
//!
//! If a client opens a HTTP connection and it does not send any requests, the
//! connection is closed after a timeout. You can test it with:
//!
//! ```text
//! telnet 127.0.0.1 1212
//! Trying 127.0.0.1...
//! Connected to 127.0.0.1.
//! Escape character is '^]'.
//! Connection closed by foreign host.
//! ```
//!
//! If you want to know more about Axum and timeouts see <https://github.com/josecelano/axum-server-timeout>.
use std::future::Ready;
use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use axum_server::Server;
use axum_server::accept::Accept;
use axum_server::tls_rustls::{RustlsAcceptor, RustlsConfig};
use futures_util::{Future, ready};
use http_body::{Body, Frame};
use hyper::Response;
use hyper_util::rt::TokioTimer;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::time::{Instant, Sleep};
use tower::Service;

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

/// Creates an Axum server from a TCP listener with the default timeouts.
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
pub fn from_tcp_with(socket: TcpListener, timeouts: Timeouts) -> ServerResult {
    axum_server::from_tcp(socket).map(|server| add_timeouts(server, &timeouts))
}

/// Creates an Axum server from a TCP listener with TLS and the default timeouts.
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

    server
}

#[derive(Clone)]
pub struct TimeoutAcceptor;

impl<I, S> Accept<I, S> for TimeoutAcceptor {
    type Stream = TimeoutStream<I>;
    type Service = TimeoutService<S>;
    type Future = Ready<std::io::Result<(Self::Stream, Self::Service)>>;

    fn accept(&self, stream: I, service: S) -> Self::Future {
        let (tx, rx) = mpsc::unbounded_channel();

        let stream = TimeoutStream::new(stream, Timeouts::default().http1_header_read, rx);
        let service = TimeoutService::new(service, tx);

        std::future::ready(Ok((stream, service)))
    }
}

#[derive(Clone)]
pub struct TimeoutService<S> {
    inner: S,
    sender: UnboundedSender<TimerSignal>,
}

impl<S> TimeoutService<S> {
    const fn new(inner: S, sender: UnboundedSender<TimerSignal>) -> Self {
        Self { inner, sender }
    }
}

impl<S, B, Request> Service<Request> for TimeoutService<S>
where
    S: Service<Request, Response = Response<B>>,
{
    type Response = Response<TimeoutBody<B>>;
    type Error = S::Error;
    type Future = TimeoutServiceFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        // send timer wait signal
        let _ = self.sender.send(TimerSignal::Wait);

        TimeoutServiceFuture::new(self.inner.call(req), self.sender.clone())
    }
}

pin_project! {
    pub struct TimeoutServiceFuture<F> {
        #[pin]
        inner: F,
        sender: Option<UnboundedSender<TimerSignal>>,
    }
}

impl<F> TimeoutServiceFuture<F> {
    const fn new(inner: F, sender: UnboundedSender<TimerSignal>) -> Self {
        Self {
            inner,
            sender: Some(sender),
        }
    }
}

impl<F, B, E> Future for TimeoutServiceFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
{
    type Output = Result<Response<TimeoutBody<B>>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.inner.poll(cx).map(|result| {
            result.map(|response| {
                response.map(|body| TimeoutBody::new(body, this.sender.take().expect("future polled after ready")))
            })
        })
    }
}

enum TimerSignal {
    Wait,
    Reset,
}

pin_project! {
    pub struct TimeoutBody<B> {
        #[pin]
        inner: B,
        sender: UnboundedSender<TimerSignal>,
    }
}

impl<B> TimeoutBody<B> {
    const fn new(inner: B, sender: UnboundedSender<TimerSignal>) -> Self {
        Self { inner, sender }
    }
}

impl<B: Body> Body for TimeoutBody<B> {
    type Data = B::Data;
    type Error = B::Error;

    fn poll_frame(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.project();
        let option = ready!(this.inner.poll_frame(cx));

        if option.is_none() {
            let _ = this.sender.send(TimerSignal::Reset);
        }

        Poll::Ready(option)
    }

    fn is_end_stream(&self) -> bool {
        let is_end_stream = self.inner.is_end_stream();

        if is_end_stream {
            let _ = self.sender.send(TimerSignal::Reset);
        }

        is_end_stream
    }

    fn size_hint(&self) -> http_body::SizeHint {
        self.inner.size_hint()
    }
}

pub struct TimeoutStream<IO> {
    inner: IO,
    // hyper requires unpin
    sleep: Pin<Box<Sleep>>,
    duration: Duration,
    waiting: bool,
    receiver: UnboundedReceiver<TimerSignal>,
    finished: bool,
}

impl<IO> TimeoutStream<IO> {
    fn new(inner: IO, duration: Duration, receiver: UnboundedReceiver<TimerSignal>) -> Self {
        Self {
            inner,
            sleep: Box::pin(tokio::time::sleep(duration)),
            duration,
            waiting: false,
            receiver,
            finished: false,
        }
    }
}

impl<IO: AsyncRead + Unpin> AsyncRead for TimeoutStream<IO> {
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        if !self.finished {
            match Pin::new(&mut self.receiver).poll_recv(cx) {
                // reset the timer
                Poll::Ready(Some(TimerSignal::Reset)) => {
                    self.waiting = false;

                    let deadline = Instant::now() + self.duration;
                    self.sleep.as_mut().reset(deadline);
                }
                // enter waiting mode (for response body last chunk)
                Poll::Ready(Some(TimerSignal::Wait)) => self.waiting = true,
                Poll::Ready(None) => self.finished = true,
                Poll::Pending => (),
            }
        }

        if !self.waiting {
            // return error if timer is elapsed
            if self.sleep.as_mut().poll(cx) == Poll::Ready(()) {
                return Poll::Ready(Err(std::io::Error::new(ErrorKind::TimedOut, "request header read timed out")));
            }
        }

        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl<IO: AsyncWrite + Unpin> AsyncWrite for TimeoutStream<IO> {
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.inner).poll_write_vectored(cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }
}
