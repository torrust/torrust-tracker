use std::time::Duration;

use reqwest::{Client as HttpClient, Url, Url as ServiceUrl};

use crate::console::clients::checker::service::{CheckError, CheckResult};

use super::structs::{CheckerOutput, Status};

#[allow(clippy::missing_panics_doc)]
pub async fn run(health_checks: &Vec<ServiceUrl>, check_results: &mut Vec<CheckResult>) -> Vec<CheckerOutput> {
    let mut health_checkers: Vec<CheckerOutput> = Vec::new();

    for health_check_url in health_checks {
        let mut health_checker = CheckerOutput {
            url: health_check_url.to_string(),
            status: Status {
                code: String::new(),
                message: String::new(),
            },
        };
        match run_health_check(health_check_url.clone()).await {
            Ok(()) => {
                check_results.push(Ok(()));
                health_checker.status.code = "ok".to_string();
            }
            Err(err) => {
                check_results.push(Err(err));
                health_checker.status.code = "error".to_string();
                health_checker.status.message = "Health API is failing.".to_string();
            }
        }
        health_checkers.push(health_checker);
    }
    health_checkers
}

async fn run_health_check(url: Url) -> Result<(), CheckError> {
    let client = HttpClient::builder().timeout(Duration::from_secs(5)).build().unwrap();

    match client.get(url.clone()).send().await {
        Ok(response) => {
            if response.status().is_success() {
                Ok(())
            } else {
                Err(CheckError::HealthCheckError { url })
            }
        }
        Err(_) => Err(CheckError::HealthCheckError { url }),
    }
}
