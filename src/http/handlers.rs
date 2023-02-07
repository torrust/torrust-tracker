use std::collections::HashMap;
use std::convert::Infallible;
use std::net::IpAddr;
use std::sync::Arc;

use log::debug;
use warp::http::Response;
use warp::{reject, Rejection, Reply};

use super::error::Error;
use super::{request, response, WebResult};
use crate::protocol::info_hash::InfoHash;
use crate::tracker::{self, auth, peer, statistics, torrent};

/// Authenticate `InfoHash` using optional `auth::Key`
///
/// # Errors
///
/// Will return `ServerError` that wraps the `Error` if unable to `authenticate_request`.
pub async fn authenticate(
    info_hash: &InfoHash,
    auth_key: &Option<auth::Key>,
    tracker: Arc<tracker::Tracker>,
) -> Result<(), Error> {
    tracker.authenticate_request(info_hash, auth_key).await.map_err(|e| match e {
        tracker::error::Error::TorrentNotWhitelisted { info_hash, location } => Error::TorrentNotWhitelisted,
        tracker::error::Error::PeerNotAuthenticated { location } => Error::PeerNotAuthenticated,
        tracker::error::Error::PeerKeyNotValid { key, source } => Error::PeerKeyNotValid,
    })
}

/// Handle announce request
///
/// # Errors
///
/// Will return `warp::Rejection` that wraps the `ServerError` if unable to `send_scrape_response`.
pub async fn handle_announce(
    announce_request: request::Announce,
    auth_key: Option<auth::Key>,
    tracker: Arc<tracker::Tracker>,
) -> WebResult<impl Reply> {
    authenticate(&announce_request.info_hash, &auth_key, tracker.clone())
        .await
        .map_err(reject::custom)?;

    debug!("{:?}", announce_request);

    let peer = peer::Peer::from_http_announce_request(&announce_request, announce_request.peer_addr, tracker.config.get_ext_ip());
    let torrent_stats = tracker
        .update_torrent_with_peer_and_get_stats(&announce_request.info_hash, &peer)
        .await;

    // get all torrent peers excluding the peer_addr
    let peers = tracker.get_torrent_peers(&announce_request.info_hash, &peer.peer_addr).await;

    let announce_interval = tracker.config.announce_interval;

    // send stats event
    match announce_request.peer_addr {
        IpAddr::V4(_) => {
            tracker.send_stats_event(statistics::Event::Tcp4Announce).await;
        }
        IpAddr::V6(_) => {
            tracker.send_stats_event(statistics::Event::Tcp6Announce).await;
        }
    }

    send_announce_response(
        &announce_request,
        &torrent_stats,
        &peers,
        announce_interval,
        tracker.config.min_announce_interval,
    )
}

/// Handle scrape request
///
/// # Errors
///
/// Will return `warp::Rejection` that wraps the `ServerError` if unable to `send_scrape_response`.
pub async fn handle_scrape(
    scrape_request: request::Scrape,
    auth_key: Option<auth::Key>,
    tracker: Arc<tracker::Tracker>,
) -> WebResult<impl Reply> {
    let mut files: HashMap<InfoHash, response::ScrapeEntry> = HashMap::new();
    let db = tracker.get_torrents().await;

    for info_hash in &scrape_request.info_hashes {
        let scrape_entry = match db.get(info_hash) {
            Some(torrent_info) => {
                if authenticate(info_hash, &auth_key, tracker.clone()).await.is_ok() {
                    let (seeders, completed, leechers) = torrent_info.get_stats();
                    response::ScrapeEntry {
                        complete: seeders,
                        downloaded: completed,
                        incomplete: leechers,
                    }
                } else {
                    response::ScrapeEntry {
                        complete: 0,
                        downloaded: 0,
                        incomplete: 0,
                    }
                }
            }
            None => response::ScrapeEntry {
                complete: 0,
                downloaded: 0,
                incomplete: 0,
            },
        };

        files.insert(*info_hash, scrape_entry);
    }

    // send stats event
    match scrape_request.peer_addr {
        IpAddr::V4(_) => {
            tracker.send_stats_event(statistics::Event::Tcp4Scrape).await;
        }
        IpAddr::V6(_) => {
            tracker.send_stats_event(statistics::Event::Tcp6Scrape).await;
        }
    }

    send_scrape_response(files)
}

/// Send announce response
#[allow(clippy::ptr_arg)]
fn send_announce_response(
    announce_request: &request::Announce,
    torrent_stats: &torrent::SwamStats,
    peers: &Vec<peer::Peer>,
    interval: u32,
    interval_min: u32,
) -> WebResult<impl Reply> {
    let http_peers: Vec<response::Peer> = peers
        .iter()
        .map(|peer| response::Peer {
            peer_id: peer.peer_id.to_string(),
            ip: peer.peer_addr.ip(),
            port: peer.peer_addr.port(),
        })
        .collect();

    let res = response::Announce {
        interval,
        interval_min,
        complete: torrent_stats.seeders,
        incomplete: torrent_stats.leechers,
        peers: http_peers,
    };

    // check for compact response request
    if let Some(1) = announce_request.compact {
        match res.write_compact() {
            Ok(body) => Ok(Response::new(body)),
            Err(_) => Err(reject::custom(Error::InternalServer)),
        }
    } else {
        Ok(Response::new(res.write().into()))
    }
}

/// Send scrape response
fn send_scrape_response(files: HashMap<InfoHash, response::ScrapeEntry>) -> WebResult<impl Reply> {
    let res = response::Scrape { files };

    match res.write() {
        Ok(body) => Ok(Response::new(body)),
        Err(_) => Err(reject::custom(Error::InternalServer)),
    }
}

/// Handle all server errors and send error reply
///
/// # Errors
///
/// Will not return a error, `Infallible`, but instead  convert the `ServerError` into a `Response`.
pub fn send_error(r: &Rejection) -> std::result::Result<impl Reply, Infallible> {
    let body = if let Some(server_error) = r.find::<Error>() {
        debug!("{:?}", server_error);
        response::Error {
            failure_reason: server_error.to_string(),
        }
        .write()
    } else {
        response::Error {
            failure_reason: Error::InternalServer.to_string(),
        }
        .write()
    };

    Ok(Response::new(body))
}
