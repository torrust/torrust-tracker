use std::sync::Arc;

use axum::extract::State;
use axum::response::Json;
use serde_json::{json, Value};

use crate::api::resource::stats::Stats;
use crate::tracker::Tracker;

#[allow(clippy::unused_async)]
pub async fn root() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[allow(clippy::unused_async)]
pub async fn get_stats(State(tracker): State<Arc<Tracker>>) -> Json<Value> {
    let mut results = Stats {
        torrents: 0,
        seeders: 0,
        completed: 0,
        leechers: 0,
        tcp4_connections_handled: 0,
        tcp4_announces_handled: 0,
        tcp4_scrapes_handled: 0,
        tcp6_connections_handled: 0,
        tcp6_announces_handled: 0,
        tcp6_scrapes_handled: 0,
        udp4_connections_handled: 0,
        udp4_announces_handled: 0,
        udp4_scrapes_handled: 0,
        udp6_connections_handled: 0,
        udp6_announces_handled: 0,
        udp6_scrapes_handled: 0,
    };

    let db = tracker.get_torrents().await;

    db.values().for_each(|torrent_entry| {
        let (seeders, completed, leechers) = torrent_entry.get_stats();
        results.seeders += seeders;
        results.completed += completed;
        results.leechers += leechers;
        results.torrents += 1;
    });

    let stats = tracker.get_stats().await;

    #[allow(clippy::cast_possible_truncation)]
    {
        results.tcp4_connections_handled = stats.tcp4_connections_handled as u32;
        results.tcp4_announces_handled = stats.tcp4_announces_handled as u32;
        results.tcp4_scrapes_handled = stats.tcp4_scrapes_handled as u32;
        results.tcp6_connections_handled = stats.tcp6_connections_handled as u32;
        results.tcp6_announces_handled = stats.tcp6_announces_handled as u32;
        results.tcp6_scrapes_handled = stats.tcp6_scrapes_handled as u32;
        results.udp4_connections_handled = stats.udp4_connections_handled as u32;
        results.udp4_announces_handled = stats.udp4_announces_handled as u32;
        results.udp4_scrapes_handled = stats.udp4_scrapes_handled as u32;
        results.udp6_connections_handled = stats.udp6_connections_handled as u32;
        results.udp6_announces_handled = stats.udp6_announces_handled as u32;
        results.udp6_scrapes_handled = stats.udp6_scrapes_handled as u32;
    }

    Json(json!(results))
}
