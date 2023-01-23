use reqwest::Response;

use super::responses::Announce;

pub async fn assert_internal_server_error(response: Response) {
    assert_eq!(response.status(), 200);
    /* cspell:disable-next-line */
    assert_eq!(response.text().await.unwrap(), "d14:failure reason21:internal server errore");
}

pub async fn assert_empty_announce_response(response: Response) {
    assert_eq!(response.status(), 200);
    let announce_response: Announce = serde_bencode::from_str(&response.text().await.unwrap()).unwrap();
    assert!(announce_response.peers.is_empty());
}

pub async fn assert_announce_response(response: Response, expected_announce_response: &Announce) {
    assert_eq!(response.status(), 200);
    let announce_response: Announce = serde_bencode::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(announce_response, *expected_announce_response);
}
