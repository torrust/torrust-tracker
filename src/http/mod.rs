//! Tracker HTTP/HTTPS Protocol:
//!
//! <https://wiki.theory.org/BitTorrentSpecification#Tracker_HTTP.2FHTTPS_Protocol>
//! <https://wiki.theory.org/BitTorrent_Tracker_Protocol>
//!
pub mod error;
pub mod filters;
pub mod handlers;
pub mod request;
pub mod response;
pub mod routes;
pub mod server;

pub type Bytes = u64;
pub type WebResult<T> = std::result::Result<T, warp::Rejection>;
