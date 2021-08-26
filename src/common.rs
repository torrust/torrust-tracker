use std::net::{SocketAddr};
use serde::{Deserialize, Serialize};

pub const MAX_PACKET_SIZE: usize = 0xffff;
pub const MAX_SCRAPE_TORRENTS: u8 = 74;
pub const PROTOCOL_ID: i64 = 4_497_486_125_440; // protocol constant

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Actions {
    Connect = 0,
    Announce = 1,
    Scrape = 2,
    Error = 3,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Events {
    None = 0,
    Complete = 1,
    Started = 2,
    Stopped = 3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AnnounceEvent {
    None,
    Completed,
    Started,
    Stopped,
}

impl AnnounceEvent {
    #[inline]
    pub fn from_i32(i: i32) -> Self {
        match i {
            0 => Self::None,
            1 => Self::Completed,
            2 => Self::Started,
            3 => Self::Stopped,
            _ => Self::None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AnnounceInterval(pub i32);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Ord)]
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

impl serde::ser::Serialize for InfoHash {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut buffer = [0u8; 40];
        let bytes_out = binascii::bin2hex(&self.0, &mut buffer).ok().unwrap();
        let str_out = std::str::from_utf8(bytes_out).unwrap();

        serializer.serialize_str(str_out)
    }
}

impl<'de> serde::de::Deserialize<'de> for InfoHash {
    fn deserialize<D: serde::de::Deserializer<'de>>(des: D) -> Result<Self, D::Error> {
        des.deserialize_str(InfoHashVisitor)
    }
}

struct InfoHashVisitor;

impl<'v> serde::de::Visitor<'v> for InfoHashVisitor {
    type Value = InfoHash;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a 40 character long hash")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        if v.len() != 40 {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"expected a 40 character long string",
            ));
        }

        let mut res = InfoHash { 0: [0u8; 20] };

        if let Err(_) = binascii::hex2bin(v.as_bytes(), &mut res.0) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"expected a hexadecimal string",
            ));
        } else {
            return Ok(res);
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

#[repr(transparent)]
#[derive(Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct PeerId(pub [u8; 20]);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PeerKey(pub u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ResponsePeerList(pub Vec<SocketAddr>);

// impl Serialize for ResponsePeerList {
//     fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//         let mut bytes = Vec::with_capacity(self.0.len() * 6);
//         let mut seq = serializer.serialize_seq(Some(self.len()))?;
//
//         for peer in self.0.iter() {
//             match peer {
//                 SocketAddr::V4(mut ipv4) => {
//                     // todo: get local network IP or external IP from host machine
//                     // check for localhost, replace with local network IP or external IP
//                     if ipv4.ip() == &Ipv4Addr::new(127, 0, 0, 1) {
//                         bytes.extend_from_slice(&Ipv4Addr::new(192, 168, 0, 182).octets());
//                     } else {
//                         bytes.extend_from_slice(&ipv4.ip().octets());
//                     }
//                 }
//                 SocketAddr::V6(ipv6) => {
//                     bytes.extend_from_slice(&ipv6.ip().octets());
//                 }
//             };
//
//             bytes.extend_from_slice(&peer.port().to_be_bytes());
//         }
//
//         seq.serialize_element()
//     }
// }

impl PeerId {
    pub fn get_client_name(&self) -> Option<&'static str> {
        if self.0[0] == b'M' {
            return Some("BitTorrent");
        }
        if self.0[0] == b'-' {
            let name = match &self.0[1..3] {
                b"AG" => "Ares",
                b"A~" => "Ares",
                b"AR" => "Arctic",
                b"AV" => "Avicora",
                b"AX" => "BitPump",
                b"AZ" => "Azureus",
                b"BB" => "BitBuddy",
                b"BC" => "BitComet",
                b"BF" => "Bitflu",
                b"BG" => "BTG (uses Rasterbar libtorrent)",
                b"BR" => "BitRocket",
                b"BS" => "BTSlave",
                b"BX" => "~Bittorrent X",
                b"CD" => "Enhanced CTorrent",
                b"CT" => "CTorrent",
                b"DE" => "DelugeTorrent",
                b"DP" => "Propagate Data Client",
                b"EB" => "EBit",
                b"ES" => "electric sheep",
                b"FT" => "FoxTorrent",
                b"FW" => "FrostWire",
                b"FX" => "Freebox BitTorrent",
                b"GS" => "GSTorrent",
                b"HL" => "Halite",
                b"HN" => "Hydranode",
                b"KG" => "KGet",
                b"KT" => "KTorrent",
                b"LH" => "LH-ABC",
                b"LP" => "Lphant",
                b"LT" => "libtorrent",
                b"lt" => "libTorrent",
                b"LW" => "LimeWire",
                b"MO" => "MonoTorrent",
                b"MP" => "MooPolice",
                b"MR" => "Miro",
                b"MT" => "MoonlightTorrent",
                b"NX" => "Net Transport",
                b"PD" => "Pando",
                b"qB" => "qBittorrent",
                b"QD" => "QQDownload",
                b"QT" => "Qt 4 Torrent example",
                b"RT" => "Retriever",
                b"S~" => "Shareaza alpha/beta",
                b"SB" => "~Swiftbit",
                b"SS" => "SwarmScope",
                b"ST" => "SymTorrent",
                b"st" => "sharktorrent",
                b"SZ" => "Shareaza",
                b"TN" => "TorrentDotNET",
                b"TR" => "Transmission",
                b"TS" => "Torrentstorm",
                b"TT" => "TuoTu",
                b"UL" => "uLeecher!",
                b"UT" => "µTorrent",
                b"UW" => "µTorrent Web",
                b"VG" => "Vagaa",
                b"WD" => "WebTorrent Desktop",
                b"WT" => "BitLet",
                b"WW" => "WebTorrent",
                b"WY" => "FireTorrent",
                b"XL" => "Xunlei",
                b"XT" => "XanTorrent",
                b"XX" => "Xtorrent",
                b"ZT" => "ZipTorrent",
                _ => return None,
            };
            Some(name)
        } else {
            None
        }
    }
}
impl Serialize for PeerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer, {
        let mut tmp = [0u8; 40];
        binascii::bin2hex(&self.0, &mut tmp).unwrap();
        let id = std::str::from_utf8(&tmp).ok();

        #[derive(Serialize)]
        struct PeerIdInfo<'a> {
            id: Option<&'a str>,
            client: Option<&'a str>,
        }

        let obj = PeerIdInfo {
            id,
            client: self.get_client_name(),
        };
        obj.serialize(serializer)
    }
}

// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type AnnounceInterval = i32;
//
// //#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub type InfoHash = [u8; 20];
//
// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
