use reqwest::Response;

use super::connection_info::ConnectionInfo;
use crate::common::http::{get, Query};

/// HTTP Tracker Client
pub struct Client {
    connection_info: ConnectionInfo,
    base_path: String,
}

impl Client {
    pub fn new(connection_info: ConnectionInfo) -> Self {
        Self {
            connection_info,
            base_path: "/".to_string(),
        }
    }

    pub async fn announce(&self, params: Query) -> Response {
        self.get("announce", params).await
    }

    pub async fn scrape(&self, params: Query) -> Response {
        self.get("scrape", params).await
    }

    async fn get(&self, path: &str, params: Query) -> Response {
        get(&self.base_url(path), Some(params)).await
    }

    fn base_url(&self, path: &str) -> String {
        format!("http://{}{}{path}", &self.connection_info.bind_address, &self.base_path)
    }
}
