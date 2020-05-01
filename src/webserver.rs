use std::collections::HashMap;
use std::sync::Arc;

use actix_web;
use actix_net;
use binascii;

use crate::config;
use crate::tracker::TorrentTracker;
use log::{error, warn, info, debug};

const SERVER: &str = concat!("udpt/", env!("CARGO_PKG_VERSION"));

pub struct WebServer {
    thread: std::thread::JoinHandle<()>,
    addr: Option<actix_web::actix::Addr<actix_net::server::Server>>,
}

mod http_responses {
    use serde::Serialize;
    use crate::tracker::InfoHash;

    #[derive(Serialize)]
    pub struct TorrentInfo {
        pub is_flagged: bool,
        pub leecher_count: u32,
        pub seeder_count: u32,
        pub completed: u32,
    }

    #[derive(Serialize)]
    pub struct TorrentList {
        pub offset: u32,
        pub length: u32,
        pub total: u32,
        pub torrents: Vec<InfoHash>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum APIResponse {
        Error(String),
        TorrentList(TorrentList),
        TorrentInfo(TorrentInfo),
    }
}

struct UdptState {
    // k=token, v=username.
    access_tokens: HashMap<String, String>,
    tracker: Arc<TorrentTracker>,
}

impl UdptState {
    fn new(tracker: Arc<TorrentTracker>, tokens: HashMap<String, String>) -> UdptState {
        UdptState {
            tracker,
            access_tokens: tokens,
        }
    }
}

#[derive(Debug)]
struct UdptRequestState {
    current_user: Option<String>,
}

impl Default for UdptRequestState {
    fn default() -> Self {
        UdptRequestState {
            current_user: Option::None,
        }
    }
}

impl UdptRequestState {
    fn get_user<S>(req: &actix_web::HttpRequest<S>) -> Option<String> {
        let exts = req.extensions();
        let req_state: Option<&UdptRequestState> = exts.get();
        match req_state {
            None => None,
            Option::Some(state) => match state.current_user {
                Option::Some(ref v) => Option::Some(v.clone()),
                None => {
                    error!(
                        "Invalid API token from {} @ {}",
                        req.peer_addr().unwrap(),
                        req.path()
                    );
                    return None;
                }
            },
        }
    }
}

struct UdptMiddleware;

impl actix_web::middleware::Middleware<UdptState> for UdptMiddleware {
    fn start(
        &self,
        req: &actix_web::HttpRequest<UdptState>,
    ) -> actix_web::Result<actix_web::middleware::Started> {
        let mut req_state = UdptRequestState::default();
        if let Option::Some(token) = req.query().get("token") {
            let app_state: &UdptState = req.state();
            if let Option::Some(v) = app_state.access_tokens.get(token) {
                req_state.current_user = Option::Some(v.clone());
            }
        }
        req.extensions_mut().insert(req_state);
        Ok(actix_web::middleware::Started::Done)
    }

    fn response(
        &self,
        _req: &actix_web::HttpRequest<UdptState>,
        mut resp: actix_web::HttpResponse,
    ) -> actix_web::Result<actix_web::middleware::Response> {
        resp.headers_mut().insert(
            actix_web::http::header::SERVER,
            actix_web::http::header::HeaderValue::from_static(SERVER),
        );

        Ok(actix_web::middleware::Response::Done(resp))
    }
}

impl WebServer {
    fn get_access_tokens(cfg: &config::HTTPConfig, tokens: &mut HashMap<String, String>) {
        for (user, token) in cfg.get_access_tokens().iter() {
            tokens.insert(token.clone(), user.clone());
        }
        if tokens.len() == 0 {
            warn!("No access tokens provided. HTTP API will not be useful.");
        }
    }

    pub fn shutdown(self) {
        match self.addr {
            Some(v) => {
                use futures::future::Future;

                v.send(actix_web::actix::signal::Signal(actix_web::actix::signal::SignalType::Term)).wait().unwrap();
            },
            None => {},
        };

        self.thread.thread().unpark();
        let _ = self.thread.join();
    }

    pub fn new(
        tracker: Arc<TorrentTracker>,
        cfg: Arc<config::Configuration>,
    ) -> WebServer {
        let cfg_cp = cfg.clone();

        let (tx_addr, rx_addr) = std::sync::mpsc::channel();

        let thread = std::thread::spawn(move || {
            let server = actix_web::server::HttpServer::new(move || {
                let mut access_tokens = HashMap::new();

                if let Some(http_cfg) = cfg_cp.get_http_config() {
                    Self::get_access_tokens(http_cfg, &mut access_tokens);
                }

                let state = UdptState::new(tracker.clone(), access_tokens);

                actix_web::App::<UdptState>::with_state(state)
                    .middleware(UdptMiddleware)
                    .resource("/t", |r| r.f(Self::view_torrent_list))
                    .scope(r"/t/{info_hash:[\dA-Fa-f]{40,40}}", |scope| {
                        scope.resource("", |r| {
                            r.method(actix_web::http::Method::GET)
                                .f(Self::view_torrent_stats);
                            r.method(actix_web::http::Method::POST)
                                .f(Self::torrent_action);
                        })
                    })
                    .resource("/", |r| {
                        r.method(actix_web::http::Method::GET).f(Self::view_root)
                    })
            });

            if let Some(http_cfg) = cfg.get_http_config() {
                let bind_addr = http_cfg.get_address();
                match server.bind(bind_addr) {
                    Ok(v) => {
                        let sys = actix_web::actix::System::new("http-server");
                        let addr = v.start();
                        let _ = tx_addr.send(addr);
                        sys.run();
                    }
                    Err(err) => {
                        error!("Failed to bind http server. {}", err);
                    }
                }
            } else {
                unreachable!();
            }
        });

        let addr = match rx_addr.recv() {
            Ok(v) => Some(v),
            Err(_) => None
        };

        WebServer {
            thread,
            addr,
        }
    }

    fn view_root(_req: &actix_web::HttpRequest<UdptState>) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
            .content_type("text/html")
            .body(r#"Powered by <a href="https://github.com/naim94a/udpt">https://github.com/naim94a/udpt</a>"#)
    }

    fn view_torrent_list(req: &actix_web::HttpRequest<UdptState>) -> impl actix_web::Responder {
        use std::str::FromStr;

        if UdptRequestState::get_user(req).is_none() {
            return actix_web::Json(http_responses::APIResponse::Error(String::from(
                "access_denied",
            )));
        }

        let req_offset = match req.query().get("offset") {
            None => 0,
            Some(v) => match u32::from_str(v.as_str()) {
                Ok(v) => v,
                Err(_) => 0,
            },
        };

        let mut req_limit = match req.query().get("limit") {
            None => 0,
            Some(v) => match u32::from_str(v.as_str()) {
                Ok(v) => v,
                Err(_) => 0,
            },
        };

        if req_limit > 4096 {
            req_limit = 4096;
        } else if req_limit == 0 {
            req_limit = 1000;
        }

        let app_state: &UdptState = req.state();
        let app_db = app_state.tracker.get_database();

        let total = app_db.len() as u32;

        let mut torrents = Vec::with_capacity(req_limit as usize);

        for (info_hash, _) in app_db
            .iter()
            .skip(req_offset as usize)
            .take(req_limit as usize)
        {
            torrents.push(info_hash.clone());
        }

        actix_web::Json(http_responses::APIResponse::TorrentList(
            http_responses::TorrentList {
                total,
                length: torrents.len() as u32,
                offset: req_offset,
                torrents,
            },
        ))
    }

    fn view_torrent_stats(req: &actix_web::HttpRequest<UdptState>) -> actix_web::HttpResponse {
        use actix_web::FromRequest;

        if UdptRequestState::get_user(req).is_none() {
            return actix_web::HttpResponse::build(actix_web::http::StatusCode::UNAUTHORIZED).json(
                http_responses::APIResponse::Error(String::from("access_denied")),
            );
        }

        let path: actix_web::Path<String> = match actix_web::Path::extract(req) {
            Ok(v) => v,
            Err(_) => {
                return actix_web::HttpResponse::build(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                )
                .json(http_responses::APIResponse::Error(String::from(
                    "internal_error",
                )));
            }
        };

        let mut info_hash = [0u8; 20];
        if let Err(_) = binascii::hex2bin((*path).as_bytes(), &mut info_hash) {
            return actix_web::HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST).json(
                http_responses::APIResponse::Error(String::from("invalid_info_hash")),
            );
        }

        let app_state: &UdptState = req.state();

        let db = app_state.tracker.get_database();
        let entry = match db.get(&info_hash.into()) {
            Some(v) => v,
            None => {
                return actix_web::HttpResponse::build(actix_web::http::StatusCode::NOT_FOUND).json(
                    http_responses::APIResponse::Error(String::from("not_found")),
                );
            }
        };

        let is_flagged = entry.is_flagged();
        let (seeders, completed, leechers) = entry.get_stats();

        return actix_web::HttpResponse::build(actix_web::http::StatusCode::OK).json(
            http_responses::APIResponse::TorrentInfo(http_responses::TorrentInfo {
                is_flagged,
                seeder_count: seeders,
                leecher_count: leechers,
                completed,
            }),
        );
    }

    fn torrent_action(req: &actix_web::HttpRequest<UdptState>) -> actix_web::HttpResponse {
        use actix_web::FromRequest;

        if UdptRequestState::get_user(req).is_none() {
            return actix_web::HttpResponse::build(actix_web::http::StatusCode::UNAUTHORIZED).json(
                http_responses::APIResponse::Error(String::from("access_denied")),
            );
        }

        let query = req.query();
        let action_opt = query.get("action");
        let action = match action_opt {
            Some(v) => v,
            None => {
                return actix_web::HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST)
                    .json(http_responses::APIResponse::Error(String::from(
                        "action_required",
                    )));
            }
        };

        let app_state: &UdptState = req.state();

        let path: actix_web::Path<String> = match actix_web::Path::extract(req) {
            Ok(v) => v,
            Err(_err) => {
                return actix_web::HttpResponse::build(
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                )
                .json(http_responses::APIResponse::Error(String::from(
                    "internal_error",
                )));
            }
        };

        let info_hash_str = &(*path);
        let mut info_hash = [0u8; 20];
        if let Err(_) = binascii::hex2bin(info_hash_str.as_bytes(), &mut info_hash) {
            return actix_web::HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST).json(
                http_responses::APIResponse::Error(String::from("invalid_info_hash")),
            );
        }

        match action.as_str() {
            "flag" => {
                app_state.tracker.set_torrent_flag(&info_hash.into(), true);
                info!("Flagged {}", info_hash_str.as_str());
                return actix_web::HttpResponse::build(actix_web::http::StatusCode::OK).body("");
            }
            "unflag" => {
                app_state.tracker.set_torrent_flag(&info_hash.into(), false);
                info!("Unflagged {}", info_hash_str.as_str());
                return actix_web::HttpResponse::build(actix_web::http::StatusCode::OK).body("");
            }
            "add" => {
                let success = app_state.tracker.add_torrent(&info_hash.into()).is_ok();
                info!("Added {}, success={}", info_hash_str.as_str(), success);
                let code = if success {
                    actix_web::http::StatusCode::OK
                } else {
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                };

                return actix_web::HttpResponse::build(code).body("");
            }
            "remove" => {
                let success = app_state
                    .tracker
                    .remove_torrent(&info_hash.into(), true)
                    .is_ok();
                info!("Removed {}, success={}", info_hash_str.as_str(), success);
                let code = if success {
                    actix_web::http::StatusCode::OK
                } else {
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
                };

                return actix_web::HttpResponse::build(code).body("");
            }
            _ => {
                debug!("Invalid action {}", action.as_str());
                return actix_web::HttpResponse::build(actix_web::http::StatusCode::BAD_REQUEST)
                    .json(http_responses::APIResponse::Error(String::from(
                        "invalid_action",
                    )));
            }
        }
    }
}
