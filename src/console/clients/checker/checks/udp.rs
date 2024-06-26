use std::net::SocketAddr;
use std::num::NonZeroU16;
use std::time::Duration;

use hex_literal::hex;
use serde::Serialize;
use torrust_tracker_primitives::info_hash::InfoHash;

use crate::console::clients::udp::checker::{self, Client};
use crate::console::clients::udp::Error;

#[derive(Debug, Clone, Serialize)]
pub struct Checks {
    addr: SocketAddr,
    results: Vec<(Check, Result<(), Error>)>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Check {
    Setup,
    Announce,
    Scrape,
}

pub async fn run(udp_trackers: Vec<SocketAddr>, timeout: Duration) -> Vec<Result<Checks, Checks>> {
    let mut results = Vec::default();

    tracing::debug!("UDP trackers ...");

    let info_hash = InfoHash(hex!("9c38422213e30bff212b30c360d26f9a02136422")); // # DevSkim: ignore DS173237

    for ref addr in udp_trackers {
        let mut checks = Checks {
            addr: *addr,
            results: Vec::default(),
        };

        tracing::debug!("UDP tracker: {:?}", addr);

        // Setup Connection
        let (client, ctx) = match setup_connection(addr, &timeout).await {
            Ok((client, ctx)) => {
                checks.results.push((Check::Setup, Ok(())));
                (client, ctx)
            }
            Err(err) => {
                checks.results.push((Check::Setup, Err(err)));
                results.push(Err(checks));
                break;
            }
        };

        // Announce
        {
            let check = check_udp_announce(&client, &ctx, info_hash).await.map(|_| ());

            checks.results.push((Check::Announce, check));
        }

        // Scrape
        {
            let check = check_udp_scrape(&client, &ctx, &[info_hash]).await.map(|_| ());

            checks.results.push((Check::Scrape, check));
        }

        if checks.results.iter().any(|f| f.1.is_err()) {
            results.push(Err(checks));
        } else {
            results.push(Ok(checks));
        }
    }

    results
}

async fn setup_connection(
    addr: &SocketAddr,
    timeout: &Duration,
) -> Result<(Client, aquatic_udp_protocol::ConnectResponse), Error> {
    let client = checker::Client::bind_and_connect(addr, timeout).await?;

    let transaction_id = aquatic_udp_protocol::TransactionId::new(rand::Rng::gen(&mut rand::thread_rng()));

    let ctx = client.send_connection_request(transaction_id).await?;

    Ok((client, ctx))
}

async fn check_udp_announce(
    client: &Client,
    ctx: &aquatic_udp_protocol::ConnectResponse,
    info_hash: InfoHash,
) -> Result<aquatic_udp_protocol::Response, Error> {
    let port = NonZeroU16::new(client.local_addr()?.port()).expect("it should be non-zero");

    client
        .send_announce_request(ctx, info_hash, aquatic_udp_protocol::Port::new(port))
        .await
}

async fn check_udp_scrape(
    client: &Client,
    ctx: &aquatic_udp_protocol::ConnectResponse,
    info_hashes: &[InfoHash],
) -> Result<aquatic_udp_protocol::Response, Error> {
    client.send_scrape_request(ctx, info_hashes).await
}
