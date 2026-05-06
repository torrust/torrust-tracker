// Adapted from aquatic_peer_id 0.9.0 by Joakim Frostegard (greatest-ape).
// Source: https://crates.io/crates/aquatic_peer_id/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License: Apache License, Version 2.0

use compact_str::CompactString;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::peer_client::PeerClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::IntoBytes, zerocopy::FromBytes, zerocopy::Immutable))]
#[repr(transparent)]
pub struct PeerId(pub [u8; 20]);

impl PeerId {
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    #[must_use]
    pub fn client(&self) -> PeerClient {
        PeerClient::from_peer_id(self)
    }

    /// # Panics
    ///
    /// Never panics; the expect is unreachable because the buffer is exactly the right size.
    #[must_use]
    pub fn first_8_bytes_hex(&self) -> CompactString {
        let mut buf = [0u8; 16];

        hex::encode_to_slice(&self.0[..8], &mut buf).expect("PeerId.first_8_bytes_hex buffer too small");

        CompactString::from_utf8_lossy(&buf)
    }
}

#[cfg(feature = "quickcheck")]
impl quickcheck::Arbitrary for PeerId {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut bytes = [0u8; 20];

        for byte in &mut bytes {
            *byte = u8::arbitrary(g);
        }

        Self(bytes)
    }
}
