use std::time::Duration;

use hyper::{HeaderMap, header};
use reqwest::{Response, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;
// Re-export AddKeyForm from the protocol package for backwards compatibility.
pub use torrust_tracker_rest_api_protocol::v1::context::auth_key::forms::add_key_form::AddKeyForm;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKey;
use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::Stats;
use torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::{ListItem, Torrent};
use url::Url;
use uuid::Uuid;

use crate::common::http::{Query, QueryParam, ReqwestQuery};
use crate::connection_info::ConnectionInfo;

pub const TOKEN_PARAM_NAME: &str = "token";
pub const AUTH_BEARER_TOKEN_HEADER_PREFIX: &str = "Bearer";

const API_PATH: &str = "api/v1/";
const DEFAULT_REQUEST_TIMEOUT_IN_SECS: u64 = 5;

/// Error type for [`ApiClient`] operations.
#[derive(Debug, Error)]
pub enum ClientError {
    /// A transport-level error (connection refused, timeout, DNS failure, etc.).
    #[error("transport error: {0}")]
    TransportError(#[source] reqwest::Error),

    /// The API returned a non-2xx status code.
    #[error("API error: {status} - {body}")]
    ApiError {
        /// The HTTP status code returned by the API.
        status: StatusCode,
        /// The response body (error message).
        body: String,
    },

    /// Failed to deserialize the API response body into the expected type.
    #[error("deserialization error: {0}")]
    DeserializationError(#[source] reqwest::Error),
}

/// High-level typed client for the Torrust Tracker REST API.
///
/// Wraps [`ApiHttpClient`] and returns protocol DTOs from `rest-api-protocol`.
/// Never panics — all errors are returned as [`ClientError`].
pub struct ApiClient {
    inner: ApiHttpClient,
}

impl ApiClient {
    /// Creates a new `ApiClient`.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the HTTP client cannot be built.
    pub fn new(connection_info: ConnectionInfo) -> Result<Self, ClientError> {
        Ok(Self {
            inner: ApiHttpClient::new(connection_info).map_err(ClientError::TransportError)?,
        })
    }

    /// Returns a reference to the inner [`ApiHttpClient`] for low-level operations.
    #[must_use]
    pub fn inner(&self) -> &ApiHttpClient {
        &self.inner
    }

    /// Generates a new random authentication key valid for `seconds_valid`.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    /// Returns [`ClientError::DeserializationError`] if the response cannot be parsed.
    pub async fn generate_auth_key(&self, seconds_valid: i32) -> Result<AuthKey, ClientError> {
        let response = self.inner.generate_auth_key(seconds_valid, None).await;
        Self::parse_response(response).await
    }

    /// Adds a new authentication key using the provided form data.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    /// Returns [`ClientError::DeserializationError`] if the response cannot be parsed.
    pub async fn add_auth_key(&self, form: AddKeyForm) -> Result<AuthKey, ClientError> {
        let response = self.inner.add_auth_key(form, None).await;
        Self::parse_response(response).await
    }

    /// Deletes an authentication key.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn delete_auth_key(&self, key: &str) -> Result<(), ClientError> {
        let response = self.inner.delete_auth_key(key, None).await;
        Self::check_success(response).await
    }

    /// Reloads authentication keys from the database.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn reload_keys(&self) -> Result<(), ClientError> {
        let response = self.inner.reload_keys(None).await;
        Self::check_success(response).await
    }

    /// Whitelists a torrent by info hash.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn whitelist_a_torrent(&self, info_hash: &str) -> Result<(), ClientError> {
        let response = self.inner.whitelist_a_torrent(info_hash, None).await;
        Self::check_success(response).await
    }

    /// Removes a torrent from the whitelist.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn remove_torrent_from_whitelist(&self, info_hash: &str) -> Result<(), ClientError> {
        let response = self.inner.remove_torrent_from_whitelist(info_hash, None).await;
        Self::check_success(response).await
    }

    /// Reloads the whitelist from the database.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn reload_whitelist(&self) -> Result<(), ClientError> {
        let response = self.inner.reload_whitelist(None).await;
        Self::check_success(response).await
    }

    /// Gets a single torrent by info hash.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    /// Returns [`ClientError::DeserializationError`] if the response cannot be parsed.
    pub async fn get_torrent(&self, info_hash: &str) -> Result<Torrent, ClientError> {
        let response = self.inner.get_torrent(info_hash, None).await;
        Self::parse_response(response).await
    }

    /// Gets a list of torrents matching the query parameters.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    /// Returns [`ClientError::DeserializationError`] if the response cannot be parsed.
    pub async fn get_torrents(&self, params: Query) -> Result<Vec<ListItem>, ClientError> {
        let response = self.inner.get_torrents(params, None).await;
        Self::parse_response(response).await
    }

    /// Gets tracker statistics.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    /// Returns [`ClientError::DeserializationError`] if the response cannot be parsed.
    pub async fn get_tracker_statistics(&self) -> Result<Stats, ClientError> {
        let response = self.inner.get_tracker_statistics(None).await;
        Self::parse_response(response).await
    }

    /// Parses a successful response into the expected DTO type.
    async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, ClientError> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.map_err(ClientError::TransportError)?;
            return Err(ClientError::ApiError { status, body });
        }
        response.json::<T>().await.map_err(ClientError::DeserializationError)
    }

    /// Checks that the response has a 2xx status code, ignoring the body.
    async fn check_success(response: Response) -> Result<(), ClientError> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.map_err(ClientError::TransportError)?;
            return Err(ClientError::ApiError { status, body });
        }
        Ok(())
    }
}

/// Low-level HTTP transport for the Torrust Tracker REST API.
///
/// Handles connection info, URL building, auth headers, and raw HTTP requests.
/// Returns [`reqwest::Response`] directly. For a typed high-level API, use
/// [`ApiClient`].
#[allow(clippy::struct_field_names)]
pub struct ApiHttpClient {
    connection_info: ConnectionInfo,
    base_path: String,
    http_client: reqwest::Client,
}

impl ApiHttpClient {
    /// # Errors
    ///
    /// Will return an error if the HTTP client can't be created.
    pub fn new(connection_info: ConnectionInfo) -> Result<Self, reqwest::Error> {
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
        self.post_empty(&format!("key/{seconds_valid}"), headers).await
    }

    pub async fn add_auth_key(&self, add_key_form: AddKeyForm, headers: Option<HeaderMap>) -> Response {
        self.post_form("keys", &add_key_form, headers).await
    }

    pub async fn delete_auth_key(&self, key: &str, headers: Option<HeaderMap>) -> Response {
        self.delete(&format!("key/{key}"), headers).await
    }

    pub async fn reload_keys(&self, headers: Option<HeaderMap>) -> Response {
        self.get("keys/reload", Query::default(), headers).await
    }

    pub async fn whitelist_a_torrent(&self, info_hash: &str, headers: Option<HeaderMap>) -> Response {
        self.post_empty(&format!("whitelist/{info_hash}"), headers).await
    }

    pub async fn remove_torrent_from_whitelist(&self, info_hash: &str, headers: Option<HeaderMap>) -> Response {
        self.delete(&format!("whitelist/{info_hash}"), headers).await
    }

    pub async fn reload_whitelist(&self, headers: Option<HeaderMap>) -> Response {
        self.get("whitelist/reload", Query::default(), headers).await
    }

    pub async fn get_torrent(&self, info_hash: &str, headers: Option<HeaderMap>) -> Response {
        self.get(&format!("torrent/{info_hash}"), Query::default(), headers).await
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
        Url::parse(&format!("{}{}{path}", self.connection_info.origin, self.base_path)).unwrap()
    }
}

/// # Panics
///
/// Will panic if the request can't be sent
pub async fn get(path: Url, query: Option<Query>, headers: Option<HeaderMap>) -> Response {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_IN_SECS))
        .build()
        .unwrap();

    let mut request_builder = client.get(path);

    if let Some(params) = query {
        request_builder = request_builder.query(&ReqwestQuery::from(params));
    }

    if let Some(headers) = headers {
        request_builder = request_builder.headers(headers);
    }

    request_builder.send().await.unwrap()
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
