use std::net::SocketAddr;
use std::sync::Arc;

use reqwest::Url;

use super::checks::{self};
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
    #[allow(clippy::missing_panics_doc)]
    pub async fn run_checks(&self) -> Vec<CheckResult> {
        let mut check_results = vec![];

        let udp_checkers = checks::udp::run(&self.config.udp_trackers, &mut check_results).await;

        let http_checkers = checks::http::run(&self.config.http_trackers, &mut check_results).await;

        let health_checkers = checks::health::run(&self.config.health_checks, &mut check_results).await;

        let json_output =
            serde_json::json!({ "udp_trackers": udp_checkers, "http_trackers": http_checkers, "health_checks": health_checkers });
        self.console.println(&serde_json::to_string_pretty(&json_output).unwrap());

        check_results
    }
}
