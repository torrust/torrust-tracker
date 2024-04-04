//! Copy-Paste from axum. Thanks.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::Notify;
use tokio::time::sleep;

/// A handle for [`Server`](crate::server::Server).
#[derive(Clone, Debug, Default)]
pub struct Handle {
    inner: Arc<HandleInner>,
}

#[derive(Debug, Default)]
struct HandleInner {
    addr: Mutex<Option<SocketAddr>>,
    addr_notify: Notify,
    conn_count: AtomicUsize,
    shutdown: NotifyOnce,
    graceful: NotifyOnce,
    graceful_dur: Mutex<Option<Duration>>,
    conn_end: NotifyOnce,
}

impl Handle {
    /// Create a new handle.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the number of connections.
    #[must_use]
    pub fn connection_count(&self) -> usize {
        self.inner.conn_count.load(Ordering::SeqCst)
    }

    /// Shutdown the server.
    pub fn shutdown(&self) {
        self.inner.shutdown.notify_waiters();
    }

    /// Gracefully shutdown the server.
    ///
    /// `None` means indefinite grace period.
    ///
    #[allow(clippy::missing_panics_doc)]
    pub fn graceful_shutdown(&self, duration: Option<Duration>) {
        *self.inner.graceful_dur.lock().unwrap() = duration;

        self.inner.graceful.notify_waiters();
    }

    /// Returns local address and port when server starts listening.
    ///
    /// Returns `None` if server fails to bind.
    ///
    #[allow(clippy::missing_panics_doc)]
    pub async fn listening(&self) -> Option<SocketAddr> {
        let notified = self.inner.addr_notify.notified();

        if let Some(addr) = *self.inner.addr.lock().unwrap() {
            return Some(addr);
        }

        notified.await;

        *self.inner.addr.lock().unwrap()
    }

    pub(crate) fn notify_listening(&self, addr: Option<SocketAddr>) {
        *self.inner.addr.lock().unwrap() = addr;

        self.inner.addr_notify.notify_waiters();
    }

    pub(crate) fn watcher(&self) -> Watcher {
        Watcher::new(self.clone())
    }

    pub(crate) async fn wait_shutdown(&self) {
        self.inner.shutdown.notified().await;
    }

    pub(crate) async fn wait_graceful_shutdown(&self) {
        self.inner.graceful.notified().await;
    }

    #[allow(dead_code)]
    pub(crate) async fn wait_connections_end(&self) {
        if self.inner.conn_count.load(Ordering::SeqCst) == 0 {
            return;
        }

        let deadline = *self.inner.graceful_dur.lock().unwrap();

        match deadline {
            Some(duration) => tokio::select! {
                biased;
                () = sleep(duration) => self.shutdown(),
                () = self.inner.conn_end.notified() => (),
            },
            None => self.inner.conn_end.notified().await,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Watcher {
    handle: Handle,
}

impl Watcher {
    fn new(handle: Handle) -> Self {
        handle.inner.conn_count.fetch_add(1, Ordering::SeqCst);

        Self { handle }
    }

    #[allow(dead_code)]
    pub(crate) async fn wait_graceful_shutdown(&self) {
        self.handle.wait_graceful_shutdown().await;
    }

    pub(crate) async fn wait_shutdown(&self) {
        self.handle.wait_shutdown().await;
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        let count = self.handle.inner.conn_count.fetch_sub(1, Ordering::SeqCst) - 1;

        if count == 0 && self.handle.inner.graceful.is_notified() {
            self.handle.inner.conn_end.notify_waiters();
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct NotifyOnce {
    notified: AtomicBool,
    notify: Notify,
}

impl NotifyOnce {
    pub(crate) fn notify_waiters(&self) {
        self.notified.store(true, Ordering::SeqCst);

        self.notify.notify_waiters();
    }

    pub(crate) fn is_notified(&self) -> bool {
        self.notified.load(Ordering::SeqCst)
    }

    pub(crate) async fn notified(&self) {
        let future = self.notify.notified();

        if !self.notified.load(Ordering::SeqCst) {
            future.await;
        }
    }
}
