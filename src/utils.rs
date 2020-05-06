use core::marker::Unpin;
/// In order to use `tokio` with `futures` we need a compatability, This is probably temporary.
use core::pin::Pin;
use core::task::{Context, Poll};

/// This allows you to convert a `tokio` `AsyncWrite` into a `futures` `AsyncWrite`
#[repr(transparent)]
pub struct TokioFuturesCompat<T: Unpin>(T);

impl<T: Unpin> TokioFuturesCompat<T> {
    fn get_self(self: Pin<&mut Self>) -> Pin<&mut T> {
        unsafe { self.map_unchecked_mut(|v| &mut v.0) }
    }
}

/// tokio::AsyncWrite -> futures::AsyncWrite
impl<T: tokio::io::AsyncWrite + Unpin> futures::io::AsyncWrite for TokioFuturesCompat<T> {
    fn poll_write(
        self: Pin<&mut Self>, cx: &mut futures::task::Context, buf: &[u8],
    ) -> futures::task::Poll<Result<usize, futures::io::Error>> {
        Self::get_self(self).poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>, cx: &mut futures::task::Context,
    ) -> futures::task::Poll<Result<(), futures::io::Error>> {
        Self::get_self(self).poll_flush(cx)
    }

    fn poll_close(
        self: Pin<&mut Self>, cx: &mut futures::task::Context,
    ) -> futures::task::Poll<Result<(), futures::io::Error>> {
        Self::get_self(self).poll_shutdown(cx)
    }
}

/// tokio::AsyncRead -> futures::AsyncRead
impl<T: tokio::io::AsyncRead + Unpin> futures::io::AsyncRead for TokioFuturesCompat<T> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<Result<usize, futures::io::Error>> {
        Self::get_self(self).poll_read(cx, buf)
    }
}

// tokio::AsyncBufRead -> futures::AsyncBufRead
impl<T: tokio::io::AsyncBufRead + Unpin> futures::io::AsyncBufRead for TokioFuturesCompat<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<&[u8], futures::io::Error>> {
        Self::get_self(self).poll_fill_buf(cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        Self::get_self(self).consume(amt)
    }
}

/// futures::AsyncRead -> tokio::AsyncRead
impl<T: Unpin + futures::io::AsyncRead> tokio::io::AsyncRead for TokioFuturesCompat<T> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<tokio::io::Result<usize>> {
        Self::get_self(self).poll_read(cx, buf)
    }
}

/// futures::AsyncReadBuf -> tokio::AsyncReadBuf
impl<T: Unpin + futures::io::AsyncBufRead> tokio::io::AsyncBufRead for TokioFuturesCompat<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context) -> Poll<tokio::io::Result<&[u8]>> {
        Self::get_self(self).poll_fill_buf(cx)
    }
    fn consume(self: Pin<&mut Self>, amt: usize) {
        Self::get_self(self).consume(amt)
    }
}

impl<T: Unpin> From<T> for TokioFuturesCompat<T> {
    fn from(v: T) -> TokioFuturesCompat<T> {
        Self(v)
    }
}
