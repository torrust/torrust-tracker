//! Small domain types shared across the `qBittorrent` E2E module.
//!
//! Most types here follow the newtype pattern: a thin wrapper around a primitive
//! that gives the value a precise, self-documenting type at every call site.
use std::fmt;
use std::ops::Deref;
use std::path::Path;
use std::time::Duration;

/// A file name (base name only, no path separators).
///
/// Wraps a [`String`] and provides [`Deref`] to `str` so values can be used
/// directly wherever `&str` is expected, and [`AsRef<Path>`] so they can be
/// passed to [`Path::join`].
#[derive(Debug, Clone)]
pub(crate) struct FileName(String);

impl FileName {
    /// Creates a new [`FileName`] from any value that converts into a [`String`].
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl Deref for FileName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for FileName {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for FileName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for FileName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// An absolute path inside a Docker container (e.g. `"/downloads"`).
///
/// Distinct from host [`PathBuf`]s: a `ContainerPath` is always a
/// Linux-style absolute path that exists only within the container
/// file-system, never on the host.
///
/// [`PathBuf`]: std::path::PathBuf
#[derive(Debug, Clone)]
pub(crate) struct ContainerPath(String);

impl ContainerPath {
    /// Creates a new [`ContainerPath`] from any value that converts into a [`String`].
    pub(crate) fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }
}

impl Deref for ContainerPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ContainerPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ContainerPath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ContainerPath {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// The state of a torrent as reported by the qBittorrent Web API.
///
/// Variants map one-to-one to the string values returned by the
/// `/api/v2/torrents/info` endpoint.  Any string not listed here is captured
/// by [`TorrentState::Unknown`] and its raw value is preserved for
/// diagnostics.
///
/// Note: qBittorrent 5.0 renamed `pausedUP`/`pausedDL` to
/// `stoppedUP`/`stoppedDL`.  Both spellings are represented.
#[derive(Debug, Clone)]
pub enum TorrentState {
    /// Some error occurred.
    Error,
    /// Torrent data files are missing.
    MissingFiles,
    /// Torrent is being seeded and data is being transferred.
    Uploading,
    /// Seeder has finished and the torrent is stopped (qBittorrent ≥ 5.0).
    StoppedUp,
    /// Seeder has finished and the torrent is paused (qBittorrent < 5.0).
    PausedUp,
    /// Torrent is queued for upload.
    QueuedUp,
    /// Seeding is stalled (no peers downloading).
    StalledUp,
    /// Checking data after completing upload.
    CheckingUp,
    /// Torrent is force-seeding.
    ForcedUp,
    /// Allocating disk space for the download.
    Allocating,
    /// Torrent is downloading.
    Downloading,
    /// Fetching torrent metadata.
    MetaDl,
    /// Download is stopped (qBittorrent ≥ 5.0).
    StoppedDl,
    /// Download is paused (qBittorrent < 5.0).
    PausedDl,
    /// Torrent is queued for download.
    QueuedDl,
    /// Download is stalled (no seeds available).
    StalledDl,
    /// Checking data while downloading.
    CheckingDl,
    /// Torrent is force-downloading.
    ForcedDl,
    /// Checking resume data on startup.
    CheckingResumeData,
    /// Moving files to a new location.
    Moving,
    /// The API returned `"unknown"`.
    UnknownToApi,
    /// An unrecognized state string; the raw value is preserved for diagnostics.
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for TorrentState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "error" => Self::Error,
            "missingFiles" => Self::MissingFiles,
            "uploading" => Self::Uploading,
            "stoppedUP" => Self::StoppedUp,
            "pausedUP" => Self::PausedUp,
            "queuedUP" => Self::QueuedUp,
            "stalledUP" => Self::StalledUp,
            "checkingUP" => Self::CheckingUp,
            "forcedUP" => Self::ForcedUp,
            "allocating" => Self::Allocating,
            "downloading" => Self::Downloading,
            "metaDL" => Self::MetaDl,
            "stoppedDL" => Self::StoppedDl,
            "pausedDL" => Self::PausedDl,
            "queuedDL" => Self::QueuedDl,
            "stalledDL" => Self::StalledDl,
            "checkingDL" => Self::CheckingDl,
            "forcedDL" => Self::ForcedDl,
            "checkingResumeData" => Self::CheckingResumeData,
            "moving" => Self::Moving,
            "unknown" => Self::UnknownToApi,
            other => Self::Unknown(other.to_string()),
        })
    }
}

impl fmt::Display for TorrentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Error => "error",
            Self::MissingFiles => "missingFiles",
            Self::Uploading => "uploading",
            Self::StoppedUp => "stoppedUP",
            Self::PausedUp => "pausedUP",
            Self::QueuedUp => "queuedUP",
            Self::StalledUp => "stalledUP",
            Self::CheckingUp => "checkingUP",
            Self::ForcedUp => "forcedUP",
            Self::Allocating => "allocating",
            Self::Downloading => "downloading",
            Self::MetaDl => "metaDL",
            Self::StoppedDl => "stoppedDL",
            Self::PausedDl => "pausedDL",
            Self::QueuedDl => "queuedDL",
            Self::StalledDl => "stalledDL",
            Self::CheckingDl => "checkingDL",
            Self::ForcedDl => "forcedDL",
            Self::CheckingResumeData => "checkingResumeData",
            Self::Moving => "moving",
            Self::UnknownToApi => "unknown",
            Self::Unknown(raw) => return f.write_str(raw),
        };
        f.write_str(s)
    }
}

/// A polling-loop deadline expressed as a [`Duration`] measured from the moment
/// the loop starts.
///
/// Wraps a [`Duration`] representing the *maximum time* a polling loop may wait
/// before giving up.  Keeping it distinct from [`PollInterval`] turns an
/// accidental swap into a compile error instead of a silent logic bug.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Deadline(Duration);

impl Deadline {
    /// Creates a new [`Deadline`] from a [`Duration`].
    pub(crate) fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying [`Duration`].
    pub(crate) fn as_duration(&self) -> Duration {
        self.0
    }
}

/// The sleep duration between successive retries in a polling loop.
///
/// Wraps a [`Duration`].  Distinct from [`Deadline`] so that the two cannot
/// be accidentally swapped at a call site.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PollInterval(Duration);

impl PollInterval {
    /// Creates a new [`PollInterval`] from a [`Duration`].
    pub(crate) fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying [`Duration`].
    pub(crate) fn as_duration(&self) -> Duration {
        self.0
    }
}
