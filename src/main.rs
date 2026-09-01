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
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Torrust tracker shutting down ...");

                    jobs.cancel();

                    jobs.wait_for_all(Duration::from_secs(10)).await;

                    tracing::info!("Torrust tracker successfully shutdown.");
                }
            }
        }
        Err(error) => {
            tracing::error!(%error, "Tracker startup failed");
            report_startup_failure(&error);
            std::process::exit(1);
        }
    }
}
