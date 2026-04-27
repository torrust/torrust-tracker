use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use reqwest::header::{CONTENT_TYPE, HOST, SET_COOKIE};
use reqwest::multipart::{Form, Part};
use tokio::sync::Mutex;

use super::super::types::InfoHash;
use super::credentials::QbittorrentCredentials;
use super::torrent::{TorrentInfo, TorrentProgress};

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

#[derive(Debug, Clone)]
pub struct QbittorrentClient {
    client_label: String,
    base_url: WebUiBaseUrl,
    client: reqwest::Client,
    sid_cookie: Arc<Mutex<Option<String>>>,
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
    pub async fn login(&self, credentials: &QbittorrentCredentials) -> anyhow::Result<()> {
        let body = reqwest::Url::parse_with_params(
            "http://localhost",
            &[
                ("username", credentials.username.as_str()),
                ("password", credentials.password.as_str()),
            ],
        )
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
    #[expect(dead_code, reason = "reserved for staged scenario coverage")]
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
    #[expect(dead_code, reason = "reserved for staged scenario coverage")]
    pub async fn first_torrent_progress(&self) -> anyhow::Result<Option<TorrentProgress>> {
        Ok(self.first_torrent().await?.map(|torrent| torrent.progress))
    }

    /// Returns the [`TorrentInfo`] for the torrent identified by `hash`, or `None` if it is not
    /// in the client's list.
    ///
    /// # Errors
    ///
    /// Returns an error when querying torrents fails.
    pub async fn torrent_by_hash(&self, hash: &InfoHash) -> anyhow::Result<Option<TorrentInfo>> {
        let torrents = self
            .list_torrents()
            .await
            .with_context(|| format!("failed to list {} torrents", self.client_label))?;
        Ok(torrents.into_iter().find(|t| t.hash.as_str() == hash.as_str()))
    }

    /// # Errors
    ///
    /// Returns an error when querying torrents fails.
    pub async fn has_torrent_with_hash(&self, hash: &InfoHash) -> anyhow::Result<bool> {
        Ok(self.torrent_by_hash(hash).await?.is_some())
    }

    /// Deletes the torrent identified by `hash` without removing its downloaded files.
    ///
    /// # Errors
    ///
    /// Returns an error when the qBittorrent API call fails.
    pub async fn delete_torrent(&self, hash: &InfoHash) -> anyhow::Result<()> {
        let (webui_host, webui_origin) = self.webui_headers();
        let sid_cookie = self.sid_cookie.lock().await.clone();

        let body = format!("hashes={}&deleteFiles=false", hash.as_str());
        let request = self
            .client
            .post(format!("{}/api/v2/torrents/delete", self.base_url.as_str()))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(HOST, webui_host)
            .header("Referer", &webui_origin)
            .header("Origin", &webui_origin)
            .body(body);
        let request = if let Some(cookie) = sid_cookie {
            request.header("Cookie", cookie)
        } else {
            request
        };

        let response = request
            .send()
            .await
            .with_context(|| format!("failed to call torrents/delete on {} qBittorrent instance", self.client_label))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "qBittorrent torrents/delete failed with status {} on {} instance",
                response.status(),
                self.client_label
            ))
        }
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

    use super::extract_sid_cookie;

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
