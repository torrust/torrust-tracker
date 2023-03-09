pub mod asserts;
pub mod client;
pub mod test_environment;

/// Generates the source address for the UDP client
fn source_address(port: u16) -> String {
    format!("127.0.0.1:{port}")
}
