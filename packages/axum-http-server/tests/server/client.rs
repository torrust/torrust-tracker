use std::net::IpAddr;
use std::time::Duration;

use reqwest::{Response, Url};
use torrust_tracker_client::http::client::{Client as TrackerClient, Key as TrackerClientKey};
use torrust_tracker_core::authentication::Key;
use torrust_tracker_http_protocol::v1::requests::announce::Announce;
use torrust_tracker_http_protocol::v1::requests::scrape_builder;

const TEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Thin wrapper over the canonical [`TrackerClient`] for integration tests.
///
/// Preserves the exact same public API as the original test-specific client so
/// that no call sites needed updating. Internally delegates everything to the
/// canonical `tracker-client` crate.
pub struct Client {
    inner: TrackerClient,
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
    fn base_url(server_addr: std::net::SocketAddr) -> Url {
        Url::parse(&format!("http://{server_addr}/")).unwrap()
    }

    pub fn new(server_addr: std::net::SocketAddr) -> Self {
        Self {
            inner: TrackerClient::new(Self::base_url(server_addr), TEST_TIMEOUT).unwrap(),
        }
    }

    /// Creates the new client binding it to an specific local address.
    pub fn bind(server_addr: std::net::SocketAddr, local_address: IpAddr) -> Self {
        Self {
            inner: TrackerClient::bind(Self::base_url(server_addr), TEST_TIMEOUT, local_address).unwrap(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn authenticated(server_addr: std::net::SocketAddr, key: Key) -> Self {
        Self {
            inner: TrackerClient::authenticated(Self::base_url(server_addr), TEST_TIMEOUT, TrackerClientKey::new(key.value()))
                .unwrap(),
        }
    }

    pub async fn announce(&self, query: &Announce) -> Response {
        self.inner.announce(query).await.unwrap()
    }

    pub async fn scrape(&self, query: &scrape_builder::Query) -> Response {
        self.inner.scrape(query).await.unwrap()
    }

    pub async fn announce_with_header(&self, query: &Announce, key: &str, value: &str) -> Response {
        self.inner.announce_with_header(query, key, value).await.unwrap()
    }

    pub async fn health_check(&self) -> Response {
        self.inner.health_check().await.unwrap()
    }

    pub async fn get(&self, path: &str) -> Response {
        self.inner.get(path).await.unwrap()
    }

    pub async fn get_with_header(&self, path: &str, key: &str, value: &str) -> Response {
        self.inner.get_with_header(path, key, value).await.unwrap()
    }
}
