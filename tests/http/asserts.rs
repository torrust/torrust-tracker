use reqwest::Response;

use super::responses::{Announce, DecodedCompactAnnounce};
use crate::http::responses::{CompactAnnounce, Error};

pub async fn assert_empty_announce_response(response: Response) {
    assert_eq!(response.status(), 200);
    let announce_response: Announce = serde_bencode::from_str(&response.text().await.unwrap()).unwrap();
    assert!(announce_response.peers.is_empty());
}

pub async fn assert_announce_response(response: Response, expected_announce_response: &Announce) {
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let announce_response: Announce = serde_bencode::from_str(&body)
        .unwrap_or_else(|_| panic!("response body should be a valid announce response, got \"{}\"", &body));
    assert_eq!(announce_response, *expected_announce_response);
}

/// Sample bencoded response as byte array:
///
/// ```text
/// b"d8:intervali120e12:min intervali120e8:completei2e10:incompletei0e5:peers6:~\0\0\x01\x1f\x90e6:peers60:e"
/// ```
pub async fn assert_compact_announce_response(response: Response, expected_response: &DecodedCompactAnnounce) {
    assert_eq!(response.status(), 200);

    let bytes = response.bytes().await.unwrap();

    let compact_announce: CompactAnnounce = serde_bencode::from_bytes(&bytes).unwrap_or_else(|_| {
        panic!(
            "response body should be a valid compact announce response, got \"{:?}\"",
            &bytes
        )
    });
    let actual_response = DecodedCompactAnnounce::from(compact_announce);

    assert_eq!(actual_response, *expected_response);
}

pub async fn assert_is_announce_response(response: Response) {
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let _announce_response: Announce = serde_bencode::from_str(&body)
        .unwrap_or_else(|_| panic!("response body should be a valid announce response, got \"{}\"", &body));
}

// Error responses

pub async fn assert_internal_server_error_response(response: Response) {
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let error_response: Error = serde_bencode::from_str(&body).unwrap_or_else(|_| {
        panic!(
            "response body should be a valid bencoded string for the 'internal server' error, got \"{}\"",
            &body
        )
    });
    let expected_error_response = Error {
        failure_reason: "internal server error".to_string(),
    };
    assert_eq!(error_response, expected_error_response);
}

pub async fn assert_invalid_info_hash_error_response(response: Response) {
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let error_response: Error = serde_bencode::from_str(&body).unwrap_or_else(|_| {
        panic!(
            "response body should be a valid bencoded string for the 'invalid info_hash' error, got \"{}\"",
            &body
        )
    });
    let expected_error_response = Error {
        failure_reason: "info_hash is either missing or invalid".to_string(),
    };
    assert_eq!(error_response, expected_error_response);
}

pub async fn assert_invalid_peer_id_error_response(response: Response) {
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    let error_response: Error = serde_bencode::from_str(&body).unwrap_or_else(|_| {
        panic!(
            "response body should be a valid bencoded string for the 'invalid peer id' error, got \"{}\"",
            &body
        )
    });
    let expected_error_response = Error {
        failure_reason: "peer_id is either missing or invalid".to_string(),
    };
    assert_eq!(error_response, expected_error_response);
}
