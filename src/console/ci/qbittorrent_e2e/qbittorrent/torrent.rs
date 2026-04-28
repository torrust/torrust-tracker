use std::fmt;

use serde::Deserialize;

use super::super::types::InfoHash;

#[derive(Debug, Deserialize)]
pub struct TorrentInfo {
    pub hash: InfoHash,
    pub progress: TorrentProgress,
    pub state: TorrentState,
}

/// A torrent download progress value in the range `0.0` (not started) to
/// `1.0` (fully complete), as reported by the qBittorrent Web API.
///
/// Wraps an `f64` to disambiguate progress from other floating-point fields
/// such as download speed. Use [`is_complete`](Self::is_complete) to test for
/// full completion and [`as_fraction`](Self::as_fraction) to obtain the raw
/// `0.0`-`1.0` value for arithmetic or formatted output.
#[derive(Debug, Clone, Copy)]
pub struct TorrentProgress(f64);

impl TorrentProgress {
    /// Returns `true` when the torrent has reached 100 % (`progress >= 1.0`).
    #[must_use]
    pub fn is_complete(self) -> bool {
        self.0 >= 1.0
    }

    /// Returns the raw fraction in the range `0.0`-`1.0`.
    #[must_use]
    pub fn as_fraction(self) -> f64 {
        self.0
    }
}

impl<'de> serde::Deserialize<'de> for TorrentProgress {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <f64 as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

/// The state of a torrent as reported by the qBittorrent Web API.
///
/// Variants map one-to-one to the string values returned by the
/// `/api/v2/torrents/info` endpoint. Any string not listed here is captured
/// by [`TorrentState::Unknown`] and its raw value is preserved for diagnostics.
///
/// Note: qBittorrent 5.0 renamed `pausedUP`/`pausedDL` to
/// `stoppedUP`/`stoppedDL`. Both spellings are represented.
#[derive(Debug, Clone)]
pub enum TorrentState {
    /// Some error occurred.
    Error,
    /// Torrent data files are missing.
    MissingFiles,
    /// Torrent is being seeded and data is being transferred.
    Uploading,
    /// Seeder has finished and the torrent is stopped (qBittorrent >= 5.0).
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
    /// Download is stopped (qBittorrent >= 5.0).
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

#[cfg(test)]
mod tests {
    use super::{TorrentProgress, TorrentState};

    #[test]
    fn it_should_report_torrent_progress_completion_threshold() {
        let complete = serde_json::from_str::<TorrentProgress>("1.0").expect("1.0 is valid progress JSON");
        let in_progress = serde_json::from_str::<TorrentProgress>("0.42").expect("0.42 is valid progress JSON");

        assert!(complete.is_complete());
        assert!((complete.as_fraction() - 1.0).abs() < f64::EPSILON);

        assert!(!in_progress.is_complete());
        assert!((in_progress.as_fraction() - 0.42).abs() < f64::EPSILON);
    }

    #[test]
    fn it_should_deserialize_torrent_state_known_variant() {
        let parsed = serde_json::from_str::<TorrentState>("\"stoppedDL\"").expect("stoppedDL is a valid state JSON");

        assert!(matches!(parsed, TorrentState::StoppedDl), "expected StoppedDl, got {parsed}");
    }

    #[test]
    fn it_should_deserialize_unknown_torrent_state_preserving_raw_value() {
        let parsed = serde_json::from_str::<TorrentState>("\"futureState\"").expect("futureState is valid state JSON");

        let TorrentState::Unknown(raw) = parsed else {
            panic!("expected Unknown variant, got {parsed}");
        };
        assert_eq!(raw, "futureState");
    }

    #[test]
    fn it_should_display_known_and_unknown_torrent_state_values() {
        assert_eq!(TorrentState::PausedDl.to_string(), "pausedDL");
        assert_eq!(TorrentState::Unknown(String::from("custom")).to_string(), "custom");
    }
}
