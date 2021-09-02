use std::net::SocketAddr;
use crate::common::*;
use std::time::SystemTime;

pub fn get_connection_id(remote_address: &SocketAddr) -> ConnectionId {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => ConnectionId(((duration.as_secs() / 3600) | ((remote_address.port() as u64) << 36)) as i64),
        Err(_) => ConnectionId(0x7FFFFFFFFFFFFFFF),
    }
}

pub fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH).unwrap()
        .as_secs()
}
