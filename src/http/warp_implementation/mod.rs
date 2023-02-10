use warp::Rejection;

pub mod error;
pub mod filters;
pub mod handlers;
pub mod peer_builder;
pub mod request;
pub mod response;
pub mod routes;
pub mod server;

pub type Bytes = u64;
pub type WebResult<T> = std::result::Result<T, Rejection>;
