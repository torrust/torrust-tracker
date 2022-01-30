/// Pass Arc<TorrentTracker> along
fn with_tracker(tracker: Arc<TorrentTracker>) -> impl Filter<Extract = (Arc<TorrentTracker>,), Error = Infallible> + Clone {
    warp::any()
        .map(move || tracker.clone())
}

/// Check for infoHash
fn with_info_hash() -> impl Filter<Extract = (Vec<InfoHash>,), Error = Rejection> + Clone {
    warp::filters::query::raw()
        .and_then(info_hashes)
}

/// Parse InfoHash from raw query string
async fn info_hashes(raw_query: String) -> WebResult<Vec<InfoHash>> {
    let split_raw_query: Vec<&str> = raw_query.split("&").collect();
    let mut info_hashes: Vec<InfoHash> = Vec::new();

    for v in split_raw_query {
        if v.contains("info_hash") {
            let raw_info_hash = v.split("=").collect::<Vec<&str>>()[1];
            let info_hash_bytes = percent_encoding::percent_decode_str(raw_info_hash).collect::<Vec<u8>>();
            let info_hash = InfoHash::from_str(&hex::encode(info_hash_bytes));
            if let Ok(ih) = info_hash {
                info_hashes.push(ih);
            }
        }
    }

    if info_hashes.len() > MAX_SCRAPE_TORRENTS as usize {
        Err(reject::custom(ServerError::ExceededInfoHashLimit))
    } else if info_hashes.len() < 1 {
        Err(reject::custom(ServerError::InvalidInfoHash))
    } else {
        Ok(info_hashes)
    }
}

/// Pass Arc<TorrentTracker> along
fn with_auth_key() -> impl Filter<Extract = (Option<AuthKey>,), Error = warp::Rejection> + Clone {
    warp::path::param::<String>()
        .map(|key_string: String| {
            AuthKey::from_string(&key_string)
        })
}

/// Check for AnnounceRequest
fn with_announce_request() -> impl Filter<Extract = (AnnounceRequest,), Error = Rejection> + Clone {
    warp::filters::query::query::<AnnounceRequestQuery>()
        .and(with_info_hash())
        .and(warp::addr::remote())
        .and_then(announce_request)
}

/// Parse AnnounceRequest from raw AnnounceRequestQuery, InfoHash and Option<SocketAddr>
async fn announce_request(announce_request_query: AnnounceRequestQuery, info_hashes: Vec<InfoHash>, remote_addr: Option<SocketAddr>) -> WebResult<AnnounceRequest> {
    if remote_addr.is_none() { return Err(reject::custom(ServerError::AddressNotFound)) }

    Ok(AnnounceRequest {
        info_hash: info_hashes[0],
        peer_addr: remote_addr.unwrap(),
        downloaded: announce_request_query.downloaded,
        uploaded: announce_request_query.uploaded,
        peer_id: announce_request_query.peer_id,
        port: announce_request_query.port,
        left: announce_request_query.left,
        event: announce_request_query.event,
        compact: announce_request_query.compact
    })
}

/// Check for ScrapeRequest
fn with_scrape_request() -> impl Filter<Extract = (ScrapeRequest,), Error = Rejection> + Clone {
    warp::any()
        .and(with_info_hash())
        .and_then(scrape_request)
}

/// Parse ScrapeRequest from InfoHash
async fn scrape_request(info_hashes: Vec<InfoHash>) -> WebResult<ScrapeRequest> {
    Ok(ScrapeRequest {
        info_hashes,
    })
}
