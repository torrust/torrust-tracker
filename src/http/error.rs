use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum Error {
    #[error("internal server error")]
    InternalServer,

    #[error("info_hash is either missing or invalid")]
    InvalidInfo,

    #[error("peer_id is either missing or invalid")]
    InvalidPeerId,

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

impl Reject for Error {}
