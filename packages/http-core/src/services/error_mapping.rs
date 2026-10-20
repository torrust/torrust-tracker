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
        unknown => HttpProtocolErrorResponse {
            failure_reason: format!("Tracker error: {unknown}"),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::panic::Location;

    use rstest::rstest;
    use torrust_tracker_core::authentication::key::Error as AuthenticationKeyError;
    use torrust_tracker_core::error::{AnnounceError, ScrapeError, TrackerCoreError, WhitelistError};

    use super::protocol_error_from_tracker_core_error;
    use crate::tests::sample_info_hash;

    // Each helper returns an equal error on every call, because `Location::caller()`
    // names the call site inside the helper. A test builds the error under test and
    // the text it expects from two separate calls.
    fn not_whitelisted() -> WhitelistError {
        WhitelistError::TorrentNotWhitelisted {
            info_hash: sample_info_hash(),
            location: Location::caller(),
        }
    }

    fn missing_key() -> AuthenticationKeyError {
        AuthenticationKeyError::MissingAuthKey {
            location: Location::caller(),
        }
    }

    #[rstest]
    #[case::announce(
        TrackerCoreError::from(AnnounceError::from(not_whitelisted())),
        format!("Tracker announce error: {}", AnnounceError::from(not_whitelisted()))
    )]
    #[case::scrape(
        TrackerCoreError::from(ScrapeError::from(not_whitelisted())),
        format!("Tracker scrape error: {}", ScrapeError::from(not_whitelisted()))
    )]
    #[case::whitelist(
        TrackerCoreError::from(not_whitelisted()),
        format!("Tracker whitelist error: {}", not_whitelisted())
    )]
    #[case::authentication(
        TrackerCoreError::from(missing_key()),
        format!("Tracker authentication error: {}", missing_key())
    )]
    fn it_should_report_each_tracker_core_error_with_the_prefix_of_its_own_area(
        #[case] error: TrackerCoreError,
        #[case] expected_failure_reason: String,
    ) {
        // Act
        let response = protocol_error_from_tracker_core_error(error);

        // Assert
        assert_eq!(response.failure_reason, expected_failure_reason);
    }
}
