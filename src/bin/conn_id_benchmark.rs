use libc::{c_char, c_void};
use torrust_tracker::udp::byte_array_32::ByteArray32;
use torrust_tracker::udp::connection_id::get_connection_id;
use std::ptr::{null, null_mut};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
    print!("{}", String::from_utf8_lossy(unsafe {
        std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
    }));
}

fn mem_print() {
    unsafe { jemalloc_sys::malloc_stats_print(Some(write_cb), null_mut(), null()) }
}

/// Test function to locate the result in the output
fn test_function() {
    mem_print();
    let _heap = Vec::<u8>::with_capacity (1024 * 128); // 131072 bytes
    mem_print();
}

const SALT: &str = "SALT";

/// First implementation by @WarmBeer
fn test_old_get_connection_id(remote_address: &SocketAddr, time_as_seconds: u64) -> i64 {
    mem_print();

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

    mem_print();

    connection_id
}

/// New implementation by @josecelano
fn test_new_implementation() {
    mem_print();

    let server_secret = ByteArray32::new([0u8;32]);

    let client_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let now_as_timestamp = 946684800u64; // GMT/UTC date and time is: 01-01-2000 00:00:00

    let _connection_id = get_connection_id(&server_secret, &client_addr, now_as_timestamp);

    mem_print();
}

/// cargo run --bin conn_id_benchmark -q
fn main() {
    // Total allocated: 241120 - 99808 = 141312 bytes
    test_function();

    // Total allocated: 113248 - 99808 = 13440 bytes
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    test_old_get_connection_id(&address, 0);

    // Total allocated: 117344 - 99808 = 17536 bytes
    test_new_implementation()
}