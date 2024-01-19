use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;

use aquatic_udp_protocol::common::InfoHash;
use aquatic_udp_protocol::{
    AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, NumberOfBytes, NumberOfPeers, PeerId, PeerKey, Port, Response,
    TransactionId,
};
use log::{debug, LevelFilter};
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash as TorrustInfoHash;
use torrust_tracker::shared::bit_torrent::tracker::udp::client::{UdpClient, UdpTrackerClient};

const ASSIGNED_BY_OS: i32 = 0;
const RANDOM_TRANSACTION_ID: i32 = -888_840_697;

#[tokio::main]
async fn main() {
    setup_logging(LevelFilter::Info);

    let (remote_socket_addr, info_hash) = parse_arguments();

    // Configuration
    let local_port = ASSIGNED_BY_OS;
    let transaction_id = RANDOM_TRANSACTION_ID;
    let bind_to = format!("0.0.0.0:{local_port}");

    // Bind to local port

    debug!("Binding to: {bind_to}");
    let udp_client = UdpClient::bind(&bind_to).await;
    let bound_to = udp_client.socket.local_addr().unwrap();
    debug!("Bound to:   {bound_to}");

    // Connect to remote socket

    debug!("Connecting to remote: udp://{remote_socket_addr}");
    udp_client.connect(&remote_socket_addr).await;

    let udp_tracker_client = UdpTrackerClient { udp_client };

    let transaction_id = TransactionId(transaction_id);

    let connection_id = send_connection_request(transaction_id, &udp_tracker_client).await;

    let response = send_announce_request(
        connection_id,
        transaction_id,
        info_hash,
        Port(bound_to.port()),
        &udp_tracker_client,
    )
    .await;

    println!("{response:#?}");
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

fn parse_arguments() -> (String, TorrustInfoHash) {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Error: invalid number of arguments!");
        eprintln!("Usage:   cargo run --bin udp_tracker_client <UDP_TRACKER_SOCKET_ADDRESS> <INFO_HASH>");
        eprintln!("Example: cargo run --bin udp_tracker_client 144.126.245.19:6969 9c38422213e30bff212b30c360d26f9a02136422");
        std::process::exit(1);
    }

    let remote_socket_addr = &args[1];
    let _valid_socket_addr = remote_socket_addr.parse::<SocketAddr>().unwrap_or_else(|_| {
        panic!(
            "Invalid argument: `{}`. Argument 1 should be a valid socket address. For example: `144.126.245.19:6969`.",
            args[1]
        )
    });
    let info_hash = TorrustInfoHash::from_str(&args[2]).unwrap_or_else(|_| {
        panic!(
            "Invalid argument: `{}`. Argument 2 should be a valid infohash. For example: `9c38422213e30bff212b30c360d26f9a02136422`.",
            args[2]
        )
    });

    (remote_socket_addr.to_string(), info_hash)
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
