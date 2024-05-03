use std::str::FromStr;

use log::debug;
use reqwest::Url as ServiceUrl;
use torrust_tracker_primitives::info_hash::InfoHash;
use url::Url;

use super::structs::{CheckerOutput, Status};
use crate::console::clients::checker::service::{CheckError, CheckResult};
use crate::shared::bit_torrent::tracker::http::client::requests::announce::QueryBuilder;
use crate::shared::bit_torrent::tracker::http::client::responses::announce::Announce;
use crate::shared::bit_torrent::tracker::http::client::responses::scrape;
use crate::shared::bit_torrent::tracker::http::client::{requests, Client};

#[allow(clippy::missing_panics_doc)]
pub async fn run(http_trackers: &Vec<ServiceUrl>, check_results: &mut Vec<CheckResult>) -> Vec<CheckerOutput> {
    let mut http_checkers: Vec<CheckerOutput> = Vec::new();

    for http_tracker in http_trackers {
        let mut http_checker = CheckerOutput {
            url: http_tracker.to_string(),
            status: Status {
                code: String::new(),
                message: String::new(),
            },
        };

        match check_http_announce(http_tracker).await {
            Ok(()) => {
                check_results.push(Ok(()));
                http_checker.status.code = "ok".to_string();
            }
            Err(err) => {
                check_results.push(Err(err));
                http_checker.status.code = "error".to_string();
                http_checker.status.message = "Announce is failing.".to_string();
            }
        }

        match check_http_scrape(http_tracker).await {
            Ok(()) => {
                check_results.push(Ok(()));
                http_checker.status.code = "ok".to_string();
            }
            Err(err) => {
                check_results.push(Err(err));
                http_checker.status.code = "error".to_string();
                http_checker.status.message = "Scrape is failing.".to_string();
            }
        }
        http_checkers.push(http_checker);
    }
    http_checkers
}

async fn check_http_announce(tracker_url: &Url) -> Result<(), CheckError> {
    let info_hash_str = "9c38422213e30bff212b30c360d26f9a02136422".to_string(); // # DevSkim: ignore DS173237
    let info_hash = InfoHash::from_str(&info_hash_str).expect("a valid info-hash is required");

    // todo: HTTP request could panic.For example, if the server is not accessible.
    // We should change the client to catch that error and return a `CheckError`.
    // Otherwise the checking process will stop. The idea is to process all checks
    // and return a final report.
    let Ok(client) = Client::new(tracker_url.clone()) else {
        return Err(CheckError::HttpError {
            url: (tracker_url.to_owned()),
        });
    };
    let Ok(response) = client
        .announce(&QueryBuilder::with_default_values().with_info_hash(&info_hash).query())
        .await
    else {
        return Err(CheckError::HttpError {
            url: (tracker_url.to_owned()),
        });
    };

    if let Ok(body) = response.bytes().await {
        if let Ok(_announce_response) = serde_bencode::from_bytes::<Announce>(&body) {
            Ok(())
        } else {
            debug!("announce body {:#?}", body);
            Err(CheckError::HttpError {
                url: tracker_url.clone(),
            })
        }
    } else {
        Err(CheckError::HttpError {
            url: tracker_url.clone(),
        })
    }
}

async fn check_http_scrape(url: &Url) -> Result<(), CheckError> {
    let info_hashes: Vec<String> = vec!["9c38422213e30bff212b30c360d26f9a02136422".to_string()]; // # DevSkim: ignore DS173237
    let query = requests::scrape::Query::try_from(info_hashes).expect("a valid array of info-hashes is required");

    // todo: HTTP request could panic.For example, if the server is not accessible.
    // We should change the client to catch that error and return a `CheckError`.
    // Otherwise the checking process will stop. The idea is to process all checks
    // and return a final report.

    let Ok(client) = Client::new(url.clone()) else {
        return Err(CheckError::HttpError { url: (url.to_owned()) });
    };
    let Ok(response) = client.scrape(&query).await else {
        return Err(CheckError::HttpError { url: (url.to_owned()) });
    };

    if let Ok(body) = response.bytes().await {
        if let Ok(_scrape_response) = scrape::Response::try_from_bencoded(&body) {
            Ok(())
        } else {
            debug!("scrape body {:#?}", body);
            Err(CheckError::HttpError { url: url.clone() })
        }
    } else {
        Err(CheckError::HttpError { url: url.clone() })
    }
}
