use std::sync::Arc;
use log::info;
use torrust_tracker::Configuration;
use torrust_tracker::logging;
use torrust_tracker::setup;
use torrust_tracker::tracker::tracker::TorrentTracker;

extern crate crypto;

#[tokio::main]
async fn main() {
    const CONFIG_PATH: &str = "config.toml";

    // Initialize Torrust config
    let config = match Configuration::load_from_file(CONFIG_PATH) {
        Ok(config) => Arc::new(config),
        Err(error) => {
            panic!("{}", error)
        }
    };

    // Initialize Torrust tracker
    let tracker = match TorrentTracker::new(config.clone()) {
        Ok(tracker) => Arc::new(tracker),
        Err(error) => {
            panic!("{}", error)
        }
    };

    // Initialize logging
    logging::setup_logging(&config);

    // Run jobs
    let jobs = setup::setup(&config, tracker.clone()).await;

    // handle the signals here
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Torrust shutting down..");

            // Await for all jobs to shutdown
            futures::future::join_all(jobs).await;
            info!("Torrust successfully shutdown.");
        }
    }
}
