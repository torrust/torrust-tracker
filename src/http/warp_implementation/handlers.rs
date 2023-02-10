use std::collections::HashMap;
use std::convert::Infallible;
use std::net::IpAddr;
use std::panic::Location;
use std::sync::Arc;

use log::debug;
use warp::http::Response;
use warp::{reject, Rejection, Reply};

use super::error::Error;
use super::{request, response, WebResult};
use crate::http::warp_implementation::peer_builder;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::{self, auth, peer, statistics, torrent};

/// Authenticate `InfoHash` using optional `auth::Key`
///
/// # Errors
///
/// Will return `ServerError` that wraps the `tracker::error::Error` if unable to `authenticate_request`.
pub async fn authenticate(
    info_hash: &InfoHash,
    auth_key: &Option<auth::Key>,
    tracker: Arc<tracker::Tracker>,
) -> Result<(), Error> {
    tracker
        .authenticate_request(info_hash, auth_key)
        .await
        .map_err(|e| Error::TrackerError {
            source: (Arc::new(e) as Arc<dyn std::error::Error + Send + Sync>).into(),
        })
}

/// # Errors
///
/// Will return `warp::Rejection` that wraps the `ServerError` if unable to `send_announce_response`.
pub async fn handle_announce(
    announce_request: request::Announce,
    auth_key: Option<auth::Key>,
    tracker: Arc<tracker::Tracker>,
) -> WebResult<impl Reply> {
    debug!("http announce request: {:#?}", announce_request);

    let info_hash = announce_request.info_hash;
    let remote_client_ip = announce_request.peer_addr;

    authenticate(&info_hash, &auth_key, tracker.clone()).await?;

    let mut peer = peer_builder::from_request(&announce_request, &remote_client_ip);

    let response = tracker.announce(&info_hash, &mut peer, &remote_client_ip).await;

    match remote_client_ip {
        IpAddr::V4(_) => {
            tracker.send_stats_event(statistics::Event::Tcp4Announce).await;
        }
        IpAddr::V6(_) => {
            tracker.send_stats_event(statistics::Event::Tcp6Announce).await;
        }
    }

    send_announce_response(
        &announce_request,
        &response.swam_stats,
        &response.peers,
        tracker.config.announce_interval,
        tracker.config.min_announce_interval,
    )
}

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
            Err(e) => Err(reject::custom(Error::InternalServer {
                message: e.to_string(),
                location: Location::caller(),
            })),
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
        Err(e) => Err(reject::custom(Error::InternalServer {
            message: e.to_string(),
            location: Location::caller(),
        })),
    }
}

/// Handle all server errors and send error reply
///
/// # Errors
///
/// Will not return a error, `Infallible`, but instead  convert the `ServerError` into a `Response`.
pub fn send_error(r: &Rejection) -> std::result::Result<impl Reply, Infallible> {
    let warp_reject_error = r.find::<Error>();

    let body = if let Some(error) = warp_reject_error {
        debug!("{:?}", error);
        response::Error {
            failure_reason: error.to_string(),
        }
        .write()
    } else {
        response::Error {
            failure_reason: Error::InternalServer {
                message: "Undefined".to_string(),
                location: Location::caller(),
            }
            .to_string(),
        }
        .write()
    };

    Ok(Response::new(body))
}
