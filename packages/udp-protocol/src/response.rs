// Copied from aquatic_udp_protocol 0.9.0 by Joakim Frostegard (greatest-ape).
// Source:     https://crates.io/crates/aquatic_udp_protocol/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License:    Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//
// This in-house crate started from the aquatic 0.9.0 sources that were previously vendored
// under packages/aquatic-udp-protocol.
use std::borrow::Cow;
use std::io::{self, Write};
use std::mem::size_of;

use byteorder::{NetworkEndian, WriteBytesExt};
use zerocopy::{FromBytes, IntoBytes};

#[cfg(test)]
use super::announce::AnnounceInterval;
use super::announce::{AnnounceResponse, AnnounceResponseFixedData};
use super::common::*;
use super::connect::ConnectResponse;
pub use super::scrape::{ScrapeResponse, TorrentScrapeStatistics};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Response {
    Connect(ConnectResponse),
    AnnounceIpv4(AnnounceResponse<Ipv4AddrBytes>),
    AnnounceIpv6(AnnounceResponse<Ipv6AddrBytes>),
    Scrape(ScrapeResponse),
    Error(ErrorResponse),
}

impl Response {
    #[inline]
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        match self {
            Self::Connect(r) => r.write_bytes(bytes),
            Self::AnnounceIpv4(r) => r.write_bytes(bytes),
            Self::AnnounceIpv6(r) => r.write_bytes(bytes),
            Self::Scrape(r) => r.write_bytes(bytes),
            Self::Error(r) => r.write_bytes(bytes),
        }
    }

    #[inline]
    pub fn parse_bytes(mut bytes: &[u8], ipv4: bool) -> Result<Self, io::Error> {
        let action = read_i32_ne(&mut bytes)?;

        match action.get() {
            0 => Ok(Self::Connect(
                ConnectResponse::read_from_prefix(bytes).map_err(|_| invalid_data())?.0,
            )),
            1 if ipv4 => {
                let fixed = AnnounceResponseFixedData::read_from_prefix(bytes)
                    .map_err(|_| invalid_data())?
                    .0;

                let peers = if let Some(bytes) = bytes.get(size_of::<AnnounceResponseFixedData>()..) {
                    let (chunks, remainder) = bytes.as_chunks::<{ size_of::<ResponsePeer<Ipv4AddrBytes>>() }>();

                    if !remainder.is_empty() {
                        return Err(invalid_data());
                    }

                    chunks
                        .iter()
                        .map(|chunk| {
                            ResponsePeer::<Ipv4AddrBytes>::read_from_prefix(chunk.as_slice())
                                .map(|(peer, _)| peer)
                                .map_err(|_| invalid_data())
                        })
                        .collect::<Result<Vec<_>, _>>()?
                } else {
                    Vec::new()
                };

                Ok(Self::AnnounceIpv4(AnnounceResponse { fixed, peers }))
            }
            1 if !ipv4 => {
                let fixed = AnnounceResponseFixedData::read_from_prefix(bytes)
                    .map_err(|_| invalid_data())?
                    .0;

                let peers = if let Some(bytes) = bytes.get(size_of::<AnnounceResponseFixedData>()..) {
                    let (chunks, remainder) = bytes.as_chunks::<{ size_of::<ResponsePeer<Ipv6AddrBytes>>() }>();

                    if !remainder.is_empty() {
                        return Err(invalid_data());
                    }

                    chunks
                        .iter()
                        .map(|chunk| {
                            ResponsePeer::<Ipv6AddrBytes>::read_from_prefix(chunk.as_slice())
                                .map(|(peer, _)| peer)
                                .map_err(|_| invalid_data())
                        })
                        .collect::<Result<Vec<_>, _>>()?
                } else {
                    Vec::new()
                };

                Ok(Self::AnnounceIpv6(AnnounceResponse { fixed, peers }))
            }
            2 => {
                let transaction_id = read_i32_ne(&mut bytes).map(TransactionId)?;

                let (chunks, remainder) = bytes.as_chunks::<{ size_of::<TorrentScrapeStatistics>() }>();

                if !remainder.is_empty() {
                    return Err(invalid_data());
                }

                let torrent_stats = chunks
                    .iter()
                    .map(|chunk| {
                        TorrentScrapeStatistics::read_from_prefix(chunk.as_slice())
                            .map(|(stats, _)| stats)
                            .map_err(|_| invalid_data())
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok((ScrapeResponse {
                    transaction_id,
                    torrent_stats,
                })
                .into())
            }
            3 => {
                let transaction_id = read_i32_ne(&mut bytes).map(TransactionId)?;
                let message = String::from_utf8_lossy(bytes).into_owned().into();

                Ok((ErrorResponse { transaction_id, message }).into())
            }
            _ => Err(invalid_data()),
        }
    }
}

impl From<ConnectResponse> for Response {
    fn from(r: ConnectResponse) -> Self {
        Self::Connect(r)
    }
}

impl From<AnnounceResponse<Ipv4AddrBytes>> for Response {
    fn from(r: AnnounceResponse<Ipv4AddrBytes>) -> Self {
        Self::AnnounceIpv4(r)
    }
}

impl From<AnnounceResponse<Ipv6AddrBytes>> for Response {
    fn from(r: AnnounceResponse<Ipv6AddrBytes>) -> Self {
        Self::AnnounceIpv6(r)
    }
}

impl From<ScrapeResponse> for Response {
    fn from(r: ScrapeResponse) -> Self {
        Self::Scrape(r)
    }
}

impl From<ErrorResponse> for Response {
    fn from(r: ErrorResponse) -> Self {
        Self::Error(r)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ErrorResponse {
    pub transaction_id: TransactionId,
    pub message: Cow<'static, str>,
}

impl ErrorResponse {
    #[inline]
    pub fn write_bytes(&self, bytes: &mut impl Write) -> Result<(), io::Error> {
        bytes.write_i32::<NetworkEndian>(3)?;
        bytes.write_all(self.transaction_id.as_bytes())?;
        bytes.write_all(self.message.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;
    use zerocopy::network_endian::{I32, I64};

    use super::*;

    impl quickcheck::Arbitrary for Ipv4AddrBytes {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self([u8::arbitrary(g), u8::arbitrary(g), u8::arbitrary(g), u8::arbitrary(g)])
        }
    }

    impl quickcheck::Arbitrary for Ipv6AddrBytes {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut bytes = [0; 16];

            for byte in bytes.iter_mut() {
                *byte = u8::arbitrary(g)
            }

            Self(bytes)
        }
    }

    impl quickcheck::Arbitrary for TorrentScrapeStatistics {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                seeders: NumberOfPeers(I32::new(i32::arbitrary(g))),
                completed: NumberOfDownloads(I32::new(i32::arbitrary(g))),
                leechers: NumberOfPeers(I32::new(i32::arbitrary(g))),
            }
        }
    }

    impl quickcheck::Arbitrary for ConnectResponse {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                connection_id: ConnectionId(I64::new(i64::arbitrary(g))),
                transaction_id: TransactionId(I32::new(i32::arbitrary(g))),
            }
        }
    }

    impl<I: Ip + quickcheck::Arbitrary> quickcheck::Arbitrary for AnnounceResponse<I> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let peers = (0..u8::arbitrary(g)).map(|_| ResponsePeer::arbitrary(g)).collect();

            Self {
                fixed: AnnounceResponseFixedData {
                    transaction_id: TransactionId(I32::new(i32::arbitrary(g))),
                    announce_interval: AnnounceInterval(I32::new(i32::arbitrary(g))),
                    leechers: NumberOfPeers(I32::new(i32::arbitrary(g))),
                    seeders: NumberOfPeers(I32::new(i32::arbitrary(g))),
                },
                peers,
            }
        }
    }

    impl quickcheck::Arbitrary for ScrapeResponse {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let torrent_stats = (0..u8::arbitrary(g)).map(|_| TorrentScrapeStatistics::arbitrary(g)).collect();

            Self {
                transaction_id: TransactionId(I32::new(i32::arbitrary(g))),
                torrent_stats,
            }
        }
    }

    fn same_after_conversion(response: Response, ipv4: bool) -> bool {
        let mut buf = Vec::new();

        response.write_bytes(&mut buf).unwrap();
        let r2 = Response::parse_bytes(&buf[..], ipv4).unwrap();

        let success = response == r2;

        if !success {
            ::pretty_assertions::assert_eq!(response, r2);
        }

        success
    }

    #[quickcheck]
    fn test_connect_response_convert_identity(response: ConnectResponse) -> bool {
        same_after_conversion(response.into(), true)
    }

    #[quickcheck]
    fn test_announce_response_ipv4_convert_identity(response: AnnounceResponse<Ipv4AddrBytes>) -> bool {
        same_after_conversion(response.into(), true)
    }

    #[quickcheck]
    fn test_announce_response_ipv6_convert_identity(response: AnnounceResponse<Ipv6AddrBytes>) -> bool {
        same_after_conversion(response.into(), false)
    }

    #[quickcheck]
    fn test_scrape_response_convert_identity(response: ScrapeResponse) -> bool {
        same_after_conversion(response.into(), true)
    }
}
