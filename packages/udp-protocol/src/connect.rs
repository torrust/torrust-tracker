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

use super::common::{ConnectionId, TransactionId};

pub(crate) const PROTOCOL_IDENTIFIER: i64 = 4_497_486_125_440;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ConnectRequest {
    pub transaction_id: TransactionId,
}

impl ConnectRequest {
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        bytes.write_i64::<NetworkEndian>(PROTOCOL_IDENTIFIER)?;
        bytes.write_i32::<NetworkEndian>(0)?;
        bytes.write_all(self.transaction_id.as_bytes())?;

        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, IntoBytes, FromBytes, Immutable)]
#[repr(C, packed)]
pub struct ConnectResponse {
    pub transaction_id: TransactionId,
    pub connection_id: ConnectionId,
}

impl ConnectResponse {
    #[inline]
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        bytes.write_i32::<NetworkEndian>(0)?;
        bytes.write_all(self.as_bytes())?;

        Ok(())
    }
}
