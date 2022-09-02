//! Connection ID is a value generate by the tracker and sent to the client
//! to avoid the client spoofing it's source IP address.
//!
//! Detailed info in [BEP 15](https://www.bittorrent.org/beps/bep_0015.html)
//!
//! In order for the client to connect to the tracker, it must send a connection ID
//! previously generated by the server.
//!
//! The client has to send a "connect" request:
//!
//! | Offset | Size           | Name           | Value                           |
//! |--------|----------------|----------------|---------------------------------|
//! | 0      | 64-bit integer | protocol_id    | 0x41727101980 // magic constant |
//! | 8      | 32-bit integer | action         | 0             // connect        |
//! | 12     | 32-bit integer | transaction_id |                                 |
//! | 16     |                |                |                                 |
//!
//! And it receives a Connection ID in the response:
//!
//! | Offset | Size           | Name           | Value        |
//! |--------|----------------|----------------|--------------|
//! | 0      | 32-bit integer | action         | 0 // connect |
//! | 4      | 32-bit integer | transaction_id |              |
//! | 8      | 64-bit integer | connection_id  |              |
//! | 16     |                |                |              |
//!
//! The client has to send the Connection ID in all subsequent requests.
//! The tracker verifies the connection_id and ignores the request if it doesn't match.
//!
//! From the BEP 15 specification a Connection ID:
//!
//! - Should not be guessable by the client.
//! - Can be used for multiple requests.
//! - Can be used by a client until one minute after it has received it.
//! - Can be accepted by the tracker until two minutes after it has been send.
//! 
//! Additionally we define the Connection ID as a value that:
//! 
//! - That is unpredictable. The user should not be able to construct their own Connection ID.
//! - That is unique to the the particular connection. Locked to a IP and Port.
//! - That is time bound. It expires after certain time.
//! - That is memoryless. The server doesn't remember what ID's it gave out.
//! - That is stateless. The issuer and the verifier can work interdependently without a dynamic common state.
//!
//! # Why do we need a connection ID?
//!
//! With the Connection ID we check for two things:
//!
//! - The announcing client owns the ip and port it is announcing with.
//! - The announcing client is an online BitTorrent peer.
//!
//! It's a kind of "proof of IP ownership" and "proof of online BitTorrent peer".
//! This makes sure that the client is not a fake client. And it makes harder for attackers
//! to fill the tracker peer list with fake clients.
//!
//! The only way to send an "announce" request is actually being an active and accessible BitTorrent client.
//!
//! It also  avoid clients to send requests on behave of other clients.
//! If there is a legitimate client on the network, attackers could impersonate that client,
//! since they know the IP and port of the legitimate client.
//! An attacker could send an "announce" request for a torrent that the legitimate client does not have.
//! That's a kind of DOS attack because it would make harder to find a torrent.
//! The information about what torrents have each client could be easily manipulated.
//!
//! # Implementation
//!
//! Some tracker implementations use a time bound connection ID to avoid storing the connection ID
//! in memory or in the DB.
//!
//! ```ignore
//! static uint64_t _genCiD (uint32_t ip, uint16_t port)
//! {
//!   uint64_t x;
//!   x = (time(NULL) / 3600) * port; // x will probably overload.
//!   x = (ip ^ port);
//!   x <<= 16;
//!   x |= (~port);
//!   return x;
//! }
//! ```
//!
//! From [here](https://github.com/troydm/udpt/blob/master/src/db/driver_sqlite.cpp#L410-L418).
//!
//! We use the same approach but using a hash with a server secret in order to keep the connection ID
//! not guessable. We also use the client IP and port to make it unique for each client.
//!
//! The secret used for hashing changes every time the server starts.
//!
use std::{net::SocketAddr};
use std::net::IpAddr;
use aquatic_udp_protocol::ConnectionId;
use blake3::OUT_LEN;
use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::{BlockEncryptor, BlockDecryptor};
use std::convert::From;

use super::byte_array_32::ByteArray32;
use super::time_bound_pepper::Timestamp;

/// It generates a connection id needed for the BitTorrent UDP Tracker Protocol.
pub fn get_connection_id(server_secret: &ByteArray32, remote_address: &SocketAddr, current_timestamp: Timestamp) -> ConnectionId {

    let client_id = generate_id_for_socket_address(remote_address);

    let connection_id = concat(client_id, timestamp_to_le_bytes(current_timestamp));

    let encrypted_connection_id = encrypt(&connection_id, server_secret);

    ConnectionId(byte_array_to_i64(encrypted_connection_id))
}

/// Verifies whether a connection id is valid at this time for a given remote socket address (ip + port)
pub fn verify_connection_id(connection_id: ConnectionId, server_secret: &ByteArray32, _remote_address: &SocketAddr, current_timestamp: Timestamp) -> Result<(), ()> {
    
    let id_as_byte_array = decrypt(&connection_id.0.to_le_bytes(), server_secret);

    let timestamp_bytes = &id_as_byte_array[4..];
    let timestamp_array = [timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3], 0, 0, 0, 0]; // Little Endian
    let created_at_timestamp = u64::from_le_bytes(timestamp_array);

    let expire_timestamp = created_at_timestamp + 120;

    if expire_timestamp < current_timestamp {
        return Err(())
    }

    Ok(())
}

/// It generates an unique ID for a socket address (IP + port)
fn generate_id_for_socket_address(remote_address: &SocketAddr) -> [u8; 4] {
    let socket_addr_as_bytes: Vec<u8> = convert_socket_address_into_bytes(remote_address);

    let hashed_socket_addr = hash(&socket_addr_as_bytes);

    let remote_id = get_first_four_bytes_from(&hashed_socket_addr);

    remote_id
}

fn convert_socket_address_into_bytes(socket_addr: &SocketAddr) -> Vec<u8> {
    let bytes: Vec<u8> = [
        convert_ip_into_bytes(socket_addr.ip()).as_slice(),
        convert_port_into_bytes(socket_addr.port()).as_slice(),
    ].concat();
    bytes
}

fn convert_ip_into_bytes(ip_addr: IpAddr) -> Vec<u8> {
    match ip_addr {
        IpAddr::V4(ip) => ip.octets().to_vec(),
        IpAddr::V6(ip) => ip.octets().to_vec(),
    }
}

fn convert_port_into_bytes(port: u16) -> [u8; 2] {
    port.to_be_bytes()
}

fn hash(bytes: &[u8]) -> [u8; OUT_LEN]{
    let hash = blake3::hash(bytes);
    let bytes = hash.as_bytes().clone();
    bytes
}

fn get_first_four_bytes_from(bytes: &[u8; OUT_LEN]) -> [u8; 4] {
    let mut first_four_bytes: [u8; 4] = [0u8; 4]; // 4 bytes = 32 bits
    first_four_bytes.copy_from_slice(&bytes[..4]);
    first_four_bytes
}

fn timestamp_to_le_bytes(current_timestamp: Timestamp) -> [u8; 4] {
    let mut bytes: [u8; 4] = [0u8; 4];
    bytes.copy_from_slice(&current_timestamp.to_le_bytes()[..4]);
    bytes
}

/// Contact two 4-byte arrays
fn concat(remote_id: [u8; 4], timestamp: [u8; 4]) -> [u8; 8] {
    let connection_id: Vec<u8> = [
        remote_id.as_slice(),
        timestamp.as_slice(),
    ].concat();

    let connection_as_array: [u8; 8] = connection_id.try_into().unwrap();

    connection_as_array
}

fn encrypt(connection_id: &[u8; 8], server_secret: &ByteArray32) -> [u8; 8] {
    // TODO: pass as an argument. It's expensive.
    let blowfish = Blowfish::new(&server_secret.as_generic_byte_array());

    let mut encrypted_connection_id = [0u8; 8];

    blowfish.encrypt_block(connection_id, &mut encrypted_connection_id);

    encrypted_connection_id
}

fn decrypt(encrypted_connection_id: &[u8; 8], server_secret: &ByteArray32) -> [u8; 8] {
    // TODO: pass as an argument. It's expensive.
    let blowfish = Blowfish::new(&server_secret.as_generic_byte_array());

    let mut connection_id = [0u8; 8];

    blowfish.decrypt_block(encrypted_connection_id, &mut connection_id);

    connection_id
}

fn byte_array_to_i64(connection_id: [u8;8]) -> i64 {
    i64::from_le_bytes(connection_id)
}

impl From<IpAddr> for ByteArray32 {
    //// Converts an IpAddr to a ByteArray32
    fn from(ip: IpAddr) -> Self {
        let peer_ip_as_bytes = match ip {
            IpAddr::V4(ip) => [
                [0u8; 28].as_slice(),   // 28 bytes
                ip.octets().as_slice(), //  4 bytes
            ].concat(),
            IpAddr::V6(ip) => [
                [0u8; 16].as_slice(),   // 16 bytes
                ip.octets().as_slice(), // 16 bytes
            ].concat(),
        };
    
        let peer_ip_address_32_bytes: [u8; 32] = match peer_ip_as_bytes.try_into() {
            Ok(bytes) => bytes,
            Err(_) => panic!("Expected a Vec of length 32"),
        };

        ByteArray32::new(peer_ip_address_32_bytes)
    }
}

impl From<u16> for ByteArray32 {
    /// Converts a u16 to a ByteArray32
    fn from(port: u16) -> Self {
        let port = [
            [0u8; 30].as_slice(),          // 30 bytes
            port.to_be_bytes().as_slice(), //  2 bytes
        ].concat();
    
        let port_32_bytes: [u8; 32] = match port.try_into() {
            Ok(bytes) => bytes,
            Err(_) => panic!("Expected a Vec of length 32"),
        };

        ByteArray32::new(port_32_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr}};

    fn generate_server_secret_for_testing() -> ByteArray32 {
        ByteArray32::new([0u8;32])
    }

    #[test]
    fn ip_address_should_be_converted_to_a_32_bytes_array() {
        let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(ByteArray32::from(ip_address), ByteArray32::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 127, 0, 0, 1]));
    }

    #[test]
    fn ipv4_address_should_be_converted_to_a_byte_vector() {
        let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let bytes = convert_ip_into_bytes(ip_address);
        assert_eq!(bytes, vec![127, 0, 0, 1]); // 4 bytes
    }

    #[test]
    fn ipv6_address_should_be_converted_to_a_byte_vector() {
        let ip_address = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        let bytes = convert_ip_into_bytes(ip_address);
        assert_eq!(bytes, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]); // 16 bytes
    }

    #[test]
    fn socket_port_should_be_converted_to_a_32_bytes_array() {
        let port = 0x1F_90u16; // 8080
        assert_eq!(ByteArray32::from(port), ByteArray32::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1F, 0x90]));
    }    

    /*#[test]
    fn it_should_be_the_same_for_one_client_during_two_minutes() {
        let server_secret = generate_server_secret_for_testing();

        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now = 946684800u64;

        let connection_id = get_connection_id(&server_secret, &client_addr, now);

        let in_two_minutes = now + 120 - 1;

        let connection_id_after_two_minutes = get_connection_id(&server_secret, &client_addr, in_two_minutes);

        assert_eq!(connection_id, connection_id_after_two_minutes);
    }*/

    #[test]
    fn it_should_be_valid_for_two_minutes_after_the_generation() {
        let server_secret = generate_server_secret_for_testing();
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let now = 946684800u64; // 01-01-2000 00:00:00

        let connection_id = get_connection_id(&server_secret, &client_addr, now);

        let ret = verify_connection_id(connection_id, &server_secret, &client_addr, now);

        println!("ret: {:?}", ret);

        assert_eq!(ret, Ok(()));

        let after_two_minutes = now + (2*60) - 1;

        assert_eq!(verify_connection_id(connection_id, &server_secret, &client_addr, after_two_minutes), Ok(()));
    }

    #[test]
    fn it_should_expire_after_two_minutes_from_the_generation() {
        let server_secret = generate_server_secret_for_testing();
        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let now = 946684800u64;

        let connection_id = get_connection_id(&server_secret, &client_addr, now);

        let ret = verify_connection_id(connection_id, &server_secret, &client_addr, now);

        println!("ret: {:?}", ret);

        let after_more_than_two_minutes = now + (2*60) + 1;

        assert_eq!(verify_connection_id(connection_id, &server_secret, &client_addr, after_more_than_two_minutes), Err(()));
    }    

    #[test]
    fn it_should_change_for_the_same_client_ip_and_port_after_two_minutes() {
        let server_secret = generate_server_secret_for_testing();

        let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        let now = 946684800u64;

        let connection_id = get_connection_id(&server_secret, &client_addr, now);

        let after_two_minutes = now + 120;

        let connection_id_after_two_minutes = get_connection_id(&server_secret, &client_addr, after_two_minutes);

        assert_ne!(connection_id, connection_id_after_two_minutes);
    }

    #[test]
    fn it_should_be_different_for_each_client_at_the_same_time_if_they_use_a_different_ip() {
        let server_secret = generate_server_secret_for_testing();

        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 0001);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);

        let now = 946684800u64;

        let connection_id_for_client_1 = get_connection_id(&server_secret, &client_1_addr, now);
        let connection_id_for_client_2 = get_connection_id(&server_secret, &client_2_addr, now);

        assert_ne!(connection_id_for_client_1, connection_id_for_client_2);
    }

    #[test]
    fn it_should_be_different_for_each_client_at_the_same_time_if_they_use_a_different_port() {
        let server_secret = generate_server_secret_for_testing();

        let client_1_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0001);
        let client_2_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0002);

        let now = 946684800u64;

        let connection_id_for_client_1 = get_connection_id(&server_secret, &client_1_addr, now);
        let connection_id_for_client_2 = get_connection_id(&server_secret, &client_2_addr, now);

        assert_ne!(connection_id_for_client_1, connection_id_for_client_2);
    }
}