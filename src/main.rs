use std::time::Duration;

use torrust_tracker_lib::app;

#[allow(clippy::print_stderr)]
fn report_startup_failure(error: &app::Error) {
    eprintln!("Tracker startup failed: {error}");
}

#[tokio::main]
async fn main() {
    match app::start().await {
        Ok((_app_container, jobs)) => {
            #[cfg(unix)]
            let shutdown_signal = {
                let ctrl_c = tokio::signal::ctrl_c();
                tokio::pin!(ctrl_c);
                let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("failed to install SIGTERM handler");

                tokio::select! {
                    biased;
                    result = &mut ctrl_c => {
                        result.expect("failed to install Ctrl-C handler");
                        "SIGINT"
                    },
                    _ = sigterm.recv() => "SIGTERM",
                    () = std::future::ready(()) => {
                        tracing::info!("Tracker shutdown signal handlers installed.");

                        tokio::select! {
                            result = &mut ctrl_c => {
                                result.expect("failed to install Ctrl-C handler");
                                "SIGINT"
                            },
                            _ = sigterm.recv() => "SIGTERM",
                        }
                    },
                }
            };

            #[cfg(not(unix))]
            let shutdown_signal = {
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
            };

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
