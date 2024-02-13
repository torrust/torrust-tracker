//! Error returned by the core `Tracker`.
//!
//! Error | Context | Description
//! ---|---|---
//! `PeerKeyNotValid` | Authentication | The supplied key is not valid. It may not be registered or expired.
//! `PeerNotAuthenticated` | Authentication | The peer did not provide the authentication key.
//! `TorrentNotWhitelisted` | Authorization | The action cannot be perform on a not-whitelisted torrent (it only applies for trackers running in `listed` or `private_listed` modes).
//!
use std::panic::Location;

use torrust_tracker_located_error::LocatedError;
use torrust_tracker_primitives::info_hash::InfoHash;

/// Authentication or authorization error returned by the core `Tracker`
#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    // Authentication errors
    #[error("The supplied key: {key:?}, is not valid: {source}")]
    PeerKeyNotValid {
        key: super::auth::Key,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
    #[error("The peer is not authenticated, {location}")]
    PeerNotAuthenticated { location: &'static Location<'static> },

    // Authorization errors
    #[error("The torrent: {info_hash}, is not whitelisted, {location}")]
    TorrentNotWhitelisted {
        info_hash: InfoHash,
        location: &'static Location<'static>,
    },
}
