use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{SocketAddr};
use std::sync::Arc;
use std::str::FromStr;
use log::{debug};
use warp::{reply::Reply, Filter, Rejection, reject};
use warp::http::{Response, StatusCode};
use super::{AnnounceResponse};
use crate::tracker::{TorrentTracker};
use crate::{TorrentPeer, TorrentStats};
use crate::key_manager::AuthKey;
use crate::common::*;
use crate::torrust_http_tracker::request::AnnounceRequestQuery;
use crate::torrust_http_tracker::{AnnounceRequest, ErrorResponse, Peer, ScrapeRequest, ScrapeResponse, ScrapeResponseEntry};
use crate::torrust_http_tracker::errors::ServerError;
use crate::utils::url_encode_bytes;

type WebResult<T> = std::result::Result<T, Rejection>;

/// Server that listens on HTTP, needs a TorrentTracker
#[derive(Clone)]
pub struct HttpServer {
    tracker: Arc<TorrentTracker>,
}

impl HttpServer {
    pub fn new(tracker: Arc<TorrentTracker>) -> HttpServer {
        HttpServer {
            tracker
        }
    }

    /// Start the HttpServer
    pub async fn start(&self, socket_addr: SocketAddr) {
        warp::serve(routes(self.tracker.clone()))
            .run(socket_addr).await;
    }

    /// Start the HttpServer in TLS mode
    pub async fn start_tls(&self, socket_addr: SocketAddr, ssl_cert_path: &str, ssl_key_path: &str) {
        warp::serve(routes(self.tracker.clone()))
            .tls()
            .cert_path(ssl_cert_path)
            .key_path(ssl_key_path)
            .run(socket_addr).await;
    }
}

/// All routes
fn routes(tracker: Arc<TorrentTracker>,) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    announce(tracker.clone())
        .or(scrape(tracker.clone()))
        .recover(handle_error)
}

/// Pass Arc<TorrentTracker> along
fn with_tracker(tracker: Arc<TorrentTracker>) -> impl Filter<Extract = (Arc<TorrentTracker>,), Error = Infallible> + Clone {
    warp::any()
        .map(move || tracker.clone())
}

/// Check for infoHash
fn with_info_hash() -> impl Filter<Extract = (Vec<InfoHash>,), Error = Rejection> + Clone {
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

/// Pass Arc<TorrentTracker> along
fn with_auth_key() -> impl Filter<Extract = (Option<AuthKey>,), Error = warp::Rejection> + Clone {
    warp::path::param::<String>()
        .map(|key_string: String| {
            AuthKey::from_string(&key_string)
        })
}

/// Check for AnnounceRequest
fn with_announce_request() -> impl Filter<Extract = (AnnounceRequest,), Error = Rejection> + Clone {
    warp::filters::query::query::<AnnounceRequestQuery>()
        .and(with_info_hash())
        .and(warp::addr::remote())
        .and_then(announce_request)
}

/// Parse AnnounceRequest from raw AnnounceRequestQuery, InfoHash and Option<SocketAddr>
async fn announce_request(announce_request_query: AnnounceRequestQuery, info_hashes: Vec<InfoHash>, remote_addr: Option<SocketAddr>) -> WebResult<AnnounceRequest> {
    if remote_addr.is_none() { return Err(reject::custom(ServerError::AddressNotFound)) }

    Ok(AnnounceRequest {
        info_hash: info_hashes[0],
        peer_addr: remote_addr.unwrap(),
        downloaded: announce_request_query.downloaded,
        uploaded: announce_request_query.uploaded,
        peer_id: announce_request_query.peer_id,
        port: announce_request_query.port,
        left: announce_request_query.left,
        event: announce_request_query.event,
        compact: announce_request_query.compact
    })
}

/// Check for ScrapeRequest
fn with_scrape_request() -> impl Filter<Extract = (ScrapeRequest,), Error = Rejection> + Clone {
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

/// Authenticate AnnounceRequest using optional AuthKey
async fn authenticate(info_hash: &InfoHash, auth_key: &Option<AuthKey>, tracker: Arc<TorrentTracker>) -> Result<(), ServerError> {
    match tracker.authenticate_request(info_hash, auth_key).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerError::from(e))
    }
}

/// GET /announce/<key>
fn announce(tracker: Arc<TorrentTracker>,) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::path("announce")
        .and(warp::filters::method::get())
        .and(with_announce_request())
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_announce)
}

/// GET /scrape/<key>
fn scrape(tracker: Arc<TorrentTracker>,) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::path("scrape")
        .and(warp::filters::method::get())
        .and(with_scrape_request())
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_scrape)
}

/// Handle announce request
pub async fn handle_announce(announce_request: AnnounceRequest, auth_key: Option<AuthKey>, tracker: Arc<TorrentTracker>,) -> WebResult<impl Reply> {
    if let Err(e) = authenticate(&announce_request.info_hash, &auth_key, tracker.clone()).await {
        return Err(reject::custom(e))
    }

    let peer = TorrentPeer::from_http_announce_request(&announce_request, announce_request.peer_addr, tracker.config.get_ext_ip());

    match tracker.update_torrent_with_peer_and_get_stats(&announce_request.info_hash, &peer).await {
        Err(e) => Err(reject::custom(ServerError::from(e))),
        Ok(torrent_stats) => {
            // get all peers excluding the client_addr
            let peers = tracker.get_torrent_peers(&announce_request.info_hash, &peer.peer_addr).await;
            if peers.is_none() { return Err(reject::custom(ServerError::NoPeersFound)) }

            // success response
            let announce_interval = tracker.config.http_tracker.announce_interval;
            send_announce_response(&announce_request, torrent_stats, peers.unwrap(), announce_interval)
        }
    }
}

/// Handle scrape request
pub async fn handle_scrape(scrape_request: ScrapeRequest, auth_key: Option<AuthKey>, tracker: Arc<TorrentTracker>,) -> WebResult<impl Reply> {
    let mut files: HashMap<String, ScrapeResponseEntry> = HashMap::new();
    let db = tracker.get_torrents().await;

    for info_hash in scrape_request.info_hashes.iter() {
        // authenticate every info_hash
        if authenticate(info_hash, &auth_key, tracker.clone()).await.is_err() { continue }

        let scrape_entry = match db.get(&info_hash) {
            Some(torrent_info) => {
                let (seeders, completed, leechers) = torrent_info.get_stats();

                ScrapeResponseEntry { complete: seeders, downloaded: completed, incomplete: leechers }
            }
            None => {
                ScrapeResponseEntry { complete: 0, downloaded: 0, incomplete: 0 }
            }
        };

        if let Ok(encoded_info_hash) = url_encode_bytes(&info_hash.0) {
            files.insert(encoded_info_hash, scrape_entry);
        }
    }

    send_scrape_response(files)
}

/// Send announce response
fn send_announce_response(announce_request: &AnnounceRequest, torrent_stats: TorrentStats, peers: Vec<TorrentPeer>, interval: u32) -> WebResult<impl Reply> {
    let http_peers: Vec<Peer> = peers.iter().map(|peer| Peer {
        peer_id: String::from_utf8_lossy(&peer.peer_id.0).to_string(),
        ip: peer.peer_addr.ip(),
        port: peer.peer_addr.port()
    }).collect();

    let res = AnnounceResponse {
        interval,
        complete: torrent_stats.seeders,
        incomplete: torrent_stats.leechers,
        peers: http_peers
    };

    // check for compact response request
    if let Some(1) = announce_request.compact {
        match res.write_compact() {
            Ok(body) => Ok(Response::new(body)),
            Err(_) => Err(reject::custom(ServerError::InternalServerError))
        }
    } else {
        Ok(Response::new(res.write().into()))
    }
}

/// Send scrape response
fn send_scrape_response(files: HashMap<String, ScrapeResponseEntry>) -> WebResult<impl Reply> {
    Ok(Response::new(ScrapeResponse { files }.write()))
}

/// Handle all server errors and send error reply
async fn handle_error(r: Rejection) -> std::result::Result<impl Reply, Infallible> {
    if let Some(e) = r.find::<ServerError>() {
        debug!("{:?}", e);
        let reply = warp::reply::json(&ErrorResponse { failure_reason: e.to_string() });
        Ok(warp::reply::with_status(reply, StatusCode::BAD_REQUEST))
    } else {
        let reply = warp::reply::json(&ErrorResponse { failure_reason: "internal server error".to_string() });
        Ok(warp::reply::with_status(reply, StatusCode::INTERNAL_SERVER_ERROR))
    }
}
