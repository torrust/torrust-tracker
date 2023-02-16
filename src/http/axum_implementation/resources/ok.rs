use serde::{Deserialize, Serialize};

use crate::http::axum_implementation::extractors::remote_client_ip::RemoteClientIp;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Ok {
    pub remote_client_ip: RemoteClientIp,
}
