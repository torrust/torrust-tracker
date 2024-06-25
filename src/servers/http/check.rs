use std::net::SocketAddr;
use std::sync::Arc;

use futures::{FutureExt as _, TryFutureExt as _};
use torrust_tracker_located_error::DynError;
use tracing::info;

use crate::servers::registar;

/// Build a health check future task for checking
/// the http tracker health check endpoint.
///
#[must_use]
pub(super) fn check_builder(addr: SocketAddr) -> registar::HeathCheckFuture<'static> {
    let url = format!("http://{addr}/health_check");

    info!("checking http tracker health check at: {url}");

    let response = reqwest::get(url)
        .map_err(move |e| registar::Error::UnableToGetAnyResponse {
            addr,
            msg: "Udp Client".to_string(),
            err: DynError::into(Arc::new(e)),
        })
        .boxed();

    let check = response
        .and_then(move |r| async move {
            r.error_for_status()
                .map_err(move |e| registar::Error::UnableToObtainGoodResponse {
                    addr,
                    msg: "Udp Client".to_string(),
                    err: DynError::into(Arc::new(e)),
                })
        })
        .boxed();

    check
        .map_ok(move |r| registar::Success::AllGood {
            addr,
            msg: r.status().to_string(),
        })
        .map(registar::HeathCheckResult::from)
        .boxed()
}
