use std::net::SocketAddr;

use aquatic_udp_protocol::{Port, TransactionId};
use colored::Colorize;
use hex_literal::hex;
use torrust_tracker_primitives::info_hash::InfoHash;
use tracing::debug;

use crate::console::clients::checker::console::Console;
use crate::console::clients::checker::printer::Printer;
use crate::console::clients::checker::service::{CheckError, CheckResult};
use crate::console::clients::udp::checker;

const ASSIGNED_BY_OS: u16 = 0;
const RANDOM_TRANSACTION_ID: i32 = -888_840_697;

pub async fn run(udp_trackers: &Vec<SocketAddr>, console: &Console, check_results: &mut Vec<CheckResult>) {
    console.println("UDP trackers ...");

    for udp_tracker in udp_trackers {
        debug!("UDP tracker: {:?}", udp_tracker);

        let colored_tracker_url = udp_tracker.to_string().yellow();

        let transaction_id = TransactionId(RANDOM_TRANSACTION_ID);

        let mut client = checker::Client::default();

        debug!("Bind and connect");

        let Ok(bound_to) = client.bind_and_connect(ASSIGNED_BY_OS, udp_tracker).await else {
            check_results.push(Err(CheckError::UdpError {
                socket_addr: *udp_tracker,
            }));
            console.println(&format!("{} - Can't connect to socket {}", "✗".red(), colored_tracker_url));
            break;
        };

        debug!("Send connection request");

        let Ok(connection_id) = client.send_connection_request(transaction_id).await else {
            check_results.push(Err(CheckError::UdpError {
                socket_addr: *udp_tracker,
            }));
            console.println(&format!(
                "{} - Can't make tracker connection request to {}",
                "✗".red(),
                colored_tracker_url
            ));
            break;
        };

        let info_hash = InfoHash(hex!("9c38422213e30bff212b30c360d26f9a02136422")); // # DevSkim: ignore DS173237

        debug!("Send announce request");

        if (client
            .send_announce_request(connection_id, transaction_id, info_hash, Port(bound_to.port()))
            .await)
            .is_ok()
        {
            check_results.push(Ok(()));
            console.println(&format!("{} - Announce at {} is OK", "✓".green(), colored_tracker_url));
        } else {
            let err = CheckError::UdpError {
                socket_addr: *udp_tracker,
            };
            check_results.push(Err(err));
            console.println(&format!("{} - Announce at {} is failing", "✗".red(), colored_tracker_url));
        }

        debug!("Send scrape request");

        let info_hashes = vec![InfoHash(hex!("9c38422213e30bff212b30c360d26f9a02136422"))]; // # DevSkim: ignore DS173237

        if (client.send_scrape_request(connection_id, transaction_id, info_hashes).await).is_ok() {
            check_results.push(Ok(()));
            console.println(&format!("{} - Announce at {} is OK", "✓".green(), colored_tracker_url));
        } else {
            let err = CheckError::UdpError {
                socket_addr: *udp_tracker,
            };
            check_results.push(Err(err));
            console.println(&format!("{} - Announce at {} is failing", "✗".red(), colored_tracker_url));
        }
    }
}
