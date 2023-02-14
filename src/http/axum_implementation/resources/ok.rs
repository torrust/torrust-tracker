use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Ok {
    pub remote_client_insecure_ip: IpAddr,
    pub remote_client_secure_ip: IpAddr,
}
