//! Binary entry point for the qBittorrent end-to-end smoke test.
//!
//! This runner validates the full `BitTorrent` seeder→tracker→leecher flow using
//! real qBittorrent 5.1.4 containers:
//!
//! 1. Builds a local Torrust Tracker Docker image.
//! 2. Creates an ephemeral workspace (temporary directory) with all required
//!    configuration files and pre-generated torrent + payload.
//! 3. Starts a Docker Compose stack (`compose.qbittorrent-e2e.yaml`) containing
//!    a tracker, a seeder, and a leecher — all using randomly assigned host ports
//!    so multiple runs can coexist.
//! 4. Authenticates with both `qBittorrent` `WebUI` instances.
//! 5. Uploads the torrent to the seeder and the leecher.
//! 6. Logs the torrent count reported by each client.
//! 7. Tears down the compose stack (RAII — even on failure).
//!
//! # Prerequisites
//!
//! - Docker (or compatible OCI runtime) must be installed and running.
//! - The `docker compose` plugin (v2) must be available on `PATH`.
//! - The workspace must be the repository root (default compose file and tracker
//!   config template are resolved relative to the current working directory).
//!
//! # Usage
//!
//! ```text
//! cargo run --bin qbittorrent_e2e_runner -- \
//!     --compose-file ./compose.qbittorrent-e2e.yaml \
//!     --timeout-seconds 180
//! ```
//!
//! ## Key CLI flags
//!
//! | Flag | Default | Description |
//! |------|---------|-------------|
//! | `--compose-file` | `compose.qbittorrent-e2e.yaml` | Compose file for the scenario |
//! | `--tracker-config-template` | `share/default/config/tracker.e2e.container.sqlite3.toml` | Tracker config copied into the workspace |
//! | `--timeout-seconds` | `180` | Per-operation HTTP timeout for `WebUI` calls |
//! | `--tracker-image` | `torrust-tracker:qbt-e2e-local` | Local Docker image tag built for the tracker |
//! | `--qbittorrent-image` | `lscr.io/linuxserver/qbittorrent:5.1.4` | qBittorrent image for seeder and leecher |
//! | `--project-prefix` | `qbt-e2e` | Prefix for the randomised compose project name |
//!
//! # Debugging
//!
//! See `contrib/dev-tools/debugging/qbt/` for standalone shell scripts that
//! probe a single qBittorrent container in isolation and validate the compose
//! stack without running the full Rust runner.
use torrust_tracker_lib::console::ci::qbittorrent_e2e;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    qbittorrent_e2e::runner::run().await
}
