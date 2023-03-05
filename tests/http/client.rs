use std::net::IpAddr;

use reqwest::{Client as ReqwestClient, Response};
use torrust_tracker::tracker::auth::Key;

use super::connection_info::ConnectionInfo;
use super::requests::announce::{self, Query};
use super::requests::scrape;

/// HTTP Tracker Client
pub struct Client {
    connection_info: ConnectionInfo,
    reqwest_client: ReqwestClient,
    key_id: Option<Key>,
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
    pub fn new(connection_info: ConnectionInfo) -> Self {
        Self {
            connection_info,
            reqwest_client: reqwest::Client::builder().build().unwrap(),
            key_id: None,
        }
    }

    /// Creates the new client binding it to an specific local address
    pub fn bind(connection_info: ConnectionInfo, local_address: IpAddr) -> Self {
        Self {
            connection_info,
            reqwest_client: reqwest::Client::builder().local_address(local_address).build().unwrap(),
            key_id: None,
        }
    }

    pub fn authenticated(connection_info: ConnectionInfo, key_id: Key) -> Self {
        Self {
            connection_info,
            reqwest_client: reqwest::Client::builder().build().unwrap(),
            key_id: Some(key_id),
        }
    }

    pub async fn announce(&self, query: &announce::Query) -> Response {
        self.get(&self.build_announce_path_and_query(query)).await
    }

    pub async fn scrape(&self, query: &scrape::Query) -> Response {
        self.get(&self.build_scrape_path_and_query(query)).await
    }

    pub async fn announce_with_header(&self, query: &Query, key_id: &str, value: &str) -> Response {
        self.get_with_header(&self.build_announce_path_and_query(query), key_id, value)
            .await
    }

    pub async fn get(&self, path: &str) -> Response {
        self.reqwest_client.get(self.build_url(path)).send().await.unwrap()
    }

    pub async fn get_with_header(&self, path: &str, key: &str, value: &str) -> Response {
        self.reqwest_client
            .get(self.build_url(path))
            .header(key, value)
            .send()
            .await
            .unwrap()
    }

    fn build_announce_path_and_query(&self, query: &announce::Query) -> String {
        format!("{}?{query}", self.build_path("announce"))
    }

    fn build_scrape_path_and_query(&self, query: &scrape::Query) -> String {
        format!("{}?{query}", self.build_path("scrape"))
    }

    fn build_path(&self, path: &str) -> String {
        match &self.key_id {
            Some(key_id) => format!("{path}/{key_id}"),
            None => path.to_string(),
        }
    }

    fn build_url(&self, path: &str) -> String {
        let base_url = self.base_url();
        format!("{base_url}{path}")
    }

    fn base_url(&self) -> String {
        format!("http://{}/", &self.connection_info.bind_address)
    }
}
