use std::net::SocketAddr;
use std::time::Duration;

use aquatic_udp_protocol::TransactionId;
use hex_literal::hex;
use serde::Serialize;

use crate::console::clients::udp::checker::Client;
use crate::console::clients::udp::Error;

#[derive(Debug, Clone, Serialize)]
pub struct Checks {
    remote_addr: SocketAddr,
    results: Vec<(Check, Result<(), Error>)>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Check {
    Setup,
    Connect,
    Announce,
    Scrape,
}

#[allow(clippy::missing_panics_doc)]
pub async fn run(udp_trackers: Vec<SocketAddr>, timeout: Duration) -> Vec<Result<Checks, Checks>> {
    let mut results = Vec::default();

    tracing::debug!("UDP trackers ...");

    let info_hash = aquatic_udp_protocol::InfoHash(hex!("9c38422213e30bff212b30c360d26f9a02136422")); // # DevSkim: ignore DS173237

    for remote_addr in udp_trackers {
        let mut checks = Checks {
            remote_addr,
            results: Vec::default(),
        };

        tracing::debug!("UDP tracker: {:?}", remote_addr);

        // Setup
        let client = match Client::new(remote_addr, timeout).await {
            Ok(client) => {
                checks.results.push((Check::Setup, Ok(())));
                client
            }
            Err(err) => {
                checks.results.push((Check::Setup, Err(err)));
                results.push(Err(checks));
                break;
            }
        };

        let transaction_id = TransactionId::new(1);

        // Connect Remote
        let connection_id = match client.send_connection_request(transaction_id).await {
            Ok(connection_id) => {
                checks.results.push((Check::Connect, Ok(())));
                connection_id
            }
            Err(err) => {
                checks.results.push((Check::Connect, Err(err)));
                results.push(Err(checks));
                break;
            }
        };

        // Announce
        {
            let check = client
                .send_announce_request(transaction_id, connection_id, info_hash.into())
                .await
                .map(|_| ());

            checks.results.push((Check::Announce, check));
        }

        // Scrape
        {
            let check = client
                .send_scrape_request(connection_id, transaction_id, &[info_hash.into()])
                .await
                .map(|_| ());

            checks.results.push((Check::Announce, check));
        }

        if checks.results.iter().any(|f| f.1.is_err()) {
            results.push(Err(checks));
        } else {
            results.push(Ok(checks));
        }
    }

    results
}
