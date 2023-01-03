use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;
use warp::{filters, reply, Filter};

use super::resource::auth_key::AuthKey;
use super::resource::peer;
use super::resource::stats::Stats;
use super::resource::torrent::{ListItem, Torrent};
use super::{ActionStatus, TorrentInfoQuery};
use crate::protocol::info_hash::InfoHash;
use crate::tracker;
use crate::tracker::services::statistics::get_metrics;

fn authenticate(tokens: HashMap<String, String>) -> impl Filter<Extract = (), Error = warp::reject::Rejection> + Clone {
    #[derive(Deserialize)]
    struct AuthToken {
        token: Option<String>,
    }

    let tokens: HashSet<String> = tokens.into_values().collect();

    let tokens = Arc::new(tokens);
    warp::filters::any::any()
        .map(move || tokens.clone())
        .and(filters::query::query::<AuthToken>())
        .and_then(|tokens: Arc<HashSet<String>>, token: AuthToken| async move {
            match token.token {
                Some(token) => {
                    if !tokens.contains(&token) {
                        return Err(warp::reject::custom(ActionStatus::Err {
                            reason: "token not valid".into(),
                        }));
                    }

                    Ok(())
                }
                None => Err(warp::reject::custom(ActionStatus::Err {
                    reason: "unauthorized".into(),
                })),
            }
        })
        .untuple_one()
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn routes(tracker: &Arc<tracker::Tracker>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /api/torrents?offset=:u32&limit=:u32
    // View torrent list
    let api_torrents = tracker.clone();
    let view_torrent_list = filters::method::get()
        .and(filters::path::path("torrents"))
        .and(filters::path::end())
        .and(filters::query::query())
        .map(move |limits| {
            let tracker = api_torrents.clone();
            (limits, tracker)
        })
        .and_then(|(limits, tracker): (TorrentInfoQuery, Arc<tracker::Tracker>)| async move {
            let offset = limits.offset.unwrap_or(0);
            let limit = min(limits.limit.unwrap_or(1000), 4000);

            let db = tracker.get_torrents().await;
            let results: Vec<_> = db
                .iter()
                .map(|(info_hash, torrent_entry)| {
                    let (seeders, completed, leechers) = torrent_entry.get_stats();
                    ListItem {
                        info_hash: info_hash.to_string(),
                        seeders,
                        completed,
                        leechers,
                        peers: None,
                    }
                })
                .skip(offset as usize)
                .take(limit as usize)
                .collect();

            Result::<_, warp::reject::Rejection>::Ok(reply::json(&results))
        });

    // GET /api/stats
    // View tracker status
    let api_stats = tracker.clone();
    let view_stats_list = filters::method::get()
        .and(filters::path::path("stats"))
        .and(filters::path::end())
        .map(move || api_stats.clone())
        .and_then(|tracker: Arc<tracker::Tracker>| async move {
            Result::<_, warp::reject::Rejection>::Ok(reply::json(&Stats::from(get_metrics(tracker.clone()).await)))
        });

    // GET /api/torrent/:info_hash
    // View torrent info
    let t2 = tracker.clone();
    let view_torrent_info = filters::method::get()
        .and(filters::path::path("torrent"))
        .and(filters::path::param())
        .and(filters::path::end())
        .map(move |info_hash: InfoHash| {
            let tracker = t2.clone();
            (info_hash, tracker)
        })
        .and_then(|(info_hash, tracker): (InfoHash, Arc<tracker::Tracker>)| async move {
            let db = tracker.get_torrents().await;
            let torrent_entry_option = db.get(&info_hash);

            let torrent_entry = match torrent_entry_option {
                Some(torrent_entry) => torrent_entry,
                None => {
                    return Result::<_, warp::reject::Rejection>::Ok(reply::json(&"torrent not known"));
                }
            };
            let (seeders, completed, leechers) = torrent_entry.get_stats();

            let peers = torrent_entry.get_peers(None);

            let peer_resources = peers.iter().map(|peer| peer::Peer::from(**peer)).collect();

            Ok(reply::json(&Torrent {
                info_hash: info_hash.to_string(),
                seeders,
                completed,
                leechers,
                peers: Some(peer_resources),
            }))
        });

    // DELETE /api/whitelist/:info_hash
    // Delete info hash from whitelist
    let t3 = tracker.clone();
    let delete_torrent = filters::method::delete()
        .and(filters::path::path("whitelist"))
        .and(filters::path::param())
        .and(filters::path::end())
        .map(move |info_hash: InfoHash| {
            let tracker = t3.clone();
            (info_hash, tracker)
        })
        .and_then(|(info_hash, tracker): (InfoHash, Arc<tracker::Tracker>)| async move {
            match tracker.remove_torrent_from_whitelist(&info_hash).await {
                Ok(_) => Ok(warp::reply::json(&ActionStatus::Ok)),
                Err(_) => Err(warp::reject::custom(ActionStatus::Err {
                    reason: "failed to remove torrent from whitelist".into(),
                })),
            }
        });

    // POST /api/whitelist/:info_hash
    // Add info hash to whitelist
    let t4 = tracker.clone();
    let add_torrent = filters::method::post()
        .and(filters::path::path("whitelist"))
        .and(filters::path::param())
        .and(filters::path::end())
        .map(move |info_hash: InfoHash| {
            let tracker = t4.clone();
            (info_hash, tracker)
        })
        .and_then(|(info_hash, tracker): (InfoHash, Arc<tracker::Tracker>)| async move {
            match tracker.add_torrent_to_whitelist(&info_hash).await {
                Ok(..) => Ok(warp::reply::json(&ActionStatus::Ok)),
                Err(..) => Err(warp::reject::custom(ActionStatus::Err {
                    reason: "failed to whitelist torrent".into(),
                })),
            }
        });

    // POST /api/key/:seconds_valid
    // Generate new key
    let t5 = tracker.clone();
    let create_key = filters::method::post()
        .and(filters::path::path("key"))
        .and(filters::path::param())
        .and(filters::path::end())
        .map(move |seconds_valid: u64| {
            let tracker = t5.clone();
            (seconds_valid, tracker)
        })
        .and_then(|(seconds_valid, tracker): (u64, Arc<tracker::Tracker>)| async move {
            match tracker.generate_auth_key(Duration::from_secs(seconds_valid)).await {
                Ok(auth_key) => Ok(warp::reply::json(&AuthKey::from(auth_key))),
                Err(..) => Err(warp::reject::custom(ActionStatus::Err {
                    reason: "failed to generate key".into(),
                })),
            }
        });

    // DELETE /api/key/:key
    // Delete key
    let t6 = tracker.clone();
    let delete_key = filters::method::delete()
        .and(filters::path::path("key"))
        .and(filters::path::param())
        .and(filters::path::end())
        .map(move |key: String| {
            let tracker = t6.clone();
            (key, tracker)
        })
        .and_then(|(key, tracker): (String, Arc<tracker::Tracker>)| async move {
            match tracker.remove_auth_key(&key).await {
                Ok(_) => Ok(warp::reply::json(&ActionStatus::Ok)),
                Err(_) => Err(warp::reject::custom(ActionStatus::Err {
                    reason: "failed to delete key".into(),
                })),
            }
        });

    // GET /api/whitelist/reload
    // Reload whitelist
    let t7 = tracker.clone();
    let reload_whitelist = filters::method::get()
        .and(filters::path::path("whitelist"))
        .and(filters::path::path("reload"))
        .and(filters::path::end())
        .map(move || t7.clone())
        .and_then(|tracker: Arc<tracker::Tracker>| async move {
            match tracker.load_whitelist().await {
                Ok(_) => Ok(warp::reply::json(&ActionStatus::Ok)),
                Err(_) => Err(warp::reject::custom(ActionStatus::Err {
                    reason: "failed to reload whitelist".into(),
                })),
            }
        });

    // GET /api/keys/reload
    // Reload whitelist
    let t8 = tracker.clone();
    let reload_keys = filters::method::get()
        .and(filters::path::path("keys"))
        .and(filters::path::path("reload"))
        .and(filters::path::end())
        .map(move || t8.clone())
        .and_then(|tracker: Arc<tracker::Tracker>| async move {
            match tracker.load_keys().await {
                Ok(_) => Ok(warp::reply::json(&ActionStatus::Ok)),
                Err(_) => Err(warp::reject::custom(ActionStatus::Err {
                    reason: "failed to reload keys".into(),
                })),
            }
        });

    let api_routes = filters::path::path("api").and(
        view_torrent_list
            .or(delete_torrent)
            .or(view_torrent_info)
            .or(view_stats_list)
            .or(add_torrent)
            .or(create_key)
            .or(delete_key)
            .or(reload_whitelist)
            .or(reload_keys),
    );

    api_routes.and(authenticate(tracker.config.http_api.access_tokens.clone()))
}
