use std::{net::SocketAddr};
use std::net::IpAddr;
use aquatic_udp_protocol::ConnectionId;

use super::byte_array_32::ByteArray32;
use super::time_bound_pepper::{TimeBoundPepper, Timestamp};

/// It generates a connection id needed for the BitTorrent UDP Tracker Protocol
pub fn get_connection_id(server_secret: &ByteArray32, remote_address: &SocketAddr, current_timestamp: Timestamp) -> ConnectionId {

    /* WIP: New proposal by @da2ce7

    Static_Sever_Secret = Random (32-bytes), generated on sever start.
    
    Time_Bound_Pepper = Hash(Static_Secret || Unix_Time_Minutes / 2) (32-bytes), cached, expires every two minutes.
    
    Authentication_String = IP_Address || Port || User Token || Etc. (32-bytes), unique for each client.
    
    ConnectionID = Hash(Time_Bound_Pepper || Authentication_String) (64-bit)
    */

    // todo: not used yet.
    let _time_bound_pepper = TimeBoundPepper::new(&server_secret, current_timestamp);

    let peer_ip_as_bytes = match remote_address.ip() {
        IpAddr::V4(ip) => ip.octets().to_vec(),
        IpAddr::V6(ip) => ip.octets().to_vec(),
    };

    let input: Vec<u8> = [
        (current_timestamp / 120).to_be_bytes().as_slice(),
        peer_ip_as_bytes.as_slice(),
        remote_address.port().to_be_bytes().as_slice(),
        server_secret.as_generic_byte_array().as_slice()
    ].concat();

    let hash = blake3::hash(&input);

    let mut truncated_hash: [u8; 8] = [0u8; 8];

    truncated_hash.copy_from_slice(&hash.as_bytes()[..8]);

    let connection_id = i64::from_le_bytes(truncated_hash);

    ConnectionId(connection_id)
}

/// Verifies whether a connection id is valid at this time for a given remote address (ip + port)
pub fn verify_connection_id(connection_id: ConnectionId, server_secret: &ByteArray32, remote_address: &SocketAddr, current_timestamp: Timestamp) -> Result<(), ()> {
    match connection_id {
        cid if cid == get_connection_id(server_secret, remote_address, current_timestamp) => Ok(()),
        cid if cid == get_connection_id(server_secret, remote_address, current_timestamp - 120) => Ok(()),
        _ => Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{net::{SocketAddr, IpAddr, Ipv4Addr}};

    fn generate_server_secret_for_testing() -> ByteArray32 {
        ByteArray32::new([0u8;32])
    }

    #[test]
    fn connection_id_is_generated_by_hashing_the_client_ip_and_port_with_a_server_secret() {
        let server_secret = generate_server_secret_for_testing();

        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now_as_timestamp = 946684800u64; // GMT/UTC date and time is: 01-01-2000 00:00:00

        let connection_id = get_connection_id(&server_secret, &client_addr, now_as_timestamp);

        assert_eq!(connection_id, ConnectionId(-7545411207427689958));

    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_the_same_for_one_client_during_two_minutes() {
        let server_secret = generate_server_secret_for_testing();

        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now = 946684800u64;

        let connection_id = get_connection_id(&server_secret, &client_addr, now);

        let in_two_minutes = now + 120 - 1;

        let connection_id_after_two_minutes = get_connection_id(&server_secret, &client_addr, in_two_minutes);

        assert_eq!(connection_id, connection_id_after_two_minutes);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_change_for_the_same_client_ip_and_port_after_two_minutes() {
        let server_secret = generate_server_secret_for_testing();

        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now = 946684800u64;

        let connection_id = get_connection_id(&server_secret, &client_addr, now);

        let after_two_minutes = now + 120;

        let connection_id_after_two_minutes = get_connection_id(&server_secret, &client_addr, after_two_minutes);

        assert_ne!(connection_id, connection_id_after_two_minutes);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_different_for_each_client_at_the_same_time_if_they_use_a_different_ip() {
        let server_secret = generate_server_secret_for_testing();

        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 0001);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);

        let now = 946684800u64;

        let connection_id_for_client_1 = get_connection_id(&server_secret, &client_1_addr, now);
        let connection_id_for_client_2 = get_connection_id(&server_secret, &client_2_addr, now);

        assert_ne!(connection_id_for_client_1, connection_id_for_client_2);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_different_for_each_client_at_the_same_time_if_they_use_a_different_port() {
        let server_secret = generate_server_secret_for_testing();

        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0002);

        let now = 946684800u64;

        let connection_id_for_client_1 = get_connection_id(&server_secret, &client_1_addr, now);
        let connection_id_for_client_2 = get_connection_id(&server_secret, &client_2_addr, now);

        assert_ne!(connection_id_for_client_1, connection_id_for_client_2);
    }

    #[test]
    fn connection_id_in_udp_tracker_should_be_valid_for_the_current_two_minute_window_since_unix_epoch_and_the_previous_window() {

        // The implementation generates a different connection id for each client and port every two minutes.
        // Connection should expire 2 minutes after the generation but we do not store the exact time 
        // when it was generated. In order to implement a stateless connection ID generation, 
        // we change it automatically and we approximate it to the 2-minute window.
        //
        // | Date                  | Timestamp | Unix Epoch in minutes | Connection IDs |
        // |----------------------------------------------------------------------------|
        // | 1/1/1970, 12:00:00 AM | 0         | minute 0              | X              |
        // | 1/1/1970, 12:01:00 AM | 60        | minute 1              | X              |
        // | 1/1/1970, 12:02:00 AM | 120       | minute 2              | Y = X          |
        // | 1/1/1970, 12:03:00 AM | 180       | minute 3              | Y = X          |
        // | 1/1/1970, 12:04:00 AM | 240       | minute 4              | Z != X         |
        // | 1/1/1970, 12:05:00 AM | 300       | minute 5              | Z != X         |
        //
        // Because of the implementation, the have to verify the current connection id and the previous one.
        // If the ID was generated at the end of a 2-minute slot I won't be valid just after some seconds.
        // For the worse scenario if the ID was generated at the beginning of a 2-minute slot,
        // It will be valid for almost 4 minutes.

        let server_secret = generate_server_secret_for_testing();

        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);

        let unix_epoch = 0u64;

        let connection_id = get_connection_id(&server_secret, &client_addr, unix_epoch);

        assert_eq!(verify_connection_id(connection_id, &server_secret, &client_addr, unix_epoch), Ok(()));

        // X = Y
        assert_eq!(verify_connection_id(connection_id, &server_secret, &client_addr, unix_epoch + 120), Ok(()));

        // X != Z
        assert_eq!(verify_connection_id(connection_id, &server_secret, &client_addr, unix_epoch + 240 + 1), Err(()));
    }
}