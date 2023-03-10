pub mod context;
pub mod middlewares;
pub mod responses;
pub mod routes;
pub mod server;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct InfoHashParam(pub String);
