//! This module contains functions to handle signals.

use derive_more::Display;
use futures::future::BoxFuture;
use futures::FutureExt;
use tracing::info;

/// This is the message that the "launcher" spawned task receives from the main
/// application process to notify the service to shutdown.
///
#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]
pub enum Halted {
    Normal,
    Dropped,
}

/// Creates a Future to Await the Terminate Signal (unix only)
///
/// # Panics
///
/// Panics if unable to connect to the global signal handle.
///
#[must_use]
pub fn global_terminate_signal<'a>() -> BoxFuture<'a, ()> {
    #[cfg(unix)]
    let terminate: BoxFuture<'a, ()> = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    }
    .boxed();

    #[cfg(not(unix))]
    let terminate: BoxFuture<'a, ()> = std::future::pending::<()>().boxed();

    terminate
}

/// Creates a Future to Await the Interrupt, i.e `ctrl_c` Signal
///
/// # Panics
///
/// Panics if unable to connect to the global signal handle.
///
#[must_use]
pub fn global_interrupt_signal<'a>() -> BoxFuture<'a, ()> {
    let interrupt: BoxFuture<'a, ()> = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    }
    .boxed();

    interrupt
}

/// Resolves on `ctrl_c` or the `terminate` signal.
///
pub async fn global_shutdown_signal() {
    let interrupt = global_interrupt_signal();
    let terminate = global_terminate_signal();

    tokio::select! {
        () = interrupt => {},
        () = terminate => {}
    }
}

/// Resolves when the `stop_receiver` or the `global_shutdown_signal()` resolves.
///
/// # Panics
///
/// Will panic if unable to connect to the receiving channel.
///
pub async fn shutdown_signal(rx_halt: tokio::sync::oneshot::Receiver<Halted>) {
    let halt = async {
        match rx_halt.await {
            Ok(signal) => signal,
            Err(err) => panic!("Failed to install stop signal: {err}"),
        }
    };

    tokio::select! {
        signal = halt => { info!("Halt signal processed: {}", signal) },
        () = global_shutdown_signal() => { info!("Global shutdown signal processed") }
    }
}

/// Same as `shutdown_signal()`, but shows a message when it resolves.
pub async fn shutdown_signal_with_message(rx_halt: tokio::sync::oneshot::Receiver<Halted>, message: String) {
    shutdown_signal(rx_halt).await;

    info!("{message}");
}
