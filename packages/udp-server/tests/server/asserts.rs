use torrust_tracker_udp_protocol::{Response, TransactionId};

pub fn get_error_response_message(response: &Response) -> Option<String> {
    match response {
        Response::Error(error_response) => Some(error_response.message.to_string()),
        _ => None,
    }
}

pub fn is_connect_response(response: &Response, transaction_id: TransactionId) -> bool {
    match response {
        Response::Connect(connect_response) => connect_response.transaction_id == transaction_id,
        _ => false,
    }
}

pub const fn is_ipv4_announce_response(response: &Response) -> bool {
    matches!(response, Response::AnnounceIpv4(_))
}

pub const fn is_scrape_response(response: &Response) -> bool {
    matches!(response, Response::Scrape(_))
}

/// Fails when `message` contains anything that looks like a source path: a workspace path, a
/// registry checkout, the standard library sources, or a `file.rs:line` suffix, written with
/// either path separator. It reports a failure at the caller's line.
#[track_caller]
pub fn assert_names_no_source_path(message: &str) {
    for fragment in [
        ".rs:",
        "packages/",
        "packages\\",
        "src/",
        "src\\",
        ".cargo/registry",
        "rustlib/src",
    ] {
        assert!(
            !message.contains(fragment),
            "the error message must not hold a source path, but it contains {fragment:?}: {message:?}"
        );
    }
}
