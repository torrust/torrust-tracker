pub mod redaction;
pub mod requests;
pub mod responses;

use std::fmt;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

use derive_more::Display;
use hyper::StatusCode;
use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use torrust_tracker_http_protocol::v1::requests::announce::Announce;
use torrust_tracker_http_protocol::v1::requests::scrape_builder;

use self::redaction::{mask_key_segments, mask_tracker_key, request_error_text};

/// An HTTP tracker client error.
///
/// Its `Display` and `Debug` texts never carry the tracker key: the request URL is masked with
/// [`redaction::mask_tracker_key`] and, for an authenticated client, with its own key
/// ([`redaction::mask_key_segments`]), and the texts of [`reqwest::Error`] and [`Response`], which
/// carry the full URL, are not used.
#[derive(Clone, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Failed to build an HTTP client: {}", request_error_text(err))]
    ClientBuildingError { err: Arc<reqwest::Error> },
    #[error("Failed to get a response: {}", request_error_text(err))]
    ResponseError { err: Arc<reqwest::Error> },
    #[error("Returned a non-success code: \"{code}\" for url ({url})")]
    UnsuccessfulResponse {
        code: StatusCode,
        /// The request URL with its secrets masked.
        url: Url,
        response: Arc<Response>,
    },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClientBuildingError { err } => f
                .debug_struct("ClientBuildingError")
                .field("err", &request_error_text(err))
                .finish(),
            Self::ResponseError { err } => f
                .debug_struct("ResponseError")
                .field("err", &request_error_text(err))
                .finish(),
            Self::UnsuccessfulResponse { code, url, .. } => f
                .debug_struct("UnsuccessfulResponse")
                .field("code", code)
                .field("url", &url.as_str())
                .finish(),
        }
    }
}

/// HTTP Tracker Client
#[allow(
    clippy::struct_field_names,
    reason = "fields distinguish the HTTP transport client and tracker base URL"
)]
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
    pub async fn announce(&self, query: &Announce) -> Result<Response, Error> {
        let response = self.get_url(self.build_announce_url(query)).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(self.unsuccessful_response(response))
        }
    }

    /// # Errors
    ///
    /// This method fails if the returned response was not successful
    pub async fn scrape(&self, query: &scrape_builder::Query) -> Result<Response, Error> {
        let response = self.get_url(self.build_scrape_url(query)).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(self.unsuccessful_response(response))
        }
    }

    /// # Errors
    ///
    /// This method fails if the returned response was not successful
    pub async fn announce_with_header(&self, query: &Announce, key: &str, value: &str) -> Result<Response, Error> {
        let response = self.get_url_with_header(self.build_announce_url(query), key, value).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(self.unsuccessful_response(response))
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
            Err(self.unsuccessful_response(response))
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
            .map_err(|e| self.response_error(e))
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
            .map_err(|e| self.response_error(e))
    }

    async fn get_url(&self, url: Url) -> Result<Response, Error> {
        self.http_client.get(url).send().await.map_err(|e| self.response_error(e))
    }

    async fn get_url_with_header(&self, url: Url, key: &str, value: &str) -> Result<Response, Error> {
        self.http_client
            .get(url)
            .header(key, value)
            .send()
            .await
            .map_err(|e| self.response_error(e))
    }

    /// The request URL with its secrets masked: the key after `announce` or `scrape`, and this
    /// client's own key wherever its path puts it.
    fn masked_url(&self, url: &Url) -> Url {
        let masked = mask_tracker_key(url);

        match &self.key {
            Some(key) => mask_key_segments(&masked, key.value()),
            None => masked,
        }
    }

    fn response_error(&self, error: reqwest::Error) -> Error {
        let error = match error.url().map(|url| self.masked_url(url)) {
            Some(masked_url) => error.with_url(masked_url),
            None => error,
        };

        Error::ResponseError { err: error.into() }
    }

    fn unsuccessful_response(&self, response: Response) -> Error {
        Error::UnsuccessfulResponse {
            code: response.status(),
            url: self.masked_url(response.url()),
            response: response.into(),
        }
    }

    fn build_announce_url(&self, query: &Announce) -> Url {
        let mut url = self.build_endpoint_url("announce");
        url.set_query(Some(&query.to_string()));
        url
    }

    fn build_scrape_url(&self, query: &scrape_builder::Query) -> Url {
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
                let key_segment = key.value();
                let already_has_key = path_without_trailing_slash
                    .rsplit('/')
                    .next()
                    .is_some_and(|segment| segment == key_segment);

                if already_has_key {
                    path_without_trailing_slash.to_string()
                } else {
                    format!("{path_without_trailing_slash}/{key}")
                }
            }
            None => normalized_path,
        };

        url.set_path(&final_path);
        url
    }

    fn build_path(&self, path: &str) -> String {
        self.key
            .as_ref()
            .map_or_else(|| path.to_string(), |key| format!("{path}/{key}"))
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
    fn it_does_not_append_auth_key_when_path_already_ends_with_same_key() {
        let client = Client::authenticated(
            Url::parse("https://tracker.example.com/announce/secret-key").unwrap(),
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

    mod error_text {
        use std::net::{SocketAddr, TcpListener};
        use std::time::Duration;

        use reqwest::Url;
        use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
        use tokio::task::JoinHandle;
        use torrust_tracker_http_protocol::v1::requests::announce::{Announce, AnnounceBuilder};
        use torrust_tracker_http_protocol::v1::requests::scrape_builder;

        use super::super::{Client, Key};

        const KEY: &str = "SecretTrackerKey123";

        /// Bounds every awaited request, so a hang fails the test instead of blocking the run.
        const TEST_DEADLINE: Duration = Duration::from_secs(10);

        /// An address where nothing listens: the port is taken from a listener that is then dropped.
        fn closed_local_address() -> SocketAddr {
            let listener = TcpListener::bind("127.0.0.1:0").expect("it should bind a local port");
            listener.local_addr().expect("the listener should have an address")
        }

        /// A local HTTP server that answers one request with `404 Not Found` and closes. The returned
        /// task is aborted when the test ends (`AbortOnDrop`).
        async fn start_not_found_responder() -> (SocketAddr, AbortOnDrop) {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("the responder should bind a local port");
            let address = listener.local_addr().expect("the responder should have an address");

            let task = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.expect("the responder should accept the request");
                let mut request = [0_u8; 4096];
                let request_length = stream
                    .read(&mut request)
                    .await
                    .expect("the responder should read the request");
                assert!(request_length > 0, "the responder should receive a request");
                stream
                    .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                    .await
                    .expect("the responder should send its reply");
            });

            (address, AbortOnDrop(task))
        }

        struct AbortOnDrop(JoinHandle<()>);

        impl Drop for AbortOnDrop {
            fn drop(&mut self) {
                self.0.abort();
            }
        }

        fn keyed_announce_url(address: SocketAddr) -> Url {
            Url::parse(&format!("http://{address}/announce/{KEY}")).expect("the keyed URL should parse") // DevSkim: ignore DS137138
        }

        fn keyed_scrape_url(address: SocketAddr) -> Url {
            Url::parse(&format!("http://{address}/scrape/{KEY}")).expect("the keyed URL should parse") // DevSkim: ignore DS137138
        }

        fn client(url: Url) -> Client {
            Client::new(url, Duration::from_secs(2)).expect("the client should build")
        }

        /// A client that adds `KEY` to the paths it builds, for a tracker at `base_url`.
        fn authenticated_client(base_url: &str) -> Client {
            Client::authenticated(
                Url::parse(base_url).expect("the base URL should parse"),
                Duration::from_secs(2),
                Key::new(KEY),
            )
            .expect("the client should build")
        }

        fn announce_query() -> Announce {
            AnnounceBuilder::with_default_values().query()
        }

        fn scrape_query() -> scrape_builder::Query {
            scrape_builder::Query::try_from(vec!["9c38422213e30bff212b30c360d26f9a02136422".to_string()]) // DevSkim: ignore DS173237
                .expect("the info-hash should parse")
        }

        #[tokio::test]
        async fn it_should_mask_the_tracker_key_when_an_announce_gets_no_response() {
            let client = client(keyed_announce_url(closed_local_address()));

            let error = tokio::time::timeout(TEST_DEADLINE, client.announce(&announce_query()))
                .await
                .expect("the announce should end before the test deadline")
                .expect_err("the announce to a closed port should fail");

            let text = error.to_string();
            assert!(!text.contains(KEY), "the error text should not contain the key: {text}");
            assert!(
                text.contains("/announce/****"),
                "the error text should show the masked URL: {text}"
            );
            assert!(
                !text.contains("reqwest::Error"),
                "the error text should not be reqwest Debug text: {text}"
            );
        }

        #[tokio::test]
        async fn it_should_mask_the_tracker_key_when_a_scrape_gets_no_response() {
            let client = client(keyed_scrape_url(closed_local_address()));

            let error = tokio::time::timeout(TEST_DEADLINE, client.scrape(&scrape_query()))
                .await
                .expect("the scrape should end before the test deadline")
                .expect_err("the scrape to a closed port should fail");

            let text = error.to_string();
            assert!(!text.contains(KEY), "the error text should not contain the key: {text}");
            assert!(
                text.contains("/scrape/****"),
                "the error text should show the masked URL: {text}"
            );
        }

        #[tokio::test]
        async fn it_should_mask_the_tracker_key_when_an_announce_gets_a_non_success_status() {
            let (address, _responder) = start_not_found_responder().await;
            let client = client(keyed_announce_url(address));

            let error = tokio::time::timeout(TEST_DEADLINE, client.announce(&announce_query()))
                .await
                .expect("the announce should end before the test deadline")
                .expect_err("a 404 reply should fail the announce");

            let text = error.to_string();
            assert!(!text.contains(KEY), "the error text should not contain the key: {text}");
            assert!(text.contains("404"), "the error text should name the status: {text}");
            assert!(
                text.contains("/announce/****"),
                "the error text should show the masked URL: {text}"
            );
        }

        #[tokio::test]
        async fn it_should_not_write_the_tracker_key_in_the_debug_text_of_the_error() {
            let client = client(keyed_announce_url(closed_local_address()));

            let error = tokio::time::timeout(TEST_DEADLINE, client.announce(&announce_query()))
                .await
                .expect("the announce should end before the test deadline")
                .expect_err("the announce to a closed port should fail");

            let text = format!("{error:?}");
            assert!(!text.contains(KEY), "the Debug text should not contain the key: {text}");
        }

        #[tokio::test]
        async fn it_should_keep_the_transport_cause_when_a_request_gets_no_response() {
            let client = client(keyed_announce_url(closed_local_address()));

            let error = tokio::time::timeout(TEST_DEADLINE, client.announce(&announce_query()))
                .await
                .expect("the announce should end before the test deadline")
                .expect_err("the announce to a closed port should fail");

            let text = error.to_string();
            assert!(
                text.contains("tcp connect error"),
                "the error text should keep the transport cause: {text}"
            );
        }

        #[tokio::test]
        async fn it_should_mask_the_client_key_when_a_request_to_a_custom_path_gets_no_response() {
            let client = authenticated_client(&format!("http://{}/custom-tracker-endpoint", closed_local_address())); // DevSkim: ignore DS137138

            let error = tokio::time::timeout(TEST_DEADLINE, client.announce(&announce_query()))
                .await
                .expect("the announce should end before the test deadline")
                .expect_err("the announce to a closed port should fail");

            let text = format!("{error} {error:?}");
            assert!(!text.contains(KEY), "the error texts should not contain the key: {text}");
            assert!(
                text.contains("/custom-tracker-endpoint/****"),
                "the error text should show the masked URL: {text}"
            );
        }

        #[tokio::test]
        async fn it_should_mask_the_client_key_when_a_request_to_a_custom_path_gets_a_non_success_status() {
            let (address, _responder) = start_not_found_responder().await;
            let client = authenticated_client(&format!("http://{address}/custom-tracker-endpoint")); // DevSkim: ignore DS137138

            let error = tokio::time::timeout(TEST_DEADLINE, client.announce(&announce_query()))
                .await
                .expect("the announce should end before the test deadline")
                .expect_err("a 404 reply should fail the announce");

            let text = format!("{error} {error:?}");
            assert!(!text.contains(KEY), "the error texts should not contain the key: {text}");
            assert!(
                text.contains("/custom-tracker-endpoint/****"),
                "the error text should show the masked URL: {text}"
            );
        }

        #[tokio::test]
        async fn it_should_mask_the_client_key_when_a_health_check_gets_no_response() {
            let client = authenticated_client(&format!("http://{}/", closed_local_address())); // DevSkim: ignore DS137138

            let error = tokio::time::timeout(TEST_DEADLINE, client.health_check())
                .await
                .expect("the health check should end before the test deadline")
                .expect_err("the health check to a closed port should fail");

            let text = format!("{error} {error:?}");
            assert!(!text.contains(KEY), "the error texts should not contain the key: {text}");
            assert!(
                text.contains("/health_check/****"),
                "the error text should show the masked URL: {text}"
            );
        }
    }
}
