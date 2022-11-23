use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

use warp::{reject, Filter, Rejection};

use super::errors::ServerError;
use super::request::{Announce, AnnounceRequestQuery, Scrape};
use super::WebResult;
use crate::protocol::common::{InfoHash, PeerId, MAX_SCRAPE_TORRENTS};
use crate::tracker::key::AuthKey;
use crate::tracker::TorrentTracker;

/// Pass Arc<TorrentTracker> along
#[must_use]
pub fn with_tracker(tracker: Arc<TorrentTracker>) -> impl Filter<Extract = (Arc<TorrentTracker>,), Error = Infallible> + Clone {
    warp::any().map(move || tracker.clone())
}

/// Check for infoHash
pub fn with_info_hash() -> impl Filter<Extract = (Vec<InfoHash>,), Error = Rejection> + Clone {
    warp::filters::query::raw().and_then(info_hashes)
}

/// Check for `PeerId`
pub fn with_peer_id() -> impl Filter<Extract = (PeerId,), Error = Rejection> + Clone {
    warp::filters::query::raw().and_then(peer_id)
}

/// Pass Arc<TorrentTracker> along
#[must_use]
pub fn with_auth_key() -> impl Filter<Extract = (Option<AuthKey>,), Error = Infallible> + Clone {
    warp::path::param::<String>()
        .map(|key: String| AuthKey::from_string(&key))
        .or_else(|_| async { Ok::<(Option<AuthKey>,), Infallible>((None,)) })
}

/// Check for `PeerAddress`
pub fn with_peer_addr(on_reverse_proxy: bool) -> impl Filter<Extract = (IpAddr,), Error = Rejection> + Clone {
    warp::addr::remote()
        .and(warp::header::optional::<String>("X-Forwarded-For"))
        .map(move |remote_addr: Option<SocketAddr>, x_forwarded_for: Option<String>| {
            (on_reverse_proxy, remote_addr, x_forwarded_for)
        })
        .and_then(peer_addr)
}

/// Check for `AnnounceRequest`
pub fn with_announce_request(on_reverse_proxy: bool) -> impl Filter<Extract = (Announce,), Error = Rejection> + Clone {
    warp::filters::query::query::<AnnounceRequestQuery>()
        .and(with_info_hash())
        .and(with_peer_id())
        .and(with_peer_addr(on_reverse_proxy))
        .and_then(announce_request)
}

/// Check for `ScrapeRequest`
pub fn with_scrape_request(on_reverse_proxy: bool) -> impl Filter<Extract = (Scrape,), Error = Rejection> + Clone {
    warp::any()
        .and(with_info_hash())
        .and(with_peer_addr(on_reverse_proxy))
        .and_then(scrape_request)
}

/// Parse `InfoHash` from raw query string
async fn info_hashes(raw_query: String) -> WebResult<Vec<InfoHash>> {
    let split_raw_query: Vec<&str> = raw_query.split('&').collect();
    let mut info_hashes: Vec<InfoHash> = Vec::new();

    for v in split_raw_query {
        if v.contains("info_hash") {
            let raw_info_hash = v.split('=').collect::<Vec<&str>>()[1];
            let info_hash_bytes = percent_encoding::percent_decode_str(raw_info_hash).collect::<Vec<u8>>();
            let info_hash = InfoHash::from_str(&hex::encode(info_hash_bytes));
            if let Ok(ih) = info_hash {
                info_hashes.push(ih);
            }
        }
    }

    if info_hashes.len() > MAX_SCRAPE_TORRENTS as usize {
        Err(reject::custom(ServerError::ExceededInfoHashLimit))
    } else if info_hashes.is_empty() {
        Err(reject::custom(ServerError::InvalidInfoHash))
    } else {
        Ok(info_hashes)
    }
}

/// Parse `PeerId` from raw query string
async fn peer_id(raw_query: String) -> WebResult<PeerId> {
    // put all query params in a vec
    let split_raw_query: Vec<&str> = raw_query.split('&').collect();

    let mut peer_id: Option<PeerId> = None;

    for v in split_raw_query {
        // look for the peer_id param
        if v.contains("peer_id") {
            // get raw percent_encoded peer_id
            let raw_peer_id = v.split('=').collect::<Vec<&str>>()[1];

            // decode peer_id
            let peer_id_bytes = percent_encoding::percent_decode_str(raw_peer_id).collect::<Vec<u8>>();

            // peer_id must be 20 bytes
            if peer_id_bytes.len() != 20 {
                return Err(reject::custom(ServerError::InvalidPeerId));
            }

            // clone peer_id_bytes into fixed length array
            let mut byte_arr: [u8; 20] = Default::default();
            byte_arr.clone_from_slice(peer_id_bytes.as_slice());

            peer_id = Some(PeerId(byte_arr));
            break;
        }
    }

    if peer_id.is_none() {
        Err(reject::custom(ServerError::InvalidPeerId))
    } else {
        Ok(peer_id.unwrap())
    }
}

/// Get `PeerAddress` from `RemoteAddress` or Forwarded
async fn peer_addr(
    (on_reverse_proxy, remote_addr, x_forwarded_for): (bool, Option<SocketAddr>, Option<String>),
) -> WebResult<IpAddr> {
    if !on_reverse_proxy && remote_addr.is_none() {
        return Err(reject::custom(ServerError::AddressNotFound));
    }

    if on_reverse_proxy && x_forwarded_for.is_none() {
        return Err(reject::custom(ServerError::AddressNotFound));
    }

    match on_reverse_proxy {
        true => {
            let mut x_forwarded_for_raw = x_forwarded_for.unwrap();
            // remove whitespace chars
            x_forwarded_for_raw.retain(|c| !c.is_whitespace());
            // get all forwarded ip's in a vec
            let x_forwarded_ips: Vec<&str> = x_forwarded_for_raw.split(',').collect();
            // set client ip to last forwarded ip
            let x_forwarded_ip = *x_forwarded_ips.last().unwrap();

            IpAddr::from_str(x_forwarded_ip).map_err(|_| reject::custom(ServerError::AddressNotFound))
        }
        false => Ok(remote_addr.unwrap().ip()),
    }
}

/// Parse `AnnounceRequest` from raw `AnnounceRequestQuery`, `InfoHash` and Option<SocketAddr>
async fn announce_request(
    announce_request_query: AnnounceRequestQuery,
    info_hashes: Vec<InfoHash>,
    peer_id: PeerId,
    peer_addr: IpAddr,
) -> WebResult<Announce> {
    Ok(Announce {
        info_hash: info_hashes[0],
        peer_addr,
        downloaded: announce_request_query.downloaded.unwrap_or(0),
        uploaded: announce_request_query.uploaded.unwrap_or(0),
        peer_id,
        port: announce_request_query.port,
        left: announce_request_query.left.unwrap_or(0),
        event: announce_request_query.event,
        compact: announce_request_query.compact,
    })
}

/// Parse `ScrapeRequest` from `InfoHash`
async fn scrape_request(info_hashes: Vec<InfoHash>, peer_addr: IpAddr) -> WebResult<Scrape> {
    Ok(Scrape { info_hashes, peer_addr })
}
