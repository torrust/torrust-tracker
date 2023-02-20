use std::net::IpAddr;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{self, Deserialize, Serialize};

use crate::tracker::{self, AnnounceResponse};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Announce {
    pub interval: u32,
    #[serde(rename = "min interval")]
    pub interval_min: u32,
    pub complete: u32,
    pub incomplete: u32,
    pub peers: Vec<Peer>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Peer {
    pub peer_id: String,
    pub ip: IpAddr,
    pub port: u16,
}

impl From<tracker::peer::Peer> for Peer {
    fn from(peer: tracker::peer::Peer) -> Self {
        Peer {
            peer_id: peer.peer_id.to_string(),
            ip: peer.peer_addr.ip(),
            port: peer.peer_addr.port(),
        }
    }
}

impl Announce {
    /// # Panics
    ///
    /// It would panic if the `Announce` struct contained an inappropriate type.
    #[must_use]
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }
}

impl IntoResponse for Announce {
    fn into_response(self) -> Response {
        (StatusCode::OK, self.write()).into_response()
    }
}

impl From<AnnounceResponse> for Announce {
    fn from(domain_announce_response: AnnounceResponse) -> Self {
        let peers: Vec<Peer> = domain_announce_response.peers.iter().map(|peer| Peer::from(*peer)).collect();

        Self {
            interval: domain_announce_response.interval,
            interval_min: domain_announce_response.interval_min,
            complete: domain_announce_response.swam_stats.seeders,
            incomplete: domain_announce_response.swam_stats.leechers,
            peers,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::net::IpAddr;
    use std::str::FromStr;

    use super::{Announce, Peer};

    #[test]
    fn announce_response_can_be_bencoded() {
        let response = Announce {
            interval: 1,
            interval_min: 2,
            complete: 3,
            incomplete: 4,
            peers: vec![Peer {
                peer_id: "-qB00000000000000001".to_string(),
                ip: IpAddr::from_str("127.0.0.1").unwrap(),
                port: 8080,
            }],
        };

        // cspell:disable-next-line
        assert_eq!(response.write(), "d8:completei3e10:incompletei4e8:intervali1e12:min intervali2e5:peersld2:ip9:127.0.0.17:peer_id20:-qB000000000000000014:porti8080eeee");
    }
}
