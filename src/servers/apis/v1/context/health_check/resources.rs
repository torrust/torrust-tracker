//! API resources for the [`stats`](crate::servers::apis::v1::context::health_check)
//! API context.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Error,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Report {
    pub status: Status,
}
