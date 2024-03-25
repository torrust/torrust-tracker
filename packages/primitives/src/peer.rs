//! Peer struct used by the core `Tracker`.
//!
//! A sample peer:
//!
//! ```rust,no_run
//! use torrust_tracker_primitives::peer;
//! use std::net::SocketAddr;
//! use std::net::IpAddr;
//! use std::net::Ipv4Addr;
//! use torrust_tracker_primitives::DurationSinceUnixEpoch;
//!
//!
//! peer::Peer {
//!     peer_id: peer::Id(*b"-qB00000000000000000"),
//!     peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
//!     updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
//!     uploaded: NumberOfBytes(0),
//!     downloaded: NumberOfBytes(0),
//!     left: NumberOfBytes(0),
//!     event: AnnounceEvent::Started,
//! };
//! ```

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use serde::Serialize;

use crate::announce_event::AnnounceEvent;
use crate::{ser_unix_time_value, DurationSinceUnixEpoch, IPVersion, NumberOfBytes};

/// Peer struct used by the core `Tracker`.
///
/// A sample peer:
///
/// ```rust,no_run
/// use torrust_tracker_primitives::peer;
/// use std::net::SocketAddr;
/// use std::net::IpAddr;
/// use std::net::Ipv4Addr;
/// use torrust_tracker_primitives::DurationSinceUnixEpoch;
///
///
/// peer::Peer {
///     peer_id: peer::Id(*b"-qB00000000000000000"),
///     peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
///     updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
///     uploaded: NumberOfBytes(0),
///     downloaded: NumberOfBytes(0),
///     left: NumberOfBytes(0),
///     event: AnnounceEvent::Started,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Peer {
    /// ID used by the downloader peer
    pub peer_id: Id,
    /// The IP and port this peer is listening on
    pub peer_addr: SocketAddr,
    /// The last time the the tracker receive an announce request from this peer (timestamp)
    #[serde(serialize_with = "ser_unix_time_value")]
    pub updated: DurationSinceUnixEpoch,
    /// The total amount of bytes uploaded by this peer so far
    pub uploaded: NumberOfBytes,
    /// The total amount of bytes downloaded by this peer so far
    pub downloaded: NumberOfBytes,
    /// The number of bytes this peer still has to download
    pub left: NumberOfBytes,
    /// This is an optional key which maps to started, completed, or stopped (or empty, which is the same as not being present).
    pub event: AnnounceEvent,
}

pub trait ReadInfo {
    fn is_seeder(&self) -> bool;
    fn get_event(&self) -> AnnounceEvent;
    fn get_id(&self) -> Id;
    fn get_updated(&self) -> DurationSinceUnixEpoch;
    fn get_address(&self) -> SocketAddr;
}

impl ReadInfo for Peer {
    fn is_seeder(&self) -> bool {
        self.left.0 <= 0 && self.event != AnnounceEvent::Stopped
    }

    fn get_event(&self) -> AnnounceEvent {
        self.event
    }

    fn get_id(&self) -> Id {
        self.peer_id
    }

    fn get_updated(&self) -> DurationSinceUnixEpoch {
        self.updated
    }

    fn get_address(&self) -> SocketAddr {
        self.peer_addr
    }
}

impl ReadInfo for Arc<Peer> {
    fn is_seeder(&self) -> bool {
        self.left.0 <= 0 && self.event != AnnounceEvent::Stopped
    }

    fn get_event(&self) -> AnnounceEvent {
        self.event
    }

    fn get_id(&self) -> Id {
        self.peer_id
    }

    fn get_updated(&self) -> DurationSinceUnixEpoch {
        self.updated
    }

    fn get_address(&self) -> SocketAddr {
        self.peer_addr
    }
}

impl Peer {
    #[must_use]
    pub fn is_seeder(&self) -> bool {
        self.left.0 <= 0 && self.event != AnnounceEvent::Stopped
    }

    pub fn ip(&mut self) -> IpAddr {
        self.peer_addr.ip()
    }

    pub fn change_ip(&mut self, new_ip: &IpAddr) {
        self.peer_addr = SocketAddr::new(*new_ip, self.peer_addr.port());
    }

    /// The IP version used by the peer: IPV4 or IPV6
    #[must_use]
    pub fn ip_version(&self) -> IPVersion {
        if self.peer_addr.is_ipv4() {
            return IPVersion::IPv4;
        }
        IPVersion::IPv6
    }
}

use std::panic::Location;

use thiserror::Error;

/// Error returned when trying to convert an invalid peer id from another type.
///
/// Usually because the source format does not contain 20 bytes.
#[derive(Error, Debug)]
pub enum IdConversionError {
    #[error("not enough bytes for peer id: {message} {location}")]
    NotEnoughBytes {
        location: &'static Location<'static>,
        message: String,
    },
    #[error("too many bytes for peer id: {message} {location}")]
    TooManyBytes {
        location: &'static Location<'static>,
        message: String,
    },
}

impl From<[u8; 20]> for Id {
    fn from(bytes: [u8; 20]) -> Self {
        Id(bytes)
    }
}

impl From<i32> for Id {
    fn from(number: i32) -> Self {
        let peer_id = number.to_le_bytes();
        Id::from([
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, peer_id[0], peer_id[1], peer_id[2],
            peer_id[3],
        ])
    }
}

impl TryFrom<Vec<u8>> for Id {
    type Error = IdConversionError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < PEER_ID_BYTES_LEN {
            return Err(IdConversionError::NotEnoughBytes {
                location: Location::caller(),
                message: format! {"got {} bytes, expected {}", bytes.len(), PEER_ID_BYTES_LEN},
            });
        }
        if bytes.len() > PEER_ID_BYTES_LEN {
            return Err(IdConversionError::TooManyBytes {
                location: Location::caller(),
                message: format! {"got {} bytes, expected {}", bytes.len(), PEER_ID_BYTES_LEN},
            });
        }
        Ok(Self::from_bytes(&bytes))
    }
}

impl std::str::FromStr for Id {
    type Err = IdConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.as_bytes().to_vec())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_hex_string() {
            Some(hex) => write!(f, "{hex}"),
            None => write!(f, ""),
        }
    }
}

/// Peer ID. A 20-byte array.
///
/// A string of length 20 which this downloader uses as its id.
/// Each downloader generates its own id at random at the start of a new download.
///
/// A sample peer ID:
///
/// ```rust,no_run
/// use torrust_tracker_primitives::peer;
///
/// let peer_id = peer::Id(*b"-qB00000000000000000");
/// ```
///
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Copy)]
pub struct Id(pub [u8; 20]);

pub const PEER_ID_BYTES_LEN: usize = 20;

impl Id {
    /// # Panics
    ///
    /// Will panic if byte slice does not contains the exact amount of bytes need for the `Id`.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(
            PEER_ID_BYTES_LEN,
            bytes.len(),
            "we are testing the equality of the constant: `PEER_ID_BYTES_LEN` ({}) and the supplied `bytes` length: {}",
            PEER_ID_BYTES_LEN,
            bytes.len(),
        );
        let mut ret = Self([0u8; PEER_ID_BYTES_LEN]);
        ret.0.clone_from_slice(bytes);
        ret
    }

    #[must_use]
    pub fn to_bytes(&self) -> [u8; 20] {
        self.0
    }

    #[must_use]
    /// Converts to hex string.
    ///
    /// For the Id `-qB00000000000000000` it returns `2d71423030303030303030303030303030303030`
    ///
    /// For example:
    ///
    ///```text
    /// Bytes                = Hex
    /// -qB00000000000000000 = 2d71423030303030303030303030303030303030
    /// -qB00000000000000000 = 2d 71 42 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30
    ///
    /// -------------
    /// |Char | Hex |
    /// -------------
    /// | -   | 2D  |
    /// | q   | 71  |
    /// | B   | 42  |
    /// | 0   | 30  |
    /// -------------
    /// ```
    ///
    /// Return `None` is some of the bytes are invalid UTF8 values.
    ///
    /// # Panics
    ///
    /// It will panic if the `binascii::bin2hex` from a too-small output buffer.
    pub fn to_hex_string(&self) -> Option<String> {
        let buff_size = self.0.len() * 2;
        let mut tmp: Vec<u8> = vec![0; buff_size];

        binascii::bin2hex(&self.0, &mut tmp).unwrap();

        match std::str::from_utf8(&tmp) {
            Ok(hex) => Some(format!("0x{hex}")),
            Err(_) => None,
        }
    }

    #[must_use]
    pub fn get_client_name(&self) -> Option<String> {
        let peer_id = tdyne_peer_id::PeerId::from(self.0);
        tdyne_peer_id_registry::parse(peer_id).ok().map(|parsed| parsed.client)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct PeerIdInfo {
            id: Option<String>,
            client: Option<String>,
        }

        let obj = PeerIdInfo {
            id: self.to_hex_string(),
            client: self.get_client_name(),
        };
        obj.serialize(serializer)
    }
}

/// Marker Trait for Peer Vectors
pub trait Encoding: From<Peer> + PartialEq {}

impl<P: Encoding> FromIterator<Peer> for Vec<P> {
    fn from_iter<T: IntoIterator<Item = Peer>>(iter: T) -> Self {
        let mut peers: Vec<P> = vec![];

        for peer in iter {
            peers.push(peer.into());
        }

        peers
    }
}

pub mod fixture {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use super::{Id, Peer};
    use crate::announce_event::AnnounceEvent;
    use crate::{DurationSinceUnixEpoch, NumberOfBytes};

    #[derive(PartialEq, Debug)]

    pub struct PeerBuilder {
        peer: Peer,
    }

    #[allow(clippy::derivable_impls)]
    impl Default for PeerBuilder {
        fn default() -> Self {
            Self { peer: Peer::default() }
        }
    }

    impl PeerBuilder {
        #[allow(dead_code)]
        #[must_use]
        pub fn with_peer_id(mut self, peer_id: &Id) -> Self {
            self.peer.peer_id = *peer_id;
            self
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn with_peer_addr(mut self, peer_addr: &SocketAddr) -> Self {
            self.peer.peer_addr = *peer_addr;
            self
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn with_bytes_pending_to_download(mut self, left: i64) -> Self {
            self.peer.left = NumberOfBytes(left);
            self
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn with_no_bytes_pending_to_download(mut self) -> Self {
            self.peer.left = NumberOfBytes(0);
            self
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn build(self) -> Peer {
            self.into()
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn into(self) -> Peer {
            self.peer
        }
    }

    impl Default for Peer {
        fn default() -> Self {
            Self {
                peer_id: Id::default(),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes(0),
                downloaded: NumberOfBytes(0),
                left: NumberOfBytes(0),
                event: AnnounceEvent::Started,
            }
        }
    }

    impl Default for Id {
        fn default() -> Self {
            Self(*b"-qB00000000000000000")
        }
    }
}

#[cfg(test)]
pub mod test {
    mod torrent_peer_id {
        use crate::peer;

        #[test]
        fn should_be_instantiated_from_a_byte_slice() {
            let id = peer::Id::from_bytes(&[
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            let expected_id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            assert_eq!(id, expected_id);
        }

        #[test]
        #[should_panic = "we are testing the equality of the constant: `PEER_ID_BYTES_LEN` (20) and the supplied `bytes` length: 19"]
        fn should_fail_trying_to_instantiate_from_a_byte_slice_with_less_than_20_bytes() {
            let less_than_20_bytes = [0; 19];
            let _: peer::Id = peer::Id::from_bytes(&less_than_20_bytes);
        }

        #[test]
        #[should_panic = "we are testing the equality of the constant: `PEER_ID_BYTES_LEN` (20) and the supplied `bytes` length: 21"]
        fn should_fail_trying_to_instantiate_from_a_byte_slice_with_more_than_20_bytes() {
            let more_than_20_bytes = [0; 21];
            let _: peer::Id = peer::Id::from_bytes(&more_than_20_bytes);
        }

        #[test]
        fn should_be_instantiated_from_a_string() {
            let id = "-qB00000000000000001".parse::<peer::Id>().unwrap();

            let expected_id = peer::Id([
                45, 113, 66, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 49,
            ]);

            assert_eq!(id, expected_id);
        }

        #[test]
        fn should_be_converted_from_a_20_byte_array() {
            let id = peer::Id::from([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            let expected_id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            assert_eq!(id, expected_id);
        }

        #[test]
        fn should_be_converted_from_a_byte_vector() {
            let id = peer::Id::try_from(
                [
                    0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
                ]
                .to_vec(),
            )
            .unwrap();

            let expected_id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);

            assert_eq!(id, expected_id);
        }

        #[test]
        #[should_panic = "NotEnoughBytes"]
        fn should_fail_trying_to_convert_from_a_byte_vector_with_less_than_20_bytes() {
            let _: peer::Id = peer::Id::try_from([0; 19].to_vec()).unwrap();
        }

        #[test]
        #[should_panic = "TooManyBytes"]
        fn should_fail_trying_to_convert_from_a_byte_vector_with_more_than_20_bytes() {
            let _: peer::Id = peer::Id::try_from([0; 21].to_vec()).unwrap();
        }

        #[test]
        fn should_be_converted_to_hex_string() {
            let id = peer::Id(*b"-qB00000000000000000");
            assert_eq!(id.to_hex_string().unwrap(), "0x2d71423030303030303030303030303030303030");

            let id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);
            assert_eq!(id.to_hex_string().unwrap(), "0x009f9296009f9296009f9296009f9296009f9296");
        }

        #[test]
        fn should_be_converted_into_string_type_using_the_hex_string_format() {
            let id = peer::Id(*b"-qB00000000000000000");
            assert_eq!(id.to_string(), "0x2d71423030303030303030303030303030303030");

            let id = peer::Id([
                0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
            ]);
            assert_eq!(id.to_string(), "0x009f9296009f9296009f9296009f9296009f9296");
        }

        #[test]
        fn should_return_the_inner_bytes() {
            assert_eq!(peer::Id(*b"-qB00000000000000000").to_bytes(), *b"-qB00000000000000000");
        }
    }
}
