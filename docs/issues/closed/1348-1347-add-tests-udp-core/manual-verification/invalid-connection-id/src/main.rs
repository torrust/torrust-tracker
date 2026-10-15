//! Sends UDP tracker requests that carry an invalid connection ID (zero), or a
//! valid connect, from a chosen local source IP, and prints each response.
//!
//! ```text
//! invalid-connection-id-client <invalid-announce|invalid-scrape|connect> <tracker-addr> [source-ip] [count]
//! ```
//!
//! Linux accepts any `127.0.0.0/8` source address on loopback, so a source IP
//! such as `127.0.0.2` simulates a separate client for ban tests.
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::num::NonZeroU16;
use std::process::ExitCode;
use std::time::Duration;

use torrust_tracker_client::udp::client::{UdpClient, UdpTrackerClient};
use torrust_tracker_udp_protocol::{
    AnnounceActionPlaceholder, AnnounceEvent, AnnounceRequest, ConnectRequest, ConnectionId, InfoHash, NumberOfBytes,
    NumberOfPeers, PeerKey, Port, Request, ScrapeRequest, TransactionId,
};

const TIMEOUT: Duration = Duration::from_secs(2);
const USAGE: &str =
    "usage: invalid-connection-id-client <invalid-announce|invalid-scrape|connect> <tracker-addr> [source-ip] [count]";

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(command), Some(tracker)) = (args.first(), args.get(1)) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let Ok(tracker) = tracker.parse::<SocketAddr>() else {
        eprintln!("invalid tracker address: {tracker}");
        return ExitCode::from(2);
    };
    let source_ip: IpAddr = args
        .get(2)
        .map_or(Ok(IpAddr::V4(Ipv4Addr::LOCALHOST)), |ip| ip.parse())
        .unwrap_or_else(|_| {
            eprintln!("invalid source IP");
            std::process::exit(2);
        });
    let count: u32 = args.get(3).map_or(Ok(1), |count| count.parse()).unwrap_or_else(|_| {
        eprintln!("invalid count");
        std::process::exit(2);
    });

    let client = match bound_client(source_ip, tracker).await {
        Ok(client) => client,
        Err(error) => {
            eprintln!("cannot create a UDP client from {source_ip}: {error}");
            return ExitCode::FAILURE;
        }
    };

    for transaction in 1..=count {
        let transaction_id = TransactionId::new(i32::try_from(transaction).unwrap_or(i32::MAX));
        let request: Request = match command.as_str() {
            "invalid-announce" => invalid_announce(transaction_id).into(),
            "invalid-scrape" => invalid_scrape(transaction_id).into(),
            "connect" => ConnectRequest { transaction_id }.into(),
            _ => {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
        };

        if let Err(error) = client.send(request).await {
            eprintln!("request {transaction}: send failed: {error}");
            return ExitCode::FAILURE;
        }

        match client.receive().await {
            Ok(response) => println!("request {transaction} from {source_ip}: {response:?}"),
            Err(error) => println!("request {transaction} from {source_ip}: no response ({error})"),
        }
    }

    ExitCode::SUCCESS
}

async fn bound_client(source_ip: IpAddr, tracker: SocketAddr) -> Result<UdpTrackerClient, String> {
    let client = UdpClient::bound(SocketAddr::new(source_ip, 0), TIMEOUT)
        .await
        .map_err(|error| error.to_string())?;
    client.connect(tracker).await.map_err(|error| error.to_string())?;
    Ok(UdpTrackerClient { client })
}

fn invalid_announce(transaction_id: TransactionId) -> AnnounceRequest {
    AnnounceRequest {
        connection_id: ConnectionId::new(0),
        action_placeholder: AnnounceActionPlaceholder::default(),
        transaction_id,
        info_hash: InfoHash([0; 20]),
        peer_id: torrust_peer_id::PeerId([0; 20]),
        bytes_downloaded: NumberOfBytes::new(0),
        bytes_uploaded: NumberOfBytes::new(0),
        bytes_left: NumberOfBytes::new(0),
        event: AnnounceEvent::Started.into(),
        ip_address: Ipv4Addr::UNSPECIFIED.into(),
        key: PeerKey::new(0),
        peers_wanted: NumberOfPeers::new(1),
        port: Port::new(NonZeroU16::new(6881).expect("a non-zero port")),
    }
}

fn invalid_scrape(transaction_id: TransactionId) -> ScrapeRequest {
    ScrapeRequest {
        connection_id: ConnectionId::new(0),
        transaction_id,
        info_hashes: vec![InfoHash([0; 20])],
    }
}
