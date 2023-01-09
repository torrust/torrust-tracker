use reqwest::Response;

pub async fn assert_token_not_valid(response: Response) {
    assert_eq!(response.status(), 500);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/plain; charset=utf-8");
    assert_eq!(
        response.text().await.unwrap(),
        "Unhandled rejection: Err { reason: \"token not valid\" }"
    );
}

pub async fn assert_unauthorized(response: Response) {
    assert_eq!(response.status(), 500);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/plain; charset=utf-8");
    assert_eq!(
        response.text().await.unwrap(),
        "Unhandled rejection: Err { reason: \"unauthorized\" }"
    );
}

pub async fn assert_torrent_not_known(response: Response) {
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    assert_eq!(response.text().await.unwrap(), "\"torrent not known\"");
}
