use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use aquatic_udp_protocol::{AnnounceInterval, AnnounceRequest, AnnounceResponse, ConnectRequest, ConnectResponse, ErrorResponse, NumberOfDownloads, NumberOfPeers, Port, Request, Response, ResponsePeer, ScrapeRequest, ScrapeResponse, TorrentScrapeStatistics, TransactionId};
use crate::{InfoHash, MAX_SCRAPE_TORRENTS, TorrentError, TorrentPeer, TorrentTracker};
use crate::torrust_udp_tracker::errors::ServerError;
use crate::torrust_udp_tracker::request::AnnounceRequestWrapper;
use crate::utils::get_connection_id;

pub async fn authenticate(info_hash: &InfoHash, tracker: Arc<TorrentTracker>) -> Result<(), ServerError> {
    match tracker.authenticate_request(info_hash, &None).await {
        Ok(_) => Ok(()),
        Err(e) => {
            let err = match e {
                TorrentError::TorrentNotWhitelisted => ServerError::TorrentNotWhitelisted,
                TorrentError::PeerNotAuthenticated => ServerError::PeerNotAuthenticated,
                TorrentError::PeerKeyNotValid => ServerError::PeerKeyNotValid,
                TorrentError::NoPeersFound => ServerError::NoPeersFound,
                TorrentError::CouldNotSendResponse => ServerError::InternalServerError,
                TorrentError::InvalidInfoHash => ServerError::InvalidInfoHash,
            };

            Err(err)
        }
    }
}

pub async fn handle_packet(remote_addr: SocketAddr, payload: &[u8], tracker: Arc<TorrentTracker>) -> Response {
    match Request::from_bytes(&payload[..payload.len()], MAX_SCRAPE_TORRENTS).map_err(|_| ServerError::InternalServerError) {
        Ok(request) => {
            let transaction_id = match &request {
                Request::Connect(connect_request) => {
                    connect_request.transaction_id
                }
                Request::Announce(announce_request) => {
                    announce_request.transaction_id
                }
                Request::Scrape(scrape_request) => {
                    scrape_request.transaction_id
                }
            };

            match handle_request(request, remote_addr, tracker).await {
                Ok(response) => response,
                Err(e) => handle_error(e, transaction_id)
            }
        }
        // bad request
        Err(_) => handle_error(ServerError::BadRequest, TransactionId(0))
    }
}

pub async fn handle_request(request: Request, remote_addr: SocketAddr, tracker: Arc<TorrentTracker>) -> Result<Response, ServerError> {
    match request {
        Request::Connect(connect_request) => {
            handle_connect(remote_addr, &connect_request, tracker).await
        }
        Request::Announce(announce_request) => {
            handle_announce(remote_addr, &announce_request, tracker).await
        }
        Request::Scrape(scrape_request) => {
            handle_scrape(remote_addr, &scrape_request, tracker).await
        }
    }
}

pub async fn handle_connect(remote_addr: SocketAddr, request: &ConnectRequest, tracker: Arc<TorrentTracker>) -> Result<Response, ServerError> {
    let connection_id = get_connection_id(&remote_addr);

    let response = Response::from(ConnectResponse {
        transaction_id: request.transaction_id,
        connection_id,
    });

    let tracker_copy = tracker.clone();
    tokio::spawn(async move {
        let mut status_writer = tracker_copy.set_stats().await;
        if remote_addr.is_ipv4() {
            status_writer.udp4_connections_handled += 1;
        } else {
            status_writer.udp6_connections_handled += 1;
        }
    });

    Ok(response)
}

pub async fn handle_announce(remote_addr: SocketAddr, announce_request: &AnnounceRequest, tracker: Arc<TorrentTracker>) -> Result<Response, ServerError> {
    let wrapped_announce_request = AnnounceRequestWrapper::new(announce_request.clone());

    authenticate(&wrapped_announce_request.info_hash, tracker.clone()).await?;

    let peer = TorrentPeer::from_udp_announce_request(&wrapped_announce_request.announce_request, remote_addr.ip(), tracker.config.get_ext_ip());

    //let torrent_stats = tracker.update_torrent_with_peer_and_get_stats(&wrapped_announce_request.info_hash, &peer).await;

    let torrent_stats = tracker.update_torrent_with_peer_and_get_stats(&wrapped_announce_request.info_hash, &peer).await;

    // get all peers excluding the client_addr
    let peers = tracker.get_torrent_peers(&wrapped_announce_request.info_hash, &peer.peer_addr).await;

    let tracker_copy = tracker.clone();
    tokio::spawn(async move {
        let mut status_writer = tracker_copy.set_stats().await;
        if remote_addr.is_ipv4() {
            status_writer.udp4_announces_handled += 1;
        } else {
            status_writer.udp6_announces_handled += 1;
        }
    });

    let announce_response = if remote_addr.is_ipv4() {
        Response::from(AnnounceResponse {
            transaction_id: wrapped_announce_request.announce_request.transaction_id,
            announce_interval: AnnounceInterval(tracker.config.announce_interval as i32),
            leechers: NumberOfPeers(torrent_stats.leechers as i32),
            seeders: NumberOfPeers(torrent_stats.seeders as i32),
            peers: peers.iter()
                .filter_map(|peer| if let IpAddr::V4(ip) =  peer.peer_addr.ip() {
                    Some(ResponsePeer::<Ipv4Addr> {
                        ip_address: ip,
                        port: Port(peer.peer_addr.port())
                    })
                } else {
                    None
                }
                ).collect()
        })
    } else {
        Response::from(AnnounceResponse {
            transaction_id: wrapped_announce_request.announce_request.transaction_id,
            announce_interval: AnnounceInterval(tracker.config.announce_interval as i32),
            leechers: NumberOfPeers(torrent_stats.leechers as i32),
            seeders: NumberOfPeers(torrent_stats.seeders as i32),
            peers: peers.iter()
                .filter_map(|peer| if let IpAddr::V6(ip) =  peer.peer_addr.ip() {
                    Some(ResponsePeer::<Ipv6Addr> {
                        ip_address: ip,
                        port: Port(peer.peer_addr.port())
                    })
                } else {
                    None
                }
            ).collect()
        })
    };

    Ok(announce_response)
}

// todo: refactor this, db lock can be a lot shorter
pub async fn handle_scrape(remote_addr: SocketAddr, request: &ScrapeRequest, tracker: Arc<TorrentTracker>) -> Result<Response, ServerError> {
    let db = tracker.get_torrents().await;

    let mut torrent_stats: Vec<TorrentScrapeStatistics> = Vec::new();

    for info_hash in request.info_hashes.iter() {
        let info_hash = InfoHash(info_hash.0);

        if authenticate(&info_hash,  tracker.clone()).await.is_err() { continue }

        let scrape_entry = match db.get(&info_hash) {
            Some(torrent_info) => {
                let (seeders, completed, leechers) = torrent_info.get_stats();

                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(seeders as i32),
                    completed: NumberOfDownloads(completed as i32),
                    leechers: NumberOfPeers(leechers as i32),
                }
            }
            None => {
                TorrentScrapeStatistics {
                    seeders: NumberOfPeers(0),
                    completed: NumberOfDownloads(0),
                    leechers: NumberOfPeers(0),
                }
            }
        };

        torrent_stats.push(scrape_entry);
    }

    let tracker_copy = tracker.clone();
    tokio::spawn(async move {
        let mut status_writer = tracker_copy.set_stats().await;
        if remote_addr.is_ipv4() {
            status_writer.udp4_scrapes_handled += 1;
        } else {
            status_writer.udp6_scrapes_handled += 1;
        }
    });

    Ok(Response::from(ScrapeResponse {
        transaction_id: request.transaction_id,
        torrent_stats
    }))
}

fn handle_error(e: ServerError, transaction_id: TransactionId) -> Response {
    let message = e.to_string();
    Response::from(ErrorResponse { transaction_id, message: message.into() })
}
