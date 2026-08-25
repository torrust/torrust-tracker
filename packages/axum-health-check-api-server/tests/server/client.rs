use std::sync::Once;

use reqwest::Response;

static RUSTLS_CRYPTO_PROVIDER: Once = Once::new();

pub fn install_rustls_crypto_provider() {
    RUSTLS_CRYPTO_PROVIDER.call_once(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("ring should be the Rustls crypto provider for integration tests");
    });
}

pub async fn get(path: &str) -> Response {
    install_rustls_crypto_provider();

    reqwest::Client::builder().build().unwrap().get(path).send().await.unwrap()
}
