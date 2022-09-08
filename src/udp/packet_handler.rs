use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use aquatic_udp_protocol::{AnnounceInterval, AnnounceRequest, AnnounceResponse, ConnectRequest, ConnectResponse, NumberOfDownloads, NumberOfPeers, Port, Request, Response, ResponsePeer, ScrapeRequest, ScrapeResponse, TorrentScrapeStatistics, ConnectionId, ErrorResponse, TransactionId};
use log::debug;
use crate::torrent::TorrentError;
use crate::udp::connection::connection_id_issuer::{EncryptedConnectionIdIssuer, ConnectionIdIssuer};
use crate::udp::connection::secret::Secret;
use crate::{InfoHash, MAX_SCRAPE_TORRENTS};
use crate::peer::TorrentPeer;
use crate::udp::errors::ServerError;
use crate::udp::request::AnnounceRequestWrapper;
use crate::tracker::statistics::TrackerStatisticsEvent;
use crate::tracker::tracker::TorrentTracker;
use crate::protocol::clock::current_timestamp;

pub struct PacketHandler {
    encrypted_connection_id_issuer: EncryptedConnectionIdIssuer,
    // todo: inject also a crate::protocol::Clock in order to make it easier to test it.
}

impl PacketHandler {
    pub fn new(secret: Secret) -> Self {
        let encrypted_connection_id_issuer = EncryptedConnectionIdIssuer::new(secret);
        Self { encrypted_connection_id_issuer }
    }

    pub async fn handle_packet(&self, remote_addr: SocketAddr, payload: Vec<u8>, tracker: Arc<TorrentTracker>) -> Option<Response> {
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

                match self.handle_request(request, remote_addr, tracker).await {
                    Ok(response) => Some(response),
                    Err(ServerError::InvalidConnectionId) => None,
                    Err(e) => Some(Self::handle_error(e, transaction_id))
                }
            }
            // don't respond to bad requests
            Err(_) => None
        }
    }

    fn handle_error(e: ServerError, transaction_id: TransactionId) -> Response {
        let message = e.to_string();
        Response::from(ErrorResponse { transaction_id, message: message.into() })
    }


    async fn handle_request(&self, request: Request, remote_addr: SocketAddr, tracker: Arc<TorrentTracker>) -> Result<Response, ServerError> {
        match request {
            Request::Connect(connect_request) => {
                self.handle_connect(remote_addr, &connect_request, tracker).await
            }
            Request::Announce(announce_request) => {
                self.handle_announce(remote_addr, &announce_request, tracker).await
            }
            Request::Scrape(scrape_request) => {
                self.handle_scrape(remote_addr, &scrape_request, tracker).await
            }
        }
    }

    async fn handle_connect(&self, remote_addr: SocketAddr, request: &ConnectRequest, tracker: Arc<TorrentTracker>) -> Result<Response, ServerError> {
        let connection_id = self.generate_new_connection_id(&remote_addr);

        let response = Response::from(ConnectResponse {
            transaction_id: request.transaction_id,
            connection_id,
        });

        // send stats event
        match remote_addr {
            SocketAddr::V4(_) => { tracker.send_stats_event(TrackerStatisticsEvent::Udp4Connect).await; }
            SocketAddr::V6(_) => { tracker.send_stats_event(TrackerStatisticsEvent::Udp6Connect).await; }
        }

        Ok(response)
    }

    async fn handle_announce(&self, remote_addr: SocketAddr, announce_request: &AnnounceRequest, tracker: Arc<TorrentTracker>) -> Result<Response, ServerError> {
        let valid = self.is_connection_id_valid(&announce_request.connection_id, &remote_addr);
        if !valid {
            return Err(ServerError::InvalidConnectionId);
        }

        let wrapped_announce_request = AnnounceRequestWrapper::new(announce_request.clone());

        self.authenticate(&wrapped_announce_request.info_hash, tracker.clone()).await?;

        let peer = TorrentPeer::from_udp_announce_request(&wrapped_announce_request.announce_request, remote_addr.ip(), tracker.config.get_ext_ip());

        //let torrent_stats = tracker.update_torrent_with_peer_and_get_stats(&wrapped_announce_request.info_hash, &peer).await;

        let torrent_stats = tracker.update_torrent_with_peer_and_get_stats(&wrapped_announce_request.info_hash, &peer).await;

        // get all peers excluding the client_addr
        let peers = tracker.get_torrent_peers(&wrapped_announce_request.info_hash, &peer.peer_addr).await;

        let announce_response = if remote_addr.is_ipv4() {
            Response::from(AnnounceResponse {
                transaction_id: wrapped_announce_request.announce_request.transaction_id,
                announce_interval: AnnounceInterval(tracker.config.announce_interval as i32),
                leechers: NumberOfPeers(torrent_stats.leechers as i32),
                seeders: NumberOfPeers(torrent_stats.seeders as i32),
                peers: peers.iter()
                    .filter_map(|peer| if let IpAddr::V4(ip) = peer.peer_addr.ip() {
                        Some(ResponsePeer::<Ipv4Addr> {
                            ip_address: ip,
                            port: Port(peer.peer_addr.port()),
                        })
                    } else {
                        None
                    }
                    ).collect(),
            })
        } else {
            Response::from(AnnounceResponse {
                transaction_id: wrapped_announce_request.announce_request.transaction_id,
                announce_interval: AnnounceInterval(tracker.config.announce_interval as i32),
                leechers: NumberOfPeers(torrent_stats.leechers as i32),
                seeders: NumberOfPeers(torrent_stats.seeders as i32),
                peers: peers.iter()
                    .filter_map(|peer| if let IpAddr::V6(ip) = peer.peer_addr.ip() {
                        Some(ResponsePeer::<Ipv6Addr> {
                            ip_address: ip,
                            port: Port(peer.peer_addr.port()),
                        })
                    } else {
                        None
                    }
                    ).collect(),
            })
        };

        // send stats event
        match remote_addr {
            SocketAddr::V4(_) => { tracker.send_stats_event(TrackerStatisticsEvent::Udp4Announce).await; }
            SocketAddr::V6(_) => { tracker.send_stats_event(TrackerStatisticsEvent::Udp6Announce).await; }
        }

        Ok(announce_response)
    }

    // todo: refactor this, db lock can be a lot shorter
    async fn handle_scrape(&self, remote_addr: SocketAddr, request: &ScrapeRequest, tracker: Arc<TorrentTracker>) -> Result<Response, ServerError> {
        let valid = self.is_connection_id_valid(&request.connection_id, &remote_addr);
        if !valid {
            return Err(ServerError::InvalidConnectionId);
        }

        let db = tracker.get_torrents().await;

        let mut torrent_stats: Vec<TorrentScrapeStatistics> = Vec::new();

        for info_hash in request.info_hashes.iter() {
            let info_hash = InfoHash(info_hash.0);

            let scrape_entry = match db.get(&info_hash) {
                Some(torrent_info) => {
                    if self.authenticate(&info_hash, tracker.clone()).await.is_ok() {
                        let (seeders, completed, leechers) = torrent_info.get_stats();

                        TorrentScrapeStatistics {
                            seeders: NumberOfPeers(seeders as i32),
                            completed: NumberOfDownloads(completed as i32),
                            leechers: NumberOfPeers(leechers as i32),
                        }
                    } else {
                        TorrentScrapeStatistics {
                            seeders: NumberOfPeers(0),
                            completed: NumberOfDownloads(0),
                            leechers: NumberOfPeers(0),
                        }
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

        drop(db);

        // send stats event
        match remote_addr {
            SocketAddr::V4(_) => { tracker.send_stats_event(TrackerStatisticsEvent::Udp4Scrape).await; }
            SocketAddr::V6(_) => { tracker.send_stats_event(TrackerStatisticsEvent::Udp6Scrape).await; }
        }

        Ok(Response::from(ScrapeResponse {
            transaction_id: request.transaction_id,
            torrent_stats,
        }))
    }

    async fn authenticate(&self, info_hash: &InfoHash, tracker: Arc<TorrentTracker>) -> Result<(), ServerError> {
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

    fn generate_new_connection_id(&self, remote_addr: &SocketAddr) -> ConnectionId {
        let current_timestamp = current_timestamp();

        let connection_id = self.encrypted_connection_id_issuer.new_connection_id(remote_addr, current_timestamp);

        debug!("new connection id: {:?}, current timestamp: {:?}", connection_id, current_timestamp);

        connection_id
    }

    fn is_connection_id_valid(&self, connection_id: &ConnectionId, remote_addr: &SocketAddr) -> bool {
        let current_timestamp = current_timestamp();

        let valid = self.encrypted_connection_id_issuer.is_connection_id_valid(connection_id, remote_addr, current_timestamp);

        debug!("verify connection id: {:?}, current timestamp: {:?}, valid: {:?}", connection_id, current_timestamp, valid);

        valid
    }
}
