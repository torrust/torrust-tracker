use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use colored::Colorize;
use thiserror::Error;
use torrust_tracker_primitives::info_hash::InfoHash;
use url::Url;

use crate::console::clients::checker::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::console::clients::checker::service::{CheckError, CheckResult};
use crate::shared::bit_torrent::tracker::http::client::requests::announce::QueryBuilder;
use crate::shared::bit_torrent::tracker::http::client::responses::announce::Announce;
use crate::shared::bit_torrent::tracker::http::client::responses::scrape;
use crate::shared::bit_torrent::tracker::http::client::{requests, Client};

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Http request did not receive a response within the timeout: {err:?}")]
    HttpClientError {
        err: crate::shared::bit_torrent::tracker::http::client::Error,
    },
    #[error("Http failed to get a response at all: {err:?}")]
    ResponseError { err: Arc<reqwest::Error> },
    #[error("Failed to deserialize the serde bencoded response data with the error: \"{err:?}\"")]
    ParseSerdeBencodeError {
        data: hyper::body::Bytes,
        err: Arc<serde_bencode::Error>,
    },

    #[error("Failed to deserialize the bencoded response data with the error: \"{err:?}\"")]
    ParseScrapeBencodeError {
        data: hyper::body::Bytes,
        err: Arc<scrape::BencodeParseError>,
    },
}

pub async fn run(http_trackers: Vec<Url>, timeout: Duration, console: Console) -> Vec<CheckResult> {
    let mut check_results = Vec::default();

    console.println("HTTP trackers ...");

    for ref url in http_trackers {
        let colored_url = url.to_string().yellow();

        check_results.push(match check_http_announce(url, timeout).await {
            Ok(_) => {
                console.println(&format!("{} - Announce at {} is OK", "✓".green(), colored_url));
                Ok(())
            }
            Err(err) => {
                console.println(&format!("{} - Announce at {} is failing", "✗".red(), colored_url));
                Err(CheckError::HttpCheckError { url: url.clone(), err })
            }
        });

        check_results.push(match check_http_scrape(url, timeout).await {
            Ok(_) => {
                console.println(&format!("{} - Scrape at {} is OK", "✓".green(), colored_url));
                Ok(())
            }
            Err(err) => {
                console.println(&format!("{} - Scrape at {} is failing", "✗".red(), colored_url));
                Err(CheckError::HttpCheckError { url: url.clone(), err })
            }
        });
    }

    check_results
}

async fn check_http_announce(url: &Url, timeout: Duration) -> Result<Announce, Error> {
    let info_hash_str = "9c38422213e30bff212b30c360d26f9a02136422".to_string(); // # DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&info_hash_str).expect("a valid info-hash is required");

    let client = Client::new(url.clone(), timeout).map_err(|err| Error::HttpClientError { err })?;

    let response = client
        .announce(&QueryBuilder::with_default_values().with_info_hash(&info_hash).query())
        .await
        .map_err(|err| Error::HttpClientError { err })?;

    let body = response.bytes().await.map_err(|e| Error::ResponseError { err: e.into() })?;

    serde_bencode::from_bytes::<Announce>(&body).map_err(|e| Error::ParseSerdeBencodeError {
        data: body,
        err: e.into(),
    })
}

async fn check_http_scrape(url: &Url, timeout: Duration) -> Result<scrape::Response, Error> {
    let info_hashes: Vec<String> = vec!["9c38422213e30bff212b30c360d26f9a02136422".to_string()]; // # DevSkim: ignore DS173237
    let query = requests::scrape::Query::try_from(info_hashes).expect("a valid array of info-hashes is required");

    let client = Client::new(url.clone(), timeout).map_err(|err| Error::HttpClientError { err })?;

    let response = client.scrape(&query).await.map_err(|err| Error::HttpClientError { err })?;

    let body = response.bytes().await.map_err(|e| Error::ResponseError { err: e.into() })?;

    scrape::Response::try_from_bencoded(&body).map_err(|e| Error::ParseScrapeBencodeError {
        data: body,
        err: e.into(),
    })
}
