use std;
use hyper;
use futures::Future;
use futures::future::FutureResult;

use std::net::SocketAddr;

use tracker;
use tracker::TorrentTracker;

enum APIError {
    NoSuchMethod,
    BadAPICall,
    NotFound,
    InvalidAccessToken,
}

struct WebApplication {
    tracker: std::sync::Arc<TorrentTracker>,
    token: std::sync::Arc<String>,
}

impl std::error::Error for APIError {}

use std::fmt;
impl fmt::Debug for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Hello world!")
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Hello world!")
    }
}

impl WebApplication {
    pub fn new(tracker: std::sync::Arc<TorrentTracker>, token: String) -> WebApplication {
        WebApplication{
            tracker,
            token: std::sync::Arc::new(token),
        }
    }

    fn handle_root(&self, request: &hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, APIError> {
        Ok(hyper::Response::new(hyper::Body::from("<a href=\"https://naim94a.github.io/udpt\">https://naim94a.github.io/udpt</a>")))
    }

    fn handle_announce(&self, request: &hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, APIError> {
        Err(APIError::NoSuchMethod)
    }

    fn handle_scrape(&self, request: &hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, APIError> {
        Err(APIError::NoSuchMethod)
    }

    fn handle_stats(&self, request: &hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, APIError> {
        Err(APIError::NoSuchMethod)
    }

    fn parse_query(query: &str) -> std::collections::HashMap<&str, &str> {
        let mut res = std::collections::HashMap::new();

        let mut pair_start = 0;

        loop {
            let remaining = &query[pair_start..];
            let pair_len = match remaining.find("&") {
                Some(v) => v,
                None => remaining.len(),
            };

            let pair_str = &remaining[..pair_len];

            {
                let key_end = match pair_str.find("=") {
                    Some(v) => v,
                    None => pair_str.len(),
                };

                let mut val_start = key_end + 1;
                if val_start > pair_str.len() {
                    val_start = pair_str.len();
                }

                res.insert(&pair_str[..key_end], &pair_str[val_start..]);
            }

            pair_start += pair_len + 1;

            if pair_start >= query.len() {
                break;
            }
        }

        return res;
    }

    fn handle_api(&self, request: &hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, APIError> {
        // before processing request, check client's access.
        let partial = &request.uri().path()[4..]; // slice "/api" out...

        if partial == "/torrents" {
            if let Some(q) = request.uri().query() {
                let parsed_query = Self::parse_query(q);
                match parsed_query.get("token") {
                    Some(&token) => {
                        if token != self.token.as_str() {
                            return Err(APIError::InvalidAccessToken);
                        }
                    },
                    None => {
                        return Err(APIError::InvalidAccessToken);
                    }
                }

                let action = match parsed_query.get("action") {
                    Some(&v) => v,
                    None => {
                        return Err(APIError::BadAPICall);
                    }
                };
                if action == "list" {
                    let mut response = String::from("[");
                    let mut idx = 0;

                    let db = self.tracker.get_database();
                    for (info_hash, entry) in db.iter() {
                        use tracker::HexConv;

                        if idx > 0 {
                            response += ", ";
                        }
                        response += "{\"info_hash\":\"";
                        response += info_hash.to_hex().as_str();
                        response += "\"}";

                        idx += 1;
                    }
                    response += "]";

                    return Ok(hyper::Response::new(hyper::Body::from(response)));
                } else if action == "add" {
                    use tracker::HexConv;
                    let info_hash: tracker::InfoHash = match parsed_query.get("info_hash") {
                        Some(&v) => {
                            match tracker::InfoHash::from_hex(v) {
                                Some(ih) => ih,
                                None => {
                                    return Err(APIError::BadAPICall);
                                }
                            }
                        },
                        None => {
                            return Err(APIError::BadAPICall);
                        }
                    };

                    match self.tracker.add_torrent(&info_hash) {
                        Ok(_) => {
                            return Ok(hyper::Response::new(hyper::Body::from("{\"ok\": 1}")));
                        },
                        Err(_) => {
                            let mut resp = hyper::Response::new(hyper::Body::from("{\"ok\": 0}"));
                            *resp.status_mut() = hyper::StatusCode::NOT_FOUND;
                            return Ok(resp);
                        }
                    }
                } else if action == "remove" {
                    use tracker::HexConv;
                    let info_hash: tracker::InfoHash = match parsed_query.get("info_hash") {
                        Some(&v) => {
                            match tracker::InfoHash::from_hex(v) {
                                Some(ih) => ih,
                                None => {
                                    return Err(APIError::BadAPICall);
                                }
                            }
                        },
                        None => {
                            return Err(APIError::BadAPICall);
                        }
                    };

                    match self.tracker.remove_torrent(&info_hash, true) {
                        Ok(_) => {
                            return Ok(hyper::Response::new(hyper::Body::from("{\"ok\": 1}")));
                        },
                        Err(_) => {
                            let mut resp = hyper::Response::new(hyper::Body::from("{\"ok\": 0}"));
                            *resp.status_mut() = hyper::StatusCode::NOT_FOUND;
                            return Ok(resp);
                        }
                    };

                } else if action == "info" {

                } else {
                    return Err(APIError::NoSuchMethod);
                }
            } else {
                return Err(APIError::BadAPICall);
            }
        }
        else {
            return Err(APIError::NoSuchMethod);
        }

        Ok(hyper::Response::new(hyper::Body::from("api")))
    }

    pub fn handle_request(&mut self, request: &hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, APIError> {
        if request.uri().path() == "/" {
            return self.handle_root(request);
        } else if request.uri().path() == "/announce" {
            return self.handle_announce(request);
        } else if request.uri().path() == "/scrape" {
            return self.handle_scrape(request);
        } else if request.uri().path() == "/stats" {
            return self.handle_stats(request);
        } else if request.uri().path().starts_with("/api") {
            return self.handle_api(request);
        } else {
            Ok(hyper::Response::new(hyper::Body::from("Invalid url")))
        }
    }

    fn handle_error(req: &hyper::Request<hyper::Body>, err: &APIError) -> hyper::Response<hyper::Body> {
        hyper::Response::new(hyper::Body::from("Error report"))
    }
}

impl hyper::service::Service for WebApplication {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = APIError;
    type Future = FutureResult<hyper::Response<Self::ResBody>, Self::Error>;

    fn call(&mut self, req: hyper::Request<Self::ReqBody>) -> Self::Future {
        use futures;

        let mut res = match self.handle_request(&req) {
            Ok(res) => res,
            Err(err) => Self::handle_error(&req, &err),
        };

        futures::future::ok(res)
    }
}

impl hyper::service::NewService for WebApplication {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = APIError;
    type Service = Self;
    type Future = FutureResult<Self::Service, Self::InitError>;
    type InitError = hyper::Error;

    fn new_service(&self) -> Self::Future {
        use futures;

        futures::future::ok(WebApplication{
            tracker: self.tracker.clone(),
            token: self.token.clone(),
        })
    }
}

pub fn start_server(addr: SocketAddr, tracker: std::sync::Arc<TorrentTracker>, token: &str) {
    let svc = WebApplication::new(tracker, String::from(token));
    let server = hyper::Server::bind(&addr).serve( svc);

    hyper::rt::run(server.map_err(|_e|{
        println!("error: {}", _e);
    }));
}
