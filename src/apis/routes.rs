use axum::response::Json;
use serde_json::{json, Value};

#[allow(clippy::unused_async)]
pub async fn root() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
