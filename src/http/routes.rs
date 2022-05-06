use std::convert::Infallible;
use std::sync::Arc;

use warp::{Filter, Rejection};

use crate::http::handle_announce;
use crate::http::handle_scrape;
use crate::http::send_error;
use crate::http::with_announce_request;
use crate::http::with_auth_key;
use crate::http::with_scrape_request;
use crate::http::with_tracker;
use crate::tracker::tracker::TorrentTracker;

/// All routes
pub fn routes(tracker: Arc<TorrentTracker>) -> impl Filter<Extract=impl warp::Reply, Error=Infallible> + Clone {
    root(tracker.clone())
        .or(announce(tracker.clone()))
        .or(scrape(tracker.clone()))
        .recover(send_error)
}

/// GET / or /<key>
fn root(tracker: Arc<TorrentTracker>) -> impl Filter<Extract=impl warp::Reply, Error=Rejection> + Clone {
    warp::any()
        .and(warp::filters::method::get())
        .and(with_announce_request(tracker.config.on_reverse_proxy))
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_announce)
}

/// GET /announce or /announce/<key>
fn announce(tracker: Arc<TorrentTracker>) -> impl Filter<Extract=impl warp::Reply, Error=Rejection> + Clone {
    warp::path::path("announce")
        .and(warp::filters::method::get())
        .and(with_announce_request(tracker.config.on_reverse_proxy))
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_announce)
}

/// GET /scrape/<key>
fn scrape(tracker: Arc<TorrentTracker>) -> impl Filter<Extract=impl warp::Reply, Error=Rejection> + Clone {
    warp::path::path("scrape")
        .and(warp::filters::method::get())
        .and(with_scrape_request(tracker.config.on_reverse_proxy))
        .and(with_auth_key())
        .and(with_tracker(tracker))
        .and_then(handle_scrape)
}
