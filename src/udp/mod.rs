//! BitTorrent UDP Tracker Implementation.
//!
//! Protocol Specification:
//!
//! [BEP 15](https://www.bittorrent.org/beps/bep_0015.html)

pub use self::errors::*;
pub use self::handlers::*;
pub use self::request::*;
pub use self::server::*;

pub mod byte_array_32;
pub mod connection_id;
pub mod errors;
pub mod request;
pub mod server;
pub mod handlers;

pub type Bytes = u64;
pub type Port = u16;
pub type TransactionId = i64;

pub const MAX_PACKET_SIZE: usize = 1496;
pub const PROTOCOL_ID: i64 = 0x41727101980;
