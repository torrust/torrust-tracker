//! UDP request types.
//!
//! Torrust Tracker uses the [`aquatic_udp_protocol`](https://crates.io/crates/aquatic_udp_protocol)
//! crate to parse and serialize UDP requests.
//!
//! Some of the type in this module are wrappers around the types in the
//! `aquatic_udp_protocol` crate.
use aquatic_udp_protocol::AnnounceRequest;

use crate::shared::bit_torrent::info_hash::InfoHash;

/// Wrapper around [`AnnounceRequest`](aquatic_udp_protocol::request::AnnounceRequest).
pub struct AnnounceWrapper {
    /// [`AnnounceRequest`](aquatic_udp_protocol::request::AnnounceRequest) to wrap.
    pub announce_request: AnnounceRequest,
    /// Info hash of the torrent.
    pub info_hash: InfoHash,
}

impl AnnounceWrapper {
    /// Creates a new [`AnnounceWrapper`] from an [`AnnounceRequest`].
    #[must_use]
    pub fn new(announce_request: &AnnounceRequest) -> Self {
        AnnounceWrapper {
            announce_request: announce_request.clone(),
            info_hash: InfoHash(announce_request.info_hash.0),
        }
    }
}
