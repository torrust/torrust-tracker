use std::net::SocketAddr;
use std::time::SystemTime;
use std::error::Error;
use std::fmt::Write;
use std::io::Cursor;
use aquatic_udp_protocol::ConnectionId;
use byteorder::{BigEndian, ReadBytesExt};

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

// Function that will convert a small or big number into the smallest form of a byte array.
pub async fn convert_int_to_bytes(number: &u64) -> Vec<u8> {
    let mut return_data: Vec<u8> = Vec::new();
    // return_data.extend(number.to_be_bytes().reverse());
    for i in 1..8 {
        if number < &256u64.pow(i) {
            let start: usize = 16usize - i as usize;
            return_data.extend(number.to_be_bytes()[start..8].iter());
            return return_data;
        }
    }
    return return_data;
}

pub async fn convert_bytes_to_int(array: &Vec<u8>) -> u64 {
    let mut array_fixed: Vec<u8> = Vec::new();
    let size = 8 - array.len();
    for _ in 0..size {
        array_fixed.push(0);
    }
    array_fixed.extend(array);
    let mut rdr = Cursor::new(array_fixed);
    return rdr.read_u64::<BigEndian>().unwrap();
}
