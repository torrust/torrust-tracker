use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{SocketAddr};
use std::sync::Arc;
use std::str::FromStr;
use log::{debug};
use warp::{reply::Reply, Filter, Rejection, reject};
use warp::http::{Response, StatusCode};
use super::{AnnounceResponse};
use crate::tracker::{TorrentTracker};
use crate::{TorrentPeer, TorrentStats};
use crate::key_manager::AuthKey;
use crate::common::*;
use crate::torrust_http_tracker::request::AnnounceRequestQuery;
use crate::torrust_http_tracker::{AnnounceRequest, ErrorResponse, Peer, routes, ScrapeRequest, ScrapeResponse, ScrapeResponseEntry};
use crate::torrust_http_tracker::errors::ServerError;
use crate::utils::url_encode_bytes;

type WebResult<T> = std::result::Result<T, Rejection>;

/// Server that listens on HTTP, needs a TorrentTracker
#[derive(Clone)]
pub struct HttpServer {
    tracker: Arc<TorrentTracker>,
}

impl HttpServer {
    pub fn new(tracker: Arc<TorrentTracker>) -> HttpServer {
        HttpServer {
            tracker
        }
    }

    /// Start the HttpServer
    pub async fn start(&self, socket_addr: SocketAddr) {
        warp::serve(routes(self.tracker.clone()))
            .run(socket_addr).await;
    }

    /// Start the HttpServer in TLS mode
    pub async fn start_tls(&self, socket_addr: SocketAddr, ssl_cert_path: &str, ssl_key_path: &str) {
        warp::serve(routes(self.tracker.clone()))
            .tls()
            .cert_path(ssl_cert_path)
            .key_path(ssl_key_path)
            .run(socket_addr).await;
    }
}
