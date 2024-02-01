use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use colored::Colorize;
use reqwest::{Client as HttpClient, Url};

use super::checks;
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

        checks::http::run(&self.config.http_trackers, &self.console, &mut check_results).await;

        self.run_health_checks(&mut check_results).await;

        check_results
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
