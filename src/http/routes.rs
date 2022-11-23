use std::convert::Infallible;
use std::sync::Arc;

use warp::{Filter, Rejection};

use crate::http::{
    handle_announce, handle_scrape, send_error, with_announce_request, with_auth_key, with_scrape_request, with_tracker,
};
use crate::tracker::TorrentTracker;

/// All routes
pub fn routes(tracker: Arc<TorrentTracker>) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    announce(tracker.clone()).or(scrape(tracker)).recover(send_error)
}

/// GET /announce or /announce/<key>
fn announce(tracker: Arc<TorrentTracker>) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path::path("announce")
        .and(warp::filters::method::get())
        .and(with_announce_request(tracker.config.on_reverse_proxy))
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_announce)
}

/// GET /scrape/<key>
fn scrape(tracker: Arc<TorrentTracker>) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path::path("scrape")
        .and(warp::filters::method::get())
        .and(with_scrape_request(tracker.config.on_reverse_proxy))
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_scrape)
}
