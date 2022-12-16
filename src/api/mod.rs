pub mod resource;
pub mod routes;
pub mod server;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TorrentInfoQuery {
    offset: Option<u32>,
    limit: Option<u32>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
enum ActionStatus<'a> {
    Ok,
    Err { reason: std::borrow::Cow<'a, str> },
}

impl warp::reject::Reject for ActionStatus<'static> {}
