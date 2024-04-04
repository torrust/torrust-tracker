use std::sync::Arc;
use std::time::Duration;

use thiserror::Error;
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::peer;
use url::Url;

use crate::shared::bit_torrent::tracker::http::client::requests::{announce, scrape};
use crate::shared::bit_torrent::tracker::http::client::{responses, Client};

pub mod app;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Http request did not receive a response within the timeout: {err:?}")]
    HttpClientError {
        err: crate::shared::bit_torrent::tracker::http::client::Error,
    },
    #[error("Http failed to get a response at all: {err:?}")]
    ResponseError { err: Arc<reqwest::Error> },

    #[error("Failed to deserialize the bencoded response data with the error: \"{err:?}\"")]
    ParseBencodeError {
        data: hyper::body::Bytes,
        err: responses::BencodeParseError,
    },
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub async fn check_http_announce(url: &Url, &timeout: &Duration, &info_hash: &InfoHash) -> Result<responses::Announce, Error> {
    let client = Client::new(url.clone(), timeout).map_err(|err| Error::HttpClientError { err })?;

    let response = client
        .announce(&announce::QueryBuilder::new(info_hash, peer::Id::from(1), 17548).build())
        .await
        .map_err(|err| Error::HttpClientError { err })?;

    let body = response.bytes().await.map_err(|e| Error::ResponseError { err: e.into() })?;

    responses::announce::ResponseBuilder::try_from(&body)
        .map_err(|err| Error::ParseBencodeError { data: body, err })
        .map(responses::announce::ResponseBuilder::build)
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub async fn check_http_scrape(url: &Url, &timeout: &Duration, info_hashes: &[InfoHash]) -> Result<responses::Scrape, Error> {
    let query = info_hashes.iter().copied().collect::<scrape::QueryBuilder>().build();

    let client = Client::new(url.clone(), timeout).map_err(|err| Error::HttpClientError { err })?;

    let response = client.scrape(&query).await.map_err(|err| Error::HttpClientError { err })?;

    let body = response.bytes().await.map_err(|e| Error::ResponseError { err: e.into() })?;

    responses::scrape::ResponseBuilder::try_from(&body)
        .map_err(|err| Error::ParseBencodeError { data: body, err })
        .map(responses::scrape::ResponseBuilder::build)
}
