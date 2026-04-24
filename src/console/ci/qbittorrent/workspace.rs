use std::path::{Path, PathBuf};
use std::time::Duration;

pub(crate) struct WorkspaceResources {
    pub(crate) root_path: PathBuf,
    pub(crate) tracker_config_path: PathBuf,
    pub(crate) tracker_storage_path: PathBuf,
    pub(crate) shared_path: PathBuf,
    pub(crate) seeder_config_path: PathBuf,
    pub(crate) leecher_config_path: PathBuf,
    pub(crate) seeder_downloads_path: PathBuf,
    pub(crate) leecher_downloads_path: PathBuf,
    pub(crate) torrent_bytes: Vec<u8>,
    pub(crate) timeout: Duration,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) login_poll_interval: Duration,
    pub(crate) torrent_poll_interval: Duration,
    pub(crate) torrent_file_name: String,
    pub(crate) payload_file_name: String,
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
