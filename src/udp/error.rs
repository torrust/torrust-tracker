use thiserror::Error;

use crate::tracker;

#[derive(Error, Debug)]
pub enum Error {
    #[error("internal server error")]
    InternalServer,

    #[error("info_hash is either missing or invalid")]
    InvalidInfoHash,

    #[error("connection id could not be verified")]
    InvalidConnectionId,

    #[error("could not find remote address")]
    AddressNotFound,

    #[error("torrent has no peers")]
    NoPeersFound,

    #[error("torrent not on whitelist")]
    TorrentNotWhitelisted,

    #[error("peer not authenticated")]
    PeerNotAuthenticated,

    #[error("invalid authentication key")]
    PeerKeyNotValid,

    #[error("exceeded info_hash limit")]
    ExceededInfoHashLimit,

    #[error("bad request")]
    BadRequest,
}

impl From<tracker::error::Error> for Error {
    fn from(e: tracker::error::Error) -> Self {
        match e {
            tracker::error::Error::TorrentNotWhitelisted {
                info_hash: _,
                location: _,
            } => Error::TorrentNotWhitelisted,
            tracker::error::Error::PeerNotAuthenticated { location: _ } => Error::PeerNotAuthenticated,
            tracker::error::Error::PeerKeyNotValid { key: _, source: _ } => Error::PeerKeyNotValid,
        }
    }
}
