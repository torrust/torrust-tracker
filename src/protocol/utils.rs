use std::error::Error;
use std::fmt::Write;
use std::net::SocketAddr;
use std::time::SystemTime;

use aquatic_udp_protocol::ConnectionId;

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

pub fn url_encode_bytes(content: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut out: String = String::new();

    for byte in content.iter() {
        match *byte as char {
            '0'..='9' | 'a'..='z' | 'A'..='Z' | '.' | '-' | '_' | '~' => out.push(*byte as char),
            _ => write!(&mut out, "%{:02x}", byte)?,
        };
    }

    Ok(out)
}

pub fn ser_instant<S: serde::Serializer>(inst: &std::time::Instant, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u64(inst.elapsed().as_millis() as u64)
}
