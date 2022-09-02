use std::net::{IpAddr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use serde;
use serde::{Serialize};

use crate::protocol::common::{NumberOfBytesDef, AnnounceEventDef};
use crate::protocol::utils::ser_instant;
use crate::http::AnnounceRequest;
use crate::{Configuration, PeerId};
use crate::udp::AnnounceRequestWrapper;

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct TorrentPeer {
    pub peer_id: PeerId,
    pub peer_addr: SocketAddr,
    #[serde(serialize_with = "ser_instant")]
    pub updated: std::time::Instant,
    #[serde(with = "NumberOfBytesDef")]
    pub uploaded: NumberOfBytes,
    #[serde(with = "NumberOfBytesDef")]
    pub downloaded: NumberOfBytes,
    #[serde(with = "NumberOfBytesDef")]
    pub left: NumberOfBytes,
    #[serde(with = "AnnounceEventDef")]
    pub event: AnnounceEvent,
}

impl TorrentPeer {
    pub fn from_udp_announce_request(announce_request: &AnnounceRequestWrapper, config: &Configuration) -> Self {
        let peer_addr = Self::determine_peer_addr(announce_request.peer_addr.ip(), announce_request.announce_request.port.0, config);

        TorrentPeer {
            peer_id: PeerId(announce_request.announce_request.peer_id.0),
            peer_addr,
            updated: std::time::Instant::now(),
            uploaded: announce_request.announce_request.bytes_uploaded,
            downloaded: announce_request.announce_request.bytes_downloaded,
            left: announce_request.announce_request.bytes_left,
            event: announce_request.announce_request.event,
        }
    }

    pub fn from_http_announce_request(announce_request: &AnnounceRequest, config: &Configuration) -> Self {
        let peer_addr = Self::determine_peer_addr(announce_request.peer_addr, announce_request.port, config);

        let event: AnnounceEvent = if let Some(event) = &announce_request.event {
            match event.as_ref() {
                "started" => AnnounceEvent::Started,
                "stopped" => AnnounceEvent::Stopped,
                "completed" => AnnounceEvent::Completed,
                _ => AnnounceEvent::None
            }
        } else {
            AnnounceEvent::None
        };

        TorrentPeer {
            peer_id: announce_request.peer_id.clone(),
            peer_addr,
            updated: std::time::Instant::now(),
            uploaded: NumberOfBytes(announce_request.uploaded as i64),
            downloaded: NumberOfBytes(announce_request.downloaded as i64),
            left: NumberOfBytes(announce_request.left as i64),
            event,
        }
    }

    pub fn determine_peer_addr(remote_ip: IpAddr, port: u16, config: &Configuration) -> SocketAddr {
        if let IpAddr::V4(ip) = remote_ip {
            // replace 192.168.X.X and 127.0.0.1 with external ipv4
            if (config.replace_local_peer_ip_with_external_ip && config.external_ipv4.is_some()) && ip.is_private() || ip.is_loopback() {
                // external ipv4 address
                SocketAddr::new(IpAddr::from(config.external_ipv4.unwrap()), port)
            } else {
                // ipv4
                SocketAddr::new(remote_ip, port)
            }
        } else {
            // ipv6
            SocketAddr::new(remote_ip, port)
        }
    }

    pub fn is_seeder(&self) -> bool { self.left.0 <= 0 && self.event != AnnounceEvent::Stopped }
}
