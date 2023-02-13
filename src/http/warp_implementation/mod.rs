pub mod error;
pub mod filter_helpers;
pub mod filters;
pub mod handlers;
pub mod peer_builder;
pub mod request;
pub mod response;
pub mod routes;
pub mod server;

use warp::Rejection;

pub type Bytes = u64;
pub type WebResult<T> = std::result::Result<T, Rejection>;
