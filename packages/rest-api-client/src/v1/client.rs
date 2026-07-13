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

    /// An internal error (URL construction failure, etc.).
    #[error("internal error: {0}")]
    InternalError(String),
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        Self::TransportError(err)
    }
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
        let response = self.inner.post_empty_result(&format!("key/{seconds_valid}"), None).await?;
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
        let response = self.inner.post_form_result("keys", &form, None).await?;
        Self::parse_response(response).await
    }

    /// Deletes an authentication key.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn delete_auth_key(&self, key: &str) -> Result<(), ClientError> {
        let response = self.inner.delete_result(&format!("key/{key}"), None).await?;
        Self::check_success(response).await
    }

    /// Reloads authentication keys from the database.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn reload_keys(&self) -> Result<(), ClientError> {
        let response = self.inner.get_result("keys/reload", Query::default(), None).await?;
        Self::check_success(response).await
    }

    /// Whitelists a torrent by info hash.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn whitelist_a_torrent(&self, info_hash: &str) -> Result<(), ClientError> {
        let response = self.inner.post_empty_result(&format!("whitelist/{info_hash}"), None).await?;
        Self::check_success(response).await
    }

    /// Removes a torrent from the whitelist.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn remove_torrent_from_whitelist(&self, info_hash: &str) -> Result<(), ClientError> {
        let response = self.inner.delete_result(&format!("whitelist/{info_hash}"), None).await?;
        Self::check_success(response).await
    }

    /// Reloads the whitelist from the database.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::TransportError`] if the request fails.
    /// Returns [`ClientError::ApiError`] if the API returns a non-2xx status.
    pub async fn reload_whitelist(&self) -> Result<(), ClientError> {
        let response = self.inner.get_result("whitelist/reload", Query::default(), None).await?;
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
        let response = self
            .inner
            .get_result(&format!("torrent/{info_hash}"), Query::default(), None)
            .await?;
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
        let response = self.inner.get_result("torrents", params, None).await?;
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
        let response = self.inner.get_result("stats", Query::default(), None).await?;
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

    /// Generates a new random authentication key valid for `seconds_valid`.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn generate_auth_key(&self, seconds_valid: i32, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.post_empty_result(&format!("key/{seconds_valid}"), headers).await
    }

    /// Adds a new authentication key using the provided form data.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn add_auth_key(&self, add_key_form: AddKeyForm, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.post_form_result("keys", &add_key_form, headers).await
    }

    /// Deletes an authentication key.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn delete_auth_key(&self, key: &str, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.delete_result(&format!("key/{key}"), headers).await
    }

    /// Reloads authentication keys from the database.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn reload_keys(&self, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.get_result("keys/reload", Query::default(), headers).await
    }

    /// Whitelists a torrent by info hash.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn whitelist_a_torrent(&self, info_hash: &str, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.post_empty_result(&format!("whitelist/{info_hash}"), headers).await
    }

    /// Removes a torrent from the whitelist.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn remove_torrent_from_whitelist(
        &self,
        info_hash: &str,
        headers: Option<HeaderMap>,
    ) -> Result<Response, ClientError> {
        self.delete_result(&format!("whitelist/{info_hash}"), headers).await
    }

    /// Reloads the whitelist from the database.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn reload_whitelist(&self, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.get_result("whitelist/reload", Query::default(), headers).await
    }

    /// Gets a single torrent by info hash.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn get_torrent(&self, info_hash: &str, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.get_result(&format!("torrent/{info_hash}"), Query::default(), headers)
            .await
    }

    /// Gets a list of torrents matching the query parameters.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn get_torrents(&self, params: Query, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.get_result("torrents", params, headers).await
    }

    /// Gets tracker statistics.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn get_tracker_statistics(&self, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.get_result("stats", Query::default(), headers).await
    }

    /// Performs a GET request.
    ///
    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn get(&self, path: &str, params: Query, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.get_result(path, params, headers).await
    }

    /// Fallible version of [`Self::get`] that returns a `Result` instead of panicking.
    pub(crate) async fn get_result(
        &self,
        path: &str,
        params: Query,
        headers: Option<HeaderMap>,
    ) -> Result<Response, ClientError> {
        let mut query: Query = params;

        if let Some(token) = &self.connection_info.api_token {
            query.add_param(QueryParam::new(TOKEN_PARAM_NAME, token));
        }

        self.get_request_with_query_result(path, query, headers).await
    }

    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn post_empty(&self, path: &str, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        self.post_empty_result(path, headers).await
    }

    /// Fallible version of [`Self::post_empty`] that returns a `Result` instead of panicking.
    pub(crate) async fn post_empty_result(&self, path: &str, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        let builder = self.http_client.post(self.base_url(path)?.clone());

        let builder = match headers {
            Some(headers) => builder.headers(headers),
            None => builder,
        };

        let builder = match &self.connection_info.api_token {
            Some(token) => builder.header(header::AUTHORIZATION, format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")),
            None => builder,
        };

        Ok(builder.send().await?)
    }

    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn post_form<T: Serialize + ?Sized>(
        &self,
        path: &str,
        form: &T,
        headers: Option<HeaderMap>,
    ) -> Result<Response, ClientError> {
        self.post_form_result(path, form, headers).await
    }

    /// Fallible version of [`Self::post_form`] that returns a `Result` instead of panicking.
    pub(crate) async fn post_form_result<T: Serialize + ?Sized>(
        &self,
        path: &str,
        form: &T,
        headers: Option<HeaderMap>,
    ) -> Result<Response, ClientError> {
        let builder = self.http_client.post(self.base_url(path)?.clone()).json(&form);

        let builder = match headers {
            Some(headers) => builder.headers(headers),
            None => builder,
        };

        let builder = match &self.connection_info.api_token {
            Some(token) => builder.header(header::AUTHORIZATION, format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")),
            None => builder,
        };

        Ok(builder.send().await?)
    }

    /// Fallible version of [`Self::delete`] that returns a `Result` instead of panicking.
    async fn delete_result(&self, path: &str, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
        let builder = self.http_client.delete(self.base_url(path)?.clone());

        let builder = match headers {
            Some(headers) => builder.headers(headers),
            None => builder,
        };

        let builder = match &self.connection_info.api_token {
            Some(token) => builder.header(header::AUTHORIZATION, format!("{AUTH_BEARER_TOKEN_HEADER_PREFIX} {token}")),
            None => builder,
        };

        Ok(builder.send().await?)
    }

    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn get_request_with_query(
        &self,
        path: &str,
        params: Query,
        headers: Option<HeaderMap>,
    ) -> Result<Response, ClientError> {
        self.get_request_with_query_result(path, params, headers).await
    }

    /// Fallible version of [`Self::get_request_with_query`] that returns a `Result` instead of panicking.
    pub(crate) async fn get_request_with_query_result(
        &self,
        path: &str,
        params: Query,
        headers: Option<HeaderMap>,
    ) -> Result<Response, ClientError> {
        let url = self.base_url(path)?;
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

                get_result(url, Some(params), Some(headers)).await
            }
            None => get_result(url, Some(params), headers).await,
        }
    }

    /// # Errors
    ///
    /// Will return an error if the request can't be sent.
    pub async fn get_request(&self, path: &str) -> Result<Response, ClientError> {
        let url = self.base_url(path)?;
        get_result(url, None, None).await
    }

    fn base_url(&self, path: &str) -> Result<Url, ClientError> {
        Url::parse(&format!("{}{}{path}", self.connection_info.origin, self.base_path))
            .map_err(|e| ClientError::InternalError(format!("invalid URL: {e}")))
    }
}

/// # Errors
///
/// Will return an error if the request can't be sent.
pub async fn get(path: Url, query: Option<Query>, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
    get_result(path, query, headers).await
}

/// Fallible version of [`get`] that returns a `Result` instead of panicking.
pub(crate) async fn get_result(path: Url, query: Option<Query>, headers: Option<HeaderMap>) -> Result<Response, ClientError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(DEFAULT_REQUEST_TIMEOUT_IN_SECS))
        .build()?;

    let mut request_builder = client.get(path);

    if let Some(params) = query {
        request_builder = request_builder.query(&ReqwestQuery::from(params));
    }

    if let Some(headers) = headers {
        request_builder = request_builder.headers(headers);
    }

    request_builder.send().await.map_err(ClientError::TransportError)
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
