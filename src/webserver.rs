use crate::tracker::{InfoHash, TorrentTracker};
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use warp::{filters, reply, reply::Reply, serve, Filter, Server};

fn view_root() -> impl Reply {
    reply::html(concat!(
        r#"<html>
        <head>
            <title>udpt/"#,
        env!("CARGO_PKG_VERSION"),
        r#"</title>
        </head>
        <body>
            This is your <a href="https://github.com/naim94a/udpt">udpt</a> torrent tracker.
        </body>
    </html>"#
    ))
}

#[derive(Deserialize, Debug)]
struct TorrentInfoQuery {
    offset: Option<u32>,
    limit: Option<u32>,
}

#[derive(Serialize)]
struct TorrentEntry<'a> {
    info_hash: &'a InfoHash,
    #[serde(flatten)]
    data: &'a crate::tracker::TorrentEntry,
}

#[derive(Serialize, Deserialize)]
struct TorrentFlag {
    is_flagged: bool,
}

#[derive(Serialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
enum ActionStatus<'a> {
    Ok,
    Err { reason: std::borrow::Cow<'a, str> },
}

impl warp::reject::Reject for ActionStatus<'static> {}

fn authenticate(tokens: HashMap<String, String>) -> impl Filter<Extract = (), Error = warp::reject::Rejection> + Clone {
    #[derive(Deserialize)]
    struct AuthToken {
        token: Option<String>,
    }

    let tokens: HashSet<String> = tokens.into_iter().map(|(_, v)| v).collect();

    let tokens = Arc::new(tokens);
    warp::filters::any::any()
        .map(move || tokens.clone())
        .and(filters::query::query::<AuthToken>())
        .and(filters::addr::remote())
        .and_then(
            |tokens: Arc<HashSet<String>>, token: AuthToken, peer_addr: Option<std::net::SocketAddr>| {
                async move {
                    if let Some(addr) = peer_addr {
                        if let Some(token) = token.token {
                            if addr.ip().is_loopback() && tokens.contains(&token) {
                                return Ok(());
                            }
                        }
                    }
                    Err(warp::reject::custom(ActionStatus::Err {
                        reason: "Access Denied".into(),
                    }))
                }
            },
        )
        .untuple_one()
}

pub fn build_server(
    tracker: Arc<TorrentTracker>, tokens: HashMap<String, String>,
) -> Server<impl Filter<Extract = impl Reply> + Clone + Send + Sync + 'static> {
    let root = filters::path::end().map(|| view_root());

    let t1 = tracker.clone();
    // view_torrent_list -> GET /t/?offset=:u32&limit=:u32 HTTP/1.1
    let view_torrent_list = filters::path::end()
        .and(filters::method::get())
        .and(filters::query::query())
        .map(move |limits| {
            let tracker = t1.clone();
            (limits, tracker)
        })
        .and_then(|(limits, tracker): (TorrentInfoQuery, Arc<TorrentTracker>)| {
            async move {
                let offset = limits.offset.unwrap_or(0);
                let limit = min(limits.limit.unwrap_or(1000), 4000);

                let db = tracker.get_database().await;
                let results: Vec<_> = db
                    .iter()
                    .map(|(k, v)| TorrentEntry { info_hash: k, data: v })
                    .skip(offset as usize)
                    .take(limit as usize)
                    .collect();

                Result::<_, warp::reject::Rejection>::Ok(reply::json(&results))
            }
        });

    let t2 = tracker.clone();
    // view_torrent_info -> GET /t/:infohash HTTP/*
    let view_torrent_info = filters::method::get()
        .and(filters::path::param())
        .map(move |info_hash: InfoHash| {
            let tracker = t2.clone();
            (info_hash, tracker)
        })
        .and_then(|(info_hash, tracker): (InfoHash, Arc<TorrentTracker>)| {
            async move {
                let db = tracker.get_database().await;
                let info = match db.get(&info_hash) {
                    Some(v) => v,
                    None => return Err(warp::reject::reject()),
                };

                Ok(reply::json(&TorrentEntry {
                    info_hash: &info_hash,
                    data: info,
                }))
            }
        });

    // DELETE /t/:info_hash
    let t3 = tracker.clone();
    let delete_torrent = filters::method::post()
        .and(filters::path::param())
        .map(move |info_hash: InfoHash| {
            let tracker = t3.clone();
            (info_hash, tracker)
        })
        .and_then(|(info_hash, tracker): (InfoHash, Arc<TorrentTracker>)| {
            async move {
                let resp = match tracker.remove_torrent(&info_hash, true).await.is_ok() {
                    true => ActionStatus::Ok,
                    false => {
                        ActionStatus::Err {
                            reason: "failed to delete torrent".into(),
                        }
                    }
                };

                Result::<_, warp::Rejection>::Ok(reply::json(&resp))
            }
        });

    let t4 = tracker.clone();
    // add_torrent/alter: POST /t/:info_hash
    // (optional) BODY: json: {"is_flagged": boolean}
    let change_torrent = filters::method::post()
        .and(filters::path::param())
        .and(filters::body::content_length_limit(4096))
        .and(filters::body::json())
        .map(move |info_hash: InfoHash, body: Option<TorrentFlag>| {
            let tracker = t4.clone();
            (info_hash, tracker, body)
        })
        .and_then(
            |(info_hash, tracker, body): (InfoHash, Arc<TorrentTracker>, Option<TorrentFlag>)| {
                async move {
                    let is_flagged = body.map(|e| e.is_flagged).unwrap_or(false);
                    if !tracker.set_torrent_flag(&info_hash, is_flagged).await {
                        // torrent doesn't exist, add it...

                        if is_flagged {
                            if tracker.add_torrent(&info_hash).await.is_ok() {
                                tracker.set_torrent_flag(&info_hash, is_flagged).await;
                            } else {
                                return Err(warp::reject::custom(ActionStatus::Err {
                                    reason: "failed to flag torrent".into(),
                                }));
                            }
                        }
                    }

                    Result::<_, warp::Rejection>::Ok(reply::json(&ActionStatus::Ok))
                }
            },
        );
    let torrent_mgmt =
        filters::path::path("t").and(view_torrent_list.or(delete_torrent).or(view_torrent_info).or(change_torrent));

    let server = root.or(authenticate(tokens).and(torrent_mgmt));

    serve(server)
}
