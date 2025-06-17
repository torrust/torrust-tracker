use std::time::Duration;

use torrust_tracker_lib::app;

#[tokio::main]
async fn main() {
    let (_app_container, jobs) = app::run().await;

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Torrust tracker shutting down ...");

            jobs.cancel();

            jobs.wait_for_all(Duration::from_secs(10)).await;

            tracing::info!("Torrust tracker successfully shutdown.");
        }
    }
}
