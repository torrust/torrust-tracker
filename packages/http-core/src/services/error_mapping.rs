use torrust_tracker_core::error::TrackerCoreError;
use torrust_tracker_http_protocol::v1::responses::error::Error as HttpProtocolErrorResponse;

pub fn protocol_error_from_tracker_core_error(error: TrackerCoreError) -> HttpProtocolErrorResponse {
    match error {
        TrackerCoreError::AnnounceError { source } => HttpProtocolErrorResponse {
            failure_reason: format!("Tracker announce error: {source}"),
        },
        TrackerCoreError::ScrapeError { source } => HttpProtocolErrorResponse {
            failure_reason: format!("Tracker scrape error: {source}"),
        },
        TrackerCoreError::WhitelistError { source } => HttpProtocolErrorResponse {
            failure_reason: format!("Tracker whitelist error: {source}"),
        },
        TrackerCoreError::AuthenticationError { source } => HttpProtocolErrorResponse {
            failure_reason: format!("Tracker authentication error: {source}"),
        },
    }
}
