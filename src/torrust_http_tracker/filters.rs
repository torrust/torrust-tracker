use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use warp::{Filter, reject, Rejection};
use crate::{InfoHash, MAX_SCRAPE_TORRENTS, PeerId, TorrentTracker};
use crate::key_manager::AuthKey;
use crate::torrust_http_tracker::{AnnounceRequest, AnnounceRequestQuery, ScrapeRequest, ServerError, WebResult};

/// Pass Arc<TorrentTracker> along
pub fn with_tracker(tracker: Arc<TorrentTracker>) -> impl Filter<Extract = (Arc<TorrentTracker>,), Error = Infallible> + Clone {
    warp::any()
        .map(move || tracker.clone())
}

/// Check for infoHash
pub fn with_info_hash() -> impl Filter<Extract = (Vec<InfoHash>,), Error = Rejection> + Clone {
    warp::filters::query::raw()
        .and_then(info_hashes)
}

/// Parse InfoHash from raw query string
async fn info_hashes(raw_query: String) -> WebResult<Vec<InfoHash>> {
    let split_raw_query: Vec<&str> = raw_query.split("&").collect();
    let mut info_hashes: Vec<InfoHash> = Vec::new();

    for v in split_raw_query {
        if v.contains("info_hash") {
            let raw_info_hash = v.split("=").collect::<Vec<&str>>()[1];
            let info_hash_bytes = percent_encoding::percent_decode_str(raw_info_hash).collect::<Vec<u8>>();
            let info_hash = InfoHash::from_str(&hex::encode(info_hash_bytes));
            if let Ok(ih) = info_hash {
                info_hashes.push(ih);
            }
        }
    }

    if info_hashes.len() > MAX_SCRAPE_TORRENTS as usize {
        Err(reject::custom(ServerError::ExceededInfoHashLimit))
    } else if info_hashes.len() < 1 {
        Err(reject::custom(ServerError::InvalidInfoHash))
    } else {
        Ok(info_hashes)
    }
}

/// Check for PeerId
pub fn with_peer_id() -> impl Filter<Extract = (PeerId,), Error = Rejection> + Clone {
    warp::filters::query::raw()
        .and_then(peer_id)
}

/// Parse PeerId from raw query string
async fn peer_id(raw_query: String) -> WebResult<PeerId> {
    // put all query params in a vec
    let split_raw_query: Vec<&str> = raw_query.split("&").collect();

    let mut peer_id: Option<PeerId> = None;

    for v in split_raw_query {
        // look for the peer_id param
        if v.contains("peer_id") {
            // get raw percent_encoded peer_id
            let raw_peer_id = v.split("=").collect::<Vec<&str>>()[1];

            // decode peer_id
            let peer_id_bytes = percent_encoding::percent_decode_str(raw_peer_id).collect::<Vec<u8>>();

            // peer_id must be 20 bytes
            if peer_id_bytes.len() > 20 {
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

/// Pass Arc<TorrentTracker> along
pub fn with_auth_key() -> impl Filter<Extract = (Option<AuthKey>,), Error = Infallible> + Clone {
    warp::path::param::<String>()
        .map(|key: String| {
            AuthKey::from_string(&key)
        })
        .or_else(|_| async {
            Ok::<(Option<AuthKey>,), Infallible>((None,))
        })
}

/// Check for AnnounceRequest
pub fn with_announce_request() -> impl Filter<Extract = (AnnounceRequest,), Error = Rejection> + Clone {
    warp::filters::query::query::<AnnounceRequestQuery>()
        .and(with_info_hash())
        .and(with_peer_id())
        .and(warp::addr::remote())
        .and(warp::header::optional::<String>("X-Forwarded-For"))
        .and_then(announce_request)
}

/// Parse AnnounceRequest from raw AnnounceRequestQuery, InfoHash and Option<SocketAddr>
async fn announce_request(announce_request_query: AnnounceRequestQuery, info_hashes: Vec<InfoHash>, peer_id: PeerId, remote_addr: Option<SocketAddr>, forwarded_for: Option<String>) -> WebResult<AnnounceRequest> {
    if remote_addr.is_none() { return Err(reject::custom(ServerError::AddressNotFound)) }

    // get first forwarded ip
    let forwarded_ip = match forwarded_for {
        None => None,
        Some(forwarded_for_str) => {
            forwarded_for_str.split(",").next()
                .and_then(|ip_str| IpAddr::from_str(ip_str).ok())
        }
    };

    Ok(AnnounceRequest {
        info_hash: info_hashes[0],
        peer_addr: remote_addr.unwrap(),
        forwarded_ip,
        downloaded: announce_request_query.downloaded,
        uploaded: announce_request_query.uploaded,
        peer_id,
        port: announce_request_query.port,
        left: announce_request_query.left,
        event: announce_request_query.event,
        compact: announce_request_query.compact
    })
}

/// Check for ScrapeRequest
pub fn with_scrape_request() -> impl Filter<Extract = (ScrapeRequest,), Error = Rejection> + Clone {
    warp::any()
        .and(with_info_hash())
        .and_then(scrape_request)
}

/// Parse ScrapeRequest from InfoHash
async fn scrape_request(info_hashes: Vec<InfoHash>) -> WebResult<ScrapeRequest> {
    Ok(ScrapeRequest {
        info_hashes,
    })
}
