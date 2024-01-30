use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use colored::Colorize;
use reqwest::{Client, Url};

use super::config::Configuration;
use super::console::Console;
use crate::console::clients::checker::printer::Printer;

pub struct Service {
    pub(crate) config: Arc<Configuration>,
    pub(crate) console: Console,
}

pub type CheckResult = Result<(), CheckError>;

#[derive(Debug)]
pub enum CheckError {
    UdpError,
    HttpError,
    HealthCheckError { url: Url },
}

impl Service {
    /// # Errors
    ///
    /// Will return OK is all checks pass or an array with the check errors.
    pub async fn run_checks(&self) -> Vec<CheckResult> {
        self.console.println("Running checks for trackers ...");

        self.check_udp_trackers();
        self.check_http_trackers();

        self.run_health_checks().await
    }

    fn check_udp_trackers(&self) {
        self.console.println("UDP trackers ...");

        for udp_tracker in &self.config.udp_trackers {
            self.check_udp_tracker(udp_tracker);
        }
    }

    fn check_http_trackers(&self) {
        self.console.println("HTTP trackers ...");

        for http_tracker in &self.config.http_trackers {
            self.check_http_tracker(http_tracker);
        }
    }

    async fn run_health_checks(&self) -> Vec<CheckResult> {
        self.console.println("Health checks ...");

        let mut check_results = vec![];

        for health_check_url in &self.config.health_checks {
            match self.run_health_check(health_check_url.clone()).await {
                Ok(()) => check_results.push(Ok(())),
                Err(err) => check_results.push(Err(err)),
            }
        }

        check_results
    }

    fn check_udp_tracker(&self, address: &SocketAddr) {
        // todo:
        // - Make announce request
        // - Make scrape request
        self.console
            .println(&format!("{} - UDP tracker at {:?} is OK (TODO)", "✓".green(), address));
    }

    fn check_http_tracker(&self, url: &Url) {
        // todo:
        // - Make announce request
        // - Make scrape request
        self.console
            .println(&format!("{} - HTTP tracker at {} is OK (TODO)", "✓".green(), url));
    }

    async fn run_health_check(&self, url: Url) -> Result<(), CheckError> {
        let client = Client::builder().timeout(Duration::from_secs(5)).build().unwrap();

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
