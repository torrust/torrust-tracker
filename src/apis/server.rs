use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use futures::Future;
use log::info;
use warp::hyper;

use super::middlewares::auth::auth;
use super::routes::{
    add_torrent_to_whitelist_handler, delete_torrent_from_whitelist_handler, get_stats_handler, get_torrent_handler,
    get_torrents_handler, reload_whitelist_handler,
};
use crate::tracker;

pub fn start(socket_addr: SocketAddr, tracker: &Arc<tracker::Tracker>) -> impl Future<Output = hyper::Result<()>> {
    // todo: duplicate routes definition. See `start_tls` function.
    let app = Router::new()
        // Stats
        .route("/stats", get(get_stats_handler).with_state(tracker.clone()))
        // Torrents
        .route("/torrent/:info_hash", get(get_torrent_handler).with_state(tracker.clone()))
        .route("/torrents", get(get_torrents_handler).with_state(tracker.clone()))
        // Whitelisted torrents
        .route(
            "/whitelist/:info_hash",
            post(add_torrent_to_whitelist_handler).with_state(tracker.clone()),
        )
        .route(
            "/whitelist/:info_hash",
            delete(delete_torrent_from_whitelist_handler).with_state(tracker.clone()),
        )
        // Whitelist command
        .route(
            "/whitelist/:info_hash",
            get(reload_whitelist_handler).with_state(tracker.clone()),
        )
        .layer(middleware::from_fn_with_state(tracker.config.clone(), auth));

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust APIs server on http://{} ...", socket_addr);
    })
}

pub fn start_tls(
    socket_addr: SocketAddr,
    ssl_config: RustlsConfig,
    tracker: &Arc<tracker::Tracker>,
) -> impl Future<Output = Result<(), std::io::Error>> {
    // todo: duplicate routes definition. See `start` function.
    let app = Router::new()
        // Stats
        .route("/stats", get(get_stats_handler).with_state(tracker.clone()))
        // Torrents
        .route("/torrent/:info_hash", get(get_torrent_handler).with_state(tracker.clone()))
        .route("/torrents", get(get_torrents_handler).with_state(tracker.clone()))
        // Whitelisted torrents
        .route(
            "/whitelist/:info_hash",
            post(add_torrent_to_whitelist_handler).with_state(tracker.clone()),
        )
        .route(
            "/whitelist/:info_hash",
            delete(delete_torrent_from_whitelist_handler).with_state(tracker.clone()),
        )
        // Whitelist command
        .route(
            "/whitelist/:info_hash",
            get(reload_whitelist_handler).with_state(tracker.clone()),
        )
        .layer(middleware::from_fn_with_state(tracker.config.clone(), auth));

    let handle = Handle::new();
    let shutdown_handle = handle.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust APIs server on https://{} ...", socket_addr);
        shutdown_handle.shutdown();
    });

    axum_server::bind_rustls(socket_addr, ssl_config)
        .handle(handle)
        .serve(app.into_make_service())
}
