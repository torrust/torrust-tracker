use std::net::SocketAddr;

pub mod cookie;
pub mod error;
pub mod handlers;

use derive_more::Display;

#[derive(Display, Debug)]
#[display(fmt = "from (target): {from}")]
pub(crate) struct UdpRequest {
    pub payload: Vec<u8>,
    pub from: SocketAddr,
}
