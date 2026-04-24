use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use reqwest::header::{CONTENT_TYPE, HOST, SET_COOKIE};
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use tokio::sync::Mutex;

use super::types::{TorrentProgress, TorrentState};

const QBITTORRENT_WEBUI_PORT: u16 = 8080;

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
    base_url: String,
    client: reqwest::Client,
    sid_cookie: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Deserialize)]
pub struct TorrentInfo {
    pub hash: String,
    pub progress: TorrentProgress,
    pub state: TorrentState,
}

impl QbittorrentClient {
    /// # Errors
    ///
    /// Returns an error when the HTTP client cannot be built.
    pub fn new(client_label: &str, base_url: &str, timeout: Duration) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .context("failed to build qBittorrent HTTP client")?;

        Ok(Self {
            client_label: client_label.to_string(),
            base_url: base_url.to_string(),
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
        let (webui_host, webui_origin) = self
            .webui_headers()
            .context("failed to prepare qBittorrent WebUI CSRF headers")?;

        let response = self
            .client
            .post(format!("{}/api/v2/auth/login", self.base_url))
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
        let (webui_host, webui_origin) = self
            .webui_headers()
            .context("failed to prepare qBittorrent WebUI CSRF headers")?;
        let sid_cookie = self.sid_cookie.lock().await.clone();

        let request = self
            .client
            .get(format!("{}/api/v2/app/version", self.base_url))
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
        let (webui_host, webui_origin) = self
            .webui_headers()
            .context("failed to prepare qBittorrent WebUI CSRF headers")?;
        let sid_cookie = self.sid_cookie.lock().await.clone();

        let part = Part::bytes(torrent_bytes.to_vec()).file_name(torrent_name.to_string());
        let form = Form::new()
            .part("torrents", part)
            .text("savepath", save_path.to_string())
            .text("paused", "false")
            .text("skip_checking", "false");

        let request = self
            .client
            .post(format!("{}/api/v2/torrents/add", self.base_url))
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
        let (webui_host, webui_origin) = self
            .webui_headers()
            .context("failed to prepare qBittorrent WebUI CSRF headers")?;
        let sid_cookie = self.sid_cookie.lock().await.clone();

        let request = self
            .client
            .get(format!("{}/api/v2/torrents/info", self.base_url))
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

    fn webui_headers(&self) -> anyhow::Result<(String, String)> {
        let parsed_url = reqwest::Url::parse(&self.base_url)
            .with_context(|| format!("failed to parse qBittorrent base URL '{}'", self.base_url))?;
        let host = parsed_url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("qBittorrent base URL has no host: '{}'", self.base_url))?;
        let scheme = parsed_url.scheme();

        Ok((
            format!("{host}:{QBITTORRENT_WEBUI_PORT}"),
            format!("{scheme}://{host}:{QBITTORRENT_WEBUI_PORT}"),
        ))
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
