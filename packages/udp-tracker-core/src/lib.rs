pub mod connection_cookie;
pub mod container;
pub mod crypto;
pub mod services;
pub mod statistics;

#[macro_use]
extern crate lazy_static;

/// The maximum number of connection id errors per ip. Clients will be banned if
/// they exceed this limit.
pub const MAX_CONNECTION_ID_ERRORS_PER_IP: u32 = 10;

pub const UDP_TRACKER_LOG_TARGET: &str = "UDP TRACKER";
