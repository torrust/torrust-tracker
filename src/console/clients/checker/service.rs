use std::net::SocketAddr;
use std::sync::Arc;

use reqwest::Url;

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

        checks::health::run(&self.config.health_checks, &self.console, &mut check_results).await;

        check_results
    }
}
