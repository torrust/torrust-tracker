//! Statistics helpers — query aggregate metrics from the REST API.

use url::Url;

/// Global statistics with only metrics relevant to the test.
#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct PartialGlobalStatistics {
    pub tcp4_announces_handled: u64,
    pub udp4_announces_handled: u64,
    pub udp_banned_ips_total: u64,
    pub udp_requests_banned: u64,
    pub udp4_requests: u64,
    pub udp4_connections_handled: u64,
    pub udp4_responses: u64,
    pub udp4_errors_handled: u64,
}

#[allow(dead_code)]
pub async fn get_tracker_statistics(api_url: &Url, token: &str) -> PartialGlobalStatistics {
    use torrust_tracker_rest_api_client::connection_info::{ConnectionInfo, Origin};
    use torrust_tracker_rest_api_client::v1::client::ApiHttpClient as TrackerApiClient;

    let response = TrackerApiClient::new(ConnectionInfo::authenticated(Origin::new(api_url.as_str()).unwrap(), token))
        .unwrap()
        .get_tracker_statistics(None)
        .await
        .expect("failed to get tracker statistics");

    response
        .json::<PartialGlobalStatistics>()
        .await
        .expect("Failed to parse JSON response")
}
