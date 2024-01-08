use reqwest::Response;

pub async fn get(path: &str) -> Response {
    reqwest::Client::builder().build().unwrap().get(path).send().await.unwrap()
}
