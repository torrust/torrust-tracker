pub mod routes;
pub mod server;
pub mod v1;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct InfoHashParam(pub String);
