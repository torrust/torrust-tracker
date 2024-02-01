use std::time::Duration;

use colored::Colorize;
use reqwest::{Client as HttpClient, Url, Url as ServiceUrl};

use crate::console::clients::checker::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::console::clients::checker::service::{CheckError, CheckResult};

pub async fn run(health_checks: &Vec<ServiceUrl>, console: &Console, check_results: &mut Vec<CheckResult>) {
    console.println("Health checks ...");

    for health_check_url in health_checks {
        match run_health_check(health_check_url.clone(), console).await {
            Ok(()) => check_results.push(Ok(())),
            Err(err) => check_results.push(Err(err)),
        }
    }
}

async fn run_health_check(url: Url, console: &Console) -> Result<(), CheckError> {
    let client = HttpClient::builder().timeout(Duration::from_secs(5)).build().unwrap();

    let colored_url = url.to_string().yellow();

    match client.get(url.clone()).send().await {
        Ok(response) => {
            if response.status().is_success() {
                console.println(&format!("{} - Health API at {} is OK", "✓".green(), colored_url));
                Ok(())
            } else {
                console.eprintln(&format!(
                    "{} - Health API at {} is failing: {:?}",
                    "✗".red(),
                    colored_url,
                    response
                ));
                Err(CheckError::HealthCheckError { url })
            }
        }
        Err(err) => {
            console.eprintln(&format!(
                "{} - Health API at {} is failing: {:?}",
                "✗".red(),
                colored_url,
                err
            ));
            Err(CheckError::HealthCheckError { url })
        }
    }
}
