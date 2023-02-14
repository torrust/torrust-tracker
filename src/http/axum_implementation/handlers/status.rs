/// Temporary handler for testing and debugging the new Axum implementation
/// It should be removed once the migration to Axum is finished.
use axum::response::Json;
use axum_client_ip::{InsecureClientIp, SecureClientIp};

use crate::http::axum_implementation::resources::ok::Ok;
use crate::http::axum_implementation::responses::ok;

#[allow(clippy::unused_async)]
pub async fn get_status_handler(insecure_ip: InsecureClientIp, secure_ip: SecureClientIp) -> Json<Ok> {
    ok::response(&insecure_ip.0, &secure_ip.0)
}
