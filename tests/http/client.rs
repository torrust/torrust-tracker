use reqwest::Response;

use super::connection_info::ConnectionInfo;
use super::requests::AnnounceQuery;

/// HTTP Tracker Client
pub struct Client {
    connection_info: ConnectionInfo,
}

impl Client {
    pub fn new(connection_info: ConnectionInfo) -> Self {
        Self { connection_info }
    }

    pub async fn announce(&self, query: &AnnounceQuery) -> Response {
        let path_with_query = format!("announce?{query}");
        self.get(&path_with_query).await
    }

    pub async fn get(&self, path: &str) -> Response {
        reqwest::Client::builder()
            .build()
            .unwrap()
            .get(self.base_url(path))
            .send()
            .await
            .unwrap()
    }

    fn base_url(&self, path: &str) -> String {
        format!("http://{}/{path}", &self.connection_info.bind_address)
    }
}
