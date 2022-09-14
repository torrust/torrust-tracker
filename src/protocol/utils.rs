use std::net::SocketAddr;

use aquatic_udp_protocol::ConnectionId;

use super::clock::clock::{DefaultClock, DurationSinceUnixEpoch, Time};

pub fn get_connection_id(remote_address: &SocketAddr) -> ConnectionId {
    ConnectionId(((current_time() / 3600) | ((remote_address.port() as u64) << 36)) as i64)
}

pub fn current_time() -> u64 {
    DefaultClock::now().as_secs()
}

pub fn ser_unix_time_value<S: serde::Serializer>(unix_time_value: &DurationSinceUnixEpoch, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u64(unix_time_value.as_millis() as u64)
}
