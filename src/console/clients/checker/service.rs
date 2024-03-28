use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use reqwest::Url;
use thiserror::Error;
use tokio::task::JoinSet;

use super::checks;
use super::config::Configuration;
use super::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::console::clients::http;

pub struct Service {
    pub(crate) config: Arc<Configuration>,
    pub(crate) console: Console,
}

pub type CheckResult = Result<(), CheckError>;

#[derive(Debug, Clone, Error)]
pub enum CheckError {
    #[error("Error In Udp: socket: {socket_addr:?}")]
    UdpError { socket_addr: SocketAddr },
    #[error("Error In Http: url: {url:?}")]
    HttpCheckError { url: Url, err: http::Error },
    #[error("Error In HeathCheck: url: {url:?}")]
    HealthCheckError { url: Url, err: checks::health::Error },
}

impl Service {
    /// # Errors
    ///
    /// It will return an error if some of the tests panic or otherwise fail to run.
    /// On success it will return a vector of `Ok(())` of [`CheckResult`].
    pub async fn run_checks(self) -> Result<Vec<CheckResult>> {
        self.console.println("Running checks for trackers ...");

        let mut check_results = Vec::<CheckResult>::default();

        let timeout = self.config.client_timeout;

        let mut checks = JoinSet::new();
        checks.spawn(checks::udp::run(self.config.udp_trackers.clone(), timeout, self.console));
        checks.spawn(checks::http::run(self.config.http_trackers.clone(), timeout, self.console));
        checks.spawn(checks::health::run(self.config.health_checks.clone(), timeout, self.console));

        while let Some(results) = checks.join_next().await {
            check_results.append(&mut results.context("failed to join check")?);
        }

        Ok(check_results)
    }
}
