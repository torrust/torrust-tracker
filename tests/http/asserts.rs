use reqwest::Response;

pub async fn assert_internal_server_error(response: Response) {
    assert_eq!(response.status(), 200);
    /* cspell:disable-next-line */
    assert_eq!(response.text().await.unwrap(), "d14:failure reason21:internal server errore");
}
