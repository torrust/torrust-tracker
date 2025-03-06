//! `Error` response for the HTTP tracker.
//!
//! Data structures and logic to build the error responses.
//!
//! From the [BEP 03. The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html):
//!
//! _"Tracker responses are bencoded dictionaries. If a tracker response has a
//! key failure reason, then that maps to a human readable string which explains
//! why the query failed, and no other keys are required."_
//!
//! > **NOTICE**: error responses are bencoded and always have a `200 OK` status
//! > code. The official `BitTorrent` specification does not specify the status
//! > code.
use serde::Serialize;

use crate::v1::auth;
use crate::v1::services::peer_ip_resolver::PeerIpResolutionError;

/// `Error` response for the HTTP tracker.
#[derive(Serialize, Debug, PartialEq)]
pub struct Error {
    /// Human readable string which explains why the request failed.
    #[serde(rename = "failure reason")]
    pub failure_reason: String,
}

impl Error {
    /// Returns the bencoded representation of the `Error` struct.
    ///
    /// ```rust
    /// use bittorrent_http_tracker_protocol::v1::responses::error::Error;
    ///
    /// let err = Error {
    ///    failure_reason: "error message".to_owned(),
    /// };
    ///
    /// // cspell:disable-next-line
    /// assert_eq!(err.write(), "d14:failure reason13:error messagee");
    /// ```
    ///
    /// # Panics
    ///
    /// It would panic if the `Error` struct contained an inappropriate field
    /// type.
    #[must_use]
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }
}

impl From<auth::Error> for Error {
    fn from(err: auth::Error) -> Self {
        Self {
            failure_reason: format!("Tracker authentication error: {err}"),
        }
    }
}

impl From<PeerIpResolutionError> for Error {
    fn from(err: PeerIpResolutionError) -> Self {
        Self {
            failure_reason: format!("Error resolving peer IP: {err}"),
        }
    }
}

impl From<bittorrent_tracker_core::error::AnnounceError> for Error {
    fn from(err: bittorrent_tracker_core::error::AnnounceError) -> Self {
        Error {
            failure_reason: format!("Tracker announce error: {err}"),
        }
    }
}

impl From<bittorrent_tracker_core::error::ScrapeError> for Error {
    fn from(err: bittorrent_tracker_core::error::ScrapeError) -> Self {
        Error {
            failure_reason: format!("Tracker scrape error: {err}"),
        }
    }
}

impl From<bittorrent_tracker_core::error::WhitelistError> for Error {
    fn from(err: bittorrent_tracker_core::error::WhitelistError) -> Self {
        Error {
            failure_reason: format!("Tracker whitelist error: {err}"),
        }
    }
}

impl From<bittorrent_tracker_core::authentication::Error> for Error {
    fn from(err: bittorrent_tracker_core::authentication::Error) -> Self {
        Error {
            failure_reason: format!("Tracker authentication error: {err}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic::Location;

    use super::Error;
    use crate::v1::responses;
    use crate::v1::services::peer_ip_resolver::PeerIpResolutionError;

    #[test]
    fn http_tracker_errors_can_be_bencoded() {
        let err = Error {
            failure_reason: "error message".to_owned(),
        };

        assert_eq!(err.write(), "d14:failure reason13:error messagee"); // cspell:disable-line
    }

    fn assert_error_response(error: &responses::error::Error, error_message: &str) {
        assert!(
            error.failure_reason.contains(error_message),
            "Error response does not contain message: '{error_message}'. Error: {error:?}"
        );
    }

    #[test]
    fn it_should_map_a_peer_ip_resolution_error_into_an_error_response() {
        let response = responses::error::Error::from(PeerIpResolutionError::MissingRightMostXForwardedForIp {
            location: Location::caller(),
        });

        assert_error_response(&response, "Error resolving peer IP");
    }
}
