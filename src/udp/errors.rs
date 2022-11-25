use thiserror::Error;

use crate::tracker::torrent;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("internal server error")]
    InternalServerError,

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

impl From<torrent::Error> for ServerError {
    fn from(e: torrent::Error) -> Self {
        match e {
            torrent::Error::TorrentNotWhitelisted => ServerError::TorrentNotWhitelisted,
            torrent::Error::PeerNotAuthenticated => ServerError::PeerNotAuthenticated,
            torrent::Error::PeerKeyNotValid => ServerError::PeerKeyNotValid,
            torrent::Error::NoPeersFound => ServerError::NoPeersFound,
            torrent::Error::CouldNotSendResponse => ServerError::InternalServerError,
            torrent::Error::InvalidInfoHash => ServerError::InvalidInfoHash,
        }
    }
}
