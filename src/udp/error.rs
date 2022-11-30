use thiserror::Error;

use crate::tracker::torrent;

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

impl From<torrent::Error> for Error {
    fn from(e: torrent::Error) -> Self {
        match e {
            torrent::Error::TorrentNotWhitelisted => Error::TorrentNotWhitelisted,
            torrent::Error::PeerNotAuthenticated => Error::PeerNotAuthenticated,
            torrent::Error::PeerKeyNotValid => Error::PeerKeyNotValid,
            torrent::Error::NoPeersFound => Error::NoPeersFound,
            torrent::Error::CouldNotSendResponse => Error::InternalServer,
            torrent::Error::InvalidInfoHash => Error::InvalidInfoHash,
        }
    }
}
