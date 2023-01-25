use std::net::IpAddr;

use reqwest::{Client as ReqwestClient, Response};

use super::connection_info::ConnectionInfo;
use super::requests::AnnounceQuery;

/// HTTP Tracker Client
pub struct Client {
    connection_info: ConnectionInfo,
    reqwest_client: ReqwestClient,
}

impl Client {
    pub fn new(connection_info: ConnectionInfo) -> Self {
        Self {
            connection_info,
            reqwest_client: reqwest::Client::builder().build().unwrap(),
        }
    }

    /// Creates the new client binding it to an specific local address
    pub fn bind(connection_info: ConnectionInfo, local_address: IpAddr) -> Self {
        Self {
            connection_info,
            reqwest_client: reqwest::Client::builder().local_address(local_address).build().unwrap(),
        }
    }

    pub async fn announce(&self, query: &AnnounceQuery) -> Response {
        self.get(&format!("announce?{query}")).await
    }

    pub async fn get(&self, path: &str) -> Response {
        self.reqwest_client.get(self.base_url(path)).send().await.unwrap()
    }

    fn base_url(&self, path: &str) -> String {
        format!("http://{}/{path}", &self.connection_info.bind_address)
    }
}
