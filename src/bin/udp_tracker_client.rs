//! UDP Tracker client:
//!
//! Examples:
//!
//! Announce request:
//!
//! ```text
//! cargo run --bin udp_tracker_client 127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422 | jq
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
/// ````
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;

use anyhow::Context;
use aquatic_udp_protocol::common::InfoHash;
use aquatic_udp_protocol::Response::{AnnounceIpv4, AnnounceIpv6};
use aquatic_udp_protocol::{
    AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, NumberOfBytes, NumberOfPeers, PeerId, PeerKey, Port, Response,
    TransactionId,
};
use clap::{Parser, Subcommand};
use log::{debug, LevelFilter};
use serde_json::json;
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash as TorrustInfoHash;
use torrust_tracker::shared::bit_torrent::tracker::udp::client::{UdpClient, UdpTrackerClient};

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging(LevelFilter::Info);

    let args = Args::parse();

    // Configuration
    let local_port = ASSIGNED_BY_OS;
    let transaction_id = RANDOM_TRANSACTION_ID;
    let bind_to = format!("0.0.0.0:{local_port}");

    // Bind to local port
    debug!("Binding to: {bind_to}");
    let udp_client = UdpClient::bind(&bind_to).await;
    let bound_to = udp_client.socket.local_addr().unwrap();
    debug!("Bound to:   {bound_to}");

    let response = match args.command {
        Command::Announce {
            tracker_socket_addr,
            info_hash,
        } => {
            debug!("Connecting to remote: udp://{tracker_socket_addr}");

            udp_client.connect(&tracker_socket_addr.to_string()).await;

            let udp_tracker_client = UdpTrackerClient { udp_client };

            let transaction_id = TransactionId(transaction_id);

            let connection_id = send_connection_request(transaction_id, &udp_tracker_client).await;

            send_announce_request(
                connection_id,
                transaction_id,
                info_hash,
                Port(bound_to.port()),
                &udp_tracker_client,
            )
            .await
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
        _ => println!("{response:#?}"),
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

fn parse_socket_addr(s: &str) -> anyhow::Result<SocketAddr> {
    s.parse().with_context(|| format!("failed to parse socket address: `{s}`"))
}

fn parse_info_hash(s: &str) -> anyhow::Result<TorrustInfoHash> {
    TorrustInfoHash::from_str(s).map_err(|e| anyhow::Error::msg(format!("failed to parse info-hash `{s}`: {e:?}")))
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
