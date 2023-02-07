//! Tracker HTTP/HTTPS Protocol:
//!
//! Original specification in BEP 3 (section "Trackers"):
//!
//! <https://www.bittorrent.org/beps/bep_0003.html>
//!
//! Other resources:
//!
//! - <https://wiki.theory.org/BitTorrentSpecification#Tracker_HTTP.2FHTTPS_Protocol>
//! - <https://wiki.theory.org/BitTorrent_Tracker_Protocol>
//!

use serde::{Deserialize, Serialize};
pub mod axum;
pub mod error;
pub mod filters;
pub mod handlers;
pub mod percent_encoding;
pub mod request;
pub mod response;
pub mod routes;
pub mod server;

pub type Bytes = u64;
pub type WebResult<T> = std::result::Result<T, warp::Rejection>;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Version {
    Warp,
    Axum,
}
