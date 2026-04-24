use std::path::{Path, PathBuf};
use std::time::Duration;

pub(crate) struct TrackerFilesystem {
    /// Path to `tracker-config.toml` on the host.
    pub(crate) config_path: PathBuf,
    /// Path to the `tracker-storage/` directory on the host.
    pub(crate) storage_path: PathBuf,
}

pub(crate) struct SharedFixtures {
    /// Path to the `shared/` directory on the host.
    pub(crate) path: PathBuf,
    /// File name of the payload (e.g. `"payload.bin"`).
    pub(crate) payload_file_name: String,
    /// File name of the torrent file (e.g. `"payload.torrent"`).
    pub(crate) torrent_file_name: String,
    /// Raw bytes of the torrent file, held in memory.
    pub(crate) torrent_bytes: Vec<u8>,
}

pub(crate) struct TimingConfig {
    /// Maximum time any single polling loop will wait before giving up.
    /// Passed directly to `Poller::new` as the loop deadline.
    pub(crate) polling_deadline: Duration,
    /// Sleep duration between login-readiness retries.
    pub(crate) login_poll_interval: Duration,
    /// Sleep duration between torrent-state retries.
    pub(crate) torrent_poll_interval: Duration,
}

pub(crate) struct WorkspaceResources {
    pub(crate) root_path: PathBuf,
    pub(crate) tracker: TrackerFilesystem,
    pub(crate) seeder_config_path: PathBuf,
    pub(crate) leecher_config_path: PathBuf,
    pub(crate) seeder_downloads_path: PathBuf,
    pub(crate) leecher_downloads_path: PathBuf,
    pub(crate) shared: SharedFixtures,
    pub(crate) timing: TimingConfig,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) downloads_path: String,
}

pub(crate) struct EphemeralWorkspace {
    pub(crate) _temp_dir: tempfile::TempDir,
    pub(crate) resources: WorkspaceResources,
}

pub(crate) struct PermanentWorkspace {
    pub(crate) resources: WorkspaceResources,
}

pub(crate) enum PreparedWorkspace {
    Ephemeral(EphemeralWorkspace),
    Permanent(PermanentWorkspace),
}

impl PreparedWorkspace {
    pub(crate) fn resources(&self) -> &WorkspaceResources {
        match self {
            Self::Ephemeral(workspace) => &workspace.resources,
            Self::Permanent(workspace) => &workspace.resources,
        }
    }

    pub(crate) fn root_path(&self) -> &Path {
        &self.resources().root_path
    }
}
