use std::net::SocketAddr;
use std::sync::Arc;

use aquatic_udp_protocol::{ConnectRequest, Response, TransactionId};
use axum::extract::State;
use axum::Json;
use torrust_tracker_configuration::Configuration;

use super::resources::Report;
use super::responses;
use crate::shared::bit_torrent::udp::client::new_udp_tracker_client_connected;

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

    // Health check for API

    if config.http_api.enabled {
        let addr: SocketAddr = config.http_api.bind_address.parse().expect("invalid socket address for API");

        if addr.port() != UNKNOWN_PORT {
            let health_check_url = format!("http://{addr}/health_check");

            if !get_req_is_ok(&health_check_url).await {
                return responses::error(format!("API is not healthy. Health check endpoint: {health_check_url}"));
            }
        }
    }

    // Health check for HTTP Trackers

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

    // Health check for UDP Trackers

    for udp_tracker_config in &config.udp_trackers {
        if !udp_tracker_config.enabled {
            continue;
        }

        let addr: SocketAddr = udp_tracker_config
            .bind_address
            .parse()
            .expect("invalid socket address for UDP tracker");

        if addr.port() != UNKNOWN_PORT && !can_connect_to_udp_tracker(&addr.to_string()).await {
            return responses::error(format!("UDP Tracker is not healthy. Can't connect to: {addr}"));
        }
    }

    responses::ok()
}

async fn get_req_is_ok(url: &str) -> bool {
    match reqwest::get(url).await {
        Ok(response) => response.status().is_success(),
        Err(_err) => false,
    }
}

/// Tries to connect to an UDP tracker. It returns true if it succeeded.
async fn can_connect_to_udp_tracker(url: &str) -> bool {
    let client = new_udp_tracker_client_connected(url).await;

    let connect_request = ConnectRequest {
        transaction_id: TransactionId(123),
    };

    client.send(connect_request.into()).await;

    let response = client.receive().await;

    matches!(response, Response::Connect(_connect_response))
}
