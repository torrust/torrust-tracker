use std::time::Duration;

use torrust_tracker_lib::app;

#[tokio::main]
async fn main() {
    match app::start().await {
        Ok((_app_container, jobs)) => {
            let shutdown_signal = wait_for_shutdown_signal().await;

            tracing::info!("Torrust tracker shutting down ({shutdown_signal}) ...");

            jobs.cancel();

            jobs.wait_for_all(Duration::from_secs(10)).await;

            tracing::info!("Torrust tracker successfully shutdown.");
        }
        Err(error) => {
            tracing::error!(%error, "Tracker startup failed");
            report_startup_failure(&error);
            std::process::exit(1);
        }
    }
}

/// Waits until the process receives a supported shutdown signal.
///
/// Tokio registers the Ctrl-C listener when its future is first polled. The
/// outer, biased `select!` polls Ctrl-C after the SIGTERM stream is created
/// and before its immediately-ready branch logs the observable readiness
/// marker. The native executable-boundary tests wait for that marker, so they
/// can signal the child without racing listener registration.
///
/// Pin `ctrl_c` because it is polled in the outer `select!` and then awaited
/// again in the inner signal wait.
#[cfg(unix)]
async fn wait_for_shutdown_signal() -> &'static str {
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);
    let mut sigterm =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).expect("failed to install SIGTERM handler");

    tokio::select! {
        biased;
        result = &mut ctrl_c => {
            result.expect("failed to install Ctrl-C handler");
            "SIGINT"
        },
        result = sigterm.recv() => {
            result.expect("SIGTERM handler stream closed unexpectedly");
            "SIGTERM"
        },
        () = std::future::ready(()) => {
            tracing::info!("Tracker shutdown signal handlers installed.");

            tokio::select! {
                result = &mut ctrl_c => {
                    result.expect("failed to install Ctrl-C handler");
                    "SIGINT"
                },
                result = sigterm.recv() => {
                    result.expect("SIGTERM handler stream closed unexpectedly");
                    "SIGTERM"
                },
            }
        },
    }
}

/// Waits until the process receives Ctrl-C on a non-Unix platform.
#[cfg(not(unix))]
async fn wait_for_shutdown_signal() -> &'static str {
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    tokio::select! {
        biased;
        result = &mut ctrl_c => {
            result.expect("failed to install Ctrl-C handler");
            "SIGINT"
        },
        _ = std::future::ready(()) => {
            tracing::info!("Tracker shutdown signal handlers installed.");
            ctrl_c.await.expect("failed to install Ctrl-C handler");
            "SIGINT"
        },
    }
}

#[allow(clippy::print_stderr)]
fn report_startup_failure(error: &app::Error) {
    eprintln!("Tracker startup failed: {error}");
}
