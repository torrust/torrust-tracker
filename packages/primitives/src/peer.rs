//! Peer struct used by the core `Tracker`.
//!
//! A sample peer:
//!
//! ```rust,no_run
//! use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
//! use torrust_tracker_primitives::peer;
//! use std::net::SocketAddr;
//! use std::net::IpAddr;
//! use std::net::Ipv4Addr;
//! use torrust_tracker_primitives::DurationSinceUnixEpoch;
//!
//!
//! peer::Peer {
//!     peer_id: PeerId(*b"-qB00000000000000000"),
//!     peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
//!     updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
//!     uploaded: NumberOfBytes::new(0),
//!     downloaded: NumberOfBytes::new(0),
//!     left: NumberOfBytes::new(0),
//!     event: AnnounceEvent::Started,
//! };
//! ```

use std::net::{IpAddr, SocketAddr};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
use serde::Serialize;
use zerocopy::FromBytes as _;

use crate::{DurationSinceUnixEpoch, IPVersion};

/// Peer struct used by the core `Tracker`.
///
/// A sample peer:
///
/// ```rust,no_run
/// use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes, PeerId};
/// use torrust_tracker_primitives::peer;
/// use std::net::SocketAddr;
/// use std::net::IpAddr;
/// use std::net::Ipv4Addr;
/// use torrust_tracker_primitives::DurationSinceUnixEpoch;
///
///
/// peer::Peer {
///     peer_id: PeerId(*b"-qB00000000000000000"),
///     peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
///     updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
///     uploaded: NumberOfBytes::new(0),
///     downloaded: NumberOfBytes::new(0),
///     left: NumberOfBytes::new(0),
///     event: AnnounceEvent::Started,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Hash)]
pub struct Peer {
    /// ID used by the downloader peer
    #[serde(serialize_with = "ser_peer_id")]
    pub peer_id: PeerId,
    /// The IP and port this peer is listening on
    pub peer_addr: SocketAddr,
    /// The last time the the tracker receive an announce request from this peer (timestamp)
    #[serde(serialize_with = "ser_unix_time_value")]
    pub updated: DurationSinceUnixEpoch,
    /// The total amount of bytes uploaded by this peer so far
    #[serde(serialize_with = "ser_number_of_bytes")]
    pub uploaded: NumberOfBytes,
    /// The total amount of bytes downloaded by this peer so far
    #[serde(serialize_with = "ser_number_of_bytes")]
    pub downloaded: NumberOfBytes,
    /// The number of bytes this peer still has to download
    #[serde(serialize_with = "ser_number_of_bytes")]
    pub left: NumberOfBytes,
    /// This is an optional key which maps to started, completed, or stopped (or empty, which is the same as not being present).
    #[serde(serialize_with = "ser_announce_event")]
    pub event: AnnounceEvent,
}

/// Serializes a `DurationSinceUnixEpoch` as a Unix timestamp in milliseconds.
/// # Errors
///
/// Will return `serde::Serializer::Error` if unable to serialize the `unix_time_value`.
pub fn ser_unix_time_value<S: serde::Serializer>(unix_time_value: &DurationSinceUnixEpoch, ser: S) -> Result<S::Ok, S::Error> {
    #[allow(clippy::cast_possible_truncation)]
    ser.serialize_u64(unix_time_value.as_millis() as u64)
}

#[derive(Serialize)]
pub enum AnnounceEventSer {
    Started,
    Stopped,
    Completed,
    None,
}

/// Serializes a `Announce Event` as a enum.
///
/// # Errors
///
/// If will return an error if the internal serializer was to fail.
pub fn ser_announce_event<S: serde::Serializer>(announce_event: &AnnounceEvent, ser: S) -> Result<S::Ok, S::Error> {
    let event_ser = match announce_event {
        AnnounceEvent::Started => AnnounceEventSer::Started,
        AnnounceEvent::Stopped => AnnounceEventSer::Stopped,
        AnnounceEvent::Completed => AnnounceEventSer::Completed,
        AnnounceEvent::None => AnnounceEventSer::None,
    };

    ser.serialize_some(&event_ser)
}

/// Serializes a `Announce Event` as a i64.
///
/// # Errors
///
/// If will return an error if the internal serializer was to fail.
pub fn ser_number_of_bytes<S: serde::Serializer>(number_of_bytes: &NumberOfBytes, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_i64(number_of_bytes.0.get())
}

/// Serializes a `PeerId` as a `peer::Id`.
///
/// # Errors
///
/// If will return an error if the internal serializer was to fail.
pub fn ser_peer_id<S: serde::Serializer>(peer_id: &PeerId, ser: S) -> Result<S::Ok, S::Error> {
    let id = Id { data: *peer_id };
    ser.serialize_some(&id)
}

impl Ord for Peer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.peer_id.cmp(&other.peer_id)
    }
}

impl PartialOrd for Peer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.peer_id.cmp(&other.peer_id))
    }
}

pub trait ReadInfo {
    fn is_seeder(&self) -> bool;
    fn get_event(&self) -> AnnounceEvent;
    fn get_id(&self) -> PeerId;
    fn get_updated(&self) -> DurationSinceUnixEpoch;
    fn get_address(&self) -> SocketAddr;
}

impl ReadInfo for Peer {
    fn is_seeder(&self) -> bool {
        self.left.0.get() <= 0 && self.event != AnnounceEvent::Stopped
    }

    fn get_event(&self) -> AnnounceEvent {
        self.event
    }

    fn get_id(&self) -> PeerId {
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
        self.left.0.get() <= 0 && self.event != AnnounceEvent::Stopped
    }

    fn get_event(&self) -> AnnounceEvent {
        self.event
    }

    fn get_id(&self) -> PeerId {
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
        self.left.0.get() <= 0 && self.event != AnnounceEvent::Stopped
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

pub struct Id {
    data: PeerId,
}

impl From<PeerId> for Id {
    fn from(id: PeerId) -> Self {
        Self { data: id }
    }
}

impl Deref for Id {
    type Target = PeerId;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl From<[u8; 20]> for Id {
    fn from(bytes: [u8; 20]) -> Self {
        let data = PeerId(bytes);
        Self { data }
    }
}

impl From<i32> for Id {
    fn from(number: i32) -> Self {
        let number = number.to_le_bytes();
        let bytes = [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, number[0], number[1], number[2],
            number[3],
        ];

        Id::from(bytes)
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

        let data = PeerId::read_from(&bytes).expect("it should have the correct amount of bytes");
        Ok(Self { data })
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

pub const PEER_ID_BYTES_LEN: usize = 20;

impl Id {
    #[must_use]
    /// Converts to hex string.
    ///
    /// For the `PeerId` `-qB00000000000000000` it returns `2d71423030303030303030303030303030303030`
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

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};

    use super::{Id, Peer, PeerId};
    use crate::DurationSinceUnixEpoch;

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
        pub fn seeder() -> Self {
            let peer = Peer {
                peer_id: PeerId(*b"-qB00000000000000001"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes::new(0),
                downloaded: NumberOfBytes::new(0),
                left: NumberOfBytes::new(0),
                event: AnnounceEvent::Completed,
            };

            Self { peer }
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn leecher() -> Self {
            let peer = Peer {
                peer_id: PeerId(*b"-qB00000000000000002"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 8080),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes::new(0),
                downloaded: NumberOfBytes::new(0),
                left: NumberOfBytes::new(10),
                event: AnnounceEvent::Started,
            };

            Self { peer }
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn with_peer_id(mut self, peer_id: &PeerId) -> Self {
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
            self.peer.left = NumberOfBytes::new(left);
            self
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn with_no_bytes_pending_to_download(mut self) -> Self {
            self.peer.left = NumberOfBytes::new(0);
            self
        }

        #[allow(dead_code)]
        #[must_use]
        pub fn last_updated_on(mut self, updated: DurationSinceUnixEpoch) -> Self {
            self.peer.updated = updated;
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
                peer_id: PeerId(*b"-qB00000000000000000"),
                peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
                updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
                uploaded: NumberOfBytes::new(0),
                downloaded: NumberOfBytes::new(0),
                left: NumberOfBytes::new(0),
                event: AnnounceEvent::Started,
            }
        }
    }

    impl Default for Id {
        fn default() -> Self {
            let data = PeerId(*b"-qB00000000000000000");
            Self { data }
        }
    }
}

#[cfg(test)]
pub mod test {
    mod torrent_peer_id {
        use aquatic_udp_protocol::PeerId;

        use crate::peer;

        #[test]
        #[should_panic = "NotEnoughBytes"]
        fn should_fail_trying_to_convert_from_a_byte_vector_with_less_than_20_bytes() {
            let _ = peer::Id::try_from([0; 19].to_vec()).unwrap();
        }

        #[test]
        #[should_panic = "TooManyBytes"]
        fn should_fail_trying_to_convert_from_a_byte_vector_with_more_than_20_bytes() {
            let _ = peer::Id::try_from([0; 21].to_vec()).unwrap();
        }

        #[test]
        fn should_be_converted_to_hex_string() {
            let id = peer::Id {
                data: PeerId(*b"-qB00000000000000000"),
            };
            assert_eq!(id.to_hex_string().unwrap(), "0x2d71423030303030303030303030303030303030");

            let id = peer::Id {
                data: PeerId([
                    0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
                ]),
            };
            assert_eq!(id.to_hex_string().unwrap(), "0x009f9296009f9296009f9296009f9296009f9296");
        }

        #[test]
        fn should_be_converted_into_string_type_using_the_hex_string_format() {
            let id = peer::Id {
                data: PeerId(*b"-qB00000000000000000"),
            };
            assert_eq!(id.to_string(), "0x2d71423030303030303030303030303030303030");

            let id = peer::Id {
                data: PeerId([
                    0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150, 0, 159, 146, 150,
                ]),
            };
            assert_eq!(id.to_string(), "0x009f9296009f9296009f9296009f9296009f9296");
        }
    }
}
