// Copied from aquatic_udp_protocol 0.9.0 by Joakim Frostegard (greatest-ape).
// Source:     https://crates.io/crates/aquatic_udp_protocol/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License:    Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//
// This in-house crate started from the aquatic 0.9.0 sources that were previously vendored
// under packages/aquatic-udp-protocol.
use std::io::{self, Cursor, Write};
use std::mem::size_of;

use either::Either;
use zerocopy::FromBytes;
use zerocopy::byteorder::network_endian::I32;

use super::announce::AnnounceRequest;
use super::common::{ConnectionId, InfoHash, MAX_PACKET_SIZE, TransactionId, read_i32_ne, read_i64_ne};
use super::connect::{ConnectRequest, PROTOCOL_IDENTIFIER};
pub use super::scrape::ScrapeRequest;

/// Bytes before the info hashes in a scrape request: connection ID, action, and transaction ID.
const SCRAPE_REQUEST_HEADER_SIZE: usize = size_of::<ConnectionId>() + size_of::<I32>() + size_of::<TransactionId>();

/// The maximum number of info hashes kept from a UDP scrape request.
///
/// As many as fit in one [`MAX_PACKET_SIZE`] datagram after the request header,
/// which is 74 (BEP 15's "up to about 74 torrents"). The parser ignores the rest.
///
/// See `docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md`.
pub const MAX_SCRAPE_INFO_HASHES: usize = (MAX_PACKET_SIZE - SCRAPE_REQUEST_HEADER_SIZE) / size_of::<InfoHash>();

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Request {
    Connect(ConnectRequest),
    Announce(AnnounceRequest),
    Scrape(ScrapeRequest),
}

impl Request {
    /// # Errors
    ///
    /// Returns an error if the request cannot be written to `bytes`.
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        match self {
            Self::Connect(r) => r.write_bytes(bytes),
            Self::Announce(r) => r.write_bytes(bytes),
            Self::Scrape(r) => r.write_bytes(bytes),
        }
    }

    /// Scrape requests keep only their first `max_scrape_info_hashes` info
    /// hashes; the tracker passes [`MAX_SCRAPE_INFO_HASHES`].
    ///
    /// # Errors
    ///
    /// Returns an error if `bytes` does not contain a valid UDP tracker request.
    pub fn parse_bytes(bytes: &[u8], max_scrape_info_hashes: usize) -> Result<Self, RequestParseError> {
        let action_bytes = bytes
            .get(8..12)
            .ok_or_else(|| RequestParseError::unsendable_text("Couldn't parse action"))?;
        let action = I32::from_bytes(
            action_bytes
                .try_into()
                .map_err(|_| RequestParseError::unsendable_text("Couldn't parse action"))?,
        );

        match action.get() {
            0 => {
                let mut bytes = Cursor::new(bytes);

                let protocol_identifier = read_i64_ne(&mut bytes).map_err(RequestParseError::unsendable_io)?;
                let _action = read_i32_ne(&mut bytes).map_err(RequestParseError::unsendable_io)?;
                let transaction_id = read_i32_ne(&mut bytes)
                    .map(TransactionId)
                    .map_err(RequestParseError::unsendable_io)?;

                if protocol_identifier.get() == PROTOCOL_IDENTIFIER {
                    Ok((ConnectRequest { transaction_id }).into())
                } else {
                    Err(RequestParseError::unsendable_text("Protocol identifier missing"))
                }
            }
            1 => {
                let request = AnnounceRequest::read_from_prefix(bytes)
                    .map_err(|_| RequestParseError::unsendable_text("invalid data"))?
                    .0;

                if request.port.0.get() == 0 {
                    Err(RequestParseError::sendable_text(
                        "Port can't be 0",
                        request.connection_id,
                        request.transaction_id,
                    ))
                } else if !matches!(request.event.0.get(), 0..=3) {
                    Err(RequestParseError::sendable_text(
                        "Invalid announce event",
                        request.connection_id,
                        request.transaction_id,
                    ))
                } else {
                    Ok(Self::Announce(request))
                }
            }
            2 => {
                let mut bytes = Cursor::new(bytes);

                let connection_id = read_i64_ne(&mut bytes)
                    .map(ConnectionId)
                    .map_err(RequestParseError::unsendable_io)?;
                let _action = read_i32_ne(&mut bytes).map_err(RequestParseError::unsendable_io)?;
                let transaction_id = read_i32_ne(&mut bytes)
                    .map(TransactionId)
                    .map_err(RequestParseError::unsendable_io)?;

                let remaining_bytes = {
                    let position = bytes.position() as usize;
                    let inner = bytes.into_inner();
                    &inner[position..]
                };

                if remaining_bytes.is_empty() {
                    return Err(RequestParseError::sendable_text(
                        "Full scrapes are not allowed",
                        connection_id,
                        transaction_id,
                    ));
                }

                let (chunks, remainder) = remaining_bytes.as_chunks::<{ size_of::<InfoHash>() }>();

                if !remainder.is_empty() {
                    return Err(RequestParseError::sendable_text(
                        "Invalid info hash list",
                        connection_id,
                        transaction_id,
                    ));
                }

                let info_hashes = chunks.iter().copied().map(InfoHash).collect::<Vec<_>>();

                let info_hashes = Vec::from(&info_hashes[..max_scrape_info_hashes.min(info_hashes.len())]);

                Ok((ScrapeRequest {
                    connection_id,
                    transaction_id,
                    info_hashes,
                })
                .into())
            }
            _ => Err(RequestParseError::unsendable_text("Invalid action")),
        }
    }
}

impl From<ConnectRequest> for Request {
    fn from(r: ConnectRequest) -> Self {
        Self::Connect(r)
    }
}

impl From<AnnounceRequest> for Request {
    fn from(r: AnnounceRequest) -> Self {
        Self::Announce(r)
    }
}

impl From<ScrapeRequest> for Request {
    fn from(r: ScrapeRequest) -> Self {
        Self::Scrape(r)
    }
}

#[derive(Debug)]
pub enum RequestParseError {
    Sendable {
        connection_id: ConnectionId,
        transaction_id: TransactionId,
        err: &'static str,
    },
    Unsendable {
        err: Either<io::Error, &'static str>,
    },
}

impl RequestParseError {
    #[must_use]
    pub const fn sendable_text(text: &'static str, connection_id: ConnectionId, transaction_id: TransactionId) -> Self {
        Self::Sendable {
            connection_id,
            transaction_id,
            err: text,
        }
    }
    #[must_use]
    pub const fn unsendable_io(err: io::Error) -> Self {
        Self::Unsendable { err: Either::Left(err) }
    }
    #[must_use]
    pub const fn unsendable_text(text: &'static str) -> Self {
        Self::Unsendable {
            err: Either::Right(text),
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use zerocopy::network_endian::{I32, I64};

    use super::*;
    use crate::announce::{AnnounceActionPlaceholder, AnnounceEvent};
    use crate::common::{Ipv4AddrBytes, NumberOfBytes, NumberOfPeers, PeerId, PeerKey, Port};

    impl quickcheck::Arbitrary for AnnounceEvent {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            match (bool::arbitrary(g), bool::arbitrary(g)) {
                (_, false) => Self::Started,
                (false, true) => Self::Completed,
                (true, true) => Self::None,
            }
        }
    }

    impl quickcheck::Arbitrary for ConnectRequest {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                transaction_id: TransactionId(I32::new(i32::arbitrary(g))),
            }
        }
    }

    impl quickcheck::Arbitrary for AnnounceRequest {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut peer_id_bytes = [0u8; 20];

            for byte in &mut peer_id_bytes {
                *byte = u8::arbitrary(g);
            }

            Self {
                connection_id: ConnectionId(I64::new(i64::arbitrary(g))),
                action_placeholder: AnnounceActionPlaceholder::default(),
                transaction_id: TransactionId(I32::new(i32::arbitrary(g))),
                info_hash: InfoHash::arbitrary(g),
                peer_id: PeerId(peer_id_bytes),
                bytes_downloaded: NumberOfBytes(I64::new(i64::arbitrary(g))),
                bytes_uploaded: NumberOfBytes(I64::new(i64::arbitrary(g))),
                bytes_left: NumberOfBytes(I64::new(i64::arbitrary(g))),
                event: AnnounceEvent::arbitrary(g).into(),
                ip_address: Ipv4AddrBytes::arbitrary(g),
                key: PeerKey::new(i32::arbitrary(g)),
                peers_wanted: NumberOfPeers(I32::new(i32::arbitrary(g))),
                port: Port::new(quickcheck::Arbitrary::arbitrary(g)),
            }
        }
    }

    impl quickcheck::Arbitrary for ScrapeRequest {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let info_hashes = (0..u8::arbitrary(g)).map(|_| InfoHash::arbitrary(g)).collect();

            Self {
                connection_id: ConnectionId(I64::new(i64::arbitrary(g))),
                transaction_id: TransactionId(I32::new(i32::arbitrary(g))),
                info_hashes,
            }
        }
    }

    #[quickcheck]
    fn it_should_preserve_connect_request_when_encoded_and_parsed(request: ConnectRequest) {
        // Arrange
        let request = Request::from(request);
        let mut bytes = Vec::new();

        // Act
        request.write_bytes(&mut bytes).unwrap();
        let parsed_request = Request::parse_bytes(&bytes, usize::MAX).unwrap();

        // Assert
        ::pretty_assertions::assert_eq!(request, parsed_request);
    }

    #[quickcheck]
    fn it_should_preserve_announce_request_when_encoded_and_parsed(request: AnnounceRequest) {
        // Arrange
        let request = Request::from(request);
        let mut bytes = Vec::new();

        // Act
        request.write_bytes(&mut bytes).unwrap();
        let parsed_request = Request::parse_bytes(&bytes, usize::MAX).unwrap();

        // Assert
        ::pretty_assertions::assert_eq!(request, parsed_request);
    }

    #[quickcheck]
    fn it_should_preserve_scrape_request_when_encoded_and_parsed(request: ScrapeRequest) -> TestResult {
        // Arrange
        if request.info_hashes.is_empty() {
            return TestResult::discard();
        }

        let request = Request::from(request);
        let mut bytes = Vec::new();

        // Act
        request.write_bytes(&mut bytes).unwrap();
        let parsed_request = Request::parse_bytes(&bytes, usize::MAX).unwrap();

        // Assert
        TestResult::from_bool(request == parsed_request)
    }

    #[test]
    fn it_should_not_panic_when_parsing_all_action_codes_at_all_packet_lengths() {
        for action in 0i32..4 {
            for max_scrape_info_hashes in 0..3 {
                for num_bytes in 0..256 {
                    // Arrange
                    let mut request_bytes = ::std::iter::repeat_n(0, num_bytes).collect::<Vec<_>>();

                    if let Some(action_bytes) = request_bytes.get_mut(8..12) {
                        action_bytes.copy_from_slice(&action.to_be_bytes());
                    }

                    // Act
                    let parsing_result =
                        std::panic::catch_unwind(|| Request::parse_bytes(&request_bytes, max_scrape_info_hashes));

                    // Assert
                    assert!(
                        parsing_result.is_ok(),
                        "parsing action {action} with {num_bytes} bytes and max scrape info hashes {max_scrape_info_hashes} panicked"
                    );
                }
            }
        }
    }

    #[test]
    fn it_should_reject_scrape_request_without_info_hashes() {
        // Arrange
        let mut request_bytes = Vec::new();

        request_bytes.extend(0i64.to_be_bytes());
        request_bytes.extend(2i32.to_be_bytes());
        request_bytes.extend(0i32.to_be_bytes());

        // Act
        let parsing_result = Request::parse_bytes(&request_bytes, 1);

        // Assert
        assert!(parsing_result.is_err());
    }

    /// UDP keeps 74 scrape info hashes because no more fit in one
    /// `MAX_PACKET_SIZE` datagram. Counts are literals so that changing the
    /// packet size or the limit fails here; see
    /// `docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md`.
    mod limiting_the_number_of_scrape_info_hashes {
        use zerocopy::network_endian::{I32, I64};

        use crate::common::{ConnectionId, InfoHash, MAX_PACKET_SIZE, TransactionId};
        use crate::request::{MAX_SCRAPE_INFO_HASHES, Request, ScrapeRequest};

        fn scrape_request_with_distinct_info_hashes(count: u8) -> ScrapeRequest {
            ScrapeRequest {
                connection_id: ConnectionId(I64::new(1)),
                transaction_id: TransactionId(I32::new(2)),
                info_hashes: (0..count).map(|index| InfoHash([index; 20])).collect(),
            }
        }

        fn encode(request: &ScrapeRequest) -> Vec<u8> {
            let mut bytes = Vec::new();
            request.write_bytes(&mut bytes).unwrap();
            bytes
        }

        fn parse_scrape_info_hashes(bytes: &[u8]) -> Vec<InfoHash> {
            match Request::parse_bytes(bytes, MAX_SCRAPE_INFO_HASHES).unwrap() {
                Request::Scrape(scrape_request) => scrape_request.info_hashes,
                other => panic!("expected a scrape request, got {other:?}"),
            }
        }

        #[test]
        fn it_should_fit_74_info_hashes_and_no_more_in_one_scrape_request_packet() {
            // Arrange
            let request_with_74 = scrape_request_with_distinct_info_hashes(74);
            let request_with_75 = scrape_request_with_distinct_info_hashes(75);

            // Act
            let size_with_74 = encode(&request_with_74).len();
            let size_with_75 = encode(&request_with_75).len();

            // Assert
            assert!(size_with_74 <= MAX_PACKET_SIZE, "74 info hashes take {size_with_74} bytes");
            assert!(size_with_75 > MAX_PACKET_SIZE, "75 info hashes take {size_with_75} bytes");
        }

        #[test]
        fn it_should_keep_all_74_info_hashes_when_the_request_has_exactly_74() {
            // Arrange
            let request = scrape_request_with_distinct_info_hashes(74);
            let bytes = encode(&request);

            // Act
            let info_hashes = parse_scrape_info_hashes(&bytes);

            // Assert
            assert_eq!(info_hashes, request.info_hashes);
        }

        #[test]
        fn it_should_keep_only_the_first_74_info_hashes_in_request_order_when_the_request_has_75() {
            // Arrange
            let request = scrape_request_with_distinct_info_hashes(75);
            let bytes = encode(&request);

            // Act
            let info_hashes = parse_scrape_info_hashes(&bytes);

            // Assert
            assert_eq!(info_hashes, request.info_hashes[..74]);
        }
    }
}
