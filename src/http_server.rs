use std::collections::HashMap;
use crate::tracker::{TorrentTracker};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::str::FromStr;
use log::debug;
use warp::{filters, reply::Reply, Filter};
use warp::http::Response;
use crate::{Configuration, TorrentError, TorrentPeer, TorrentStats};
use crate::key_manager::AuthKey;
use super::common::*;

#[derive(Deserialize, Debug)]
pub struct HttpAnnounceRequest {
    pub downloaded: NumberOfBytes,
    pub uploaded: NumberOfBytes,
    pub key: String,
    pub peer_id: String,
    pub port: u16,
    pub info_hash: String,
    pub left: NumberOfBytes,
    pub event: Option<String>,
    pub compact: Option<u8>,
}

impl HttpAnnounceRequest {
    pub fn is_compact(&self) -> bool {
        self.compact.unwrap_or(0) == 1
    }
}

#[derive(Serialize)]
struct HttpPeer {
    peer_id: String,
    ip: IpAddr,
    port: u16,
}

#[derive(Serialize)]
struct HttpResponse {
    interval: u32,
    //tracker_id: String,
    complete: u32,
    incomplete: u32,
    peers: Vec<HttpPeer>
}

impl HttpResponse {
    pub fn write(&self) -> String {
        serde_bencode::to_string(&self).unwrap()
    }

    pub fn write_compact(&self) -> String {
        String::new()
    }
}

// todo: serve proper byte string format
// fn peers_to_bytes(peers: &Vec<SocketAddr>) -> String {
//     let mut bytes = Vec::with_capacity(peers.len() * 6);
//
//     for peer in peers {
//         match peer {
//             SocketAddr::V4(peer) => {
//                 println!("{:?}", peer.ip());
//                 bytes.write(b":");
//                 bytes.extend_from_slice(&u32::from(peer.ip().clone()).to_be_bytes());
//                 bytes.extend_from_slice(&peer.port().to_be_bytes());
//             }
//             SocketAddr::V6(_) => {}
//         }
//     }
//
//     println!("{:?}", String::from_utf8_lossy(&bytes).to_string());
//     String::from_utf8_lossy(&bytes).to_string()
// }

// format!("8:intervali{}e8:completei{}e10:incompletei{}e5:peers{}:{}e",
//         &self.interval,
//         &self.complete,
//         &self.incomplete,
//         &self.peers.len() * 26,
//         serde_bencode::to_string(&peers).unwrap()
// )

#[derive(Serialize)]
struct HttpErrorResponse {
    failure_reason: String
}

impl warp::Reply for HttpErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        Response::new(format!("{}", serde_bencode::to_string(&self).unwrap()).into())
    }
}

#[derive(Clone)]
pub struct HttpServer {
    pub config: Arc<Configuration>,
    pub tracker: Arc<TorrentTracker>,
}

impl HttpServer {
    pub fn new(config: Arc<Configuration>, tracker: Arc<TorrentTracker>) -> HttpServer {
        HttpServer {
            config,
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
        let announce_route =
            filters::path::path("announce")
                .and(filters::method::get())
                .and(warp::addr::remote())
                .and(opt_key)
                .and(filters::query::raw())
                .and(filters::query::query())
                .map(move |remote_addr, key, raw_query, query| {
                    (remote_addr, key, raw_query, query, http_server.clone())
                })
                .and_then(move |(remote_addr, key, raw_query, mut query, http_server): (Option<SocketAddr>, Option<String>, String, HttpAnnounceRequest, Arc<HttpServer>)| {
                    async move {
                        if remote_addr.is_none() { return HttpServer::send_error("could not get remote address") }

                        let auth_key = match key {
                            None => None,
                            Some(v) => AuthKey::from_string(&v)
                        };

                        // query.info_hash somehow receives a corrupt string
                        // so we have to get the info_hash manually from the raw query
                        let raw_info_hash = HttpServer::get_raw_info_hash_from_raw_query(&raw_query);
                        if raw_info_hash.is_none() { return HttpServer::send_error("info_hash not found") }
                        let info_hash = percent_encoding::percent_decode_str(raw_info_hash.unwrap()).collect::<Vec<u8>>();
                        query.info_hash = hex::encode(info_hash);
                        println!("{:?}", query.info_hash);

                        http_server.handle_announce(query, remote_addr.unwrap(), auth_key).await
                    }
                });

        // all routes
        warp::any().and(announce_route)
    }

    fn get_raw_info_hash_from_raw_query(raw_query: &str) -> Option<&str> {
        let split_raw_query: Vec<&str> = raw_query.split("&").collect();

        let mut raw_info_hash = None;

        for v in split_raw_query {
            if v.contains("info_hash") {
                let rih: Vec<&str> = v.split("=").collect();
                raw_info_hash = Some(rih[1]);
            }
        }

        raw_info_hash
    }

    fn send_announce_response(torrent_stats: TorrentStats, peers: Vec<TorrentPeer>, interval: u32) -> Result<warp::reply::Response, Infallible> {
        // todo: add old non-compacted response (very rarely used)

        let http_peers: Vec<HttpPeer> = peers.iter().map(|peer| HttpPeer {
            peer_id: String::from_utf8_lossy(&peer.peer_id.0).to_string(),
            ip: peer.peer_addr.ip(),
            port: peer.peer_addr.port()
        }).collect();

        let res = HttpResponse {
            interval,
            complete: torrent_stats.seeders,
            incomplete: torrent_stats.leechers,
            peers: http_peers
        };

        Ok(Response::new(res.write().into()))
    }

    fn send_error(msg: &str) -> Result<warp::reply::Response, Infallible> {
        Ok(HttpErrorResponse {
            failure_reason: msg.to_string()
        }.into_response())
    }

    async fn handle_announce(&self, query: HttpAnnounceRequest, remote_addr: SocketAddr, auth_key: Option<AuthKey>) -> Result<warp::reply::Response, Infallible> {
        let info_hash = match InfoHash::from_str(&query.info_hash) {
            Ok(v) => v,
            Err(_) => {
                return HttpServer::send_error("info_hash is invalid")
            }
        };

        if let Err(e) = self.tracker.authenticate_announce_request(&info_hash, &auth_key).await {
            return match e {
                TorrentError::TorrentNotWhitelisted => {
                    debug!("Info_hash not whitelisted.");
                    HttpServer::send_error("torrent not whitelisted")
                }
                TorrentError::PeerKeyNotValid => {
                    debug!("Peer key not valid.");
                    HttpServer::send_error("peer key not valid")
                }
                TorrentError::PeerNotAuthenticated => {
                    debug!("Peer not authenticated.");
                    HttpServer::send_error("peer not authenticated")
                }
            }
        }

        let peer = TorrentPeer::from_http_announce_request(&query, remote_addr, self.config.get_ext_ip());

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
                HttpServer::send_announce_response(torrent_stats, peers.unwrap(), self.config.get_udp_tracker_config().get_announce_interval())
            }
        }
    }
}
