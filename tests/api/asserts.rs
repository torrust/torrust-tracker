use reqwest::Response;

pub async fn assert_token_not_valid(response: Response) {
    assert_eq!(response.status(), 500);
    assert_eq!(
        response.text().await.unwrap(),
        "Unhandled rejection: Err { reason: \"token not valid\" }"
    );
}

pub async fn assert_unauthorized(response: Response) {
    assert_eq!(response.status(), 500);
    assert_eq!(
        response.text().await.unwrap(),
        "Unhandled rejection: Err { reason: \"unauthorized\" }"
    );
}
