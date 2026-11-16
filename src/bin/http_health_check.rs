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
//!
//! The records never contain the probed URL: it can carry credentials (userinfo, query parameters or
//! path segments), and the contract's redaction policy forbids exposing them.
use std::env;
use std::error::Error as _;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

use reqwest::{Client, Url};
use serde_json::{Value, json};

const EXIT_HEALTHY: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const EXIT_USAGE_ERROR: u8 = 2;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const USAGE: &str = "use `http_health_check <HEALTH_URL>`, for example `http_health_check http://127.0.0.1:1313/health_check`";

#[tokio::main]
async fn main() -> ExitCode {
    let (record, exit_code) = run(env::args_os().skip(1).collect()).await;

    // When the record cannot be written (a broken pipe), the exit code alone carries the outcome.
    drop(writeln!(io::stderr(), "{record}"));

    ExitCode::from(exit_code)
}

/// Probes the URL given as the only argument and returns the record to emit with the exit code.
async fn run(arguments: Vec<OsString>) -> (Value, u8) {
    let url = match parse_arguments(arguments) {
        Ok(url) => url,
        Err(message) => {
            return record(
                "usage_error",
                &format!("usage error: {message}; {USAGE}"),
                None,
                EXIT_USAGE_ERROR,
            );
        }
    };

    let client = match Client::builder().timeout(REQUEST_TIMEOUT).build() {
        Ok(client) => client,
        Err(err) => return unreachable(err),
    };

    match client.get(url).send().await {
        Ok(response) if response.status().is_success() => record(
            "healthy",
            &format!("the health endpoint answered {}", response.status()),
            Some(response.status().as_u16()),
            EXIT_HEALTHY,
        ),
        Ok(response) => record(
            "unhealthy",
            &format!("the health endpoint answered the non-success status {}", response.status()),
            Some(response.status().as_u16()),
            EXIT_FAILURE,
        ),
        Err(err) => unreachable(err),
    }
}

/// The single argument as a URL; the error message never repeats the argument, which may hold credentials.
fn parse_arguments(arguments: Vec<OsString>) -> Result<Url, &'static str> {
    let [argument] = <[OsString; 1]>::try_from(arguments).map_err(|_| "expected exactly one argument")?;
    let argument = argument.into_string().map_err(|_| "the argument is not valid UTF-8")?;
    Url::parse(&argument).map_err(|_| "the argument is not a valid URL")
}

/// The request failed; the URL is removed from the error and its causes before formatting.
fn unreachable(err: reqwest::Error) -> (Value, u8) {
    let err = err.without_url();
    let mut message = format!("could not probe the health endpoint: {err}");
    let mut source = err.source();
    while let Some(cause) = source {
        // Writing to a `String` cannot fail.
        let _ = write!(message, ": {cause}");
        source = cause.source();
    }
    record("unreachable", &message, None, EXIT_FAILURE)
}

fn record(kind: &str, message: &str, status: Option<u16>, exit_code: u8) -> (Value, u8) {
    let mut record = json!({ "kind": kind, "message": message });
    if let Some(status) = status {
        record["status"] = json!(status);
    }
    record["exit_code"] = json!(exit_code);
    (record, exit_code)
}
