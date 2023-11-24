use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use torrust_tracker_configuration::Configuration;

use super::resources::Report;
use super::responses;

/// Endpoint for container health check.
pub(crate) async fn health_check_handler(State(config): State<Arc<Configuration>>) -> Json<Report> {
    if config.http_api.enabled {
        let health_check_url = format!("http://{}/health_check", config.http_api.bind_address);
        if !get_req_is_ok(&health_check_url).await {
            return responses::error(format!("API is not healthy. Health check endpoint: {health_check_url}"));
        }
    }

    // todo: for all HTTP Trackers, if enabled, check if is healthy

    // todo: for all UDP  Trackers, if enabled, check if is healthy

    responses::ok()
}

async fn get_req_is_ok(url: &str) -> bool {
    match reqwest::get(url).await {
        Ok(response) => response.status().is_success(),
        Err(_err) => false,
    }
}
