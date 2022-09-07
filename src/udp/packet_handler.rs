use std::net::SocketAddr;
use std::sync::Arc;
use aquatic_udp_protocol::{ErrorResponse, Request, Response, TransactionId};
use crate::MAX_SCRAPE_TORRENTS;
use crate::udp::errors::ServerError;
use crate::tracker::tracker::TorrentTracker;
use super::connection::secret::Secret;
use super::request_handler::RequestHandler;

pub async fn handle_packet(remote_addr: SocketAddr, payload: Vec<u8>, tracker: Arc<TorrentTracker>) -> Option<Response> {
    match Request::from_bytes(&payload[..payload.len()], MAX_SCRAPE_TORRENTS).map_err(|_| ServerError::InternalServerError) {
        Ok(request) => {
            let transaction_id = match &request {
                Request::Connect(connect_request) => {
                    connect_request.transaction_id
                }
                Request::Announce(announce_request) => {
                    announce_request.transaction_id
                }
                Request::Scrape(scrape_request) => {
                    scrape_request.transaction_id
                }
            };

            // todo: server_secret should be randomly generated on startup
            let server_secret = Secret::new([0;32]);
            let request_handler = RequestHandler::new(server_secret);

            match request_handler.handle(request, remote_addr, tracker).await {
                Ok(response) => Some(response),
                Err(ServerError::InvalidConnectionId) => None,
                Err(e) => Some(handle_error(e, transaction_id))
            }
        }
        // bad request
        Err(_) => Some(handle_error(ServerError::BadRequest, TransactionId(0)))
    }
}

fn handle_error(e: ServerError, transaction_id: TransactionId) -> Response {
    let message = e.to_string();
    Response::from(ErrorResponse { transaction_id, message: message.into() })
}
