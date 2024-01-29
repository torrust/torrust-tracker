//! UDP Tracker client:
//!
//! Examples:
//!
//! Announce request:
//!
//! ```text
//! cargo run --bin udp_tracker_client announce 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
//!
//! Announce response:
//!
//! ```json
//! {
//!   "transaction_id": -888840697
//!   "announce_interval": 120,
//!   "leechers": 0,
//!   "seeders": 1,
//!   "peers": [
//!     "123.123.123.123:51289"
//!   ],
//! }
//! ```
//!
//! Scrape request:
//!
//! ```text
//! cargo run --bin udp_tracker_client scrape 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
//!
//! Scrape response:
//!
//! ```json
//! {
//!   "transaction_id": -888840697,
//!   "torrent_stats": [
//!     {
//!       "completed": 0,
//!       "leechers": 0,
//!       "seeders": 0
//!     },
//!     {
//!       "completed": 0,
//!       "leechers": 0,
//!       "seeders": 0
//!    }
//!  ]
//! }
//! ```
//!
//! You can use an URL with instead of the socket address. For example:
//!
//! ```text
//! cargo run --bin udp_tracker_client scrape udp://localhost:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! cargo run --bin udp_tracker_client scrape udp://localhost:6969/scrape 9c38422213e30bff212b30c360d26f9a02136422 | jq
//! ```
//!
//! The protocol (`udp://`) in the URL is mandatory. The path (`\scrape`) is optional. It always uses `\scrape`.
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use anyhow::Context;
use aquatic_udp_protocol::common::InfoHash;
use aquatic_udp_protocol::Response::{AnnounceIpv4, AnnounceIpv6, Scrape};
use aquatic_udp_protocol::{
    AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, NumberOfBytes, NumberOfPeers, PeerId, PeerKey, Port, Response,
    ScrapeRequest, TransactionId,
};
use clap::{Parser, Subcommand};
use log::{debug, LevelFilter};
use serde_json::json;
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash as TorrustInfoHash;
use torrust_tracker::shared::bit_torrent::tracker::udp::client::{UdpClient, UdpTrackerClient};
use url::Url;

const ASSIGNED_BY_OS: i32 = 0;
const RANDOM_TRANSACTION_ID: i32 = -888_840_697;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Announce {
        #[arg(value_parser = parse_socket_addr)]
        tracker_socket_addr: SocketAddr,
        #[arg(value_parser = parse_info_hash)]
        info_hash: TorrustInfoHash,
    },
    Scrape {
        #[arg(value_parser = parse_socket_addr)]
        tracker_socket_addr: SocketAddr,
        #[arg(value_parser = parse_info_hash, num_args = 1..=74, value_delimiter = ' ')]
        info_hashes: Vec<TorrustInfoHash>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging(LevelFilter::Info);

    let args = Args::parse();

    // Configuration
    let local_port = ASSIGNED_BY_OS;
    let local_bind_to = format!("0.0.0.0:{local_port}");
    let transaction_id = RANDOM_TRANSACTION_ID;

    // Bind to local port
    debug!("Binding to: {local_bind_to}");
    let udp_client = UdpClient::bind(&local_bind_to).await;
    let bound_to = udp_client.socket.local_addr().unwrap();
    debug!("Bound to:   {bound_to}");

    let transaction_id = TransactionId(transaction_id);

    let response = match args.command {
        Command::Announce {
            tracker_socket_addr,
            info_hash,
        } => {
            let (connection_id, udp_tracker_client) = connect(&tracker_socket_addr, udp_client, transaction_id).await;

            send_announce_request(
                connection_id,
                transaction_id,
                info_hash,
                Port(bound_to.port()),
                &udp_tracker_client,
            )
            .await
        }
        Command::Scrape {
            tracker_socket_addr,
            info_hashes,
        } => {
            let (connection_id, udp_tracker_client) = connect(&tracker_socket_addr, udp_client, transaction_id).await;
            send_scrape_request(connection_id, transaction_id, info_hashes, &udp_tracker_client).await
        }
    };

    match response {
        AnnounceIpv4(announce) => {
            let json = json!({
                "transaction_id": announce.transaction_id.0,
                "announce_interval": announce.announce_interval.0,
                "leechers": announce.leechers.0,
                "seeders": announce.seeders.0,
                "peers": announce.peers.iter().map(|peer| format!("{}:{}", peer.ip_address, peer.port.0)).collect::<Vec<_>>(),
            });
            let pretty_json = serde_json::to_string_pretty(&json).unwrap();
            println!("{pretty_json}");
        }
        AnnounceIpv6(announce) => {
            let json = json!({
                "transaction_id": announce.transaction_id.0,
                "announce_interval": announce.announce_interval.0,
                "leechers": announce.leechers.0,
                "seeders": announce.seeders.0,
                "peers6": announce.peers.iter().map(|peer| format!("{}:{}", peer.ip_address, peer.port.0)).collect::<Vec<_>>(),
            });
            let pretty_json = serde_json::to_string_pretty(&json).unwrap();
            println!("{pretty_json}");
        }
        Scrape(scrape) => {
            let json = json!({
                "transaction_id": scrape.transaction_id.0,
                "torrent_stats": scrape.torrent_stats.iter().map(|torrent_scrape_statistics| json!({
                    "seeders": torrent_scrape_statistics.seeders.0,
                    "completed": torrent_scrape_statistics.completed.0,
                    "leechers": torrent_scrape_statistics.leechers.0,
                })).collect::<Vec<_>>(),
            });
            let pretty_json = serde_json::to_string_pretty(&json).unwrap();
            println!("{pretty_json}");
        }
        _ => println!("{response:#?}"), // todo: serialize to JSON all responses.
    }

    Ok(())
}

fn setup_logging(level: LevelFilter) {
    if let Err(_err) = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("%+"),
                record.target(),
                record.level(),
                message
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
    {
        panic!("Failed to initialize logging.")
    }

    debug!("logging initialized.");
}

fn parse_socket_addr(tracker_socket_addr_str: &str) -> anyhow::Result<SocketAddr> {
    debug!("Tracker socket address: {tracker_socket_addr_str:#?}");

    // Check if the address is a valid URL. If so, extract the host and port.
    let resolved_addr = if let Ok(url) = Url::parse(tracker_socket_addr_str) {
        debug!("Tracker socket address URL: {url:?}");

        let host = url
            .host_str()
            .with_context(|| format!("invalid host in URL: `{tracker_socket_addr_str}`"))?
            .to_owned();

        let port = url
            .port()
            .with_context(|| format!("port not found in URL: `{tracker_socket_addr_str}`"))?
            .to_owned();

        (host, port)
    } else {
        // If not a URL, assume it's a host:port pair.

        let parts: Vec<&str> = tracker_socket_addr_str.split(':').collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "invalid address format: `{}`. Expected format is host:port",
                tracker_socket_addr_str
            ));
        }

        let host = parts[0].to_owned();

        let port = parts[1]
            .parse::<u16>()
            .with_context(|| format!("invalid port: `{}`", parts[1]))?
            .to_owned();

        (host, port)
    };

    debug!("Resolved address: {resolved_addr:#?}");

    // Perform DNS resolution.
    let socket_addrs: Vec<_> = resolved_addr.to_socket_addrs()?.collect();
    if socket_addrs.is_empty() {
        Err(anyhow::anyhow!("DNS resolution failed for `{}`", tracker_socket_addr_str))
    } else {
        Ok(socket_addrs[0])
    }
}

fn parse_info_hash(info_hash_str: &str) -> anyhow::Result<TorrustInfoHash> {
    TorrustInfoHash::from_str(info_hash_str)
        .map_err(|e| anyhow::Error::msg(format!("failed to parse info-hash `{info_hash_str}`: {e:?}")))
}

async fn connect(
    tracker_socket_addr: &SocketAddr,
    udp_client: UdpClient,
    transaction_id: TransactionId,
) -> (ConnectionId, UdpTrackerClient) {
    debug!("Connecting to tracker: udp://{tracker_socket_addr}");

    udp_client.connect(&tracker_socket_addr.to_string()).await;

    let udp_tracker_client = UdpTrackerClient { udp_client };

    let connection_id = send_connection_request(transaction_id, &udp_tracker_client).await;

    (connection_id, udp_tracker_client)
}

async fn send_connection_request(transaction_id: TransactionId, client: &UdpTrackerClient) -> ConnectionId {
    debug!("Sending connection request with transaction id: {transaction_id:#?}");

    let connect_request = ConnectRequest { transaction_id };

    client.send(connect_request.into()).await;

    let response = client.receive().await;

    debug!("connection request response:\n{response:#?}");

    match response {
        Response::Connect(connect_response) => connect_response.connection_id,
        _ => panic!("error connecting to udp server. Unexpected response"),
    }
}

async fn send_announce_request(
    connection_id: ConnectionId,
    transaction_id: TransactionId,
    info_hash: TorrustInfoHash,
    port: Port,
    client: &UdpTrackerClient,
) -> Response {
    debug!("Sending announce request with transaction id: {transaction_id:#?}");

    let announce_request = AnnounceRequest {
        connection_id,
        transaction_id,
        info_hash: InfoHash(info_hash.bytes()),
        peer_id: PeerId(*b"-qB00000000000000001"),
        bytes_downloaded: NumberOfBytes(0i64),
        bytes_uploaded: NumberOfBytes(0i64),
        bytes_left: NumberOfBytes(0i64),
        event: AnnounceEvent::Started,
        ip_address: Some(Ipv4Addr::new(0, 0, 0, 0)),
        key: PeerKey(0u32),
        peers_wanted: NumberOfPeers(1i32),
        port,
    };

    client.send(announce_request.into()).await;

    let response = client.receive().await;

    debug!("announce request response:\n{response:#?}");

    response
}

async fn send_scrape_request(
    connection_id: ConnectionId,
    transaction_id: TransactionId,
    info_hashes: Vec<TorrustInfoHash>,
    client: &UdpTrackerClient,
) -> Response {
    debug!("Sending scrape request with transaction id: {transaction_id:#?}");

    let scrape_request = ScrapeRequest {
        connection_id,
        transaction_id,
        info_hashes: info_hashes
            .iter()
            .map(|torrust_info_hash| InfoHash(torrust_info_hash.bytes()))
            .collect(),
    };

    client.send(scrape_request.into()).await;

    let response = client.receive().await;

    debug!("scrape request response:\n{response:#?}");

    response
}
