use std::net::SocketAddr;
use std::time::SystemTime;
use aquatic_udp_protocol::ConnectionId;

/// It generates a connection id needed for the BitTorrent UDP Tracker Protocol
pub fn get_connection_id(remote_address: &SocketAddr) -> ConnectionId {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => ConnectionId(((duration.as_secs() / 3600) | ((remote_address.port() as u64) << 36)) as i64),
        Err(_) => ConnectionId(0x7FFFFFFFFFFFFFFF),
    }
}

/// It returns the current time in Unix Epoch.
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
    use core::time;
    use std::{time::Instant, net::{SocketAddr, IpAddr, Ipv4Addr}, thread::sleep};

    #[test]
    fn connection_id_in_udp_tracker_should_be_the_same_for_one_client_during_some_hours() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let connection_id_1 = get_connection_id(&client_addr);

        // TODO: mock time passing
        sleep(time::Duration::from_secs(10));

        let connection_id_2 = get_connection_id(&client_addr);

        assert_eq!(connection_id_1, connection_id_2);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_different_for_each_tracker_client() {
        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 8080);

        let connection_id_1 = get_connection_id(&client_1_addr);
        let connection_id_2 = get_connection_id(&client_2_addr);

        assert_ne!(connection_id_1, connection_id_2);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_different_for_the_same_tracker_client_at_different_times() {
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let connection_id_1 = get_connection_id(&client_addr);

        sleep(time::Duration::from_secs(2));

        let connection_id_2 = get_connection_id(&client_addr);

        assert_ne!(connection_id_1, connection_id_2);
    }

    use serde::Serialize;

    use crate::protocol::utils::get_connection_id;

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

        let t1 = time::Instant::now();

        let s = S { time: t1 };

        // Sleep 10 milliseconds
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);

        let json_serialized_value = serde_json::to_string(&s).unwrap();

        // Json contains time duration since t1 instant in milliseconds
        assert_eq!(json_serialized_value, r#"{"time":10}"#);
    }
}