use reqwest::Response;
use serde::Serialize;

use crate::common::http::{Query, QueryParam, ReqwestQuery};
use crate::servers::api::connection_info::ConnectionInfo;

/// API Client
pub struct Client {
    connection_info: ConnectionInfo,
    base_path: String,
}

impl Client {
    pub fn new(connection_info: ConnectionInfo) -> Self {
        Self {
            connection_info,
            base_path: "/api/v1/".to_string(),
        }
    }

    pub async fn generate_auth_key(&self, seconds_valid: i32) -> Response {
        self.post_empty(&format!("key/{}", &seconds_valid)).await
    }

    pub async fn add_auth_key(&self, add_key_form: AddKeyForm) -> Response {
        self.post_form("keys", &add_key_form).await
    }

    pub async fn delete_auth_key(&self, key: &str) -> Response {
        self.delete(&format!("key/{}", &key)).await
    }

    pub async fn reload_keys(&self) -> Response {
        self.get("keys/reload", Query::default()).await
    }

    pub async fn whitelist_a_torrent(&self, info_hash: &str) -> Response {
        self.post_empty(&format!("whitelist/{}", &info_hash)).await
    }

    pub async fn remove_torrent_from_whitelist(&self, info_hash: &str) -> Response {
        self.delete(&format!("whitelist/{}", &info_hash)).await
    }

    pub async fn reload_whitelist(&self) -> Response {
        self.get("whitelist/reload", Query::default()).await
    }

    pub async fn get_torrent(&self, info_hash: &str) -> Response {
        self.get(&format!("torrent/{}", &info_hash), Query::default()).await
    }

    pub async fn get_torrents(&self, params: Query) -> Response {
        self.get("torrents", params).await
    }

    pub async fn get_tracker_statistics(&self) -> Response {
        self.get("stats", Query::default()).await
    }

    pub async fn get(&self, path: &str, params: Query) -> Response {
        let mut query: Query = params;

        if let Some(token) = &self.connection_info.api_token {
            query.add_param(QueryParam::new("token", token));
        };

        self.get_request_with_query(path, query).await
    }

    pub async fn post_empty(&self, path: &str) -> Response {
        reqwest::Client::new()
            .post(self.base_url(path).clone())
            .query(&ReqwestQuery::from(self.query_with_token()))
            .send()
            .await
            .unwrap()
    }

    pub async fn post_form<T: Serialize + ?Sized>(&self, path: &str, form: &T) -> Response {
        reqwest::Client::new()
            .post(self.base_url(path).clone())
            .query(&ReqwestQuery::from(self.query_with_token()))
            .json(&form)
            .send()
            .await
            .unwrap()
    }

    async fn delete(&self, path: &str) -> Response {
        reqwest::Client::new()
            .delete(self.base_url(path).clone())
            .query(&ReqwestQuery::from(self.query_with_token()))
            .send()
            .await
            .unwrap()
    }

    pub async fn get_request_with_query(&self, path: &str, params: Query) -> Response {
        get(&self.base_url(path), Some(params)).await
    }

    pub async fn get_request(&self, path: &str) -> Response {
        get(&self.base_url(path), None).await
    }

    fn query_with_token(&self) -> Query {
        match &self.connection_info.api_token {
            Some(token) => Query::params([QueryParam::new("token", token)].to_vec()),
            None => Query::default(),
        }
    }

    fn base_url(&self, path: &str) -> String {
        format!("http://{}{}{path}", &self.connection_info.bind_address, &self.base_path)
    }
}

pub async fn get(path: &str, query: Option<Query>) -> Response {
    match query {
        Some(params) => reqwest::Client::builder()
            .build()
            .unwrap()
            .get(path)
            .query(&ReqwestQuery::from(params))
            .send()
            .await
            .unwrap(),
        None => reqwest::Client::builder().build().unwrap().get(path).send().await.unwrap(),
    }
}

#[derive(Serialize, Debug)]
pub struct AddKeyForm {
    #[serde(rename = "key")]
    pub opt_key: Option<String>,
    pub seconds_valid: u64,
}
