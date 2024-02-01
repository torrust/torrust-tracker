use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use colored::Colorize;
use log::debug;
use reqwest::{Client as HttpClient, Url};

use super::checks;
use super::config::Configuration;
use super::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::shared::bit_torrent::info_hash::InfoHash;
use crate::shared::bit_torrent::tracker::http::client::requests::announce::QueryBuilder;
use crate::shared::bit_torrent::tracker::http::client::responses::announce::Announce;
use crate::shared::bit_torrent::tracker::http::client::responses::scrape;
use crate::shared::bit_torrent::tracker::http::client::{requests, Client};

pub struct Service {
    pub(crate) config: Arc<Configuration>,
    pub(crate) console: Console,
}

pub type CheckResult = Result<(), CheckError>;

#[derive(Debug)]
pub enum CheckError {
    UdpError { socket_addr: SocketAddr },
    HttpError { url: Url },
    HealthCheckError { url: Url },
}

impl Service {
    /// # Errors
    ///
    /// Will return OK is all checks pass or an array with the check errors.
    pub async fn run_checks(&self) -> Vec<CheckResult> {
        self.console.println("Running checks for trackers ...");

        let mut check_results = vec![];

        checks::udp::run(&self.config.udp_trackers, &self.console, &mut check_results).await;

        self.check_http_trackers(&mut check_results).await;

        self.run_health_checks(&mut check_results).await;

        check_results
    }

    async fn check_http_trackers(&self, check_results: &mut Vec<CheckResult>) {
        self.console.println("HTTP trackers ...");

        for http_tracker in &self.config.http_trackers {
            let colored_tracker_url = http_tracker.to_string().yellow();

            match self.check_http_announce(http_tracker).await {
                Ok(()) => {
                    check_results.push(Ok(()));
                    self.console
                        .println(&format!("{} - Announce at {} is OK", "✓".green(), colored_tracker_url));
                }
                Err(err) => {
                    check_results.push(Err(err));
                    self.console
                        .println(&format!("{} - Announce at {} is failing", "✗".red(), colored_tracker_url));
                }
            }

            match self.check_http_scrape(http_tracker).await {
                Ok(()) => {
                    check_results.push(Ok(()));
                    self.console
                        .println(&format!("{} - Scrape at {} is OK", "✓".green(), colored_tracker_url));
                }
                Err(err) => {
                    check_results.push(Err(err));
                    self.console
                        .println(&format!("{} - Scrape at {} is failing", "✗".red(), colored_tracker_url));
                }
            }
        }
    }

    async fn run_health_checks(&self, check_results: &mut Vec<CheckResult>) {
        self.console.println("Health checks ...");

        for health_check_url in &self.config.health_checks {
            match self.run_health_check(health_check_url.clone()).await {
                Ok(()) => check_results.push(Ok(())),
                Err(err) => check_results.push(Err(err)),
            }
        }
    }

    async fn check_http_announce(&self, tracker_url: &Url) -> Result<(), CheckError> {
        let info_hash_str = "9c38422213e30bff212b30c360d26f9a02136422".to_string(); // # DevSkim: ignore DS173237
        let info_hash = InfoHash::from_str(&info_hash_str).expect("a valid info-hash is required");

        // todo: HTTP request could panic.For example, if the server is not accessible.
        // We should change the client to catch that error and return a `CheckError`.
        // Otherwise the checking process will stop. The idea is to process all checks
        // and return a final report.
        let response = Client::new(tracker_url.clone())
            .announce(&QueryBuilder::with_default_values().with_info_hash(&info_hash).query())
            .await;

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

    async fn check_http_scrape(&self, url: &Url) -> Result<(), CheckError> {
        let info_hashes: Vec<String> = vec!["9c38422213e30bff212b30c360d26f9a02136422".to_string()]; // # DevSkim: ignore DS173237
        let query = requests::scrape::Query::try_from(info_hashes).expect("a valid array of info-hashes is required");

        // todo: HTTP request could panic.For example, if the server is not accessible.
        // We should change the client to catch that error and return a `CheckError`.
        // Otherwise the checking process will stop. The idea is to process all checks
        // and return a final report.
        let response = Client::new(url.clone()).scrape(&query).await;

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

    async fn run_health_check(&self, url: Url) -> Result<(), CheckError> {
        let client = HttpClient::builder().timeout(Duration::from_secs(5)).build().unwrap();

        let colored_url = url.to_string().yellow();

        match client.get(url.clone()).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    self.console
                        .println(&format!("{} - Health API at {} is OK", "✓".green(), colored_url));
                    Ok(())
                } else {
                    self.console.eprintln(&format!(
                        "{} - Health API at {} is failing: {:?}",
                        "✗".red(),
                        colored_url,
                        response
                    ));
                    Err(CheckError::HealthCheckError { url })
                }
            }
            Err(err) => {
                self.console.eprintln(&format!(
                    "{} - Health API at {} is failing: {:?}",
                    "✗".red(),
                    colored_url,
                    err
                ));
                Err(CheckError::HealthCheckError { url })
            }
        }
    }
}
