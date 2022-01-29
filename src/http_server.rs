use std::collections::{HashMap};
use crate::tracker::{TorrentTracker};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::error::Error;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::str::FromStr;
use log::{debug};
use warp::{filters, reply::Reply, Filter};
use warp::http::Response;
use crate::{TorrentError, TorrentPeer, TorrentStats};
use crate::key_manager::AuthKey;
use crate::utils::url_encode_bytes;
use super::common::*;

#[derive(Deserialize, Debug)]
pub struct AnnounceRequest {
    pub downloaded: u32,
    pub uploaded: u32,
    pub key: String,
    pub peer_id: String,
    pub port: u16,
    pub info_hash: String,
    pub left: u32,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

impl AnnounceRequest {
    pub fn is_compact(&self) -> bool {
        self.compact.unwrap_or(0) == 1
    }
}

#[derive(Deserialize, Debug)]
pub struct ScrapeRequest {
    pub info_hash: String,
}

#[derive(Serialize)]
struct Peer {
    peer_id: String,
    ip: IpAddr,
    port: u16,
}

#[derive(Serialize)]
struct AnnounceResponse {
    interval: u32,
    //tracker_id: String,
    complete: u32,
    incomplete: u32,
    peers: Vec<Peer>
}

impl AnnounceResponse {
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }

    pub fn write_compact(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut peers_v4: Vec<u8> = Vec::new();
        let mut peers_v6: Vec<u8> = Vec::new();

        for peer in &self.peers {
            match peer.ip {
                IpAddr::V4(ip) => {
                    peers_v4.write(&u32::from(ip).to_be_bytes())?;
                    peers_v4.write(&peer.port.to_be_bytes())?;
                }
                IpAddr::V6(ip) => {
                    peers_v6.write(&u128::from(ip).to_be_bytes())?;
                    peers_v6.write(&peer.port.to_be_bytes())?;
                }
            }
        }

        debug!("{:?}", String::from_utf8_lossy(peers_v4.as_slice()));
        debug!("{:?}", String::from_utf8_lossy(peers_v6.as_slice()));

        let mut bytes: Vec<u8> = Vec::new();
        bytes.write(b"d8:intervali")?;
        bytes.write(&self.interval.to_string().as_bytes())?;
        bytes.write(b"e8:completei")?;
        bytes.write(&self.complete.to_string().as_bytes())?;
        bytes.write(b"e10:incompletei")?;
        bytes.write(&self.incomplete.to_string().as_bytes())?;
        bytes.write(b"e5:peers")?;
        bytes.write(&peers_v4.len().to_string().as_bytes())?;
        bytes.write(b":")?;
        bytes.write(peers_v4.as_slice())?;
        bytes.write(b"e6:peers6")?;
        bytes.write(&peers_v6.len().to_string().as_bytes())?;
        bytes.write(b":")?;
        bytes.write(peers_v6.as_slice())?;
        bytes.write(b"e")?;

        debug!("{:?}", String::from_utf8_lossy(bytes.as_slice()));
        Ok(bytes)
    }
}

#[derive(Serialize)]
struct ScrapeResponse {
    files: HashMap<String, ScrapeResponseEntry>
}

impl ScrapeResponse {
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }
}

#[derive(Serialize)]
struct ScrapeResponseEntry {
    complete: u32,
    downloaded: u32,
    incomplete: u32,
}

#[derive(Serialize)]
struct ErrorResponse {
    failure_reason: String
}

impl warp::Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        Response::new(format!("{}", serde_bencode::to_string(&self).unwrap()).into())
    }
}

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

    // &self did not work here
    pub fn routes(http_server: Arc<HttpServer>) -> impl Filter<Extract = impl Reply> + Clone + Send + Sync + 'static {
        // optional tracker key
        let opt_key = warp::path::param::<String>()
            .map(Some)
            .or_else(|_| async {
                // Ok(None)
                Ok::<(Option<String>,), std::convert::Infallible>((None,))
            });

        // GET /announce?key=:String
        // Announce peer
        let hs1 = http_server.clone();
        let announce_route =
            filters::path::path("announce")
                .and(filters::method::get())
                .and(warp::addr::remote())
                .and(opt_key)
                .and(filters::query::raw())
                .and(filters::query::query())
                .map(move |remote_addr, key, raw_query, query| {
                    debug!("Request: {}", raw_query);
                    (remote_addr, key, raw_query, query, hs1.clone())
                })
                .and_then(move |(remote_addr, key, raw_query, mut query, http_server): (Option<SocketAddr>, Option<String>, String, AnnounceRequest, Arc<HttpServer>)| {
                    async move {
                        if remote_addr.is_none() { return HttpServer::send_error("could not get remote address") }

                        // query.info_hash somehow receives a corrupt string
                        // so we have to get the info_hash manually from the raw query
                        let info_hashes = HttpServer::info_hashes_from_raw_query(&raw_query);
                        if info_hashes.len() < 1 { return HttpServer::send_error("info_hash not found") }
                        query.info_hash = info_hashes[0].to_string();
                        debug!("{:?}", query.info_hash);

                        if let Some(err) = http_server.authenticate_request(&query.info_hash, key).await { return err }

                        http_server.handle_announce(query, remote_addr.unwrap()).await
                    }
                });

        // GET /scrape?key=:String
        // Get torrent info
        let hs2 = http_server.clone();
        let scrape_route =
            filters::path::path("scrape")
                .and(filters::method::get())
                .and(opt_key)
                .and(filters::query::raw())
                .map(move |key, raw_query| {
                    debug!("Request: {}", raw_query);
                    (key, raw_query, hs2.clone())
                })
                .and_then(move |(key, raw_query, http_server): (Option<String>, String, Arc<HttpServer>)| {
                    async move {
                        let info_hashes = HttpServer::info_hashes_from_raw_query(&raw_query);
                        if info_hashes.len() < 1 { return HttpServer::send_error("info_hash not found") }
                        if info_hashes.len() > 50 { return HttpServer::send_error("exceeded the max of 50 info_hashes") }
                        debug!("{:?}", info_hashes);

                        // todo: verify all info_hashes before scrape
                        if let Some(err) = http_server.authenticate_request(&info_hashes[0].to_string(), key).await { return err }

                        http_server.handle_scrape(info_hashes).await
                    }
                });

        // all routes
        warp::any().and(announce_route.or(scrape_route))
    }

    fn info_hashes_from_raw_query(raw_query: &str) -> Vec<InfoHash> {
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

        info_hashes
    }

    fn send_announce_response(query: &AnnounceRequest, torrent_stats: TorrentStats, peers: Vec<TorrentPeer>, interval: u32) -> Result<warp::reply::Response, Infallible> {
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
        let response = match query.compact {
            None => Response::new(res.write().into()),
            Some(int) => {
                if int == 1 {
                    let res_compact = res.write_compact();
                    match res_compact {
                        Ok(response) => Response::new(response.into()),
                        Err(e) => {
                            debug!("{}", e);
                            HttpServer::send_error("server error").unwrap()
                        }
                    }
                } else {
                    Response::new(res.write().into())
                }
            }
        };

        Ok(response)
    }

    fn send_error(msg: &str) -> Result<warp::reply::Response, Infallible> {
        Ok(ErrorResponse {
            failure_reason: msg.to_string()
        }.into_response())
    }

    async fn authenticate_request(&self, info_hash_str: &str, key: Option<String>) -> Option<Result<warp::reply::Response, Infallible>> {
        let info_hash= InfoHash::from_str(info_hash_str);
        if info_hash.is_err() { return Some(HttpServer::send_error("invalid info_hash")) }

        let auth_key = match key {
            None => None,
            Some(v) => AuthKey::from_string(&v)
        };

        if let Err(e) = self.tracker.authenticate_request(&info_hash.unwrap(), auth_key).await {
            return match e {
                TorrentError::TorrentNotWhitelisted => {
                    debug!("Info_hash not whitelisted.");
                    Some(HttpServer::send_error("torrent not whitelisted"))
                }
                TorrentError::PeerKeyNotValid => {
                    debug!("Peer key not valid.");
                    Some(HttpServer::send_error("peer key not valid"))
                }
                TorrentError::PeerNotAuthenticated => {
                    debug!("Peer not authenticated.");
                    Some(HttpServer::send_error("peer not authenticated"))
                }
                _ => {
                    debug!("Unhandled HTTP error.");
                    Some(HttpServer::send_error("oops"))
                }
            }
        }

        None
    }

    async fn handle_announce(&self, query: AnnounceRequest, remote_addr: SocketAddr) -> Result<warp::reply::Response, Infallible> {
        let info_hash = match InfoHash::from_str(&query.info_hash) {
            Ok(v) => v,
            Err(_) => {
                return HttpServer::send_error("info_hash is invalid")
            }
        };

        let peer = TorrentPeer::from_http_announce_request(&query, remote_addr, self.tracker.config.get_ext_ip());

        match self.tracker.update_torrent_with_peer_and_get_stats(&info_hash, &peer).await {
            Err(e) => {
                debug!("{:?}", e);
                HttpServer::send_error("server error")
            }
            Ok(torrent_stats) => {
                // get all peers excluding the client_addr
                let peers = self.tracker.get_torrent_peers(&info_hash, &peer.peer_addr).await;
                if peers.is_none() {
                    debug!("No peers found after announce.");
                    return HttpServer::send_error("peer is invalid")
                }

                // todo: add http announce interval config option
                // success response
                let announce_interval = self.tracker.config.http_tracker.as_ref().unwrap().announce_interval;
                HttpServer::send_announce_response(&query, torrent_stats, peers.unwrap(), announce_interval)
            }
        }
    }

    async fn handle_scrape(&self, info_hashes: Vec<InfoHash>) -> Result<warp::reply::Response, Infallible> {
        let mut res = ScrapeResponse {
            files: HashMap::new()
        };
        let db = self.tracker.get_torrents().await;

        for info_hash in info_hashes.iter() {
            let scrape_entry = match db.get(&info_hash) {
                Some(torrent_info) => {
                    let (seeders, completed, leechers) = torrent_info.get_stats();

                    ScrapeResponseEntry {
                        complete: seeders,
                        downloaded: completed,
                        incomplete: leechers
                    }
                }
                None => {
                    ScrapeResponseEntry {
                        complete: 0,
                        downloaded: 0,
                        incomplete: 0
                    }
                }
            };

            if let Ok(encoded_info_hash) = url_encode_bytes(&info_hash.0) {
                res.files.insert(encoded_info_hash, scrape_entry);
            }
        }

        Ok(Response::new(res.write().into()))
    }
}
