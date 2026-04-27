//! Filesystem setup for the `qBittorrent` E2E tests.
//!
//! This module creates the directory tree, service configuration files, and
//! shared test fixtures that the `Docker` Compose stack needs before it starts.
//!
//! # Workspace Layout
//!
//! After [`prepare`] returns, the workspace root contains:
//!
//! ```text
//! <workspace-root>/
//! ├── leecher-config/
//! │   └── qBittorrent/
//! │       └── qBittorrent.conf
//! ├── leecher-downloads/
//! ├── seeder-config/
//! │   └── qBittorrent/
//! │       └── qBittorrent.conf
//! ├── seeder-downloads/
//! │   └── payload.bin          ← pre-seeded payload copy
//! ├── shared/
//! │   ├── payload.bin          ← source payload file
//! │   └── payload.torrent
//! ├── tracker-config.toml
//! └── tracker-storage/
//!     └── database/
//!         └── sqlite3.db       ← created at runtime by the tracker
//! ```
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context;
use reqwest::Url;

use super::qbittorrent::{QbittorrentConfigBuilder, QbittorrentCredentials};
use super::tracker::{TrackerConfig, TrackerConfigBuilder};
use super::types::{ComposeProjectName, ContainerPath, Deadline, PollInterval};
use super::workspace::{
    EphemeralWorkspace, PeerConfig, PermanentWorkspace, PreparedWorkspace, SharedFixtures, TimingConfig, TrackerEndpoints,
    TrackerFilesystem, WorkspaceResources,
};

const QBITTORRENT_USERNAME: &str = "admin";
const SEEDER_PASSWORD: &str = "seeder-pass";
const LEECHER_PASSWORD: &str = "leecher-pass";
const QBITTORRENT_DOWNLOADS_PATH: &str = "/downloads";
const TORRENT_POLL_INTERVAL: Duration = Duration::from_millis(500);
const LOGIN_POLL_INTERVAL: Duration = Duration::from_secs(1);

/// Creates and populates the workspace for a single E2E test run.
///
/// Returns an ephemeral workspace (temporary directory, auto-cleaned on drop)
/// when `keep_containers` is `false`, or a permanent workspace under
/// `storage/qbt-e2e/<project_name>` when it is `true`.
///
/// # Errors
///
/// Returns an error when any directory or file operation fails.
pub(crate) fn prepare(
    project_name: &ComposeProjectName,
    keep_containers: bool,
    timeout: Duration,
    tracker_config: &TrackerConfig,
) -> anyhow::Result<PreparedWorkspace> {
    if keep_containers {
        let persistent_root = std::env::current_dir()
            .context("failed to resolve current working directory")?
            .join("storage")
            .join("qbt-e2e")
            .join(project_name.as_str());
        fs::create_dir_all(&persistent_root).with_context(|| {
            format!(
                "failed to create persistent qBittorrent workspace '{}'",
                persistent_root.display()
            )
        })?;
        let resources = prepare_resources(persistent_root, timeout, tracker_config)?;

        Ok(PreparedWorkspace::Permanent(PermanentWorkspace { resources }))
    } else {
        let temp_dir = tempfile::tempdir().context("failed to create temporary workspace")?;
        let root_path = temp_dir.path().to_path_buf();
        let resources = prepare_resources(root_path, timeout, tracker_config)?;

        Ok(PreparedWorkspace::Ephemeral(EphemeralWorkspace {
            _temp_dir: temp_dir,
            resources,
        }))
    }
}

fn prepare_resources(
    root_path: PathBuf,
    timeout: Duration,
    tracker_config: &TrackerConfig,
) -> anyhow::Result<WorkspaceResources> {
    let tracker = setup_tracker_workspace(&root_path, tracker_config)?;
    let seeder = setup_qbittorrent_workspace(&root_path, "seeder", SEEDER_PASSWORD)?;
    let leecher = setup_qbittorrent_workspace(&root_path, "leecher", LEECHER_PASSWORD)?;
    let shared = setup_shared_fixtures(&root_path)?;
    let tracker_endpoints = TrackerEndpoints {
        http_announce_url: Url::parse(&tracker_config.announce_url_for_compose_service())
            .context("failed to parse HTTP tracker announce URL for compose service")?,
        udp_announce_url: Url::parse(&tracker_config.udp_announce_url_for_compose_service())
            .context("failed to parse UDP tracker announce URL for compose service")?,
    };

    Ok(WorkspaceResources {
        root_path,
        tracker,
        tracker_endpoints,
        seeder,
        leecher,
        shared,
        timing: TimingConfig {
            polling_deadline: Deadline::new(timeout),
            login_poll_interval: PollInterval::new(LOGIN_POLL_INTERVAL),
            torrent_poll_interval: PollInterval::new(TORRENT_POLL_INTERVAL),
        },
    })
}

fn setup_tracker_workspace(root: &Path, tracker_config: &TrackerConfig) -> anyhow::Result<TrackerFilesystem> {
    let storage_path = root.join("tracker-storage");
    fs::create_dir_all(&storage_path).context("failed to create tracker storage directory")?;
    let config_path = TrackerConfigBuilder::new(tracker_config.clone()).write_to(root)?;
    Ok(TrackerFilesystem {
        config_path,
        storage_path,
    })
}

fn setup_qbittorrent_workspace(root: &Path, role: &str, password: &str) -> anyhow::Result<PeerConfig> {
    let config_path = root.join(format!("{role}-config"));
    let downloads_path = root.join(format!("{role}-downloads"));
    fs::create_dir_all(&downloads_path).with_context(|| format!("failed to create {role} downloads directory"))?;
    QbittorrentConfigBuilder::new(QBITTORRENT_USERNAME, password)
        .write_to(&config_path)
        .with_context(|| format!("failed to generate {role} qBittorrent config"))?;
    Ok(PeerConfig {
        config_path,
        downloads_path,
        credentials: QbittorrentCredentials {
            username: QBITTORRENT_USERNAME.to_string(),
            password: password.to_string(),
        },
        container_downloads_path: ContainerPath::new(QBITTORRENT_DOWNLOADS_PATH),
    })
}

fn setup_shared_fixtures(root: &Path) -> anyhow::Result<SharedFixtures> {
    let path = root.join("shared");
    fs::create_dir_all(&path).context("failed to create shared artifacts directory")?;
    Ok(SharedFixtures { path })
}
