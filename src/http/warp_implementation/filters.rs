use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::panic::Location;
use std::str::FromStr;
use std::sync::Arc;

use warp::{reject, Filter, Rejection};

use super::error::Error;
use super::filter_helpers::maybe_rightmost_forwarded_ip;
use super::{request, WebResult};
use crate::http::percent_encoding::{percent_decode_info_hash, percent_decode_peer_id};
use crate::protocol::common::MAX_SCRAPE_TORRENTS;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::auth::KeyId;
use crate::tracker::{self, peer};

/// Pass Arc<tracker::TorrentTracker> along
#[must_use]
pub fn with_tracker(
    tracker: Arc<tracker::Tracker>,
) -> impl Filter<Extract = (Arc<tracker::Tracker>,), Error = Infallible> + Clone {
    warp::any().map(move || tracker.clone())
}

/// Check for infoHash
#[must_use]
pub fn with_info_hash() -> impl Filter<Extract = (Vec<InfoHash>,), Error = Rejection> + Clone {
    warp::filters::query::raw().and_then(|q| async move { info_hashes(&q) })
}

/// Check for `PeerId`
#[must_use]
pub fn with_peer_id() -> impl Filter<Extract = (peer::Id,), Error = Rejection> + Clone {
    warp::filters::query::raw().and_then(|q| async move { peer_id(&q) })
}

/// Pass Arc<tracker::TorrentTracker> along
#[must_use]
pub fn with_auth_key_id() -> impl Filter<Extract = (Option<KeyId>,), Error = Infallible> + Clone {
    warp::path::param::<String>()
        .map(|key: String| {
            let key_id = KeyId::from_str(&key);
            match key_id {
                Ok(id) => Some(id),
                Err(_) => None,
            }
        })
        .or_else(|_| async { Ok::<(Option<KeyId>,), Infallible>((None,)) })
}

/// Check for `PeerAddress`
#[must_use]
pub fn with_peer_addr(on_reverse_proxy: bool) -> impl Filter<Extract = (IpAddr,), Error = Rejection> + Clone {
    warp::addr::remote()
        .and(warp::header::optional::<String>("X-Forwarded-For"))
        .map(move |remote_addr: Option<SocketAddr>, x_forwarded_for: Option<String>| {
            (on_reverse_proxy, remote_addr, x_forwarded_for)
        })
        .and_then(|q| async move { peer_addr(q) })
}

/// Check for `request::Announce`
#[must_use]
pub fn with_announce_request(on_reverse_proxy: bool) -> impl Filter<Extract = (request::Announce,), Error = Rejection> + Clone {
    warp::filters::query::query::<request::AnnounceQuery>()
        .and(with_info_hash())
        .and(with_peer_id())
        .and(with_peer_addr(on_reverse_proxy))
        .and_then(|q, r, s, t| async move { announce_request(q, &r, s, t) })
}

/// Check for `ScrapeRequest`
#[must_use]
pub fn with_scrape_request(on_reverse_proxy: bool) -> impl Filter<Extract = (request::Scrape,), Error = Rejection> + Clone {
    warp::any()
        .and(with_info_hash())
        .and(with_peer_addr(on_reverse_proxy))
        .and_then(|q, r| async move { scrape_request(q, r) })
}

/// Parse `InfoHash` from raw query string
#[allow(clippy::ptr_arg)]
fn info_hashes(raw_query: &String) -> WebResult<Vec<InfoHash>> {
    let split_raw_query: Vec<&str> = raw_query.split('&').collect();
    let mut info_hashes: Vec<InfoHash> = Vec::new();

    for v in split_raw_query {
        if v.contains("info_hash") {
            // get raw percent encoded infohash
            let raw_info_hash = v.split('=').collect::<Vec<&str>>()[1];

            let info_hash = percent_decode_info_hash(raw_info_hash);

            if let Ok(ih) = info_hash {
                info_hashes.push(ih);
            }
        }
    }

    if info_hashes.len() > MAX_SCRAPE_TORRENTS as usize {
        Err(reject::custom(Error::TwoManyInfoHashes {
            location: Location::caller(),
            message: format! {"found: {}, but limit is: {}",info_hashes.len(), MAX_SCRAPE_TORRENTS},
        }))
    } else if info_hashes.is_empty() {
        Err(reject::custom(Error::EmptyInfoHash {
            location: Location::caller(),
        }))
    } else {
        Ok(info_hashes)
    }
}

/// Parse `PeerId` from raw query string
#[allow(clippy::ptr_arg)]
fn peer_id(raw_query: &String) -> WebResult<peer::Id> {
    // put all query params in a vec
    let split_raw_query: Vec<&str> = raw_query.split('&').collect();

    let mut peer_id: Option<peer::Id> = None;

    for v in split_raw_query {
        // look for the peer_id param
        if v.contains("peer_id") {
            // get raw percent encoded peer id
            let raw_peer_id = v.split('=').collect::<Vec<&str>>()[1];

            if let Ok(id) = percent_decode_peer_id(raw_peer_id) {
                peer_id = Some(id);
            } else {
                return Err(reject::custom(Error::InvalidPeerId {
                    location: Location::caller(),
                }));
            }

            break;
        }
    }

    match peer_id {
        Some(id) => Ok(id),
        None => Err(reject::custom(Error::InvalidPeerId {
            location: Location::caller(),
        })),
    }
}

/// Get peer IP from HTTP client IP  or X-Forwarded-For HTTP header
fn peer_addr(
    (on_reverse_proxy, remote_client_ip, maybe_x_forwarded_for): (bool, Option<SocketAddr>, Option<String>),
) -> WebResult<IpAddr> {
    if on_reverse_proxy {
        if maybe_x_forwarded_for.is_none() {
            return Err(reject::custom(Error::AddressNotFound {
                location: Location::caller(),
                message: "must have a x-forwarded-for when using a reverse proxy".to_string(),
            }));
        }

        let x_forwarded_for = maybe_x_forwarded_for.unwrap();

        maybe_rightmost_forwarded_ip(&x_forwarded_for).map_err(|e| {
            reject::custom(Error::AddressNotFound {
                location: Location::caller(),
                message: format!("on remote proxy and unable to parse the last x-forwarded-ip: `{e}`, from `{x_forwarded_for}`"),
            })
        })
    } else if remote_client_ip.is_none() {
        return Err(reject::custom(Error::AddressNotFound {
            location: Location::caller(),
            message: "neither on have remote address or on a reverse proxy".to_string(),
        }));
    } else {
        return Ok(remote_client_ip.unwrap().ip());
    }
}

/// Parse `AnnounceRequest` from raw `AnnounceRequestQuery`, `InfoHash` and Option<SocketAddr>
#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::ptr_arg)]
fn announce_request(
    announce_request_query: request::AnnounceQuery,
    info_hashes: &Vec<InfoHash>,
    peer_id: peer::Id,
    peer_addr: IpAddr,
) -> WebResult<request::Announce> {
    Ok(request::Announce {
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
#[allow(clippy::unnecessary_wraps)]
fn scrape_request(info_hashes: Vec<InfoHash>, peer_addr: IpAddr) -> WebResult<request::Scrape> {
    Ok(request::Scrape { info_hashes, peer_addr })
}
