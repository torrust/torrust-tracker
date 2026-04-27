//! Program to run qBittorrent E2E checks.
//!
//! Example:
//!
//! ```text
//! cargo run --bin qbittorrent_e2e_runner -- --compose-file ./compose.qbittorrent-e2e.yaml --timeout-seconds 300
//! ```
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use tracing::level_filters::LevelFilter;

use super::tracker::TrackerConfig;
use super::types::{ComposeProjectName, QbittorrentImage, TrackerImage};
use super::{filesystem_setup, scenarios, services_setup};

const TRACKER_IMAGE: &str = "torrust-tracker:qbt-e2e-local";
const QBITTORRENT_IMAGE: &str = "lscr.io/linuxserver/qbittorrent:5.1.4";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Compose file used for the qBittorrent scenario.
    #[clap(long, default_value = "compose.qbittorrent-e2e.yaml")]
    compose_file: PathBuf,

    /// Timeout in seconds for API operations.
    #[clap(long, default_value_t = 180)]
    timeout_seconds: u64,

    /// Local docker image tag used for the tracker service.
    #[clap(long, default_value = TRACKER_IMAGE)]
    tracker_image: String,

    /// qBittorrent image used for both seeder and leecher containers.
    #[clap(long, default_value = QBITTORRENT_IMAGE)]
    qbittorrent_image: String,

    /// Prefix for the random docker compose project name.
    #[clap(long, default_value = "qbt-e2e")]
    project_prefix: String,

    /// Leave containers running after the test finishes instead of tearing them
    /// down.  Useful for post-run debugging (e.g. `docker logs <container>`).
    #[clap(long, default_value_t = false)]
    keep_containers: bool,
}

/// Runs the qBittorrent E2E smoke orchestration.
///
/// # Errors
///
/// Returns an error when compose orchestration fails.
pub async fn run() -> anyhow::Result<()> {
    tracing_stdout_init(LevelFilter::INFO);

    let args = Args::parse();
    let project_name = ComposeProjectName::generate(&args.project_prefix);
    tracing::info!("Using compose project name: {project_name}");

    let timeout = Duration::from_secs(args.timeout_seconds);
    let tracker_config = TrackerConfig::default();

    let workspace = filesystem_setup::prepare(&project_name, args.keep_containers, timeout, &tracker_config)?;
    let resources = workspace.resources();

    let tracker_image = TrackerImage::new(&args.tracker_image);
    let qbittorrent_image = QbittorrentImage::new(&args.qbittorrent_image);

    let (mut running_compose, seeder, leecher, tracker) = services_setup::start(
        &args.compose_file,
        &project_name,
        &tracker_image,
        &qbittorrent_image,
        resources,
        &tracker_config,
    )
    .await?;

    scenarios::seeder_to_leecher_transfer::run(&seeder, &leecher, &tracker, resources).await?;

    // POST-SCENARIO: optionally keep containers for debugging.
    if args.keep_containers {
        tracing::info!(
            "Keeping containers alive for debugging. Project name: '{}'. \
             Workspace: '{}'. \
             Use `docker compose -p {} logs` to inspect them, \
             then `docker compose -p {} down --volumes` to clean up.",
            running_compose.project(),
            workspace.root_path().display(),
            running_compose.project(),
            running_compose.project(),
        );
        running_compose.keep();
    }

    Ok(())
}

fn tracing_stdout_init(filter: LevelFilter) {
    tracing_subscriber::fmt().with_max_level(filter).init();
    tracing::info!("Logging initialized");
}
