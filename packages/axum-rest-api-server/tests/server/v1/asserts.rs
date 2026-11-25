// code-review: should we use macros to return the exact line where the assert fails?

use reqwest::Response;
use torrust_tracker_rest_api_protocol::v1::context::auth_key::resources::auth_key::AuthKey;
use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::Stats;
use torrust_tracker_rest_api_protocol::v1::context::torrent::resources::torrent::{ListItem, Torrent};
use torrust_tracker_test_helpers::assertions::assert_names_no_source_path;

// Resource responses

pub async fn assert_stats(response: Response, stats: Stats) {
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.json::<Stats>().await.unwrap(), stats);
}

pub async fn assert_torrent_list(response: Response, torrents: Vec<ListItem>) {
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.json::<Vec<ListItem>>().await.unwrap(), torrents);
}

pub async fn assert_torrent_info(response: Response, torrent: Torrent) {
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.json::<Torrent>().await.unwrap(), torrent);
}

pub async fn assert_auth_key_utf8(response: Response) -> AuthKey {
    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json; charset=utf-8"
    );
    response.json::<AuthKey>().await.unwrap()
}

// OK response

pub async fn assert_ok(response: Response) {
    let response_status = response.status();
    let response_headers = response.headers().get("content-type").cloned().unwrap();
    let response_text = response.text().await.unwrap();

    let details = format!(
        r#"
   status: ´{response_status}´
  headers: ´{response_headers:?}´
     text: ´"{response_text}"´"#
    );

    assert_eq!(response_status, 200, "details:{details}.");
    assert_eq!(response_headers, "application/json", "\ndetails:{details}.");
    assert_eq!(response_text, "{\"status\":\"ok\"}", "\ndetails:{details}.");
}

pub async fn assert_disabled_by_configuration(response: Response, capability: &str) {
    assert_eq!(response.status(), 409);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(
        response.text().await.unwrap(),
        format!("{{\"status\":\"err\",\"reason\":\"{capability} capability is disabled by configuration\"}}")
    );
}

// Error responses

pub async fn assert_bad_request(response: Response, body: &str) {
    assert_eq!(response.status(), 400);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/plain; charset=utf-8");
    assert_eq!(response.text().await.unwrap(), body);
}

pub async fn assert_unprocessable_content(response: Response, text: &str) {
    assert_eq!(response.status(), 422);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/plain; charset=utf-8");
    assert!(response.text().await.unwrap().contains(text));
}

pub async fn assert_not_found(response: Response) {
    assert_eq!(response.status(), 404);
    // todo: missing header in the response
    //assert_eq!(response.headers().get("content-type").unwrap(), "text/plain; charset=utf-8");
    assert_eq!(response.text().await.unwrap(), "");
}

pub async fn assert_torrent_not_known(response: Response) {
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.text().await.unwrap(), "\"torrent not known\"");
}

pub async fn assert_invalid_infohash_param(response: Response, invalid_infohash: &str) {
    assert_bad_request(
        response,
        &format!("Invalid URL: invalid infohash param: string \"{invalid_infohash}\", expected a 40 character long string"),
    )
    .await;
}

/// The reply to a `DELETE` of a malformed key states what is wrong with it (`expected_reason`)
/// and never holds the key, because a malformed key is still the user's secret.
pub async fn assert_invalid_auth_key_get_param(response: Response, invalid_auth_key: &str, expected_reason: &str) {
    assert_eq!(response.status(), 400);
    let body = response.text().await.unwrap();
    assert_eq!(body, format!("Invalid auth key id param: {expected_reason}"));
    assert!(!body.contains(invalid_auth_key), "the reply holds the key sent by the client");
}

/// The reply to a `POST` of a malformed key states what is wrong with it (`expected_reason`)
/// and never holds the key.
pub async fn assert_invalid_auth_key_post_param(response: Response, invalid_auth_key: &str, expected_reason: &str) {
    assert_eq!(response.status(), 400);
    let body = response.text().await.unwrap();
    assert_eq!(body, format!("Invalid URL: invalid auth key: {expected_reason}"));
    assert!(!body.contains(invalid_auth_key), "the reply holds the key sent by the client");
}

pub async fn assert_unprocessable_auth_key_duration_param(response: Response, _invalid_value: &str) {
    assert_unprocessable_content(
        response,
        "Failed to deserialize the JSON body into the target type: seconds_valid: invalid type",
    )
    .await;
}

/// The reply to a `POST /api/v1/key/<seconds>` whose segment is not a number of seconds. A key
/// posted to this route by mistake lands here, so the reply must not hold the rejected value.
pub async fn assert_invalid_key_duration_param(response: Response, invalid_key_duration: &str) {
    assert_eq!(response.status(), 400);
    let body = response.text().await.unwrap();
    assert_eq!(body, "Invalid URL: invalid auth key duration: expected a number of seconds");
    assert!(
        !body.contains(invalid_key_duration),
        "the reply holds the value sent by the client"
    );
}

pub async fn assert_token_not_valid(response: Response) {
    assert_unhandled_rejection(response, "token not valid").await;
}

pub async fn assert_unauthorized(response: Response) {
    assert_unhandled_rejection(response, "unauthorized").await;
}

pub async fn assert_failed_to_remove_torrent_from_whitelist(response: Response) {
    assert_unhandled_rejection(response, "failed to remove torrent from whitelist").await;
}

pub async fn assert_failed_to_whitelist_torrent(response: Response) {
    assert_unhandled_rejection(response, "failed to whitelist torrent").await;
}

pub async fn assert_failed_to_reload_whitelist(response: Response) {
    assert_unhandled_rejection(response, "failed to reload whitelist").await;
}

pub async fn assert_failed_to_generate_key(response: Response) {
    assert_unhandled_rejection(response, "failed to generate key").await;
}

pub async fn assert_failed_to_add_key(response: Response) {
    assert_unhandled_rejection(response, "failed to add key").await;
}

pub async fn assert_failed_to_delete_key(response: Response) {
    assert_unhandled_rejection(response, "failed to delete key").await;
}

pub async fn assert_failed_to_reload_keys(response: Response) {
    assert_unhandled_rejection(response, "failed to reload keys").await;
}

async fn assert_unhandled_rejection(response: Response, reason: &str) {
    assert_eq!(response.status(), 500);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/plain; charset=utf-8");

    let reason_text = format!("Unhandled rejection: Err {{ reason: \"{reason}");
    let response_text = response.text().await.unwrap();
    assert!(
        response_text.contains(&reason_text),
        ":\n  response: `\"{response_text}\"`\n  does not contain: `\"{reason_text}\"`."
    );
    assert_names_no_source_path(&response_text);
}
