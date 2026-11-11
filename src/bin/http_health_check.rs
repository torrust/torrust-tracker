//! Minimal `curl` or `wget` to be used for container health checks.
//!
//! It's convenient to avoid using third-party libraries because:
//!
//! - They are harder to maintain.
//! - They introduce new attack vectors.
//!
//! Output follows the global CLI output contract (`no-stdout-result`): nothing on stdout, one JSON
//! object per line on stderr, exit code `0` when the endpoint answers with a 2xx status, `1` when it
//! answers with another status or cannot be reached, and `2` on a usage error. Unhealthy must stay a
//! non-zero exit because the container `HEALTHCHECK` relies on it.
use std::env;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

use reqwest::Client;
use serde_json::{Value, json};

const EXIT_HEALTHY: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const EXIT_USAGE_ERROR: u8 = 2;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> ExitCode {
    let (record, exit_code) = run(env::args().collect()).await;

    if writeln!(io::stderr(), "{record}").is_err() {
        // stderr is closed or broken: nothing can be reported, the exit code still carries the outcome.
        return ExitCode::from(exit_code.max(EXIT_FAILURE));
    }

    ExitCode::from(exit_code)
}

/// Probes the URL given as the only argument and returns the record to emit with the exit code.
async fn run(args: Vec<String>) -> (Value, u8) {
    let [_, url] = args.as_slice() else {
        return (
            json!({
                "kind": "usage_error",
                "message": "usage error: expected exactly one argument; use `http_health_check <HEALTH_URL>`, for example `http_health_check http://127.0.0.1:1313/health_check`",
                "exit_code": EXIT_USAGE_ERROR,
            }),
            EXIT_USAGE_ERROR,
        );
    };

    let client = match Client::builder().timeout(REQUEST_TIMEOUT).build() {
        Ok(client) => client,
        Err(err) => return unreachable_record(url, &err),
    };

    match client.get(url).send().await {
        Ok(response) if response.status().is_success() => (
            json!({
                "kind": "healthy",
                "message": format!("`{url}` answered {}", response.status()),
                "url": url,
                "status": response.status().as_u16(),
                "exit_code": EXIT_HEALTHY,
            }),
            EXIT_HEALTHY,
        ),
        Ok(response) => (
            json!({
                "kind": "unhealthy",
                "message": format!("`{url}` answered the non-success status {}", response.status()),
                "url": url,
                "status": response.status().as_u16(),
                "exit_code": EXIT_FAILURE,
            }),
            EXIT_FAILURE,
        ),
        Err(err) => unreachable_record(url, &err),
    }
}

fn unreachable_record(url: &str, err: &reqwest::Error) -> (Value, u8) {
    (
        json!({
            "kind": "unreachable",
            "message": format!("could not probe `{url}`: {err}"),
            "url": url,
            "exit_code": EXIT_FAILURE,
        }),
        EXIT_FAILURE,
    )
}
