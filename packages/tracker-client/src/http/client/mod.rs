pub mod requests;
pub mod responses;

use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

use derive_more::Display;
use hyper::StatusCode;
use requests::{announce, scrape};
use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Failed to Build a Http Client: {err:?}")]
    ClientBuildingError { err: Arc<reqwest::Error> },
    #[error("Failed to get a response: {err:?}")]
    ResponseError { err: Arc<reqwest::Error> },
    #[error("Returned a non-success code: \"{code}\" with the response: \"{response:?}\"")]
    UnsuccessfulResponse { code: StatusCode, response: Arc<Response> },
}

/// HTTP Tracker Client
#[allow(clippy::struct_field_names)]
pub struct Client {
    http_client: reqwest::Client,
    base_url: Url,
    key: Option<Key>,
}

/// URL components in this context:
///
/// ```text
/// http://127.0.0.1:62304/announce/YZ....rJ?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22
/// \_____________________/\_______________/ \__________________________________________________________/
///            |                   |                                    |
///         base url              path                                query
/// ```
impl Client {
    /// # Errors
    ///
    /// This method fails if the client builder fails.
    pub fn new(base_url: Url, timeout: Duration) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| Error::ClientBuildingError { err: e.into() })?;

        Ok(Self {
            base_url,
            http_client: client,
            key: None,
        })
    }

    /// Creates the new client binding it to an specific local address.
    ///
    /// # Errors
    ///
    /// This method fails if the client builder fails.
    pub fn bind(base_url: Url, timeout: Duration, local_address: IpAddr) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .local_address(local_address)
            .build()
            .map_err(|e| Error::ClientBuildingError { err: e.into() })?;

        Ok(Self {
            base_url,
            http_client: client,
            key: None,
        })
    }

    /// # Errors
    ///
    /// This method fails if the client builder fails.
    pub fn authenticated(base_url: Url, timeout: Duration, key: Key) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| Error::ClientBuildingError { err: e.into() })?;

        Ok(Self {
            base_url,
            http_client: client,
            key: Some(key),
        })
    }

    /// # Errors
    ///
    /// This method fails if the returned response was not successful
    pub async fn announce(&self, query: &announce::Query) -> Result<Response, Error> {
        let response = self.get_url(self.build_announce_url(query)).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Error::UnsuccessfulResponse {
                code: response.status(),
                response: response.into(),
            })
        }
    }

    /// # Errors
    ///
    /// This method fails if the returned response was not successful
    pub async fn scrape(&self, query: &scrape::Query) -> Result<Response, Error> {
        let response = self.get_url(self.build_scrape_url(query)).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Error::UnsuccessfulResponse {
                code: response.status(),
                response: response.into(),
            })
        }
    }

    /// # Errors
    ///
    /// This method fails if the returned response was not successful
    pub async fn announce_with_header(&self, query: &announce::Query, key: &str, value: &str) -> Result<Response, Error> {
        let response = self.get_url_with_header(self.build_announce_url(query), key, value).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Error::UnsuccessfulResponse {
                code: response.status(),
                response: response.into(),
            })
        }
    }

    /// # Errors
    ///
    /// This method fails if the returned response was not successful
    pub async fn health_check(&self) -> Result<Response, Error> {
        let response = self.get(&self.build_path("health_check")).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Error::UnsuccessfulResponse {
                code: response.status(),
                response: response.into(),
            })
        }
    }

    /// # Errors
    ///
    /// This method fails if there was an error while sending request.
    pub async fn get(&self, path: &str) -> Result<Response, Error> {
        self.http_client
            .get(self.build_url(path))
            .send()
            .await
            .map_err(|e| Error::ResponseError { err: e.into() })
    }

    /// # Errors
    ///
    /// This method fails if there was an error while sending request.
    pub async fn get_with_header(&self, path: &str, key: &str, value: &str) -> Result<Response, Error> {
        self.http_client
            .get(self.build_url(path))
            .header(key, value)
            .send()
            .await
            .map_err(|e| Error::ResponseError { err: e.into() })
    }

    async fn get_url(&self, url: Url) -> Result<Response, Error> {
        self.http_client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::ResponseError { err: e.into() })
    }

    async fn get_url_with_header(&self, url: Url, key: &str, value: &str) -> Result<Response, Error> {
        self.http_client
            .get(url)
            .header(key, value)
            .send()
            .await
            .map_err(|e| Error::ResponseError { err: e.into() })
    }

    fn build_announce_url(&self, query: &announce::Query) -> Url {
        let mut url = self.build_endpoint_url("announce");
        url.set_query(Some(&query.to_string()));
        url
    }

    fn build_scrape_url(&self, query: &scrape::Query) -> Url {
        let mut url = self.build_endpoint_url("scrape");
        url.set_query(Some(&query.to_string()));
        url
    }

    fn build_endpoint_url(&self, default_endpoint: &str) -> Url {
        let mut url = self.base_url.clone();

        let current_path = url.path();
        let normalized_path = if current_path.is_empty() || current_path == "/" {
            format!("/{default_endpoint}")
        } else {
            current_path.to_owned()
        };

        let final_path = match &self.key {
            Some(key) => {
                let path_without_trailing_slash = normalized_path.trim_end_matches('/');
                format!("{path_without_trailing_slash}/{key}")
            }
            None => normalized_path,
        };

        url.set_path(&final_path);
        url
    }

    fn build_path(&self, path: &str) -> String {
        match &self.key {
            Some(key) => format!("{path}/{key}"),
            None => path.to_string(),
        }
    }

    fn build_url(&self, path: &str) -> String {
        let base_url = self.base_url();
        format!("{base_url}{path}")
    }

    fn base_url(&self) -> String {
        self.base_url.to_string()
    }
}

/// A token used for authentication.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Display, Hash)]
pub struct Key(String);

impl Key {
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self(value.to_owned())
    }

    #[must_use]
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use reqwest::Url;

    use super::{Client, Key};

    fn test_timeout() -> Duration {
        Duration::from_secs(1)
    }

    #[test]
    fn it_uses_announce_for_base_url_without_trailing_slash() {
        let client = Client::new(Url::parse("https://tracker.example.com").unwrap(), test_timeout()).unwrap();

        let url = client.build_endpoint_url("announce");

        assert_eq!(url.to_string(), "https://tracker.example.com/announce");
    }

    #[test]
    fn it_uses_announce_for_base_url_with_trailing_slash() {
        let client = Client::new(Url::parse("https://tracker.example.com/").unwrap(), test_timeout()).unwrap();

        let url = client.build_endpoint_url("announce");

        assert_eq!(url.to_string(), "https://tracker.example.com/announce");
    }

    #[test]
    fn it_keeps_existing_announce_path_unchanged() {
        let client = Client::new(Url::parse("https://tracker.example.com/announce").unwrap(), test_timeout()).unwrap();

        let url = client.build_endpoint_url("announce");

        assert_eq!(url.to_string(), "https://tracker.example.com/announce");
    }

    #[test]
    fn it_keeps_custom_path_unchanged_for_announce() {
        let client = Client::new(
            Url::parse("https://tracker.example.com/custom-tracker-endpoint").unwrap(),
            test_timeout(),
        )
        .unwrap();

        let url = client.build_endpoint_url("announce");

        assert_eq!(url.to_string(), "https://tracker.example.com/custom-tracker-endpoint");
    }

    #[test]
    fn it_appends_auth_key_to_existing_announce_path() {
        let client = Client::authenticated(
            Url::parse("https://tracker.example.com/announce").unwrap(),
            test_timeout(),
            Key::new("secret-key"),
        )
        .unwrap();

        let url = client.build_endpoint_url("announce");

        assert_eq!(url.to_string(), "https://tracker.example.com/announce/secret-key");
    }

    #[test]
    fn it_uses_scrape_for_base_url_without_trailing_slash() {
        let client = Client::new(Url::parse("https://tracker.example.com").unwrap(), test_timeout()).unwrap();

        let url = client.build_endpoint_url("scrape");

        assert_eq!(url.to_string(), "https://tracker.example.com/scrape");
    }

    #[test]
    fn it_keeps_existing_scrape_path_unchanged() {
        let client = Client::new(Url::parse("https://tracker.example.com/scrape").unwrap(), test_timeout()).unwrap();

        let url = client.build_endpoint_url("scrape");

        assert_eq!(url.to_string(), "https://tracker.example.com/scrape");
    }
}
