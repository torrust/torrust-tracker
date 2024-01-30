use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use colored::Colorize;
use reqwest::{Client as HttpClient, Url};

use super::config::Configuration;
use super::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::shared::bit_torrent::info_hash::InfoHash;
use crate::shared::bit_torrent::tracker::http::client::requests::announce::QueryBuilder;
use crate::shared::bit_torrent::tracker::http::client::responses::announce::Announce;
use crate::shared::bit_torrent::tracker::http::client::Client;

pub struct Service {
    pub(crate) config: Arc<Configuration>,
    pub(crate) console: Console,
}

pub type CheckResult = Result<(), CheckError>;

#[derive(Debug)]
pub enum CheckError {
    UdpError,
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

        self.check_udp_trackers();

        self.check_http_trackers(&mut check_results).await;

        self.run_health_checks(&mut check_results).await;

        check_results
    }

    fn check_udp_trackers(&self) {
        self.console.println("UDP trackers ...");

        for udp_tracker in &self.config.udp_trackers {
            self.check_udp_tracker(udp_tracker);
        }
    }

    async fn check_http_trackers(&self, check_results: &mut Vec<CheckResult>) {
        self.console.println("HTTP trackers ...");

        for http_tracker in &self.config.http_trackers {
            match self.check_http_tracker(http_tracker).await {
                Ok(()) => check_results.push(Ok(())),
                Err(err) => check_results.push(Err(err)),
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

    fn check_udp_tracker(&self, address: &SocketAddr) {
        // todo:
        // - Make announce request
        // - Make scrape request
        self.console
            .println(&format!("{} - UDP tracker at udp://{:?} is OK (TODO)", "✓".green(), address));
    }

    async fn check_http_tracker(&self, url: &Url) -> Result<(), CheckError> {
        let info_hash_str = "9c38422213e30bff212b30c360d26f9a02136422".to_string(); // # DevSkim: ignore DS173237
        let info_hash = InfoHash::from_str(&info_hash_str).expect("a valid info-hash is required");

        // Announce request

        let response = Client::new(url.clone())
            .announce(&QueryBuilder::with_default_values().with_info_hash(&info_hash).query())
            .await;

        if let Ok(body) = response.bytes().await {
            if let Ok(_announce_response) = serde_bencode::from_bytes::<Announce>(&body) {
                self.console.println(&format!("{} - Announce at {} is OK", "✓".green(), url));

                Ok(())
            } else {
                self.console.println(&format!("{} - Announce at {} failing", "✗".red(), url));
                Err(CheckError::HttpError { url: url.clone() })
            }
        } else {
            self.console.println(&format!("{} - Announce at {} failing", "✗".red(), url));
            Err(CheckError::HttpError { url: url.clone() })
        }

        // Scrape request

        // todo
    }

    async fn run_health_check(&self, url: Url) -> Result<(), CheckError> {
        let client = HttpClient::builder().timeout(Duration::from_secs(5)).build().unwrap();

        match client.get(url.clone()).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    self.console
                        .println(&format!("{} - Health API at {} is OK", "✓".green(), url));
                    Ok(())
                } else {
                    self.console
                        .eprintln(&format!("{} - Health API at {} failing: {:?}", "✗".red(), url, response));
                    Err(CheckError::HealthCheckError { url })
                }
            }
            Err(err) => {
                self.console
                    .eprintln(&format!("{} - Health API at {} failing: {:?}", "✗".red(), url, err));
                Err(CheckError::HealthCheckError { url })
            }
        }
    }
}
