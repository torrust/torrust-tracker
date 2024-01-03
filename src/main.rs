use log::info;
use torrust_tracker::{app, bootstrap};

#[tokio::main]
async fn main() {
    let (config, tracker) = bootstrap::app::setup();

    let jobs = app::start(config.into(), tracker.clone()).await;

    // handle the signals
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Torrust shutting down..");

            // Await for all jobs to shutdown
            futures::future::join_all(jobs).await;
            info!("Torrust successfully shutdown.");
        }
    }
}
