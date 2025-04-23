use torrust_tracker_lib::app;

#[tokio::main]
async fn main() {
    let (_app_container, jobs) = app::run().await;

    // handle the signals
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Torrust shutting down ...");

            // Await for all jobs to shutdown
            futures::future::join_all(jobs).await;
            tracing::info!("Torrust successfully shutdown.");
        }
    }
}
