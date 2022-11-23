pub mod connection_cookie;
pub mod errors;
pub mod handlers;
pub mod request;
pub mod server;

pub type Bytes = u64;
pub type Port = u16;
pub type TransactionId = i64;

pub const MAX_PACKET_SIZE: usize = 1496;
pub const PROTOCOL_ID: i64 = 0x0417_2710_1980;
