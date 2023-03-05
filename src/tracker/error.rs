use std::panic::Location;

use crate::located_error::LocatedError;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("The supplied key: {key:?}, is not valid: {source}")]
    PeerKeyNotValid {
        key: super::auth::Key,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
    #[error("The peer is not authenticated, {location}")]
    PeerNotAuthenticated { location: &'static Location<'static> },

    #[error("The torrent: {info_hash}, is not whitelisted, {location}")]
    TorrentNotWhitelisted {
        info_hash: crate::protocol::info_hash::InfoHash,
        location: &'static Location<'static>,
    },
}
