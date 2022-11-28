use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use serde::{Deserialize, Serialize};

pub const MAX_SCRAPE_TORRENTS: u8 = 74;
pub const AUTH_KEY_LENGTH: usize = 32;

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Actions {
    Connect = 0,
    Announce = 1,
    Scrape = 2,
    Error = 3,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "AnnounceEvent")]
pub enum AnnounceEventDef {
    Started,
    Stopped,
    Completed,
    None,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "NumberOfBytes")]
pub struct NumberOfBytesDef(pub i64);
