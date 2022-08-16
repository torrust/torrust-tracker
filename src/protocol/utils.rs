use std::{net::SocketAddr, time::SystemTime};
use std::net::IpAddr;
use aquatic_udp_protocol::ConnectionId;

// todo: SALT should be randomly generated on startup
const SALT: &str = "SALT";

/// It generates a connection id needed for the BitTorrent UDP Tracker Protocol
pub fn get_connection_id(remote_address: &SocketAddr, time_as_seconds: u64) -> ConnectionId {
    let peer_ip_as_bytes = match remote_address.ip() {
        IpAddr::V4(ip) => ip.octets().to_vec(),
        IpAddr::V6(ip) => ip.octets().to_vec(),
    };

    let input: Vec<u8> = [
        (time_as_seconds / 120).to_be_bytes().as_slice(),
        peer_ip_as_bytes.as_slice(),
        remote_address.port().to_be_bytes().as_slice(),
        SALT.as_bytes()
    ].concat();

    let hash = blake3::hash(&input);

    let mut truncated_hash: [u8; 8] = [0u8; 8];

    truncated_hash.copy_from_slice(&hash.as_bytes()[..8]);

    let connection_id = i64::from_le_bytes(truncated_hash);

    ConnectionId(connection_id)
}

/// Verifies whether a connection id is valid at this time for a given remote address (ip + port)
pub fn verify_connection_id(connection_id: ConnectionId, remote_address: &SocketAddr) -> Result<(), ()> {
    let current_time = current_time();

    match connection_id {
        cid if cid == get_connection_id(remote_address, current_time) => Ok(()),
        cid if cid == get_connection_id(remote_address, current_time - 120) => Ok(()),
        _ => Err(())
    }
}

pub fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH).unwrap()
        .as_secs()
}

/// Serializer for `std::time::Instant` type.
/// Before serializing, it converts the instant to time elapse since that instant in milliseconds.
///
/// You can use it like this:
///
/// ```text
/// #[serde(serialize_with = "ser_instant")]
/// pub updated: std::time::Instant,
/// ```
///
pub fn ser_instant<S: serde::Serializer>(inst: &std::time::Instant, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u64(inst.elapsed().as_millis() as u64)
}

#[cfg(test)]
mod tests {
    use std::{time::Instant, net::{SocketAddr, IpAddr, Ipv4Addr}};
    use serde::Serialize;
    use crate::protocol::utils::{ConnectionId, get_connection_id};

    #[test]
    fn connection_id_is_generated_by_hashing_the_client_ip_and_port_with_a_salt() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now_as_timestamp = 946684800u64; // GMT/UTC date and time is: 01-01-2000 00:00:00

        let connection_id = get_connection_id(&client_addr, now_as_timestamp);

        assert_eq!(connection_id, ConnectionId(-6628342936351095906));

    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_the_same_for_one_client_during_two_minutes() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now = 946684800u64;

        let connection_id = get_connection_id(&client_addr, now);

        let in_two_minutes = now + 120 - 1;

        let connection_id_after_two_minutes = get_connection_id(&client_addr, in_two_minutes);

        assert_eq!(connection_id, connection_id_after_two_minutes);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_change_for_the_same_client_ip_and_port_after_two_minutes() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now = 946684800u64;

        let connection_id = get_connection_id(&client_addr, now);

        let after_two_minutes = now + 120;

        let connection_id_after_two_minutes = get_connection_id(&client_addr, after_two_minutes);

        assert_ne!(connection_id, connection_id_after_two_minutes);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_different_for_each_client_at_the_same_time_if_they_use_a_different_ip() {
        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 0001);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);

        let now = 946684800u64;

        let connection_id_for_client_1 = get_connection_id(&client_1_addr, now);
        let connection_id_for_client_2 = get_connection_id(&client_2_addr, now);

        assert_ne!(connection_id_for_client_1, connection_id_for_client_2);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_different_for_each_client_at_the_same_time_if_they_use_a_different_port() {
        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0002);

        let now = 946684800u64;

        let connection_id_for_client_1 = get_connection_id(&client_1_addr, now);
        let connection_id_for_client_2 = get_connection_id(&client_2_addr, now);

        assert_ne!(connection_id_for_client_1, connection_id_for_client_2);
    }

    #[warn(unused_imports)]
    use super::ser_instant;

    #[derive(PartialEq, Eq, Debug, Clone, Serialize)]
    struct S {
        #[serde(serialize_with = "ser_instant")]
        pub time: Instant,
    }

    #[test]
    fn instant_types_can_be_serialized_as_elapsed_time_since_that_instant_in_milliseconds() {

        use std::{thread, time};

        let t1 = Instant::now();

        let s = S { time: t1 };

        // Sleep 10 milliseconds
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);

        let json_serialized_value = serde_json::to_string(&s).unwrap();

        // Json contains time duration since t1 instant in milliseconds
        assert_eq!(json_serialized_value, r#"{"time":10}"#);
    }
}
