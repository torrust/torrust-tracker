use std::time::Duration;

use hyper::{header, HeaderMap};
use reqwest::{Error, Response};
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::common::http::{Query, QueryParam, ReqwestQuery};
use crate::connection_info::ConnectionInfo;

pub const TOKEN_PARAM_NAME: &str = "token";
pub const AUTH_BEARER_TOKEN_HEADER_PREFIX: &str = "Bearer";

const API_PATH: &str = "api/v1/";
const DEFAULT_REQUEST_TIMEOUT_IN_SECS: u64 = 5;

/// API Client
#[allow(clippy::struct_field_names)]
pub struct Client {
    connection_info: ConnectionInfo,
    base_path: String,
    http_client: reqwest::Client,
}

impl Client {
    /// # Errors
    ///
    /// Will return an error if the HTTP client can't be created.
    pub fn new(connection_info: ConnectionInfo) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_IN_SECS))
            .build()?;

        Ok(Self {
            connection_info,
            base_path: API_PATH.to_string(),
            http_client: client,
        })
    }

    pub async fn generate_auth_key(&self, seconds_valid: i32, headers: Option<HeaderMap>) -> Response {
        self.post_empty(&format!("key/{}", &seconds_valid), headers).await
    }

    pub async fn add_auth_key(&self, add_key_form: AddKeyForm, headers: Option<HeaderMap>) -> Response {
        self.post_form("keys", &add_key_form, headers).await
    }

    pub async fn delete_auth_key(&self, key: &str, headers: Option<HeaderMap>) -> Response {
        self.delete(&format!("key/{}", &key), headers).await
    }

    pub async fn reload_keys(&self, headers: Option<HeaderMap>) -> Response {
        self.get("keys/reload", Query::default(), headers).await
    }

    pub async fn whitelist_a_torrent(&self, info_hash: &str, headers: Option<HeaderMap>) -> Response {
        self.post_empty(&format!("whitelist/{}", &info_hash), headers).await
    }

    pub async fn remove_torrent_from_whitelist(&self, info_hash: &str, headers: Option<HeaderMap>) -> Response {
        self.delete(&format!("whitelist/{}", &info_hash), headers).await
    }

    pub async fn reload_whitelist(&self, headers: Option<HeaderMap>) -> Response {
        self.get("whitelist/reload", Query::default(), headers).await
    }

    pub async fn get_torrent(&self, info_hash: &str, headers: Option<HeaderMap>) -> Response {
        self.get(&format!("torrent/{}", &info_hash), Query::default(), headers).await
    }

    pub async fn get_torrents(&self, params: Query, headers: Option<HeaderMap>) -> Response {
        self.get("torrents", params, headers).await
    }

    pub async fn get_tracker_statistics(&self, headers: Option<HeaderMap>) -> Response {
        self.get("stats", Query::default(), headers).await
    }

    pub async fn get(&self, path: &str, params: Query, headers: Option<HeaderMap>) -> Response {
        let mut query: Query = params;

        if let Some(token) = &self.connection_info.api_token {
            query.add_param(QueryParam::new(TOKEN_PARAM_NAME, token));
        }

        self.get_request_with_query(path, query, headers).await
    }

    /// # Panics
    ///
    /// Will panic if the request can't be sent
    pub async fn post_empty(&self, path: &str, headers: Option<HeaderMap>) -> Response {
        let builder = self.http_client.post(self.base_url(path).clone());

        let builder = match headers {
            Some(headers) => builder.headers(headers),
            None => builder,
        };

        let builder = match &self.connection_info.api_token {
            Some(token) => builder.header(header::AUTHORIZATION, format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")),
            None => builder,
        };

        builder.send().await.unwrap()
    }

    /// # Panics
    ///
    /// Will panic if the request can't be sent
    pub async fn post_form<T: Serialize + ?Sized>(&self, path: &str, form: &T, headers: Option<HeaderMap>) -> Response {
        let builder = self.http_client.post(self.base_url(path).clone()).json(&form);

        let builder = match headers {
            Some(headers) => builder.headers(headers),
            None => builder,
        };

        let builder = match &self.connection_info.api_token {
            Some(token) => builder.header(header::AUTHORIZATION, format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")),
            None => builder,
        };

        builder.send().await.unwrap()
    }

    /// # Panics
    ///
    /// Will panic if the request can't be sent
    async fn delete(&self, path: &str, headers: Option<HeaderMap>) -> Response {
        let builder = self.http_client.delete(self.base_url(path).clone());

        let builder = match headers {
            Some(headers) => builder.headers(headers),
            None => builder,
        };

        let builder = match &self.connection_info.api_token {
            Some(token) => builder.header(header::AUTHORIZATION, format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")),
            None => builder,
        };

        builder.send().await.unwrap()
    }

    /// # Panics
    ///
    /// Will panic if it can't convert the authentication token to a `HeaderValue`.
    pub async fn get_request_with_query(&self, path: &str, params: Query, headers: Option<HeaderMap>) -> Response {
        match &self.connection_info.api_token {
            Some(token) => {
                let headers = if let Some(headers) = headers {
                    // Headers provided -> add auth token if not already present

                    if headers.get(header::AUTHORIZATION).is_some() {
                        // Auth token already present -> use provided
                        headers
                    } else {
                        let mut headers = headers;

                        headers.insert(
                            header::AUTHORIZATION,
                            format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")
                                .parse()
                                .expect("the auth token is not a valid header value"),
                        );

                        headers
                    }
                } else {
                    // No headers provided -> create headers with auth token

                    let mut headers = HeaderMap::new();

                    headers.insert(
                        header::AUTHORIZATION,
                        format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")
                            .parse()
                            .expect("the auth token is not a valid header value"),
                    );

                    headers
                };

                get(self.base_url(path), Some(params), Some(headers)).await
            }
            None => get(self.base_url(path), Some(params), headers).await,
        }
    }

    pub async fn get_request(&self, path: &str) -> Response {
        get(self.base_url(path), None, None).await
    }

    fn base_url(&self, path: &str) -> Url {
        Url::parse(&format!("{}{}{path}", &self.connection_info.origin, &self.base_path)).unwrap()
    }
}

/// # Panics
///
/// Will panic if the request can't be sent
pub async fn get(path: Url, query: Option<Query>, headers: Option<HeaderMap>) -> Response {
    let builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_IN_SECS))
        .build()
        .unwrap();

    let builder = match query {
        Some(params) => builder.get(path).query(&ReqwestQuery::from(params)),
        None => builder.get(path),
    };

    let builder = match headers {
        Some(headers) => builder.headers(headers),
        None => builder,
    };

    builder.send().await.unwrap()
}

/// Returns a `HeaderMap` with a request id header.
///
/// # Panics
///
/// Will panic if the request ID can't be parsed into a `HeaderValue`.
#[must_use]
pub fn headers_with_request_id(request_id: Uuid) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-request-id",
        request_id
            .to_string()
            .parse()
            .expect("the request ID is not a valid header value"),
    );
    headers
}

/// Returns a `HeaderMap` with an authorization token.
///
/// # Panics
///
/// Will panic if the token can't be parsed into a `HeaderValue`.
#[must_use]
pub fn headers_with_auth_token(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")
            .parse()
            .expect("the auth token is not a valid header value"),
    );
    headers
}

#[derive(Serialize, Debug)]
pub struct AddKeyForm {
    #[serde(rename = "key")]
    pub opt_key: Option<String>,
    pub seconds_valid: Option<u64>,
}
