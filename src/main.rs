use torrust_tracker::{app, bootstrap};
use tracing::info;

#[tokio::main]
async fn main() {
    let (config, level) = bootstrap::app::config();

    let () = tracing_subscriber::fmt().compact().with_max_level(level).init();

    let tracker = bootstrap::app::tracker(&config);

    let jobs = app::start(&config, tracker).await;

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
