use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use torrust_tracker_configuration::Configuration;

use super::resources::Report;
use super::responses;

/// If port 0 is specified in the configuration the OS will automatically
/// assign a free port. But we do now know in from the configuration.
/// We can only know it after starting the socket.
const UNKNOWN_PORT: u16 = 0;

/// Endpoint for container health check.
///
/// This endpoint only checks services when we know the port from the
/// configuration. If port 0 is specified in the configuration the health check
/// for that service is skipped.
pub(crate) async fn health_check_handler(State(config): State<Arc<Configuration>>) -> Json<Report> {
    // todo: when port 0 is specified in the configuration get the port from the
    // running service, after starting it as we do for testing with ephemeral
    // configurations.

    if config.http_api.enabled {
        let addr: SocketAddr = config.http_api.bind_address.parse().expect("invalid socket address for API");

        if addr.port() != UNKNOWN_PORT {
            let health_check_url = format!("http://{addr}/health_check");

            if !get_req_is_ok(&health_check_url).await {
                return responses::error(format!("API is not healthy. Health check endpoint: {health_check_url}"));
            }
        }
    }

    for http_tracker_config in &config.http_trackers {
        if !http_tracker_config.enabled {
            continue;
        }

        let addr: SocketAddr = http_tracker_config
            .bind_address
            .parse()
            .expect("invalid socket address for HTTP tracker");

        if addr.port() != UNKNOWN_PORT {
            let health_check_url = format!("http://{addr}/health_check");

            if !get_req_is_ok(&health_check_url).await {
                return responses::error(format!(
                    "HTTP Tracker is not healthy. Health check endpoint: {health_check_url}"
                ));
            }
        }
    }

    // todo: for all UDP  Trackers, if enabled, check if is healthy

    responses::ok()
}

async fn get_req_is_ok(url: &str) -> bool {
    match reqwest::get(url).await {
        Ok(response) => response.status().is_success(),
        Err(_err) => false,
    }
}
