use std::net::SocketAddr;

use futures::FutureExt as _;
use tracing::instrument;

use crate::servers::registar;

/// Placeholder Check Function for the Health Check Itself
///
/// # Errors
///
/// This function will return an error if the check would fail.
///
#[must_use]
#[instrument]
pub fn check_builder(addr: SocketAddr) -> registar::HeathCheckFuture<'static> {
    let url = format!("http://{addr}/health_check");

    let msg = format!("todo self-check health check at: {url}");

    let success = registar::Success::AllGood { addr, msg };

    let result = registar::HeathCheckResult::from(Ok(success));

    async move { result }.boxed()
}
