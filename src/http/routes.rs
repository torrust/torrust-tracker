use std::convert::Infallible;
use std::sync::Arc;

use warp::{Filter, Rejection};

use super::filters::{with_announce_request, with_auth_key, with_scrape_request, with_tracker};
use super::handlers::{handle_announce, handle_scrape, send_error};
use crate::tracker;

/// All routes
#[must_use]
pub fn routes(tracker: Arc<tracker::Tracker>) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    announce(tracker.clone())
        .or(scrape(tracker))
        .recover(|q| async move { send_error(&q) })
}

/// GET /announce or /announce/<key>
fn announce(tracker: Arc<tracker::Tracker>) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path::path("announce")
        .and(warp::filters::method::get())
        .and(with_announce_request(tracker.config.on_reverse_proxy))
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_announce)
}

/// GET /scrape/<key>
fn scrape(tracker: Arc<tracker::Tracker>) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path::path("scrape")
        .and(warp::filters::method::get())
        .and(with_scrape_request(tracker.config.on_reverse_proxy))
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_scrape)
}
