use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection};
use crate::TorrentTracker;
use crate::torrust_http_tracker::{handle_announce, handle_error, handle_scrape, with_announce_request, with_auth_key, with_scrape_request, with_tracker};

/// All routes
pub fn routes(tracker: Arc<TorrentTracker>,) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    root(tracker.clone())
        .or(announce(tracker.clone()))
        .or(scrape(tracker.clone()))
        .recover(handle_error)
}

/// GET / or /<key>
fn root(tracker: Arc<TorrentTracker>,) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::any()
        .and(warp::filters::method::get())
        .and(with_announce_request())
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_announce)
}

/// GET /announce or /announce/<key>
fn announce(tracker: Arc<TorrentTracker>,) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path::path("announce")
        .and(warp::filters::method::get())
        .and(with_announce_request())
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_announce)
}

/// GET /scrape/<key>
fn scrape(tracker: Arc<TorrentTracker>,) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path::path("scrape")
        .and(warp::filters::method::get())
        .and(with_scrape_request())
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_scrape)
}
