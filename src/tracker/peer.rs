use std::net::{IpAddr, SocketAddr};

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use serde;
use serde::Serialize;

use crate::http::AnnounceRequest;
use crate::protocol::clock::clock::{DefaultClock, DurationSinceUnixEpoch, Time};
use crate::protocol::common::{AnnounceEventDef, NumberOfBytesDef};
use crate::protocol::utils::ser_unix_time_value;
use crate::PeerId;

#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct TorrentPeer {
    pub peer_id: PeerId,
    pub peer_addr: SocketAddr,
    #[serde(serialize_with = "ser_unix_time_value")]
    pub updated: DurationSinceUnixEpoch,
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
    pub fn from_udp_announce_request(
        announce_request: &aquatic_udp_protocol::AnnounceRequest,
        remote_ip: IpAddr,
        host_opt_ip: Option<IpAddr>,
    ) -> Self {
        let peer_addr = TorrentPeer::peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip, host_opt_ip, announce_request.port.0);

        TorrentPeer {
            peer_id: PeerId(announce_request.peer_id.0),
            peer_addr,
            updated: DefaultClock::now(),
            uploaded: announce_request.bytes_uploaded,
            downloaded: announce_request.bytes_downloaded,
            left: announce_request.bytes_left,
            event: announce_request.event,
        }
    }

    pub fn from_http_announce_request(
        announce_request: &AnnounceRequest,
        remote_ip: IpAddr,
        host_opt_ip: Option<IpAddr>,
    ) -> Self {
        let peer_addr = TorrentPeer::peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip, host_opt_ip, announce_request.port);

        let event: AnnounceEvent = if let Some(event) = &announce_request.event {
            match event.as_ref() {
                "started" => AnnounceEvent::Started,
                "stopped" => AnnounceEvent::Stopped,
                "completed" => AnnounceEvent::Completed,
                _ => AnnounceEvent::None,
            }
        } else {
            AnnounceEvent::None
        };

        TorrentPeer {
            peer_id: announce_request.peer_id.clone(),
            peer_addr,
            updated: DefaultClock::now(),
            uploaded: NumberOfBytes(announce_request.uploaded as i64),
            downloaded: NumberOfBytes(announce_request.downloaded as i64),
            left: NumberOfBytes(announce_request.left as i64),
            event,
        }
    }

    // potentially substitute localhost ip with external ip
    pub fn peer_addr_from_ip_and_port_and_opt_host_ip(remote_ip: IpAddr, host_opt_ip: Option<IpAddr>, port: u16) -> SocketAddr {
        if remote_ip.is_loopback() && host_opt_ip.is_some() {
            SocketAddr::new(host_opt_ip.unwrap(), port)
        } else {
            SocketAddr::new(remote_ip, port)
        }
    }

    pub fn is_seeder(&self) -> bool {
        self.left.0 <= 0 && self.event != AnnounceEvent::Stopped
    }
}
