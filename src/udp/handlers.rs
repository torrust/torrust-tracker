use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{
    AnnounceInterval, AnnounceRequest, AnnounceResponse, ConnectRequest, ConnectResponse, ErrorResponse, NumberOfDownloads,
    NumberOfPeers, Port, Request, Response, ResponsePeer, ScrapeRequest, ScrapeResponse, TorrentScrapeStatistics, TransactionId,
};

use crate::peer::TorrentPeer;
use crate::protocol::utils::get_connection_id;
use crate::tracker::statistics::TrackerStatisticsEvent;
use crate::tracker::torrent::TorrentError;
use crate::tracker::tracker::TorrentTracker;
use crate::udp::errors::ServerError;
use crate::udp::request::AnnounceRequestWrapper;
use crate::{InfoHash, MAX_SCRAPE_TORRENTS};

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

pub async fn handle_packet(remote_addr: SocketAddr, payload: Vec<u8>, tracker: Arc<TorrentTracker>) -> Response {
    match Request::from_bytes(&payload[..payload.len()], MAX_SCRAPE_TORRENTS).map_err(|_| ServerError::InternalServerError) {
        Ok(request) => {
            let transaction_id = match &request {
                Request::Connect(connect_request) => connect_request.transaction_id,
                Request::Announce(announce_request) => announce_request.transaction_id,
                Request::Scrape(scrape_request) => scrape_request.transaction_id,
            };

            match handle_request(request, remote_addr, tracker).await {
                Ok(response) => response,
                Err(e) => handle_error(e, transaction_id),
            }
        }
        // bad request
        Err(_) => handle_error(ServerError::BadRequest, TransactionId(0)),
    }
}

pub async fn handle_request(
    request: Request,
    remote_addr: SocketAddr,
    tracker: Arc<TorrentTracker>,
) -> Result<Response, ServerError> {
    match request {
        Request::Connect(connect_request) => handle_connect(remote_addr, &connect_request, tracker).await,
        Request::Announce(announce_request) => handle_announce(remote_addr, &announce_request, tracker).await,
        Request::Scrape(scrape_request) => handle_scrape(remote_addr, &scrape_request, tracker).await,
    }
}

pub async fn handle_connect(
    remote_addr: SocketAddr,
    request: &ConnectRequest,
    tracker: Arc<TorrentTracker>,
) -> Result<Response, ServerError> {
    let connection_id = get_connection_id(&remote_addr);

    let response = Response::from(ConnectResponse {
        transaction_id: request.transaction_id,
        connection_id,
    });

    // send stats event
    match remote_addr {
        SocketAddr::V4(_) => {
            tracker.send_stats_event(TrackerStatisticsEvent::Udp4Connect).await;
        }
        SocketAddr::V6(_) => {
            tracker.send_stats_event(TrackerStatisticsEvent::Udp6Connect).await;
        }
    }

    Ok(response)
}

pub async fn handle_announce(
    remote_addr: SocketAddr,
    announce_request: &AnnounceRequest,
    tracker: Arc<TorrentTracker>,
) -> Result<Response, ServerError> {
    let wrapped_announce_request = AnnounceRequestWrapper::new(announce_request.clone());

    authenticate(&wrapped_announce_request.info_hash, tracker.clone()).await?;

    let peer = TorrentPeer::from_udp_announce_request(
        &wrapped_announce_request.announce_request,
        remote_addr.ip(),
        tracker.config.get_ext_ip(),
    );

    //let torrent_stats = tracker.update_torrent_with_peer_and_get_stats(&wrapped_announce_request.info_hash, &peer).await;

    let torrent_stats = tracker
        .update_torrent_with_peer_and_get_stats(&wrapped_announce_request.info_hash, &peer)
        .await;

    // get all peers excluding the client_addr
    let peers = tracker
        .get_torrent_peers(&wrapped_announce_request.info_hash, &peer.peer_addr)
        .await;

    let announce_response = if remote_addr.is_ipv4() {
        Response::from(AnnounceResponse {
            transaction_id: wrapped_announce_request.announce_request.transaction_id,
            announce_interval: AnnounceInterval(tracker.config.announce_interval as i32),
            leechers: NumberOfPeers(torrent_stats.leechers as i32),
            seeders: NumberOfPeers(torrent_stats.seeders as i32),
            peers: peers
                .iter()
                .filter_map(|peer| {
                    if let IpAddr::V4(ip) = peer.peer_addr.ip() {
                        Some(ResponsePeer::<Ipv4Addr> {
                            ip_address: ip,
                            port: Port(peer.peer_addr.port()),
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        })
    } else {
        Response::from(AnnounceResponse {
            transaction_id: wrapped_announce_request.announce_request.transaction_id,
            announce_interval: AnnounceInterval(tracker.config.announce_interval as i32),
            leechers: NumberOfPeers(torrent_stats.leechers as i32),
            seeders: NumberOfPeers(torrent_stats.seeders as i32),
            peers: peers
                .iter()
                .filter_map(|peer| {
                    if let IpAddr::V6(ip) = peer.peer_addr.ip() {
                        Some(ResponsePeer::<Ipv6Addr> {
                            ip_address: ip,
                            port: Port(peer.peer_addr.port()),
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        })
    };

    // send stats event
    match remote_addr {
        SocketAddr::V4(_) => {
            tracker.send_stats_event(TrackerStatisticsEvent::Udp4Announce).await;
        }
        SocketAddr::V6(_) => {
            tracker.send_stats_event(TrackerStatisticsEvent::Udp6Announce).await;
        }
    }

    Ok(announce_response)
}

// todo: refactor this, db lock can be a lot shorter
pub async fn handle_scrape(
    remote_addr: SocketAddr,
    request: &ScrapeRequest,
    tracker: Arc<TorrentTracker>,
) -> Result<Response, ServerError> {
    let db = tracker.get_torrents().await;

    let mut torrent_stats: Vec<TorrentScrapeStatistics> = Vec::new();

    for info_hash in request.info_hashes.iter() {
        let info_hash = InfoHash(info_hash.0);

        let scrape_entry = match db.get(&info_hash) {
            Some(torrent_info) => {
                if authenticate(&info_hash, tracker.clone()).await.is_ok() {
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
            None => TorrentScrapeStatistics {
                seeders: NumberOfPeers(0),
                completed: NumberOfDownloads(0),
                leechers: NumberOfPeers(0),
            },
        };

        torrent_stats.push(scrape_entry);
    }

    drop(db);

    // send stats event
    match remote_addr {
        SocketAddr::V4(_) => {
            tracker.send_stats_event(TrackerStatisticsEvent::Udp4Scrape).await;
        }
        SocketAddr::V6(_) => {
            tracker.send_stats_event(TrackerStatisticsEvent::Udp6Scrape).await;
        }
    }

    Ok(Response::from(ScrapeResponse {
        transaction_id: request.transaction_id,
        torrent_stats,
    }))
}

fn handle_error(e: ServerError, transaction_id: TransactionId) -> Response {
    let message = e.to_string();
    Response::from(ErrorResponse {
        transaction_id,
        message: message.into(),
    })
}

#[cfg(test)]
mod tests {
    use std::{
        net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
        sync::Arc,
    };

    use tokio::sync::{mpsc::error::SendError, RwLock, RwLockReadGuard};

    use crate::{
        protocol::utils::get_connection_id,
        statistics::{
            StatsTracker, TrackerStatistics, TrackerStatisticsEvent, TrackerStatisticsEventSender, TrackerStatisticsRepository,
            TrackerStatsService,
        },
        tracker::tracker::TorrentTracker,
        udp::handle_connect,
        Configuration,
    };
    use aquatic_udp_protocol::{ConnectRequest, ConnectResponse, Response, TransactionId};
    use async_trait::async_trait;

    fn default_tracker_config() -> Arc<Configuration> {
        Arc::new(Configuration::default())
    }

    fn initialized_tracker() -> Arc<TorrentTracker> {
        Arc::new(TorrentTracker::new(default_tracker_config(), Box::new(StatsTracker::new_running_instance())).unwrap())
    }

    fn sample_remote_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    }

    fn sample_connect_request() -> ConnectRequest {
        ConnectRequest {
            transaction_id: TransactionId(0i32),
        }
    }

    fn sample_ipv4_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    }

    fn sample_ipv6_socket_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8080)
    }

    #[tokio::test]
    async fn a_connect_response_should_contain_the_same_transaction_id_as_the_connect_request() {
        let request = ConnectRequest {
            transaction_id: TransactionId(0i32),
        };

        let response = handle_connect(sample_remote_addr(), &request, initialized_tracker())
            .await
            .unwrap();

        assert_eq!(
            response,
            Response::Connect(ConnectResponse {
                connection_id: get_connection_id(&sample_remote_addr()),
                transaction_id: request.transaction_id
            })
        );
    }

    #[tokio::test]
    async fn a_connect_response_should_contain_a_new_connection_id() {
        let request = ConnectRequest {
            transaction_id: TransactionId(0i32),
        };

        let response = handle_connect(sample_remote_addr(), &request, initialized_tracker())
            .await
            .unwrap();

        assert_eq!(
            response,
            Response::Connect(ConnectResponse {
                connection_id: get_connection_id(&sample_remote_addr()),
                transaction_id: request.transaction_id
            })
        );
    }

    struct TrackerStatsServiceMock {
        stats: Arc<RwLock<TrackerStatistics>>,
        expected_event: Option<TrackerStatisticsEvent>,
    }

    impl TrackerStatsServiceMock {
        fn new() -> Self {
            Self {
                stats: Arc::new(RwLock::new(TrackerStatistics::new())),
                expected_event: None,
            }
        }

        fn should_throw_event(&mut self, expected_event: TrackerStatisticsEvent) {
            self.expected_event = Some(expected_event);
        }
    }

    #[async_trait]
    impl TrackerStatisticsEventSender for TrackerStatsServiceMock {
        async fn send_event(&self, _event: TrackerStatisticsEvent) -> Option<Result<(), SendError<TrackerStatisticsEvent>>> {
            if self.expected_event.is_some() {
                assert_eq!(_event, *self.expected_event.as_ref().unwrap());
            }
            None
        }
    }

    #[async_trait]
    impl TrackerStatisticsRepository for TrackerStatsServiceMock {
        async fn get_stats(&self) -> RwLockReadGuard<'_, TrackerStatistics> {
            self.stats.read().await
        }
    }

    impl TrackerStatsService for TrackerStatsServiceMock {}

    #[tokio::test]
    async fn it_should_send_the_upd4_connect_event_when_a_client_tries_to_connect_using_a_ip4_socket_address() {
        let mut tracker_stats_service = Box::new(TrackerStatsServiceMock::new());

        let client_socket_address = sample_ipv4_socket_address();
        tracker_stats_service.should_throw_event(TrackerStatisticsEvent::Udp4Connect);

        let torrent_tracker = Arc::new(TorrentTracker::new(default_tracker_config(), tracker_stats_service).unwrap());
        handle_connect(client_socket_address, &sample_connect_request(), torrent_tracker)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn it_should_send_the_upd6_connect_event_when_a_client_tries_to_connect_using_a_ip6_socket_address() {
        let mut tracker_stats_service = Box::new(TrackerStatsServiceMock::new());

        let client_socket_address = sample_ipv6_socket_address();
        tracker_stats_service.should_throw_event(TrackerStatisticsEvent::Udp6Connect);

        let torrent_tracker = Arc::new(TorrentTracker::new(default_tracker_config(), tracker_stats_service).unwrap());
        handle_connect(client_socket_address, &sample_connect_request(), torrent_tracker)
            .await
            .unwrap();
    }
}
