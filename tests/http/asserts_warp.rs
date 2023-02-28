use std::panic::Location;

/// todo: this mod should be removed when we remove the Warp implementation for the HTTP tracker.
use reqwest::Response;

use super::responses::announce_warp::WarpAnnounce;
use crate::http::asserts::assert_bencoded_error;

pub async fn assert_warp_announce_response(response: Response, expected_announce_response: &WarpAnnounce) {
    assert_eq!(response.status(), 200);

    let body = response.text().await.unwrap();

    let announce_response: WarpAnnounce = serde_bencode::from_str(&body)
        .unwrap_or_else(|_| panic!("response body should be a valid announce response, got \"{:#?}\"", &body));

    assert_eq!(announce_response, *expected_announce_response);
}

pub async fn assert_warp_peer_not_authenticated_error_response(response: Response) {
    assert_eq!(response.status(), 200);

    assert_bencoded_error(
        &response.text().await.unwrap(),
        "The peer is not authenticated",
        Location::caller(),
    );
}

pub async fn assert_warp_invalid_authentication_key_error_response(response: Response) {
    assert_eq!(response.status(), 200);

    assert_bencoded_error(&response.text().await.unwrap(), "is not valid", Location::caller());
}
