use std::net::SocketAddr;
use std::num::NonZeroU16;
use std::time::Duration;

use hex_literal::hex;
use torrust_tracker_primitives::info_hash::InfoHash;
use tracing::debug;

use crate::console::clients::checker::console::Console;
use crate::console::clients::checker::printer::Printer as _;
use crate::console::clients::checker::service::{CheckError, CheckResult};
use crate::console::clients::udp::checker::{self, Client};
use crate::console::clients::udp::Error;

pub async fn run(udp_trackers: Vec<SocketAddr>, timeout: Duration, console: Console) -> Vec<CheckResult> {
    let mut check_results = Vec::default();

    console.println("UDP trackers ...");

    let info_hash = InfoHash(hex!("9c38422213e30bff212b30c360d26f9a02136422")); // # DevSkim: ignore DS173237

    for ref addr in udp_trackers {
        debug!("UDP tracker: {:?}", addr);

        // Setup Connection
        let Ok((client, ctx)) = ({
            let res = setup_connection(addr, &timeout).await;

            check_results.push(match res {
                Ok(_) => {
                    console.println(&format!("{} - Setup of {} is OK", "✓", addr));
                    Ok(())
                }
                Err(ref e) => {
                    console.println(&format!("{} - Setup of {} is failing", "✗", addr));
                    Err(CheckError::UdpCheckError {
                        addr: *addr,
                        err: e.clone(),
                    })
                }
            });

            res
        }) else {
            break;
        };

        // Do Announce
        if {
            let res = check_udp_announce(&client, &ctx, info_hash).await;

            check_results.push(match res {
                Ok(_) => {
                    console.println(&format!("{} - Announce of {} is OK", "✓", addr));
                    Ok(())
                }
                Err(ref e) => {
                    console.println(&format!("{} - Announce of {} is failing", "✗", addr));
                    Err(CheckError::UdpCheckError {
                        addr: *addr,
                        err: e.clone(),
                    })
                }
            });

            res
        }
        .is_err()
        {
            break;
        };

        // Do Scrape
        if {
            let res = check_udp_scrape(&client, &ctx, &[info_hash]).await;

            check_results.push(match res {
                Ok(_) => {
                    console.println(&format!("{} - Announce of {} is OK", "✓", addr));
                    Ok(())
                }
                Err(ref e) => {
                    console.println(&format!("{} - Announce of {} is failing", "✗", addr));
                    Err(CheckError::UdpCheckError {
                        addr: *addr,
                        err: e.clone(),
                    })
                }
            });
            res
        }
        .is_err()
        {
            break;
        };
    }

    check_results
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
