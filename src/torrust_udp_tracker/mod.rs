pub mod errors;
pub mod request;
pub mod response;
pub mod server;

use self::errors::*;
use self::request::*;
use self::response::*;
use self::server::*;

pub type Bytes = u64;
pub type Port = u16;
pub type TransactionId = i64;

pub const MAX_PACKET_SIZE: usize = 0xffff;
pub const PROTOCOL_ID: i64 = 0x41727101980;
