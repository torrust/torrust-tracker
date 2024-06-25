use std::net::SocketAddr;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use futures::future::BoxFuture;
use futures::FutureExt as _;
use tracing::{error, info, trace, warn};

use crate::servers::service;
use crate::servers::signals::{global_interrupt_signal, global_terminate_signal};
use crate::shared::handle::{Handle as InnerHandle, NotifyOnce};

#[derive(Debug)]
pub struct Handle {
    inner: InnerHandle,
    abort: Vec<tokio::task::AbortHandle>,
    shutdown: Arc<NotifyOnce>,
    graceful: Arc<NotifyOnce>,
    conn_end: Arc<NotifyOnce>,
}

impl Default for Handle {
    fn default() -> Self {
        let inner = InnerHandle::default();

        Self::new(inner)
    }
}

impl service::Handle for Handle {
    fn stop(self) -> Result<(), service::Error> {
        warn!("now triggering the udp tracker direct-shutdown process!");
        self.inner.shutdown();
        Ok(())
    }

    fn into_graceful_shutdown_future<'a>(self) -> BoxFuture<'a, Result<(), service::Error>> {
        todo!();
    }

    fn listening(&self) -> service::AddrFuture<'_> {
        self.inner.listening().boxed()
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        warn!("triggering the direct-shutdown from the drop handler!");
        self.inner.shutdown();
        self.shutdown.notify_waiters();
        self.abort.iter().for_each(tokio::task::AbortHandle::abort);
    }
}

#[allow(dead_code)]
enum Task {
    Terminate,
    Interrupt,
    Shutdown,
    Graceful,
    Listening(Option<SocketAddr>),
}

impl Handle {
    #[must_use]
    pub fn new(inner: InnerHandle) -> Self {
        let mut abort: Vec<tokio::task::AbortHandle> = Vec::default();
        let shutdown: Arc<NotifyOnce> = Arc::default();
        let graceful: Arc<NotifyOnce> = Arc::default();
        let conn_end: Arc<NotifyOnce> = Arc::default();

        abort.push(tokio::task::spawn(Self::notify_shutdown(inner.clone(), shutdown.clone())).abort_handle());
        abort.push(tokio::task::spawn(Self::notify_graceful(inner.clone(), graceful.clone())).abort_handle());
        abort.push(tokio::task::spawn(Self::notify_conn_end(inner.clone(), conn_end.clone())).abort_handle());

        let mut handle = Handle {
            inner,
            abort,
            shutdown,
            graceful,
            conn_end,
        };

        drop(handle.graceful_init());

        handle
    }

    async fn notify_shutdown(inner: InnerHandle, shutdown: Arc<NotifyOnce>) {
        inner.wait_shutdown().await;
        shutdown.notify_waiters();
    }

    async fn notify_graceful(inner: InnerHandle, graceful: Arc<NotifyOnce>) {
        inner.wait_graceful_shutdown().await;
        graceful.notify_waiters();
    }

    async fn notify_conn_end(inner: InnerHandle, conn_end: Arc<NotifyOnce>) {
        inner.wait_connections_end().await;
        conn_end.notify_waiters();
    }

    fn graceful_init(&mut self) -> tokio::task::JoinSet<()> {
        let handle = self.inner.clone();
        let shutdown = self.shutdown.clone();
        let graceful = self.graceful.clone();
        let conn_end = self.conn_end.clone();

        let mut jobs = tokio::task::JoinSet::default();

        self.abort
            .push(jobs.spawn(Self::graceful_start(handle.clone(), shutdown.clone(), graceful.clone())));
        self.abort
            .push(jobs.spawn(Self::shutdown_loop(handle.clone(), shutdown.clone(), graceful.clone())));
        self.abort
            .push(jobs.spawn_blocking(move || Self::graceful_exit_loop(&handle, &shutdown, &graceful, &conn_end)));

        jobs
    }

    async fn graceful_start(handle: InnerHandle, shutdown: Arc<NotifyOnce>, graceful: Arc<NotifyOnce>) {
        let handle = handle.clone();

        let shutdown_wait = shutdown.notified();
        let graceful_wait = graceful.notified();
        let listening_wait = handle.listening();

        let start: Task = tokio::select! {
            biased;
            () = global_terminate_signal() => { Task::Terminate}
            () = global_interrupt_signal() => { Task::Interrupt}
            () = shutdown_wait => { Task::Shutdown}
            () = graceful_wait => { Task::Graceful}
            maybe_socket = listening_wait => { Task::Listening(maybe_socket)}
        };

        match start {
            Task::Listening(maybe_socket) => {
                if let Some(addr) = maybe_socket {
                    info!("Listening to: {addr}");
                } else {
                    warn!("Failed to start Listening");
                    handle.shutdown();
                }
            }

            Task::Terminate | Task::Interrupt => {
                warn!("Global Signal Before Listening");
                handle.shutdown();
            }

            Task::Shutdown | Task::Graceful => {
                warn!("Local Signal Before Listening");
                handle.shutdown();
            }
        }
    }

    async fn shutdown_loop(handle: InnerHandle, shutdown: Arc<NotifyOnce>, graceful: Arc<NotifyOnce>) {
        const GRACEFUL_DURATION: Duration = Duration::from_secs(90);

        loop {
            let handle = handle.clone();
            let shutdown = shutdown.clone();
            let graceful = graceful.clone();

            let shutdown_wait = shutdown.notified();

            let exit: Task = tokio::select! {
                biased;
                () = global_terminate_signal() => { Task::Terminate}
                () = global_interrupt_signal() => { Task::Interrupt}
                () = shutdown_wait => { Task::Shutdown}
            };

            let () = match exit {
                Task::Interrupt => {
                    if graceful.is_notified() {
                        error! {"interrupting graceful shutdown... exiting now!"};
                        handle.shutdown();

                        break;
                    }
                    warn! {"caught interrupt: starting graceful shutdown... for {} sec..", GRACEFUL_DURATION.as_secs()};
                    handle.graceful_shutdown(Some(GRACEFUL_DURATION));

                    continue;
                }

                Task::Terminate => {
                    warn!("Got Terminate: Exiting Now!");
                    handle.shutdown();

                    return;
                }
                Task::Shutdown => {
                    info!("Received Shutdown, Closing Service.");
                    return;
                }
                _ => unreachable!(),
            };
        }
    }

    fn graceful_exit_loop(
        handle: &InnerHandle,
        shutdown: &Arc<NotifyOnce>,
        graceful: &Arc<NotifyOnce>,
        conn_end: &Arc<NotifyOnce>,
    ) {
        #[allow(clippy::useless_format)]
        for n in 1_u64.. {
            let listening_to = if let Some(addr) = handle.listening().now_or_never().flatten() {
                format!("listening to: {addr}")
            } else {
                format!("(without listener)")
            };

            match (shutdown.is_notified(), graceful.is_notified()) {
                (true, _) => {
                    error!("halting service {listening_to} without graceful shutdown!");
                    return;
                }
                (false, true) => {
                    info!(
                        "
            after running for {n} seconds...
            now gracefully shutting down udp service {listening_to}...
            
            to cancel and exit now interrupt with \"ctrl+c\"
            "
                    );
                    break;
                }
                (false, false) => {
                    trace!("udp service {listening_to} tick no: {n}");
                    sleep(Duration::from_millis(1000));
                    continue;
                }
            }
        }

        for n in 1_u64.. {
            if shutdown.is_notified() {
                error!("graceful shutdown interrupted, exiting now!");
                return;
            }

            info!(
                "shutting down for ~{n} seconds, with {} remaining connections...",
                handle.connection_count()
            );

            if conn_end.is_notified() {
                break;
            };

            sleep(Duration::from_secs(1));
        }

        match handle.connection_count() {
            0 => {
                info!("successfully finished shutting down udp service (:");
            }

            n => {
                warn!("shutting down udp service timed out with {n} active connections");
            }
        };
    }
}
