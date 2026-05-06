// Copied from aquatic_udp_protocol 0.9.0 by Joakim Frostegard (greatest-ape).
// Source:     https://crates.io/crates/aquatic_udp_protocol/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License:    Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//
// This in-house crate started from the aquatic 0.9.0 sources that were previously vendored
// under packages/aquatic-udp-protocol.
use std::io::{self, Write};

use byteorder::{NetworkEndian, WriteBytesExt};
use zerocopy::{FromBytes, Immutable, IntoBytes};

use super::common::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScrapeRequest {
    pub connection_id: ConnectionId,
    pub transaction_id: TransactionId,
    pub info_hashes: Vec<InfoHash>,
}

impl ScrapeRequest {
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        bytes.write_all(self.connection_id.as_bytes())?;
        bytes.write_i32::<NetworkEndian>(2)?;
        bytes.write_all(self.transaction_id.as_bytes())?;
        bytes.write_all((*self.info_hashes.as_slice()).as_bytes())?;

        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ScrapeResponse {
    pub transaction_id: TransactionId,
    pub torrent_stats: Vec<TorrentScrapeStatistics>,
}

impl ScrapeResponse {
    #[inline]
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        bytes.write_i32::<NetworkEndian>(2)?;
        bytes.write_all(self.transaction_id.as_bytes())?;
        bytes.write_all((*self.torrent_stats.as_slice()).as_bytes())?;

        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, IntoBytes, FromBytes, Immutable)]
#[repr(C, packed)]
pub struct TorrentScrapeStatistics {
    pub seeders: NumberOfPeers,
    pub completed: NumberOfDownloads,
    pub leechers: NumberOfPeers,
}
