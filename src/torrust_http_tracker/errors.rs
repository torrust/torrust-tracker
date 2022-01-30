use warp::reject::Reject;
use thiserror::Error;
use crate::TorrentError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("internal server error")]
    InternalServerError,

    #[error("info_hash is either missing or invalid")]
    InvalidInfoHash,

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
}

impl Reject for ServerError {}

impl From<TorrentError> for ServerError {
    fn from(e: TorrentError) -> Self {
        match e {
            TorrentError::TorrentNotWhitelisted => ServerError::TorrentNotWhitelisted,
            TorrentError::PeerNotAuthenticated => ServerError::PeerNotAuthenticated,
            TorrentError::PeerKeyNotValid => ServerError::PeerKeyNotValid,
            TorrentError::NoPeersFound => ServerError::NoPeersFound,
            TorrentError::CouldNotSendResponse => ServerError::InternalServerError,
            TorrentError::InvalidInfoHash => ServerError::InvalidInfoHash,
        }
    }
}
