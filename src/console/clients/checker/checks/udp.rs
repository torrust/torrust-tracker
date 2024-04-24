use std::net::SocketAddr;

use aquatic_udp_protocol::{Port, TransactionId};
use hex_literal::hex;
use log::debug;
use torrust_tracker_primitives::info_hash::InfoHash;

use crate::console::clients::checker::service::{CheckError, CheckResult};
use crate::console::clients::udp::checker;

use crate::console::clients::checker::checks::structs::{CheckerOutput, Status};

const ASSIGNED_BY_OS: u16 = 0;
const RANDOM_TRANSACTION_ID: i32 = -888_840_697;

#[allow(clippy::missing_panics_doc)]
pub async fn run(udp_trackers: &Vec<SocketAddr>, check_results: &mut Vec<CheckResult>) -> Vec<CheckerOutput> {
    let mut udp_checkers: Vec<CheckerOutput> = Vec::new();

    for udp_tracker in udp_trackers {
        let mut checker_output = CheckerOutput {
            url: udp_tracker.to_string(),
            status: Status {
                code: String::new(),
                message: String::new(),
            },
        };

        debug!("UDP tracker: {:?}", udp_tracker);

        let transaction_id = TransactionId(RANDOM_TRANSACTION_ID);

        let mut client = checker::Client::default();

        debug!("Bind and connect");

        let Ok(bound_to) = client.bind_and_connect(ASSIGNED_BY_OS, udp_tracker).await else {
            check_results.push(Err(CheckError::UdpError {
                socket_addr: *udp_tracker,
            }));
            checker_output.status.code = "error".to_string();
            checker_output.status.message = "Can't connect to socket.".to_string();
            break;
        };

        debug!("Send connection request");

        let Ok(connection_id) = client.send_connection_request(transaction_id).await else {
            check_results.push(Err(CheckError::UdpError {
                socket_addr: *udp_tracker,
            }));
            checker_output.status.code = "error".to_string();
            checker_output.status.message = "Can't make tracker connection request.".to_string();
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
            checker_output.status.code = "ok".to_string();
        } else {
            let err = CheckError::UdpError {
                socket_addr: *udp_tracker,
            };
            check_results.push(Err(err));
            checker_output.status.code = "error".to_string();
            checker_output.status.message = "Announce is failing.".to_string();
        }

        debug!("Send scrape request");

        let info_hashes = vec![InfoHash(hex!("9c38422213e30bff212b30c360d26f9a02136422"))]; // # DevSkim: ignore DS173237

        if (client.send_scrape_request(connection_id, transaction_id, info_hashes).await).is_ok() {
            check_results.push(Ok(()));
            checker_output.status.code = "ok".to_string();
        } else {
            let err = CheckError::UdpError {
                socket_addr: *udp_tracker,
            };
            check_results.push(Err(err));
            checker_output.status.code = "error".to_string();
            checker_output.status.message = "Scrape is failing.".to_string();
        }
        udp_checkers.push(checker_output);
    }
    udp_checkers
}
