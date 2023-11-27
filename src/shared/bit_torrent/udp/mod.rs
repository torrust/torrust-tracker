pub mod client;

/// The maximum number of bytes in a UDP packet.
pub const MAX_PACKET_SIZE: usize = 1496;
/// A magic 64-bit integer constant defined in the protocol that is used to
/// identify the protocol.
pub const PROTOCOL_ID: i64 = 0x0417_2710_1980;

/// Generates the source address for the UDP client
fn source_address(port: u16) -> String {
    format!("127.0.0.1:{port}")
}
