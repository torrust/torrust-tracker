use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use reqwest::header::{CONTENT_TYPE, HOST, SET_COOKIE};
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use tokio::sync::Mutex;

const QBITTORRENT_WEBUI_PORT: u16 = 8080;

/// A validated qBittorrent `WebUI` base URL.
///
/// Parses the raw URL string once at construction time.  All subsequent
/// accessors are infallible, removing the repeated parse-and-error pattern
/// that would otherwise occur in every API method.
#[derive(Debug, Clone)]
struct WebUiBaseUrl {
    raw: String,
    host: String,
    scheme: String,
}

impl WebUiBaseUrl {
    fn new(url: &str) -> anyhow::Result<Self> {
        let parsed = reqwest::Url::parse(url).with_context(|| format!("failed to parse qBittorrent WebUI base URL '{url}'"))?;
        let host = parsed
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("qBittorrent WebUI URL has no host: '{url}'"))?
            .to_string();
        let scheme = parsed.scheme().to_string();
        Ok(Self {
            raw: url.to_string(),
            host,
            scheme,
        })
    }

    /// Returns the base URL string for composing API paths.
    fn as_str(&self) -> &str {
        &self.raw
    }

    /// Returns only the host component (e.g. `"127.0.0.1"`).
    fn host(&self) -> &str {
        &self.host
    }

    /// Returns the scheme (e.g. `"http"`).
    fn scheme(&self) -> &str {
        &self.scheme
    }
}

/// Credentials for authenticating with the `qBittorrent` web UI.
#[derive(Debug, Clone)]
pub(crate) struct QbittorrentCredentials {
    /// Web-UI username.
    pub(crate) username: String,
    /// Web-UI password.
    pub(crate) password: String,
}

#[derive(Debug, Clone)]
pub struct QbittorrentClient {
    client_label: String,
    base_url: WebUiBaseUrl,
    client: reqwest::Client,
    sid_cookie: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Deserialize)]
pub struct TorrentInfo {
    pub hash: TorrentHash,
    pub progress: TorrentProgress,
    pub state: TorrentState,
}

/// A qBittorrent torrent hash - a 40-character lowercase hex-encoded SHA-1
/// string, as returned by the `/api/v2/torrents/info` endpoint.
///
/// Distinct from the binary [`InfoHash`](primitives::InfoHash) type in the
/// `primitives` package: the API delivers hex strings, not raw bytes. Wrapping
/// it here documents the invariant and disambiguates the field from other
/// [`String`] fields such as the torrent name or save path.
#[derive(Debug, Clone)]
pub struct TorrentHash(String);

impl TorrentHash {
    /// Creates a new [`TorrentHash`] from any value that converts into a [`String`].
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    /// Returns the hash as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for TorrentHash {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TorrentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for TorrentHash {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self(value))
    }
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

impl QbittorrentClient {
    /// # Errors
    ///
    /// Returns an error when the HTTP client cannot be built.
    pub fn new(client_label: &str, base_url: &str, timeout: Duration) -> anyhow::Result<Self> {
        let base_url = WebUiBaseUrl::new(base_url)?;
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("failed to build qBittorrent HTTP client")?;

        Ok(Self {
            client_label: client_label.to_string(),
            base_url,
            client,
            sid_cookie: Arc::new(Mutex::new(None)),
        })
    }

    /// # Errors
    ///
    /// Returns an error when login fails.
    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<()> {
        let body = reqwest::Url::parse_with_params("http://localhost", &[("username", username), ("password", password)])
            .context("failed to URL-encode qBittorrent login body")?
            .query()
            .ok_or_else(|| anyhow::anyhow!("encoded qBittorrent login body is unexpectedly empty"))?
            .to_string();
        let (webui_host, webui_origin) = self.webui_headers();

        let response = self
            .client
            .post(format!("{}/api/v2/auth/login", self.base_url.as_str()))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(HOST, webui_host)
            .header("Referer", &webui_origin)
            .header("Origin", &webui_origin)
            .body(body)
            .send()
            .await
            .context("failed to call qBittorrent login API")?;

        if let Some(sid_cookie) = extract_sid_cookie(response.headers()) {
            *self.sid_cookie.lock().await = Some(sid_cookie);
        }

        let status = response.status();
        let body_text = response
            .text()
            .await
            .context("failed to read qBittorrent login response body")?;

        if status.is_success() && body_text.trim() == "Ok." {
            Ok(())
        } else {
            Err(anyhow::anyhow!("qBittorrent login failed: HTTP {status}, body: {body_text}"))
        }
    }

    /// # Errors
    ///
    /// Returns an error when reading the qBittorrent application version fails.
    pub async fn app_version(&self) -> anyhow::Result<String> {
        let (webui_host, webui_origin) = self.webui_headers();
        let sid_cookie = self.sid_cookie.lock().await.clone();

        let request = self
            .client
            .get(format!("{}/api/v2/app/version", self.base_url.as_str()))
            .header(HOST, webui_host)
            .header("Referer", webui_origin);
        let request = if let Some(cookie) = sid_cookie {
            request.header("Cookie", cookie)
        } else {
            request
        };

        let response = request.send().await.context("failed to call qBittorrent app/version API")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "qBittorrent app/version failed with status {}",
                response.status()
            ));
        }

        response.text().await.context("failed to read qBittorrent app version body")
    }

    /// # Errors
    ///
    /// Returns an error when adding a torrent file fails.
    pub async fn add_torrent_file(&self, torrent_name: &str, torrent_bytes: &[u8], save_path: &str) -> anyhow::Result<()> {
        let (webui_host, webui_origin) = self.webui_headers();
        let sid_cookie = self.sid_cookie.lock().await.clone();

        let part = Part::bytes(torrent_bytes.to_vec()).file_name(torrent_name.to_string());
        let form = Form::new()
            .part("torrents", part)
            .text("savepath", save_path.to_string())
            .text("paused", "false")
            .text("skip_checking", "false");

        let request = self
            .client
            .post(format!("{}/api/v2/torrents/add", self.base_url.as_str()))
            .header(HOST, webui_host)
            .header("Referer", &webui_origin)
            .header("Origin", &webui_origin)
            .multipart(form);
        let request = if let Some(cookie) = sid_cookie {
            request.header("Cookie", cookie)
        } else {
            request
        };

        let response = request
            .send()
            .await
            .with_context(|| format!("failed to call torrents/add on {} qBittorrent instance", self.client_label))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "qBittorrent torrents/add failed with status {} on {} instance",
                response.status(),
                self.client_label
            ))
        }
    }

    /// # Errors
    ///
    /// Returns an error when querying torrents fails.
    pub async fn list_torrents(&self) -> anyhow::Result<Vec<TorrentInfo>> {
        let (webui_host, webui_origin) = self.webui_headers();
        let sid_cookie = self.sid_cookie.lock().await.clone();

        let request = self
            .client
            .get(format!("{}/api/v2/torrents/info", self.base_url.as_str()))
            .header(HOST, webui_host)
            .header("Referer", webui_origin);
        let request = if let Some(cookie) = sid_cookie {
            request.header("Cookie", cookie)
        } else {
            request
        };

        let response = request.send().await.context("failed to call qBittorrent torrents/info API")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "qBittorrent torrents/info failed with status {}",
                response.status()
            ));
        }

        response
            .json::<Vec<TorrentInfo>>()
            .await
            .context("failed to deserialize qBittorrent torrents list")
    }

    /// # Errors
    ///
    /// Returns an error when querying torrents fails.
    pub async fn first_torrent(&self) -> anyhow::Result<Option<TorrentInfo>> {
        let torrents = self
            .list_torrents()
            .await
            .with_context(|| format!("failed to list {} torrents", self.client_label))?;

        Ok(torrents.into_iter().next())
    }

    /// # Errors
    ///
    /// Returns an error when querying torrents fails.
    pub async fn first_torrent_progress(&self) -> anyhow::Result<Option<TorrentProgress>> {
        Ok(self.first_torrent().await?.map(|torrent| torrent.progress))
    }

    /// # Errors
    ///
    /// Returns an error when querying torrents fails.
    pub async fn has_any_torrents(&self) -> anyhow::Result<bool> {
        Ok(self.torrent_count().await? > 0)
    }

    /// # Errors
    ///
    /// Returns an error when querying torrents fails.
    pub async fn torrent_count(&self) -> anyhow::Result<usize> {
        Ok(self
            .list_torrents()
            .await
            .with_context(|| format!("failed to list {} torrents", self.client_label))?
            .len())
    }

    fn webui_headers(&self) -> (String, String) {
        let host = self.base_url.host();
        let scheme = self.base_url.scheme();
        (
            format!("{host}:{QBITTORRENT_WEBUI_PORT}"),
            format!("{scheme}://{host}:{QBITTORRENT_WEBUI_PORT}"),
        )
    }
}

fn extract_sid_cookie(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find_map(|value| {
            value
                .split(';')
                .next()
                .map(str::trim)
                .filter(|cookie| cookie.starts_with("SID="))
                .map(ToOwned::to_owned)
        })
}

#[cfg(test)]
mod tests {
    use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE};

    use super::{extract_sid_cookie, TorrentHash, TorrentProgress, TorrentState};

    #[test]
    fn it_should_construct_torrent_hash_and_expose_accessors() {
        let hash = TorrentHash::new("0123456789abcdef0123456789abcdef01234567");

        assert_eq!(hash.as_str(), "0123456789abcdef0123456789abcdef01234567");
        assert_eq!(&*hash, "0123456789abcdef0123456789abcdef01234567");
        assert_eq!(hash.to_string(), "0123456789abcdef0123456789abcdef01234567");
    }

    #[test]
    fn it_should_deserialize_torrent_hash_from_json_string() {
        let parsed = serde_json::from_str::<TorrentHash>("\"abcdef0123456789abcdef0123456789abcdef01\"");

        assert!(parsed.is_ok());
        let hash = parsed.unwrap_or_else(|error| panic!("failed to parse hash: {error}"));
        assert_eq!(hash.as_str(), "abcdef0123456789abcdef0123456789abcdef01");
    }

    #[test]
    fn it_should_report_torrent_progress_completion_threshold() {
        let complete = serde_json::from_str::<TorrentProgress>("1.0");
        let in_progress = serde_json::from_str::<TorrentProgress>("0.42");

        assert!(complete.is_ok());
        assert!(in_progress.is_ok());

        let complete = complete.unwrap_or_else(|error| panic!("failed to parse complete progress: {error}"));
        let in_progress = in_progress.unwrap_or_else(|error| panic!("failed to parse in-progress value: {error}"));

        assert!(complete.is_complete());
        assert_eq!(complete.as_fraction(), 1.0);

        assert!(!in_progress.is_complete());
        assert_eq!(in_progress.as_fraction(), 0.42);
    }

    #[test]
    fn it_should_deserialize_torrent_state_known_variant() {
        let parsed = serde_json::from_str::<TorrentState>("\"stoppedDL\"");

        assert!(parsed.is_ok());
        match parsed.unwrap_or_else(|error| panic!("failed to parse state: {error}")) {
            TorrentState::StoppedDl => {}
            other => panic!("unexpected state variant: {other}"),
        }
    }

    #[test]
    fn it_should_deserialize_unknown_torrent_state_preserving_raw_value() {
        let parsed = serde_json::from_str::<TorrentState>("\"futureState\"");

        assert!(parsed.is_ok());
        match parsed.unwrap_or_else(|error| panic!("failed to parse state: {error}")) {
            TorrentState::Unknown(raw) => assert_eq!(raw, "futureState"),
            other => panic!("unexpected state variant: {other}"),
        }
    }

    #[test]
    fn it_should_display_known_and_unknown_torrent_state_values() {
        assert_eq!(TorrentState::PausedDl.to_string(), "pausedDL");
        assert_eq!(TorrentState::Unknown(String::from("custom")).to_string(), "custom");
    }

    #[test]
    fn it_should_extract_sid_cookie_when_present() {
        let mut headers = HeaderMap::new();
        headers.append(SET_COOKIE, HeaderValue::from_static("foo=bar; Path=/"));
        headers.append(SET_COOKIE, HeaderValue::from_static("SID=abc123; HttpOnly; Path=/"));

        assert_eq!(extract_sid_cookie(&headers), Some(String::from("SID=abc123")));
    }

    #[test]
    fn it_should_return_none_when_sid_cookie_is_missing() {
        let mut headers = HeaderMap::new();
        headers.append(SET_COOKIE, HeaderValue::from_static("foo=bar; Path=/"));

        assert_eq!(extract_sid_cookie(&headers), None);
    }
}
