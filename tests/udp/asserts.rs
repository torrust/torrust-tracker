use aquatic_udp_protocol::{Response, TransactionId};

pub fn is_error_response(response: &Response, error_message: &str) -> bool {
    match response {
        Response::Error(error_response) => error_response.message.starts_with(error_message),
        _ => false,
    }
}

pub fn is_connect_response(response: &Response, transaction_id: TransactionId) -> bool {
    match response {
        Response::Connect(connect_response) => connect_response.transaction_id == transaction_id,
        _ => false,
    }
}

pub fn is_ipv4_announce_response(response: &Response) -> bool {
    matches!(response, Response::AnnounceIpv4(_))
}

pub fn is_scrape_response(response: &Response) -> bool {
    matches!(response, Response::Scrape(_))
}
