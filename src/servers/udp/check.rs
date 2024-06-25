use std::net::SocketAddr;

use futures::{FutureExt as _, TryFutureExt as _};
use torrust_tracker_configuration::CLIENT_TIMEOUT_DEFAULT;
use tracing::info;

use crate::servers::registar;
use crate::shared::bit_torrent::tracker::udp::Client;

pub fn build(addr: SocketAddr) -> registar::HeathCheckFuture<'static> {
    info!("checking udp: {addr}");

    let client = Client::connect(addr, CLIENT_TIMEOUT_DEFAULT)
        .map_err(move |e| registar::Error::UnableToConnectToRemote {
            addr,
            msg: "Udp Client".to_string(),
            err: e.into(),
        })
        .boxed();

    let check = client
        .and_then(move |c| {
            c.check().map_err(move |e| registar::Error::UnableToPreformCheck {
                addr,
                msg: "Udp Client".to_string(),
                err: e.into(),
            })
        })
        .boxed();

    check
        .map_ok(move |msg| registar::Success::AllGood { addr, msg })
        .map(registar::HeathCheckResult::from)
        .boxed()
}
