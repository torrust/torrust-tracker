use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use colored::Colorize;
use hyper::StatusCode;
use reqwest::{Client as HttpClient, Response};
use thiserror::Error;
use url::Url;

use crate::console::clients::checker::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::console::clients::checker::service::{CheckError, CheckResult};

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Failed to Build a Http Client: {err:?}")]
    ClientBuildingError { err: Arc<reqwest::Error> },
    #[error("Heath check failed to get a response: {err:?}")]
    ResponseError { err: Arc<reqwest::Error> },
    #[error("Http check returned a non-success code: \"{code}\" with the response: \"{response:?}\"")]
    UnsuccessfulResponse { code: StatusCode, response: Arc<Response> },
}

pub async fn run(health_checks: Vec<Url>, timeout: Duration, console: Console) -> Vec<CheckResult> {
    let mut check_results = Vec::default();

    console.println("Health checks ...");

    for url in health_checks {
        let colored_url = url.to_string().yellow();

        match run_health_check(url.clone(), timeout).await {
            Ok(response) => {
                console.println(&format!(
                    "{} - Health API at {} is {}",
                    "✓".green(),
                    colored_url,
                    response.status()
                ));

                check_results.push(Ok(()));
            }
            Err(err) => {
                console.eprintln(&format!("{} - Health API at {} is failing: {}", "✗".red(), colored_url, err));

                check_results.push(Err(CheckError::HealthCheckError { url, err }));
            }
        }
    }

    check_results
}

async fn run_health_check(url: Url, timeout: Duration) -> Result<Response, Error> {
    let client = HttpClient::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| Error::ClientBuildingError { err: e.into() })?;

    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(|e| Error::ResponseError { err: e.into() })?;

    if response.status().is_success() {
        Ok(response)
    } else {
        Err(Error::UnsuccessfulResponse {
            code: response.status(),
            response: response.into(),
        })
    }
}
