// Copied from aquatic_udp_protocol 0.9.0 by Joakim Frostegard (greatest-ape).
// Source:     https://crates.io/crates/aquatic_udp_protocol/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License:    Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//
// This in-house crate started from the aquatic 0.9.0 sources that were previously vendored
// under packages/aquatic-udp-protocol.
use std::io::{self, Write};

use byteorder::{NetworkEndian, WriteBytesExt};
use zerocopy::byteorder::network_endian::I32;
use zerocopy::{FromBytes, FromZeros, Immutable, IntoBytes};

use super::common::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug, IntoBytes, FromBytes, Immutable)]
#[repr(C, packed)]
pub struct AnnounceRequest {
    pub connection_id: ConnectionId,
    pub action_placeholder: AnnounceActionPlaceholder,
    pub transaction_id: TransactionId,
    pub info_hash: InfoHash,
    pub peer_id: PeerId,
    pub bytes_downloaded: NumberOfBytes,
    pub bytes_left: NumberOfBytes,
    pub bytes_uploaded: NumberOfBytes,
    pub event: AnnounceEventBytes,
    pub ip_address: Ipv4AddrBytes,
    pub key: PeerKey,
    pub peers_wanted: NumberOfPeers,
    pub port: Port,
}

impl AnnounceRequest {
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        bytes.write_all(self.as_bytes())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, IntoBytes, FromBytes, Immutable)]
#[repr(transparent)]
pub struct AnnounceActionPlaceholder(pub I32);

impl Default for AnnounceActionPlaceholder {
    fn default() -> Self {
        Self(I32::new(1))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, IntoBytes, FromBytes, Immutable)]
#[repr(transparent)]
pub struct AnnounceEventBytes(pub I32);

impl From<AnnounceEvent> for AnnounceEventBytes {
    fn from(value: AnnounceEvent) -> Self {
        Self(I32::new(match value {
            AnnounceEvent::None => 0,
            AnnounceEvent::Completed => 1,
            AnnounceEvent::Started => 2,
            AnnounceEvent::Stopped => 3,
        }))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
    None,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, IntoBytes, FromBytes, Immutable)]
#[repr(transparent)]
pub struct AnnounceInterval(pub I32);

impl AnnounceInterval {
    pub const fn new(v: i32) -> Self {
        Self(I32::new(v))
    }
}

impl From<AnnounceEventBytes> for AnnounceEvent {
    fn from(value: AnnounceEventBytes) -> Self {
        match value.0.get() {
            1 => Self::Completed,
            2 => Self::Started,
            3 => Self::Stopped,
            _ => Self::None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AnnounceResponse<I: Ip> {
    pub fixed: AnnounceResponseFixedData,
    pub peers: Vec<ResponsePeer<I>>,
}

impl<I: Ip> AnnounceResponse<I> {
    pub fn empty() -> Self {
        Self {
            fixed: FromZeros::new_zeroed(),
            peers: Default::default(),
        }
    }

    #[inline]
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        bytes.write_i32::<NetworkEndian>(1)?;
        bytes.write_all(self.fixed.as_bytes())?;
        bytes.write_all((*self.peers.as_slice()).as_bytes())?;

        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, IntoBytes, FromBytes, Immutable)]
#[repr(C, packed)]
pub struct AnnounceResponseFixedData {
    pub transaction_id: TransactionId,
    pub announce_interval: AnnounceInterval,
    pub leechers: NumberOfPeers,
    pub seeders: NumberOfPeers,
}
