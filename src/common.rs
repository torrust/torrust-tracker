use std::net::IpAddr;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum IpVersion {
    IPv4,
    IPv6,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AnnounceInterval(pub i32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug, Ord)]
pub struct InfoHash(pub [u8; 20]);

impl std::fmt::Display for InfoHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut chars = [0u8; 40];
        binascii::bin2hex(&self.0, &mut chars).expect("failed to hexlify");
        write!(f, "{}", std::str::from_utf8(&chars).unwrap())
    }
}

impl std::str::FromStr for InfoHash {
    type Err = binascii::ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = Self { 0: [0u8; 20] };
        if s.len() != 40 {
            return Err(binascii::ConvertError::InvalidInputLength);
        }
        binascii::hex2bin(s.as_bytes(), &mut i.0)?;
        Ok(i)
    }
}

impl std::cmp::PartialOrd<InfoHash> for InfoHash {
    fn partial_cmp(&self, other: &InfoHash) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::convert::From<&[u8]> for InfoHash {
    fn from(data: &[u8]) -> InfoHash {
        assert_eq!(data.len(), 20);
        let mut ret = InfoHash { 0: [0u8; 20] };
        ret.0.clone_from_slice(data);
        return ret;
    }
}

impl std::convert::Into<InfoHash> for [u8; 20] {
    fn into(self) -> InfoHash {
        InfoHash { 0: self }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ConnectionId(pub i64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct TransactionId(pub i32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NumberOfBytes(pub i64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NumberOfPeers(pub i32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NumberOfDownloads(pub i32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Port(pub u16);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct PeerId(pub [u8; 20]);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PeerKey(pub u32);

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct ResponsePeer {
    pub ip_address: IpAddr,
    pub port: Port,
}

// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type AnnounceInterval = i32;
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type InfoHash = [u8; 20];
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type ConnectionId = i64;
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type TransactionId = i32;
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type NumberOfBytes = i64;
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type NumberOfPeers = i32;
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type NumberOfDownloads = i32;
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type Port = u16;
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
// pub type PeerId = [u8; 20];
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type PeerKey = u32;
