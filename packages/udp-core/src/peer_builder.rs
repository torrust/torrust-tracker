//! Logic to extract the peer info from the announce request.
use std::fmt;
use std::net::{IpAddr, SocketAddr};

use torrust_clock::clock::Time;
use torrust_tracker_primitives::{NumberOfBytes, peer};

use crate::CurrentClock;

/// Extracts the [`peer::Peer`] info from the
/// announce request.
///
/// This is the conversion from the BEP 15 wire request into the domain peer,
/// the first type the project owns. BEP 15 sends the byte counts as signed
/// 64-bit integers, so the wire type can hold a negative count; the domain
/// cannot, and the count is refused here.
///
/// # Arguments
///
/// * `peer_ip` - The real IP address of the peer, not the one in the announce request.
///
/// # Errors
///
/// It returns [`FromRequestError::NegativeNumberOfBytes`] if `left`,
/// `uploaded` or `downloaded` is negative.
// adr: docs/adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md
pub fn from_request(
    announce_request: &torrust_tracker_udp_protocol::AnnounceRequest,
    peer_ip: &IpAddr,
) -> Result<peer::Peer, FromRequestError> {
    let wire_event = torrust_tracker_udp_protocol::AnnounceEvent::from(announce_request.event);

    Ok(peer::Peer {
        peer_id: torrust_tracker_primitives::PeerId(announce_request.peer_id.0),
        peer_addr: SocketAddr::new(*peer_ip, announce_request.port.0.into()),
        updated: CurrentClock::now(),
        uploaded: number_of_bytes(ByteCountField::Uploaded, announce_request.bytes_uploaded)?,
        downloaded: number_of_bytes(ByteCountField::Downloaded, announce_request.bytes_downloaded)?,
        left: number_of_bytes(ByteCountField::Left, announce_request.bytes_left)?,
        event: match wire_event {
            torrust_tracker_udp_protocol::AnnounceEvent::Completed => torrust_tracker_primitives::AnnounceEvent::Completed,
            torrust_tracker_udp_protocol::AnnounceEvent::Started => torrust_tracker_primitives::AnnounceEvent::Started,
            torrust_tracker_udp_protocol::AnnounceEvent::Stopped => torrust_tracker_primitives::AnnounceEvent::Stopped,
            torrust_tracker_udp_protocol::AnnounceEvent::None => torrust_tracker_primitives::AnnounceEvent::None,
        },
    })
}

/// Converts one wire byte count into the domain type.
fn number_of_bytes(
    field: ByteCountField,
    wire_number_of_bytes: torrust_tracker_udp_protocol::NumberOfBytes,
) -> Result<NumberOfBytes, FromRequestError> {
    let value = wire_number_of_bytes.0.get();

    match u64::try_from(value) {
        Ok(_) => Ok(NumberOfBytes::new(value)),
        Err(_) => Err(FromRequestError::NegativeNumberOfBytes { field, value }),
    }
}

/// Errors of the conversion from a wire announce request into a domain peer.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FromRequestError {
    /// A byte count is negative. BEP 15 sends it as a signed integer, but a
    /// number of bytes cannot be negative.
    #[error("`{field}` must not be negative, got {value}")]
    NegativeNumberOfBytes { field: ByteCountField, value: i64 },
}

/// The byte-count fields of an announce request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteCountField {
    Left,
    Uploaded,
    Downloaded,
}

impl fmt::Display for ByteCountField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Left => "left",
            Self::Uploaded => "uploaded",
            Self::Downloaded => "downloaded",
        })
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::num::NonZeroU16;
    use std::time::Duration;

    use rstest::rstest;
    use torrust_clock::clock::stopped::Stopped as _;
    use torrust_tracker_primitives::{AnnounceEvent, NumberOfBytes, PeerId};
    use torrust_tracker_udp_protocol::{
        AnnounceActionPlaceholder, AnnounceEvent as WireAnnounceEvent, AnnounceRequest, ConnectionId, InfoHash, NumberOfPeers,
        PeerKey, Port, TransactionId,
    };

    use super::{ByteCountField, FromRequestError, from_request};
    use crate::CurrentClock;

    fn announce_request() -> AnnounceRequest {
        AnnounceRequest {
            connection_id: ConnectionId(0i64.into()),
            action_placeholder: AnnounceActionPlaceholder::default(),
            transaction_id: TransactionId(0i32.into()),
            info_hash: InfoHash([0x11; 20]),
            peer_id: PeerId([0x2d; 20]),
            bytes_downloaded: torrust_tracker_udp_protocol::NumberOfBytes(0i64.into()),
            bytes_uploaded: torrust_tracker_udp_protocol::NumberOfBytes(0i64.into()),
            bytes_left: torrust_tracker_udp_protocol::NumberOfBytes(0i64.into()),
            event: WireAnnounceEvent::Started.into(),
            ip_address: Ipv4Addr::UNSPECIFIED.into(),
            key: PeerKey::new(0i32),
            peers_wanted: NumberOfPeers::new(1i32),
            port: Port::new(NonZeroU16::new(6881).unwrap()),
        }
    }

    fn peer_ip() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(203, 0, 113, 10))
    }

    #[test]
    fn it_should_carry_the_request_peer_id() {
        // Arrange
        let mut request = announce_request();
        request.peer_id = PeerId(*b"-qB4650-abcdefghijkl");

        // Act
        let peer = from_request(&request, &peer_ip()).unwrap();

        // Assert
        assert_eq!(peer.peer_id, PeerId(*b"-qB4650-abcdefghijkl"));
    }

    #[test]
    fn it_should_use_the_supplied_ip_and_the_request_port_instead_of_the_ip_in_the_request() {
        // Arrange
        let mut request = announce_request();
        request.ip_address = Ipv4Addr::new(198, 51, 100, 77).into();
        request.port = Port::new(NonZeroU16::new(51_413).unwrap());

        // Act
        let peer = from_request(&request, &peer_ip()).unwrap();

        // Assert
        assert_eq!(peer.peer_addr, SocketAddr::new(peer_ip(), 51_413));
    }

    #[test]
    fn it_should_map_each_transfer_counter_to_the_matching_peer_field() {
        // Arrange
        let mut request = announce_request();
        request.bytes_uploaded = torrust_tracker_udp_protocol::NumberOfBytes(100i64.into());
        request.bytes_downloaded = torrust_tracker_udp_protocol::NumberOfBytes(200i64.into());
        request.bytes_left = torrust_tracker_udp_protocol::NumberOfBytes(300i64.into());

        // Act
        let peer = from_request(&request, &peer_ip()).unwrap();

        // Assert
        assert_eq!(
            (peer.uploaded, peer.downloaded, peer.left),
            (NumberOfBytes::new(100), NumberOfBytes::new(200), NumberOfBytes::new(300))
        );
    }

    #[rstest]
    #[case::started(WireAnnounceEvent::Started, AnnounceEvent::Started)]
    #[case::stopped(WireAnnounceEvent::Stopped, AnnounceEvent::Stopped)]
    #[case::completed(WireAnnounceEvent::Completed, AnnounceEvent::Completed)]
    #[case::none(WireAnnounceEvent::None, AnnounceEvent::None)]
    fn it_should_map_each_wire_announce_event_to_the_matching_domain_event(
        #[case] wire_event: WireAnnounceEvent,
        #[case] expected_event: AnnounceEvent,
    ) {
        // Arrange
        let mut request = announce_request();
        request.event = wire_event.into();

        // Act
        let peer = from_request(&request, &peer_ip()).unwrap();

        // Assert
        assert_eq!(peer.event, expected_event);
    }

    #[test]
    fn it_should_stamp_the_peer_with_the_current_time() {
        // Arrange
        let announce_time = Duration::from_secs(1_700_000_000);
        CurrentClock::local_set(&announce_time);

        // Act
        let peer = from_request(&announce_request(), &peer_ip()).unwrap();

        // Assert
        assert_eq!(peer.updated, announce_time);
    }

    #[rstest]
    #[case::left(ByteCountField::Left, -1)]
    #[case::left_min(ByteCountField::Left, i64::MIN)]
    #[case::uploaded(ByteCountField::Uploaded, -1)]
    #[case::downloaded(ByteCountField::Downloaded, -1)]
    fn it_should_reject_a_negative_number_of_bytes(#[case] field: ByteCountField, #[case] value: i64) {
        // Arrange
        let mut request = announce_request();
        let bytes = torrust_tracker_udp_protocol::NumberOfBytes(value.into());
        match field {
            ByteCountField::Left => request.bytes_left = bytes,
            ByteCountField::Uploaded => request.bytes_uploaded = bytes,
            ByteCountField::Downloaded => request.bytes_downloaded = bytes,
        }

        // Act
        let result = from_request(&request, &peer_ip());

        // Assert
        assert_eq!(result, Err(FromRequestError::NegativeNumberOfBytes { field, value }));
    }

    #[rstest]
    #[case::zero(0)]
    #[case::unknown_size_of_libtorrent(16 * 1024)]
    #[case::unknown_size_of_transmission(i64::MAX)]
    fn it_should_accept_a_non_negative_number_of_bytes_left(#[case] left: i64) {
        // Arrange
        let mut request = announce_request();
        request.bytes_left = torrust_tracker_udp_protocol::NumberOfBytes(left.into());

        // Act
        let peer = from_request(&request, &peer_ip()).unwrap();

        // Assert
        assert_eq!(peer.left, NumberOfBytes::new(left));
    }
}
