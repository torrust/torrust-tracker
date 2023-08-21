//! This module contains functions to handle signals.
use log::info;

/// Resolves on `ctrl_c` or the `terminate` signal.
///
/// # Panics
///
/// Will panic if the `ctrl_c` or `terminate` signal resolves with an error.
pub async fn global_shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {}
    }
}

/// Resolves when the `stop_receiver` or the `global_shutdown_signal()` resolves.
///
/// # Panics
///
/// Will panic if the `stop_receiver` resolves with an error.
pub async fn shutdown_signal(stop_receiver: tokio::sync::oneshot::Receiver<u8>) {
    let stop = async { stop_receiver.await.expect("Failed to install stop signal.") };

    tokio::select! {
        _ = stop => {},
        () = global_shutdown_signal() => {}
    }
}

/// Same as `shutdown_signal()`, but shows a message when it resolves.
pub async fn shutdown_signal_with_message(stop_receiver: tokio::sync::oneshot::Receiver<u8>, message: String) {
    shutdown_signal(stop_receiver).await;

    info!("{message}");
}
