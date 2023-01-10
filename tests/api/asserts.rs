use reqwest::Response;

pub async fn assert_torrent_not_known(response: Response) {
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.text().await.unwrap(), "\"torrent not known\"");
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

pub async fn assert_failed_to_generate_key(response: Response) {
    assert_unhandled_rejection(response, "failed to generate key").await;
}

pub async fn assert_failed_to_delete_key(response: Response) {
    assert_unhandled_rejection(response, "failed to delete key").await;
}

pub async fn assert_failed_to_reload_whitelist(response: Response) {
    assert_unhandled_rejection(response, "failed to reload whitelist").await;
}

pub async fn assert_failed_to_reload_keys(response: Response) {
    assert_unhandled_rejection(response, "failed to reload keys").await;
}

async fn assert_unhandled_rejection(response: Response, reason: &str) {
    assert_eq!(response.status(), 500);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/plain; charset=utf-8");
    assert_eq!(
        response.text().await.unwrap(),
        format!("Unhandled rejection: Err {{ reason: \"{reason}\" }}")
    );
}
