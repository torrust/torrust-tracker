//! A fake UDP tracker (BEP 15) that answers connect, announce, and scrape requests.
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};

use torrust_tracker_udp_protocol::{
    AnnounceResponse, ConnectResponse, ConnectionId, Ipv4AddrBytes, NumberOfDownloads, NumberOfPeers, Request, Response,
    ScrapeResponse, TorrentScrapeStatistics,
};

use super::{POLL_INTERVAL, ServerThread};

pub struct FakeUdpTracker {
    address: SocketAddr,
    _server: ServerThread,
}

#[derive(Clone, Copy)]
enum Behavior {
    Answering { max_scrape_info_hashes: usize },
    Silent,
}

impl FakeUdpTracker {
    /// Answers every request; announces get no peers.
    pub fn answering() -> Self {
        Self::keeping_first(usize::MAX)
    }

    /// Like [`Self::answering`], but scrapes get entries for only the first
    /// `max_info_hashes` info hashes.
    pub fn keeping_first(max_info_hashes: usize) -> Self {
        Self::start(Behavior::Answering {
            max_scrape_info_hashes: max_info_hashes,
        })
    }

    /// Receives and discards every request, so every client request times out.
    pub fn silent() -> Self {
        Self::start(Behavior::Silent)
    }

    pub const fn address(&self) -> SocketAddr {
        self.address
    }

    fn start(behavior: Behavior) -> Self {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("fake UDP tracker should bind");
        socket
            .set_read_timeout(Some(POLL_INTERVAL))
            .expect("fake UDP tracker should set a read timeout");
        let address = socket.local_addr().expect("fake UDP tracker should have an address");

        let server = ServerThread::spawn(move |stop| serve(&socket, behavior, stop));

        Self {
            address,
            _server: server,
        }
    }
}

fn serve(socket: &UdpSocket, behavior: Behavior, stop: &AtomicBool) {
    // Larger than any datagram, so receiving never truncates a request.
    let mut buffer = vec![0_u8; 65_535];

    while !stop.load(Ordering::Relaxed) {
        let (size, client) = match socket.recv_from(&mut buffer) {
            Ok(received) => received,
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => continue,
            Err(error) => panic!("fake UDP tracker failed to receive: {error}"),
        };

        let Behavior::Answering { max_scrape_info_hashes } = behavior else {
            continue;
        };

        let request = Request::parse_bytes(&buffer[..size], max_scrape_info_hashes)
            .expect("fake UDP tracker should receive valid requests");

        let mut response = Vec::new();
        respond_to(request)
            .write_bytes(&mut response)
            .expect("fake UDP tracker should serialize its response");
        socket
            .send_to(&response, client)
            .expect("fake UDP tracker should send its response");
    }
}

fn respond_to(request: Request) -> Response {
    match request {
        Request::Connect(connect) => Response::Connect(ConnectResponse {
            transaction_id: connect.transaction_id,
            connection_id: ConnectionId::new(1),
        }),
        Request::Announce(announce) => {
            let mut response = AnnounceResponse::<Ipv4AddrBytes>::empty();
            response.fixed.transaction_id = announce.transaction_id;
            Response::AnnounceIpv4(response)
        }
        Request::Scrape(scrape) => Response::Scrape(ScrapeResponse {
            transaction_id: scrape.transaction_id,
            torrent_stats: scrape.info_hashes.iter().map(|_| no_peers()).collect(),
        }),
    }
}

fn no_peers() -> TorrentScrapeStatistics {
    TorrentScrapeStatistics {
        seeders: NumberOfPeers(0.into()),
        completed: NumberOfDownloads(0.into()),
        leechers: NumberOfPeers(0.into()),
    }
}
