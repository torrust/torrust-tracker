//! Copy-Paste from axum. Thanks.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use derive_more::Display;
use tokio::sync::Notify;
use tokio::time::sleep;

/// A handle for [`Server`](crate::server::Server).
#[derive(Clone, Default, Display)]
#[display(fmt = "{}", "self.inner.get()")]
pub struct Handle {
    inner: Arc<HandleInner>,
}

impl std::fmt::Debug for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self.inner.get();

        f.debug_struct("Handle").field("state", &state).finish()
    }
}

#[derive(Default)]
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
    /// Awaits the gracefully exiting connections.
    ///
    /// - Short-circuits if called when there are no connections.
    /// - Upon elapsing of the deadline, shutdown is called.
    ///
    /// Note: this should be awaited after `wait_graceful_shutdown` returns, in a switch with `wait_shutdown`.
    ///
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

#[derive(Clone, Debug)]
struct HandleState {
    addr: Option<SocketAddr>,
    conn_count: usize,
    shutdown: bool,
    graceful: bool,
    graceful_dur: Option<Duration>,
    conn_end: bool,
}

impl HandleInner {
    fn get(&self) -> HandleState {
        let addr = *self.addr.lock().unwrap();
        let conn_count = self.conn_count.load(Ordering::SeqCst);
        let shutdown = self.shutdown.notified.load(Ordering::SeqCst);
        let graceful = self.graceful.notified.load(Ordering::SeqCst);
        let graceful_dur = *self.graceful_dur.lock().unwrap();
        let conn_end = self.conn_end.notified.load(Ordering::SeqCst);

        HandleState {
            addr,
            conn_count,
            shutdown,
            graceful,
            graceful_dur,
            conn_end,
        }
    }
}

impl std::fmt::Display for HandleState {
    #[allow(clippy::useless_format)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let has_active_connections = match self.conn_count {
            0 => format!("has no active connection"),
            1 => format!("has a single active connection"),
            n => format!("has {n} active connections"),
        };

        let listening_to = if let Some(addr) = self.addr {
            format!("listening to: {addr}")
        } else {
            format!("(without listener)")
        };

        let with_a_timeout = if let Some(timeout) = self.graceful_dur {
            format!("with timeout of {} seconds", timeout.as_secs_f32())
        } else {
            format!("of an indefinite timeout")
        };

        let status = match (self.shutdown, self.graceful, self.conn_end, self.graceful_dur) {
            // shutdown gracefully
            (true, true, true, _) => format!("stopped after a graceful shutdown {with_a_timeout}"),

            // shutdown but gracefully was interrupted (with timeout)
            (true, true, false, Some(_)) => {
                format!("interrupted (or deadline passed) a graceful shutdown {with_a_timeout}")
            }

            // shutdown but gracefully interrupted (without timeout)
            (true, true, false, None) =>                 format!("interrupted a graceful shutdown {with_a_timeout}")
            ,

            // shutdown abruptly
            (true, false, _, _) => format!("abruptly interrupted"),

            // not shutdown and gracefully completed
            (false, true, true, _) => format!("finished gracefully shutting down {with_a_timeout}"),

                        // not shutdown and gracefully exiting
                        (false, true, false, _) => format!("is gracefully shutting down {with_a_timeout}"),

            // not shutdown and not exiting
            (false, false, _, _) => format!("running"),
        };

        let state = match (self.shutdown || self.conn_end, self.addr, self.conn_count) {
            // stopped and without connections.
            // running and listening with active connections.
            (true, _, 0) | (false, Some(_), _) => format!("{listening_to}, was {status}, {has_active_connections}."),

            // stopped, is listening, but with connections!
            (true, Some(_), 1..) => format!("{listening_to}, was {status}, !unexpectedly! {has_active_connections}!"),

            // stopped, not listening, but somehow has connections!!
            (true, None, 1..) => {
                format!("{listening_to}, was {status}, !strangely and unexpectedly! {has_active_connections}!!")
            }

            // running, not listening, and without connections.
            (false, None, 0) => {
                format!("{listening_to}, was {status}, {has_active_connections}!")
            }

            // running, not listening, somehow without connections!
            (false, None, 1..) => {
                format!("{listening_to}, was {status}, !strangely! {has_active_connections}!")
            }
        };

        f.write_fmt(format_args!("state: {state}"))
    }
}
